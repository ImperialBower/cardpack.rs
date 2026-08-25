//! `SlotPile` — the shoe, as names. Not generic. Holds no card.

use crate::basic::types::permutation::Permutation;
use crate::common::errors::CardError;
use crate::seal::slot::SlotId;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An ordered shoe of card *names*. Holds no card, no payload, no scheme.
///
/// Shuffle, cut, draw and deal are permutations of labels and need no
/// knowledge — so this type can be handed to a referee, a spectator, or a
/// log without leaking anything, because there is nothing to leak. It is a
/// plain value: `Clone`, `Eq`, `Debug`, `Serialize` all derive, and "a
/// rejected operation changed nothing" is one `assert_eq!`.
///
/// Its one invariant — slot uniqueness — is enforced by every constructor
/// (including the `serde` deserializer) and re-checked by [`audit`](Self::audit).
///
/// ```
/// use cardpack::prelude::*;
///
/// let mut shoe = SlotPile::new(52);
/// shoe.shuffle_with_seed(7);
/// let hole = shoe.draw(2).unwrap();
///
/// assert_eq!(hole.len(), 2);
/// assert_eq!(shoe.len(), 50);
/// assert!(shoe.audit(50).is_ok());
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Vec<SlotId>"))]
pub struct SlotPile(Vec<SlotId>);

impl TryFrom<Vec<SlotId>> for SlotPile {
    type Error = CardError;

    fn try_from(slots: Vec<SlotId>) -> Result<Self, CardError> {
        Self::from_slots(slots)
    }
}

/// The result of [`SlotPile::audit`]: the count and any duplicate slots.
///
/// That is *all* a deck of names can check. Whether the names stand for
/// distinct cards is a property of whatever sealed them — a backend concern.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotAudit {
    pub expected: usize,
    pub actual: usize,
    pub duplicate_slots: Vec<SlotId>,
}

impl SlotAudit {
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.expected == self.actual && self.duplicate_slots.is_empty()
    }
}

impl SlotPile {
    /// Slots `0..n`, in order.
    ///
    /// **Hazard:** if you then seal an *unshuffled* deck into these slots in
    /// vocabulary order, slot `i` names ordinal `i` and the deck is public.
    /// Shuffle first (`docs/EPIC-04_Sealed_Decks.md`, decision 7).
    #[must_use]
    pub fn new(n: u16) -> Self {
        Self((0..n).map(SlotId::new).collect())
    }

    /// Builds a shoe from named slots, in the given order.
    ///
    /// # Errors
    ///
    /// [`CardError::DuplicateSlot`] naming the first repeated slot.
    pub fn from_slots(slots: Vec<SlotId>) -> Result<Self, CardError> {
        let mut seen = BTreeSet::new();
        for slot in &slots {
            if !seen.insert(*slot) {
                return Err(CardError::DuplicateSlot(slot.get()));
            }
        }
        Ok(Self(slots))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Every slot still in the shoe, in order. Public, leaks nothing.
    #[must_use]
    pub fn slots(&self) -> &[SlotId] {
        &self.0
    }

    #[must_use]
    pub fn contains(&self, slot: SlotId) -> bool {
        self.0.contains(&slot)
    }

    #[must_use]
    pub fn position(&self, slot: SlotId) -> Option<usize> {
        self.0.iter().position(|s| *s == slot)
    }

    /// Remove by name. Needs no knowledge. `None` if absent.
    pub fn take(&mut self, slot: SlotId) -> Option<SlotId> {
        let i = self.position(slot)?;
        Some(self.0.remove(i))
    }

    /// The top slot (front of the shoe), or `None` if empty.
    pub fn draw_first(&mut self) -> Option<SlotId> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }

    /// Draws `n` slots from the top. All-or-nothing, mirroring `Pile::draw`:
    /// `None` leaves the shoe untouched.
    pub fn draw(&mut self, n: usize) -> Option<Self> {
        if n > self.0.len() {
            return None;
        }
        let rest = self.0.split_off(n);
        Some(Self(core::mem::replace(&mut self.0, rest)))
    }

    /// Blind Fisher–Yates. The same call `Pile::shuffle_with_rng` makes, so
    /// the same RNG state permutes a shoe and a deck identically.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// `StdRng::seed_from_u64`; stable within one `rand` major version.
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new shoe reordered by `p`: `out[i] = self[p[i]]`.
    ///
    /// # Errors
    ///
    /// [`CardError::PermutationLength`] if `p.len() != self.len()`.
    pub fn permute(&self, p: &Permutation) -> Result<Self, CardError> {
        Ok(Self(p.apply(&self.0)?))
    }

    /// Cuts at `at`. Defined as [`Permutation::rotation`]. A rejected cut
    /// changes nothing.
    ///
    /// # Errors
    ///
    /// [`CardError::InvalidCut`] if `at > self.len()`.
    pub fn cut(&mut self, at: usize) -> Result<(), CardError> {
        let p = Permutation::rotation(self.len(), at)?;
        self.0 = p.apply(&self.0)?;
        Ok(())
    }

