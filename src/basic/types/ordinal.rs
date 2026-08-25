//! `Ordinal` and `Codebook<D>` — the canonical card ↔ number bijection per deck.
//!
//! Design: `docs/EPIC-04_Sealed_Decks.md`, Story 1.

use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::hash::Hash;
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Canonical index of a card within its deck's vocabulary: `0..V`.
///
/// Deck-relative. `Ordinal(0)` in `Standard52` and `Ordinal(0)` in `Skat` are
/// different cards. Stable from 0.11.0: reordering a shipped deck's
/// `base_vec()` is a semver-major change (`codebook__standard52_golden`).
///
/// ```
/// use cardpack::prelude::*;
///
/// let o = Ordinal::new(7);
/// assert_eq!(o.get(), 7);
/// assert_eq!(o.index(), 7);
/// assert_eq!(o.to_string(), "7");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ordinal(u16);

impl Ordinal {
    #[must_use]
    pub const fn new(i: u16) -> Self {
        Self(i)
    }

    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl Display for Ordinal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Version byte of the canonical pile encoding.
///
/// Layout `v1`: `[0x01][u16 BE name_len][deck_name utf-8][u16 BE count][u16 BE ordinal]*`,
/// cards in iteration order. Frozen: a new layout gets a new version byte.
pub const CANON_V1: u8 = 1;

/// Deduplicate a card list in first-occurrence order.
///
/// Non-generic so a `DeckKind::all()` sweep can cover every shipped deck.
/// Pinochle lists every card twice (`src/basic/decks/pinochle.rs`); its
/// vocabulary is 24, not 48.
///
/// ```
/// use cardpack::prelude::*;
///
/// assert_eq!(vocabulary(&Pinochle::base_vec()).len(), 24);
/// assert_eq!(vocabulary(&Standard52::base_vec()).len(), 52);
/// ```
#[must_use]
pub fn vocabulary(cards: &[BasicCard]) -> Vec<BasicCard> {
    // `itertools::unique` needs the `use_std` feature; a `BTreeSet` is the
    // `alloc`-only equivalent.
    let mut seen: BTreeSet<BasicCard> = BTreeSet::new();
    cards.iter().copied().filter(|c| seen.insert(*c)).collect()
}

/// The deck's vocabulary — `base_vec()` with duplicates removed in
/// first-occurrence order — held as an indexable table. Build once, keep it.
///
/// This is the pure, `no_std` answer to "give me a total `Card ↔ 0..V`
/// bijection": a linear scan over at most 120 entries, no static cache.
///
/// ```
/// use cardpack::prelude::*;
///
/// let cb = Codebook::<Standard52>::new();
/// let ace = Card::<Standard52>::from(FrenchBasicCard::ACE_SPADES);
///
/// assert_eq!(cb.len(), 52);
/// assert_eq!(cb.ordinal(&ace), Some(Ordinal::new(0)));
/// assert_eq!(cb.card(Ordinal::new(0)), Some(ace));
/// assert_eq!(cb.ordinal(&Card::<Standard52>::default()), None);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Codebook<D: DeckedBase> {
    cards: Vec<BasicCard>,
    deck: PhantomData<D>,
}

