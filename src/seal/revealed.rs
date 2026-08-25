//! `Revealed<D>` — the only place a card value can be.

use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use crate::seal::adapter::Seal;
use crate::seal::slot::SlotId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::hash::Hash;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Slot → card, for slots whose value has been turned up.
///
/// The **only** kernel type that maps a name to a value. If it is empty, no
/// card value exists anywhere in the game state. Two doors in:
///
/// * [`reveal`](Self::reveal) — the caller vouches for `(slot, card)`. For
///   protocols that verify before cardpack ever sees a value.
/// * [`reveal_with`](Self::reveal_with) — the backend's ciphertext and token
///   are checked by a [`Seal`] first. Generic at the method only; this type
///   knows nothing about the scheme.
///
/// Bounds match `Pile<D>` so every derive is free.
///
/// ```
/// use cardpack::prelude::*;
///
/// let mut seen = Revealed::<Standard52>::new();
/// seen.reveal(SlotId::new(17), Card::from(FrenchBasicCard::ACE_SPADES)).unwrap();
///
/// assert_eq!(seen.get(SlotId::new(17)).unwrap().to_string(), "A♠");
/// assert!(seen.reveal(SlotId::new(17), Card::from(FrenchBasicCard::KING_HEARTS)).is_err());
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// `D` only brands `Card<D>`; it need not itself be serializable.
#[cfg_attr(feature = "serde", serde(bound = ""))]
pub struct Revealed<D: DeckedBase + Default + Ord + Copy + Hash>(BTreeMap<SlotId, Card<D>>);

/// Why a verified reveal failed: the slot was wrong, or the backend refused.
#[derive(Debug, Eq, PartialEq)]
pub enum SealError<E> {
    Slot(CardError),
    Backend(E),
}

impl<E: Display> Display for SealError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Slot(e) => write!(f, "{e}"),
            Self::Backend(e) => write!(f, "seal backend: {e}"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for SealError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Slot(e) => Some(e),
            Self::Backend(e) => Some(e),
        }
    }
}

