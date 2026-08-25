//! Who is in the round, the seed they agree on, and (Story 3) the round.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use sha2::{Digest, Sha256};

use crate::basic::types::permutation::Permutation;
use crate::common::errors::CardError;
use crate::seal::commit::commitment::{Commitment, Contribution};
use crate::seal::commit::derive;
use crate::seal::commit::hex;

/// Domain-separation tag for [`CombinedSeed::combine`]. Part of the frozen
/// `v1` format.
pub const TAG_SEED: &[u8] = b"cardpack/commit-reveal/v1/seed";

/// A participant's label within one round. Part of the seed preimage, so
/// relabelling participants changes the seed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParticipantId(pub u16);

impl fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The seed every honest verifier reaches from the public transcript.
///
/// `Debug` and `Display` print lowercase hex.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct CombinedSeed([u8; 32]);

impl CombinedSeed {
    /// `SHA-256(TAG_SEED || u16 BE n || (u16 BE id || 32 bytes)*)`, with the
    /// pairs sorted by [`ParticipantId`] first. Input order is irrelevant;
    /// the ids are not.
    ///
    /// `n` is truncated to `u16::MAX` participants — a round that large is
    /// not a card game.
    #[must_use]
    pub fn combine(parts: &[(ParticipantId, Contribution)]) -> Self {
        let mut sorted: Vec<&(ParticipantId, Contribution)> = parts.iter().collect();
        sorted.sort_by_key(|(id, _)| *id);

        let n = u16::try_from(sorted.len()).unwrap_or(u16::MAX);
        let mut h = Sha256::new();
        h.update(TAG_SEED);
        h.update(n.to_be_bytes());
        for (id, c) in sorted {
            h.update(id.0.to_be_bytes());
            h.update(c.as_bytes());
        }
        Self(h.finalize().into())
    }

    /// The shuffle this seed determines, over `n` positions.
    ///
    /// Fisher–Yates over the identity, drawing from the SHA-256 counter-mode
    /// stream with exact rejection sampling — see [`derive`](mod@crate::seal::commit::derive) for the frozen
    /// algorithm. This is the verifier's contract: it never changes with a
    /// `rand` upgrade.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidPermutation`] if `n > u16::MAX`.
    pub fn permutation(&self, n: usize) -> Result<Permutation, CardError> {
        derive::permutation(&self.0, n)
    }

    /// The first 8 bytes, big-endian — a convenience for
    /// `Pile::shuffle_with_seed`.
    ///
    /// **Not verifier-stable.** `shuffle_with_seed` uses `StdRng`, whose
    /// output may change across `rand` major versions. Use
    /// [`permutation`](Self::permutation) when anyone must reproduce the
    /// shuffle.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let seed = CombinedSeed::combine(&[
    ///     (ParticipantId(1), Contribution::from_bytes([0x11; 32])),
    /// ]);
    /// let deck = Standard52::deck();
    ///
    /// // Convenient — but only reproducible within one `rand` major version.
    /// let quick = deck.shuffled_with_seed(seed.to_u64());
    ///
    /// // Reproducible by anyone, in any language, forever. This is the
    /// // verifier'"'"'s path, and the one to reach for.
    /// let checked = deck.permute(&seed.permutation(52)?)?;
    ///
    /// assert!(deck.same(&quick));
    /// assert!(deck.same(&checked));
    /// # Ok::<(), CardError>(())
    /// ```
    #[must_use]
    pub fn to_u64(&self) -> u64 {
        let mut b = [0u8; 8];
        b.copy_from_slice(&self.0[..8]);
        u64::from_be_bytes(b)
    }

    /// The 32 seed bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex, 64 characters.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl fmt::Debug for CombinedSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CombinedSeed({})", self.to_hex())
    }
}

impl fmt::Display for CombinedSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

/// One commit–reveal round: commit-all, then reveal-all.
///
/// **Phase A**, every participant commits; **phase B**, every participant
/// reveals. No reveal is accepted before every commitment is in, so the last
/// participant to reveal learns nothing they can use.
///
/// This is a pure state machine. Transport, signatures, and liveness are the
/// caller's. A participant who commits and then never reveals **aborts** the
/// round; they cannot bias it — the seed needs every contribution.
///
/// `BTreeMap`, not `HashMap`: deterministic, no hasher, `no_std`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShuffleRound {
    participants: Vec<ParticipantId>,
    commitments: BTreeMap<ParticipantId, Commitment>,
    reveals: BTreeMap<ParticipantId, Contribution>,
}