impl<D: DeckedBase> Default for Codebook<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: DeckedBase> Codebook<D> {
    /// Builds the vocabulary from `D::base_vec()`. A vocabulary is capped at
    /// `u16::MAX` entries so every index fits an [`Ordinal`].
    #[must_use]
    pub fn new() -> Self {
        let mut cards = vocabulary(&D::base_vec());
        cards.truncate(usize::from(u16::MAX));
        Self {
            cards,
            deck: PhantomData,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// `None` for the blank card and for any card not in the deck.
    ///
    /// Ordinals number the deck's **vocabulary** — `base_vec()` with
    /// duplicates removed — not its cards. A deck that holds a card twice
    /// gives both copies one ordinal.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let cb = Standard52::codebook();
    /// assert_eq!(cb.ordinal(&Standard52::deck().cards()[0]).unwrap(), Ordinal::new(0));
    ///
    /// // A blank card is in no deck.
    /// assert_eq!(cb.ordinal(&Card::<Standard52>::default()), None);
    ///
    /// // Pinochle holds 48 cards but only 24 distinct ones. Its two A♠
    /// // share ordinal 0 — this is the case that makes `Codebook` exist.
    /// let pinochle = Pinochle::deck();
    /// let cb = Pinochle::codebook();
    ///
    /// assert_eq!(pinochle.len(), 48);
    /// assert_eq!(cb.len(), 24);
    /// assert_eq!(cb.ordinal(&pinochle.cards()[0]), cb.ordinal(&pinochle.cards()[1]));
    /// ```
    #[must_use]
    pub fn ordinal(&self, card: &Card<D>) -> Option<Ordinal> {
        let base = card.base();
        self.cards
            .iter()
            .position(|c| *c == base)
            .and_then(|i| u16::try_from(i).ok())
            .map(Ordinal)
    }

    /// `None` when `ord >= len()`. The inverse of
    /// [`ordinal`](Self::ordinal) over the vocabulary.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let cb = Standard52::codebook();
    /// let ace = cb.card(Ordinal::new(0)).unwrap();
    ///
    /// assert_eq!(ace.to_string(), "A♠");
    /// assert_eq!(cb.ordinal(&ace).unwrap(), Ordinal::new(0));
    ///
    /// // Past the end of the vocabulary.
    /// assert_eq!(cb.card(Ordinal::new(52)), None);
    /// ```
    #[must_use]
    pub fn card(&self, ord: Ordinal) -> Option<Card<D>> {
        self.cards.get(ord.index()).map(|c| Card::from(*c))
    }

    /// Every `(ordinal, card)` pair, in ordinal order.
    pub fn iter(&self) -> impl Iterator<Item = (Ordinal, Card<D>)> + '_ {
        self.cards.iter().enumerate().map(|(i, c)| {
            (
                Ordinal(u16::try_from(i).unwrap_or(u16::MAX)),
                Card::from(*c),
            )
        })
    }
}

impl<D: DeckedBase + Default + Ord + Copy + Hash> Codebook<D> {
    /// [`CANON_V1`] bytes for a pile, in iteration order.
    ///
    /// # Errors
    ///
    /// [`CardError::CardNotInDeck`] if any card is not in the vocabulary;
    /// [`CardError::CanonicalMalformed`] if the deck name or the pile is too
    /// long for a `u16` length prefix.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let cb = Codebook::<Standard52>::new();
    /// let bytes = cb.encode_pile(&Standard52::deck()).unwrap();
    ///
    /// assert_eq!(bytes[0], CANON_V1);
    /// assert_eq!(cb.decode_pile(&bytes).unwrap(), Standard52::deck());
    /// ```
    pub fn encode_pile(&self, pile: &Pile<D>) -> Result<Vec<u8>, CardError> {
        let name = D::deck_name();
        let name_len = u16::try_from(name.len())
            .map_err(|_| CardError::CanonicalMalformed("deck name too long".to_string()))?;
        let count = u16::try_from(pile.len())
            .map_err(|_| CardError::CanonicalMalformed("pile too long".to_string()))?;

        let mut out = Vec::with_capacity(1 + 2 + name.len() + 2 + pile.len() * 2);
        out.push(CANON_V1);
        out.extend_from_slice(&name_len.to_be_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&count.to_be_bytes());
        for card in pile.cards() {
            let ord = self
                .ordinal(card)
                .ok_or_else(|| CardError::CardNotInDeck(card.to_string()))?;
            out.extend_from_slice(&ord.get().to_be_bytes());
        }
        Ok(out)
    }

    /// Inverse of [`encode_pile`](Self::encode_pile). Strict: rejects a
    /// wrong version, a wrong deck name, truncation, trailing bytes, and any
    /// ordinal outside the vocabulary.
    ///
    /// # Errors
    ///
    /// [`CardError::CanonicalMalformed`] for structural problems,
    /// [`CardError::InvalidOrdinal`] for an out-of-range card.
    pub fn decode_pile(&self, bytes: &[u8]) -> Result<Pile<D>, CardError> {
        let malformed = |what: &str| CardError::CanonicalMalformed(what.to_string());

        let (&version, rest) = bytes.split_first().ok_or_else(|| malformed("truncated"))?;
        if version != CANON_V1 {
            return Err(CardError::CanonicalMalformed(format!(
                "unsupported version {version}"
            )));
        }
        let (name_len, rest) = take_u16(rest).ok_or_else(|| malformed("truncated"))?;
        let name_len = usize::from(name_len);
        if rest.len() < name_len {
            return Err(malformed("truncated"));
        }
        let (name, rest) = rest.split_at(name_len);
        let name = core::str::from_utf8(name).map_err(|_| malformed("deck name is not UTF-8"))?;
        let expected = D::deck_name();
        if name != expected {
            return Err(CardError::CanonicalMalformed(format!(
                "deck `{name}`, expected `{expected}`"
            )));
        }
        let (count, mut rest) = take_u16(rest).ok_or_else(|| malformed("truncated"))?;

        let mut cards: Vec<BasicCard> = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let (ord, tail) = take_u16(rest).ok_or_else(|| malformed("truncated"))?;
            let card = self
                .cards
                .get(usize::from(ord))
                .ok_or(CardError::InvalidOrdinal(ord))?;
            cards.push(*card);
            rest = tail;
        }
        if !rest.is_empty() {
            return Err(malformed("trailing bytes"));
        }
        Ok(Pile::from(cards))
    }
}