    /// Counts slots and lists duplicates (in first-seen order). It does
    /// **not** and cannot check that the slots stand for distinct cards.
    #[must_use]
    pub fn audit(&self, expected: usize) -> SlotAudit {
        let mut seen = BTreeSet::new();
        let mut duplicate_slots = Vec::new();
        for slot in &self.0 {
            if !seen.insert(*slot) && !duplicate_slots.contains(slot) {
                duplicate_slots.push(*slot);
            }
        }
        SlotAudit {
            expected,
            actual: self.0.len(),
            duplicate_slots,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__slot_pile_tests {
    use super::*;
    use crate::prelude::*;
    use alloc::collections::BTreeSet;
    use alloc::vec::Vec;

    fn ids(v: &[u16]) -> Vec<SlotId> {
        v.iter().map(|&i| SlotId::new(i)).collect()
    }

    #[test]
    fn slot_pile__new_is_identity_order() {
        let p = SlotPile::new(5);
        assert_eq!(p.len(), 5);
        assert!(!p.is_empty());
        assert_eq!(p.slots(), &ids(&[0, 1, 2, 3, 4])[..]);
        assert!(SlotPile::new(0).is_empty());
        assert_eq!(SlotPile::default(), SlotPile::new(0));
    }

    #[test]
    fn slot_pile__from_slots_rejects_duplicates() {
        assert_eq!(
            SlotPile::from_slots(ids(&[0, 1, 1])),
            Err(CardError::DuplicateSlot(1))
        );
        let p = SlotPile::from_slots(ids(&[2, 0])).unwrap();
        assert_eq!(p.slots(), &ids(&[2, 0])[..], "order is kept");
    }

    #[test]
    fn slot_pile__contains_and_position() {
        let p = SlotPile::from_slots(ids(&[7, 3, 9])).unwrap();
        assert!(p.contains(SlotId::new(3)));
        assert!(!p.contains(SlotId::new(4)));
        assert_eq!(p.position(SlotId::new(9)), Some(2));
        assert_eq!(p.position(SlotId::new(4)), None);
    }

    #[test]
    fn slot_pile__take_by_name() {
        let mut p = SlotPile::new(5);
        assert_eq!(p.take(SlotId::new(2)), Some(SlotId::new(2)));
        assert_eq!(p.slots(), &ids(&[0, 1, 3, 4])[..]);
        assert!(!p.contains(SlotId::new(2)));
        assert_eq!(p.take(SlotId::new(2)), None);
    }

    #[test]
    fn slot_pile__draw_first() {
        let mut p = SlotPile::from_slots(ids(&[4, 1])).unwrap();
        assert_eq!(p.draw_first(), Some(SlotId::new(4)));
        assert_eq!(p.draw_first(), Some(SlotId::new(1)));
        assert_eq!(p.draw_first(), None);
    }

    #[test]
    fn slot_pile__draw_all_or_nothing() {
        let mut p = SlotPile::new(5);
        let hand = p.draw(3).unwrap();
        assert_eq!(hand.slots(), &ids(&[0, 1, 2])[..]);
        assert_eq!(p.slots(), &ids(&[3, 4])[..]);
        assert_eq!(p.draw(3), None);
        assert_eq!(
            p.slots(),
            &ids(&[3, 4])[..],
            "a rejected draw changes nothing"
        );
    }

    #[test]
    fn slot_pile__rejected_ops_change_nothing() {
        let before = SlotPile::new(6);
        let mut after = before.clone();
        assert_eq!(after.draw(7), None);
        assert_eq!(after.cut(7), Err(CardError::InvalidCut(7)));
        assert!(after.permute(&Permutation::identity(5).unwrap()).is_err());
        assert_eq!(before, after);
    }

    #[test]
    fn slot_pile__shuffle_permutes_slot_set() {
        let mut p = SlotPile::new(52);
        let before: BTreeSet<SlotId> = p.slots().iter().copied().collect();
        p.shuffle_with_seed(11);
        let after: BTreeSet<SlotId> = p.slots().iter().copied().collect();
        assert_eq!(before, after);
        assert_ne!(p, SlotPile::new(52));
        let mut q = SlotPile::new(52);
        q.shuffle_with_seed(11);
        assert_eq!(p, q, "deterministic for a seed");
    }

    /// A blind shuffle and a clear shuffle from one seed agree slot-for-slot:
    /// the slot at position *i* names the card at position *i*.
    #[test]
    fn slot_pile__shuffle_agrees_with_pile_shuffle_for_same_rng() {
        let deck = Standard52::deck();
        for seed in [0_u64, 5, 99] {
            let mut slots = SlotPile::new(52);
            slots.shuffle_with_seed(seed);
            let shuffled = deck.shuffled_with_seed(seed);
            for (i, slot) in slots.slots().iter().enumerate() {
                assert_eq!(
                    shuffled.cards()[i],
                    deck.cards()[slot.index()],
                    "seed {seed}"
                );
            }
        }
    }

    #[test]
    fn slot_pile__permute_and_cut() {
        let p = SlotPile::new(5);
        let rot = Permutation::rotation(5, 2).unwrap();
        assert_eq!(p.permute(&rot).unwrap().slots(), &ids(&[2, 3, 4, 0, 1])[..]);
        let mut c = p.clone();
        c.cut(2).unwrap();
        assert_eq!(c, p.permute(&rot).unwrap());
        assert_eq!(
            p.permute(&Permutation::identity(4).unwrap()),
            Err(CardError::PermutationLength {
                expected: 4,
                actual: 5
            })
        );
    }

    #[test]
    fn slot_pile__audit_counts_and_finds_duplicates() {
        let ok = SlotPile::new(5).audit(5);
        assert!(ok.is_ok());
        assert_eq!(ok.expected, 5);
        assert_eq!(ok.actual, 5);
        assert!(ok.duplicate_slots.is_empty());

        let short = SlotPile::new(4).audit(5);
        assert!(!short.is_ok());

        // Only reachable from inside the module: the constructors forbid it.
        let dup = SlotPile(ids(&[1, 2, 1, 3, 3]));
        let a = dup.audit(5);
        assert!(!a.is_ok());
        assert_eq!(a.duplicate_slots, ids(&[1, 3]));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn slot_pile__serde_roundtrip_and_rejects_duplicates() {
        let p = SlotPile::from_slots(ids(&[3, 1, 2])).unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<SlotPile>(&json).unwrap(), p);
        assert!(serde_json::from_str::<SlotPile>("[1,1]").is_err());
    }
}