impl ShuffleRound {
    /// A round over these participants, in this order.
    ///
    /// # Errors
    ///
    /// [`CardError::NoParticipants`] if empty;
    /// [`CardError::DuplicateParticipant`] on a repeated id.
    pub fn new(participants: impl IntoIterator<Item = ParticipantId>) -> Result<Self, CardError> {
        let participants: Vec<ParticipantId> = participants.into_iter().collect();
        if participants.is_empty() {
            return Err(CardError::NoParticipants);
        }
        let mut seen = alloc::collections::BTreeSet::new();
        for id in &participants {
            if !seen.insert(*id) {
                return Err(CardError::DuplicateParticipant(id.0));
            }
        }
        Ok(Self {
            participants,
            commitments: BTreeMap::new(),
            reveals: BTreeMap::new(),
        })
    }

    /// The participants, in the order given to [`new`](Self::new).
    #[must_use]
    pub fn participants(&self) -> &[ParticipantId] {
        &self.participants
    }

    /// Phase A. First commitment per participant wins.
    ///
    /// # Errors
    ///
    /// [`CardError::UnknownParticipant`], [`CardError::AlreadyCommitted`].
    pub fn commit(&mut self, who: ParticipantId, c: Commitment) -> Result<(), CardError> {
        self.check_known(who)?;
        if self.commitments.contains_key(&who) {
            return Err(CardError::AlreadyCommitted(who.0));
        }
        self.commitments.insert(who, c);
        Ok(())
    }

    /// `true` once every participant has committed.
    #[must_use]
    pub fn all_committed(&self) -> bool {
        self.commitments.len() == self.participants.len()
    }

    /// Phase B. Rejected until [`all_committed`](Self::all_committed); a
    /// contribution that does not open the participant's commitment is
    /// rejected and the round is left unchanged.
    ///
    /// # Errors
    ///
    /// [`CardError::UnknownParticipant`],
    /// [`CardError::RevealBeforeAllCommitted`],
    /// [`CardError::CommitmentMismatch`].
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let (a, b) = (ParticipantId(1), ParticipantId(2));
    /// let sa = Contribution::from_bytes([0x11; 32]); // Contribution::random in real code
    /// let sb = Contribution::from_bytes([0x22; 32]);
    ///
    /// let mut round = ShuffleRound::new([a, b])?;
    /// round.commit(a, sa.commit())?;
    ///
    /// // Nobody may reveal while a commitment is still outstanding. That
    /// // rule is the whole reason the last revealer cannot bias the seed.
    /// assert_eq!(round.reveal(a, sa), Err(CardError::RevealBeforeAllCommitted));
    ///
    /// round.commit(b, sb.commit())?;
    /// round.reveal(a, sa)?;
    ///
    /// // A contribution that does not open its own commitment is rejected,
    /// // and the round is left exactly as it was.
    /// assert_eq!(round.reveal(b, sa), Err(CardError::CommitmentMismatch(2)));
    /// assert!(!round.is_complete());
    ///
    /// round.reveal(b, sb)?;
    /// assert!(round.is_complete());
    /// # Ok::<(), CardError>(())
    /// ```
    pub fn reveal(&mut self, who: ParticipantId, c: Contribution) -> Result<(), CardError> {
        self.check_known(who)?;
        if !self.all_committed() {
            return Err(CardError::RevealBeforeAllCommitted);
        }
        let committed = self
            .commitments
            .get(&who)
            .ok_or(CardError::UnknownParticipant(who.0))?;
        if !committed.verify(&c) {
            return Err(CardError::CommitmentMismatch(who.0));
        }
        self.reveals.insert(who, c);
        Ok(())
    }