impl<D: DeckedBase + Default + Ord + Copy + Hash> Revealed<D> {
    #[must_use]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn get(&self, slot: SlotId) -> Option<Card<D>> {
        self.0.get(&slot).copied()
    }

    #[must_use]
    pub fn is_revealed(&self, slot: SlotId) -> bool {
        self.0.contains_key(&slot)
    }

    /// Every `(slot, card)` pair, in slot order.
    pub fn iter(&self) -> impl Iterator<Item = (SlotId, Card<D>)> + '_ {
        self.0.iter().map(|(s, c)| (*s, *c))
    }

    /// The revealed cards for `slots`, in that order.
    ///
    /// # Errors
    ///
    /// [`CardError::SlotNotFound`] for the first slot that is not revealed.
    pub fn pile_for(&self, slots: &[SlotId]) -> Result<Pile<D>, CardError> {
        let cards = slots
            .iter()
            .map(|s| self.get(*s).ok_or_else(|| CardError::SlotNotFound(s.get())))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Pile::from(
            cards.into_iter().map(|c| c.base()).collect::<Vec<_>>(),
        ))
    }

    /// An unverified reveal: the caller vouches for `(slot, card)`.
    ///
    /// # Errors
    ///
    /// [`CardError::SlotAlreadyRevealed`] — a value is never silently replaced.
    pub fn reveal(&mut self, slot: SlotId, card: Card<D>) -> Result<(), CardError> {
        if self.0.contains_key(&slot) {
            return Err(CardError::SlotAlreadyRevealed(slot.get()));
        }
        self.0.insert(slot, card);
        Ok(())
    }

    /// A verified reveal: `scheme` checks `sealed` against `token` before the
    /// value is admitted. On any error nothing is admitted.
    ///
    /// # Errors
    ///
    /// [`SealError::Slot`] if the slot is already revealed;
    /// [`SealError::Backend`] if the scheme refuses.
    pub fn reveal_with<S: Seal<D>>(
        &mut self,
        slot: SlotId,
        sealed: &S::Sealed,
        scheme: &S,
        token: &S::Token,
    ) -> Result<Card<D>, SealError<S::Error>> {
        if self.0.contains_key(&slot) {
            return Err(SealError::Slot(CardError::SlotAlreadyRevealed(slot.get())));
        }
        let card = scheme
            .unseal(sealed, slot, token)
            .map_err(SealError::Backend)?;
        self.0.insert(slot, card);
        Ok(card)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__revealed_tests {
    use super::*;
    use crate::prelude::*;
    use crate::seal::plaintext::{PlainSealError, PlainToken, PlaintextSeal};
    use alloc::vec;
    use alloc::vec::Vec;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn ace() -> Card<French> {
        Card::from(FrenchBasicCard::ACE_SPADES)
    }
    fn king() -> Card<French> {
        Card::from(FrenchBasicCard::KING_HEARTS)
    }

    #[test]
    fn revealed__new_is_empty() {
        let r = Revealed::<French>::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
        assert_eq!(r.get(SlotId::new(0)), None);
        assert!(!r.is_revealed(SlotId::new(0)));
        assert_eq!(r, Revealed::default());
    }

    #[test]
    fn revealed__reveal_and_get() {
        let mut r = Revealed::<French>::new();
        r.reveal(SlotId::new(3), ace()).unwrap();
        assert_eq!(r.get(SlotId::new(3)), Some(ace()));
        assert!(r.is_revealed(SlotId::new(3)));
        assert_eq!(r.len(), 1);
        assert_eq!(r.iter().collect::<Vec<_>>(), vec![(SlotId::new(3), ace())]);
    }

    #[test]
    fn revealed__reveal_twice_errors_and_changes_nothing() {
        let mut r = Revealed::<French>::new();
        r.reveal(SlotId::new(3), ace()).unwrap();
        let before = r.clone();
        assert_eq!(
            r.reveal(SlotId::new(3), king()),
            Err(CardError::SlotAlreadyRevealed(3))
        );
        assert_eq!(r, before);
    }

    #[test]
    fn revealed__pile_for_preserves_order() {
        let mut r = Revealed::<French>::new();
        r.reveal(SlotId::new(0), ace()).unwrap();
        r.reveal(SlotId::new(2), king()).unwrap();
        let pile = r.pile_for(&[SlotId::new(2), SlotId::new(0)]).unwrap();
        assert_eq!(pile.cards(), &vec![king(), ace()]);
    }

    #[test]
    fn revealed__pile_for_unrevealed_errors() {
        let r = Revealed::<French>::new();
        assert_eq!(
            r.pile_for(&[SlotId::new(9)]),
            Err(CardError::SlotNotFound(9))
        );
    }

    #[test]
    fn revealed__reveal_with_roundtrip() {
        let scheme = PlaintextSeal::new(7);
        let slot = SlotId::new(5);
        let sealed = scheme
            .seal(ace(), slot, &mut StdRng::seed_from_u64(0))
            .unwrap();
        let mut r = Revealed::<French>::new();
        assert_eq!(
            r.reveal_with(slot, &sealed, &scheme, &PlainToken(7)),
            Ok(ace())
        );
        assert_eq!(r.get(slot), Some(ace()));
    }

    #[test]
    fn revealed__reveal_with_wrong_token_errors_and_map_unchanged() {
        let scheme = PlaintextSeal::new(7);
        let slot = SlotId::new(5);
        let sealed = scheme
            .seal(ace(), slot, &mut StdRng::seed_from_u64(0))
            .unwrap();
        let mut r = Revealed::<French>::new();
        assert_eq!(
            r.reveal_with(slot, &sealed, &scheme, &PlainToken(8)),
            Err(SealError::Backend(PlainSealError::WrongToken))
        );
        assert!(r.is_empty());
    }

    #[test]
    fn revealed__reveal_with_already_revealed_is_slot_error() {
        let scheme = PlaintextSeal::new(7);
        let slot = SlotId::new(5);
        let sealed = scheme
            .seal(ace(), slot, &mut StdRng::seed_from_u64(0))
            .unwrap();
        let mut r = Revealed::<French>::new();
        r.reveal(slot, ace()).unwrap();
        assert_eq!(
            r.reveal_with(slot, &sealed, &scheme, &PlainToken(7)),
            Err(SealError::Slot(CardError::SlotAlreadyRevealed(5)))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn revealed__serde_roundtrip() {
        let mut r = Revealed::<French>::new();
        r.reveal(SlotId::new(3), ace()).unwrap();
        let json = serde_json::to_string(&r).unwrap();
        assert_eq!(serde_json::from_str::<Revealed<French>>(&json).unwrap(), r);
    }
}
