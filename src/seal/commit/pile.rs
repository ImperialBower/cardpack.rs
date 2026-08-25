//! Blind commitments to a concrete order, and the pile-side entry point.
//!
//! A dealer publishes `commit_pile(...)` before dealing and opens it after.
//! The `blind` is a fresh [`Contribution`]: without it a commitment to a
//! two-card pile has only `52 · 51` preimages and is brute-forceable. With it,
//! every commitment is hiding regardless of pile size.

use core::hash::Hash;

use sha2::{Digest, Sha256};

use crate::basic::types::ordinal::Codebook;
use crate::basic::types::permutation::Permutation;
use crate::basic::types::pile::Pile;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use crate::seal::commit::commitment::{Commitment, Contribution};
use crate::seal::commit::round::ShuffleRound;

/// Domain-separation tag for [`commit_permutation`]. Frozen `v1`.
pub const TAG_PERMUTATION: &[u8] = b"cardpack/commit-reveal/v1/permutation";
/// Domain-separation tag for [`commit_pile`]. Frozen `v1`.
pub const TAG_PILE: &[u8] = b"cardpack/commit-reveal/v1/pile";

fn digest(tag: &[u8], blind: &Contribution, body: &[u8]) -> Commitment {
    let mut h = Sha256::new();
    h.update(tag);
    h.update(blind.as_bytes());
    h.update(body);
    Commitment::from_bytes(h.finalize().into())
}

/// `SHA-256(TAG_PERMUTATION || blind || p.canonical_bytes())`.
#[must_use]
pub fn commit_permutation(p: &Permutation, blind: &Contribution) -> Commitment {
    digest(TAG_PERMUTATION, blind, &p.canonical_bytes())
}

/// Recompute [`commit_permutation`] and compare.
#[must_use]
pub fn verify_permutation(c: &Commitment, p: &Permutation, blind: &Contribution) -> bool {
    commit_permutation(p, blind) == *c
}

/// `SHA-256(TAG_PILE || blind || codebook.encode_pile(pile)?)` — a
/// commitment to this exact order of these exact cards.
///
/// # Errors
///
/// Whatever [`Codebook::encode_pile`] returns: [`CardError::CardNotInDeck`]
/// for a card outside the vocabulary, [`CardError::CanonicalMalformed`] for
/// an oversized pile.
pub fn commit_pile<D>(
    codebook: &Codebook<D>,
    pile: &Pile<D>,
    blind: &Contribution,
) -> Result<Commitment, CardError>
where
    D: DeckedBase + Default + Ord + Copy + Hash,
{
    Ok(digest(TAG_PILE, blind, &codebook.encode_pile(pile)?))
}

/// Recompute [`commit_pile`] and compare.
///
/// # Errors
///
/// As [`commit_pile`].
pub fn verify_pile<D>(
    c: &Commitment,
    codebook: &Codebook<D>,
    pile: &Pile<D>,
    blind: &Contribution,
) -> Result<bool, CardError>
where
    D: DeckedBase + Default + Ord + Copy + Hash,
{
    Ok(commit_pile(codebook, pile, blind)? == *c)
}