/// Split a big-endian `u16` off the front of a byte slice.
pub(crate) fn take_u16(bytes: &[u8]) -> Option<(u16, &[u8])> {
    let (head, tail) = bytes.split_at_checked(2)?;
    Some((u16::from_be_bytes([head[0], head[1]]), tail))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__types__ordinal_tests {
    use super::*;
    use crate::prelude::*;
    use alloc::collections::BTreeSet;
    use alloc::string::ToString;
    use alloc::vec::Vec;

    #[test]
    fn ordinal__new_get_index_display() {
        let o = Ordinal::new(7);
        assert_eq!(o.get(), 7);
        assert_eq!(o.index(), 7_usize);
        assert_eq!(o.to_string(), "7");
        assert!(Ordinal::new(1) < Ordinal::new(2));
        assert_eq!(Ordinal::default(), Ordinal::new(0));
    }

    #[test]
    fn vocabulary__dedups_first_occurrence() {
        let v = vocabulary(&Pinochle::base_vec());
        assert_eq!(v.len(), 24, "Pinochle lists every card twice");
        assert_eq!(v[0], FrenchBasicCard::ACE_SPADES);
        assert_eq!(
            v[1],
            PinochleBasicCard::TEN_SPADES,
            "first-occurrence order is kept"
        );
        let set: BTreeSet<BasicCard> = v.iter().copied().collect();
        assert_eq!(set.len(), v.len(), "no duplicates survive");
    }

    macro_rules! roundtrip {
        ($($t:ty),* $(,)?) => {$(
            {
                let cb = Codebook::<$t>::new();
                assert_eq!(cb.len(), <$t>::deck().unique_cards().len(), stringify!($t));
                for (ord, card) in cb.iter() {
                    assert_eq!(cb.ordinal(&card), Some(ord), stringify!($t));
                    assert_eq!(cb.card(ord), Some(card), stringify!($t));
                }
                for card in <$t>::deckvec() {
                    assert!(cb.ordinal(&card).is_some(), stringify!($t));
                }
            }
        )*};
    }

    #[test]
    fn codebook__roundtrip_every_shipped_deck() {
        roundtrip!(
            Canasta,
            Dashavatara,
            Euchre24,
            Euchre32,
            French,
            Mughal,
            Pinochle,
            Short,
            Skat,
            Spades,
            Standard52,
            Tarot,
            Tiny,
        );
        #[cfg(feature = "yaml")]
        roundtrip!(Razz);

        // Registry sweep, so deck 15 cannot dodge the guarantee: the vocabulary
        // of every kind is duplicate-free.
        for kind in DeckKind::all() {
            let v = vocabulary(&kind.base_vec());
            let set: BTreeSet<BasicCard> = v.iter().copied().collect();
            assert_eq!(v.len(), set.len(), "{}", kind.deck_name());
            assert!(!v.is_empty(), "{}", kind.deck_name());
        }
    }

    #[test]
    fn codebook__blank_has_no_ordinal() {
        let cb = Codebook::<French>::new();
        assert_eq!(cb.ordinal(&Card::<French>::default()), None);
        let past_end = u16::try_from(cb.len()).unwrap();
        assert_eq!(cb.card(Ordinal::new(past_end)), None);
        assert_eq!(cb.card(Ordinal::new(u16::MAX)), None);
    }

    /// From 0.11.0 this order is a contract: reordering `Standard52::DECK` is
    /// a semver-major change, and this is the test that says so.
    #[test]
    fn codebook__standard52_golden() {
        let cb = Codebook::<Standard52>::new();
        let s = cb
            .iter()
            .map(|(_, c)| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            s,
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ \
             A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ \
             A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ \
             A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
        assert_eq!(
            cb.ordinal(&Card::from(FrenchBasicCard::ACE_SPADES)),
            Some(Ordinal::new(0))
        );
        assert_eq!(
            cb.ordinal(&Card::from(FrenchBasicCard::DEUCE_CLUBS)),
            Some(Ordinal::new(51))
        );
    }

    #[test]
    fn codebook__foreign_card_has_no_ordinal() {
        // A Tarot trump is not in the French vocabulary.
        let cb = Codebook::<French>::new();
        let foreign = Card::<French>::from(Tarot::base_vec()[0]);
        assert_eq!(cb.ordinal(&foreign), None);
    }

    // ---- Story 2: canonical bytes -------------------------------------

    fn standard52_piles() -> Vec<Pile<Standard52>> {
        let mut deck = Standard52::deck();
        let hand = deck.draw(5).unwrap();
        vec![
            Standard52::deck(),
            Standard52::deck().shuffled_with_seed(7),
            hand,
            Pile::<Standard52>::default(),
        ]
    }

    #[test]
    fn canonical__roundtrip() {
        let cb = Codebook::<Standard52>::new();
        for pile in standard52_piles() {
            let bytes = cb.encode_pile(&pile).unwrap();
            assert_eq!(cb.decode_pile(&bytes).unwrap(), pile);
        }
    }

    #[test]
    fn canonical__multideck_roundtrips() {
        let cb = Codebook::<French>::new();
        let pile = French::decks(4).shuffled_with_seed(1);
        let bytes = cb.encode_pile(&pile).unwrap();
        assert_eq!(cb.decode_pile(&bytes).unwrap(), pile);
    }

    /// `[0x01][u16 BE name_len][deck_name][u16 BE count][u16 BE ordinal]*`
    #[test]
    fn canonical__golden_standard52_prefix() {
        let cb = Codebook::<Standard52>::new();
        let bytes = cb.encode_pile(&Standard52::deck()).unwrap();
        let mut expected = vec![CANON_V1, 0x00, 0x0B];
        expected.extend_from_slice(b"Standard 52");
        expected.extend_from_slice(&[0x00, 0x34, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02]);
        assert_eq!(&bytes[..expected.len()], &expected[..]);
        assert_eq!(bytes.len(), 1 + 2 + 11 + 2 + 52 * 2);
        assert_eq!(&bytes[bytes.len() - 2..], &[0x00, 0x33]);
    }

    #[test]
    fn canonical__bad_version() {
        let cb = Codebook::<Standard52>::new();
        let mut bytes = cb.encode_pile(&Standard52::deck()).unwrap();
        bytes[0] = 2;
        assert_eq!(
            cb.decode_pile(&bytes),
            Err(CardError::CanonicalMalformed(
                "unsupported version 2".to_string()
            ))
        );
    }

    #[test]
    fn canonical__truncated() {
        let cb = Codebook::<Standard52>::new();
        let bytes = cb.encode_pile(&Standard52::deck()).unwrap();
        assert!(matches!(
            cb.decode_pile(&bytes[..bytes.len() - 1]),
            Err(CardError::CanonicalMalformed(_))
        ));
        assert!(matches!(
            cb.decode_pile(&[]),
            Err(CardError::CanonicalMalformed(_))
        ));
        assert!(matches!(
            cb.decode_pile(&bytes[..3]),
            Err(CardError::CanonicalMalformed(_))
        ));
    }

    #[test]
    fn canonical__trailing_bytes() {
        let cb = Codebook::<Standard52>::new();
        let mut bytes = cb.encode_pile(&Standard52::deck()).unwrap();
        bytes.push(0);
        assert_eq!(
            cb.decode_pile(&bytes),
            Err(CardError::CanonicalMalformed("trailing bytes".to_string()))
        );
    }

    #[test]
    fn canonical__wrong_deck_name() {
        let bytes = Codebook::<Standard52>::new()
            .encode_pile(&Standard52::deck())
            .unwrap();
        assert_eq!(
            Codebook::<Skat>::new().decode_pile(&bytes),
            Err(CardError::CanonicalMalformed(
                "deck `Standard 52`, expected `Skat`".to_string()
            ))
        );
    }

    #[test]
    fn canonical__foreign_card_errors() {
        let pile = Pile::<French>::from(vec![Tarot::base_vec()[0]]);
        assert!(matches!(
            Codebook::<French>::new().encode_pile(&pile),
            Err(CardError::CardNotInDeck(_))
        ));
    }

    #[test]
    fn canonical__ordinal_out_of_range_errors() {
        let cb = Codebook::<Standard52>::new();
        let mut bytes = vec![CANON_V1, 0x00, 0x0B];
        bytes.extend_from_slice(b"Standard 52");
        bytes.extend_from_slice(&[0x00, 0x01, 0x00, 0x34]); // one card, ordinal 52
        assert_eq!(cb.decode_pile(&bytes), Err(CardError::InvalidOrdinal(52)));
    }

    #[test]
    fn decked__codebook_default_method() {
        assert_eq!(French::codebook(), Codebook::<French>::new());
        assert_eq!(French::codebook().len(), 54);
    }
}