    /// `true` once every participant has revealed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.reveals.len() == self.participants.len()
    }

    /// The commitment `who` made, if any. Public transcript material.
    #[must_use]
    pub fn commitment(&self, who: ParticipantId) -> Option<Commitment> {
        self.commitments.get(&who).copied()
    }

    /// The contribution `who` revealed, if any. Public transcript material
    /// once revealed.
    #[must_use]
    pub fn contribution(&self, who: ParticipantId) -> Option<Contribution> {
        self.reveals.get(&who).copied()
    }

    /// [`CombinedSeed::combine`] over every revealed contribution.
    ///
    /// # Errors
    ///
    /// [`CardError::RoundIncomplete`] until every participant has revealed.
    pub fn seed(&self) -> Result<CombinedSeed, CardError> {
        if !self.is_complete() {
            return Err(CardError::RoundIncomplete);
        }
        let parts: Vec<(ParticipantId, Contribution)> =
            self.reveals.iter().map(|(id, c)| (*id, *c)).collect();
        Ok(CombinedSeed::combine(&parts))
    }

    fn check_known(&self, who: ParticipantId) -> Result<(), CardError> {
        if self.participants.contains(&who) {
            Ok(())
        } else {
            Err(CardError::UnknownParticipant(who.0))
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__commit__seed_tests {
    use super::*;
    use alloc::format;

    const A: Contribution = Contribution::from_bytes([0x11; 32]);
    const B: Contribution = Contribution::from_bytes([0x22; 32]);
    /// Python: `combine([(2, b"\x22"*32), (1, b"\x11"*32)]).hex()` — see
    /// the reference script recorded in `tests/commit_reveal.rs`.
    const GOLDEN_SEED: &str = "600d8d3d6e4f300530a2ebd4301b32f1afc512237d98947703260d7577287f78";

    fn golden() -> CombinedSeed {
        CombinedSeed::combine(&[(ParticipantId(1), A), (ParticipantId(2), B)])
    }

    #[test]
    fn combine__golden_vector() {
        assert_eq!(golden().to_hex(), GOLDEN_SEED);
    }

    #[test]
    fn combine__order_of_input_is_irrelevant() {
        let swapped = CombinedSeed::combine(&[(ParticipantId(2), B), (ParticipantId(1), A)]);
        assert_eq!(swapped, golden());
    }

    #[test]
    fn combine__id_is_part_of_preimage() {
        let relabelled = CombinedSeed::combine(&[(ParticipantId(1), B), (ParticipantId(2), A)]);
        assert_ne!(relabelled, golden());
    }

    #[test]
    fn combine__count_is_part_of_preimage() {
        let one = CombinedSeed::combine(&[(ParticipantId(1), A)]);
        assert_ne!(one, golden());
    }

    #[test]
    fn seed__to_u64_is_first_eight_bytes_big_endian() {
        assert_eq!(golden().to_u64(), 6_921_343_497_321_525_253);
    }

    #[test]
    fn seed__debug_and_display_are_hex() {
        assert_eq!(
            format!("{:?}", golden()),
            format!("CombinedSeed({GOLDEN_SEED})")
        );
        assert_eq!(format!("{}", golden()), GOLDEN_SEED);
    }

    #[test]
    fn seed__as_bytes_is_32() {
        assert_eq!(golden().as_bytes().len(), 32);
    }

    #[test]
    fn participant_id__display() {
        assert_eq!(format!("{}", ParticipantId(7)), "7");
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__commit__round_tests {
    use super::*;
    use alloc::vec;

    const A: Contribution = Contribution::from_bytes([0x11; 32]);
    const B: Contribution = Contribution::from_bytes([0x22; 32]);
    const P1: ParticipantId = ParticipantId(1);
    const P2: ParticipantId = ParticipantId(2);

    fn two_party() -> ShuffleRound {
        ShuffleRound::new([P1, P2]).unwrap()
    }

    #[test]
    fn round__new_rejects_empty() {
        assert_eq!(
            ShuffleRound::new(Vec::new()).unwrap_err(),
            CardError::NoParticipants
        );
    }

    #[test]
    fn round__new_rejects_duplicates() {
        assert_eq!(
            ShuffleRound::new([P1, P2, P1]).unwrap_err(),
            CardError::DuplicateParticipant(1)
        );
    }

    #[test]
    fn round__participants_keeps_given_order() {
        let r = ShuffleRound::new([P2, P1]).unwrap();
        assert_eq!(r.participants(), &[P2, P1]);
    }

    #[test]
    fn round__commit_unknown_participant_errors() {
        let mut r = two_party();
        assert_eq!(
            r.commit(ParticipantId(9), A.commit()).unwrap_err(),
            CardError::UnknownParticipant(9)
        );
    }

    #[test]
    fn round__double_commit_errors() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        assert_eq!(
            r.commit(P1, B.commit()).unwrap_err(),
            CardError::AlreadyCommitted(1)
        );
        // The first commitment stands.
        assert_eq!(r.commitment(P1), Some(A.commit()));
    }

    #[test]
    fn round__all_committed_flips_when_everyone_is_in() {
        let mut r = two_party();
        assert!(!r.all_committed());
        r.commit(P1, A.commit()).unwrap();
        assert!(!r.all_committed());
        r.commit(P2, B.commit()).unwrap();
        assert!(r.all_committed());
    }

    #[test]
    fn round__reveal_before_all_committed_errors() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        assert_eq!(
            r.reveal(P1, A).unwrap_err(),
            CardError::RevealBeforeAllCommitted
        );
        assert_eq!(r.contribution(P1), None);
    }

    #[test]
    fn round__reveal_unknown_participant_errors() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        r.commit(P2, B.commit()).unwrap();
        assert_eq!(
            r.reveal(ParticipantId(9), A).unwrap_err(),
            CardError::UnknownParticipant(9)
        );
    }

    #[test]
    fn round__mismatched_reveal_errors_and_leaves_round_unchanged() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        r.commit(P2, B.commit()).unwrap();
        let before = r.clone();
        assert_eq!(
            r.reveal(P1, B).unwrap_err(),
            CardError::CommitmentMismatch(1)
        );
        assert_eq!(r, before);
        assert!(!r.is_complete());
    }

    #[test]
    fn round__seed_before_complete_errors() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        r.commit(P2, B.commit()).unwrap();
        r.reveal(P1, A).unwrap();
        assert_eq!(r.seed().unwrap_err(), CardError::RoundIncomplete);
    }

    #[test]
    fn round__two_party_provably_fair() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        r.commit(P2, B.commit()).unwrap();
        r.reveal(P2, B).unwrap();
        r.reveal(P1, A).unwrap();
        assert!(r.is_complete());
        assert_eq!(
            r.seed().unwrap(),
            CombinedSeed::combine(&[(P1, A), (P2, B)])
        );
    }

    #[test]
    fn round__any_verifier_reproduces_seed() {
        // Dealer's side.
        let mut dealer = two_party();
        dealer.commit(P1, A.commit()).unwrap();
        dealer.commit(P2, B.commit()).unwrap();
        dealer.reveal(P1, A).unwrap();
        dealer.reveal(P2, B).unwrap();

        // Verifier rebuilds from the public transcript only.
        let transcript: Vec<(ParticipantId, Commitment, Contribution)> = dealer
            .participants()
            .iter()
            .map(|&id| {
                (
                    id,
                    dealer.commitment(id).unwrap(),
                    dealer.contribution(id).unwrap(),
                )
            })
            .collect();
        let mut verifier = ShuffleRound::new(transcript.iter().map(|t| t.0)).unwrap();
        for (id, c, _) in &transcript {
            verifier.commit(*id, *c).unwrap();
        }
        for (id, _, x) in &transcript {
            verifier.reveal(*id, *x).unwrap();
        }
        assert_eq!(verifier.seed().unwrap(), dealer.seed().unwrap());
        assert_eq!(
            verifier.seed().unwrap().permutation(52).unwrap(),
            dealer.seed().unwrap().permutation(52).unwrap()
        );
    }

    #[test]
    fn round__reorder_of_participants_changes_seed() {
        let mut r = two_party();
        r.commit(P1, A.commit()).unwrap();
        r.commit(P2, B.commit()).unwrap();
        r.reveal(P1, A).unwrap();
        r.reveal(P2, B).unwrap();

        let mut swapped = two_party();
        swapped.commit(P1, B.commit()).unwrap();
        swapped.commit(P2, A.commit()).unwrap();
        swapped.reveal(P1, B).unwrap();
        swapped.reveal(P2, A).unwrap();

        assert_ne!(r.seed().unwrap(), swapped.seed().unwrap());
    }

    #[test]
    fn round__single_participant_is_allowed() {
        let mut r = ShuffleRound::new(vec![P1]).unwrap();
        r.commit(P1, A.commit()).unwrap();
        r.reveal(P1, A).unwrap();
        assert!(r.is_complete());
    }
}