impl<D: DeckedBase + Default + Ord + Copy + Hash> Pile<D> {
    /// `self.permute(&round.seed()?.permutation(self.len())?)` — the shuffle
    /// every participant in `round` agreed on, reproducible by any verifier.
    ///
    /// # Errors
    ///
    /// [`CardError::RoundIncomplete`] until every participant has revealed;
    /// [`CardError::InvalidPermutation`] for a pile longer than `u16::MAX`.
    pub fn shuffled_by_round(&self, round: &ShuffleRound) -> Result<Self, CardError> {
        let p = round.seed()?.permutation(self.len())?;
        self.permute(&p)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__commit__pile_tests {
    use super::*;
    use crate::basic::decks::standard52::Standard52;
    use crate::basic::types::card::Card;
    use crate::basic::types::traits::Decked;
    use crate::seal::commit::{Contribution, ParticipantId, ShuffleRound};

    const BLIND: Contribution = Contribution::from_bytes([0x11; 32]);
    const OTHER: Contribution = Contribution::from_bytes([0x12; 32]);

    fn complete_round() -> ShuffleRound {
        let a = Contribution::from_bytes([0x11; 32]);
        let b = Contribution::from_bytes([0x22; 32]);
        let mut r = ShuffleRound::new([ParticipantId(1), ParticipantId(2)]).unwrap();
        r.commit(ParticipantId(1), a.commit()).unwrap();
        r.commit(ParticipantId(2), b.commit()).unwrap();
        r.reveal(ParticipantId(1), a).unwrap();
        r.reveal(ParticipantId(2), b).unwrap();
        r
    }

    #[test]
    fn commit_permutation__golden_vector() {
        // Python: commit_permutation([0,1,2], b"\x11"*32).hex()
        let p = Permutation::identity(3).unwrap();
        assert_eq!(
            commit_permutation(&p, &BLIND).to_hex(),
            "15ecd3ef7ec6ff499db58b7289582c1d2fd8d899517e7f02987757892d4c068c"
        );
    }

    #[test]
    fn commit_permutation__roundtrip() {
        let p = Permutation::from_seed(52, 9).unwrap();
        let c = commit_permutation(&p, &BLIND);
        assert!(verify_permutation(&c, &p, &BLIND));
    }

    #[test]
    fn commit_permutation__wrong_blind_fails() {
        let p = Permutation::from_seed(52, 9).unwrap();
        let c = commit_permutation(&p, &BLIND);
        assert!(!verify_permutation(&c, &p, &OTHER));
    }

    #[test]
    fn commit_permutation__other_permutation_fails() {
        let p = Permutation::from_seed(52, 9).unwrap();
        let q = Permutation::from_seed(52, 10).unwrap();
        let c = commit_permutation(&p, &BLIND);
        assert!(!verify_permutation(&c, &q, &BLIND));
    }

    #[test]
    fn commit_pile__golden_vector() {
        // Python: commit_pile(CANON_V1 bytes of the sorted Standard 52 deck, b"\x11"*32).hex()
        let cb = Standard52::codebook();
        let c = commit_pile(&cb, &Standard52::deck(), &BLIND).unwrap();
        assert_eq!(
            c.to_hex(),
            "45cff5b043a0c8502daa305188faebe1706886020008ce415c61cb18cf28c482"
        );
    }

    #[test]
    fn commit_pile__verifies_exact_order() {
        let cb = Standard52::codebook();
        let deck = Standard52::deck().shuffled_with_seed(4);
        let c = commit_pile(&cb, &deck, &BLIND).unwrap();
        assert!(verify_pile(&c, &cb, &deck, &BLIND).unwrap());
    }

    #[test]
    fn commit_pile__detects_swapped_cards() {
        let cb = Standard52::codebook();
        let deck = Standard52::deck();
        let c = commit_pile(&cb, &deck, &BLIND).unwrap();
        let mut swapped = deck.cards().clone();
        swapped.swap(0, 1);
        let swapped = Pile::<Standard52>::from(swapped);
        assert!(!verify_pile(&c, &cb, &swapped, &BLIND).unwrap());
    }

    #[test]
    fn commit_pile__wrong_blind_fails() {
        let cb = Standard52::codebook();
        let deck = Standard52::deck();
        let c = commit_pile(&cb, &deck, &BLIND).unwrap();
        assert!(!verify_pile(&c, &cb, &deck, &OTHER).unwrap());
    }

    #[test]
    fn commit_pile__foreign_card_errors() {
        let cb = Standard52::codebook();
        let pile = Pile::<Standard52>::from(alloc::vec![Card::<Standard52>::default()]);
        assert!(matches!(
            commit_pile(&cb, &pile, &BLIND),
            Err(CardError::CardNotInDeck(_))
        ));
    }

    #[test]
    fn shuffled_by_round__preserves_multiset() {
        let deck = Standard52::deck();
        let shuffled = deck.shuffled_by_round(&complete_round()).unwrap();
        assert!(deck.same(&shuffled));
        assert_ne!(deck, shuffled);
    }

    #[test]
    fn shuffled_by_round__equals_permute_by_derived_permutation() {
        let deck = Standard52::deck();
        let round = complete_round();
        let p = round.seed().unwrap().permutation(52).unwrap();
        assert_eq!(
            deck.shuffled_by_round(&round).unwrap(),
            deck.permute(&p).unwrap()
        );
    }

    #[test]
    fn shuffled_by_round__incomplete_round_errors() {
        let mut r = ShuffleRound::new([ParticipantId(1)]).unwrap();
        r.commit(ParticipantId(1), BLIND.commit()).unwrap();
        assert_eq!(
            Standard52::deck().shuffled_by_round(&r).unwrap_err(),
            CardError::RoundIncomplete
        );
    }
}
