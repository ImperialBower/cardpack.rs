//! Property tests for the EPIC-04 seal kernel: `Permutation`, `SlotPile`,
//! `Revealed`, and their agreement with `Pile`.
//!
//! Seeded through `proptest`, so a failing case reproduces from its seed.
//! Skipped on `wasm32-unknown-unknown` for the same reason as
//! `tests/properties.rs` (proptest's `wait-timeout` is unix-only).

#![cfg(not(target_arch = "wasm32"))]
#![allow(non_snake_case)]

use cardpack::prelude::*;
use proptest::prelude::*;
use std::collections::BTreeSet;

proptest! {
    /// `p.inverse().apply(p.apply(x)) == x`
    #[test]
    fn permutation__inverse_roundtrip(seed: u64, n in 0_usize..=216) {
        let x: Vec<usize> = (0..n).collect();
        let p = Permutation::from_seed(n, seed).unwrap();
        prop_assert_eq!(p.inverse().apply(&p.apply(&x).unwrap()).unwrap(), x);
        prop_assert!(p.then(&p.inverse()).unwrap().is_identity());
    }

    /// `a.then(b).apply(x) == b.apply(a.apply(x))`
    #[test]
    fn permutation__compose_law(sa: u64, sb: u64, n in 0_usize..=216) {
        let x: Vec<usize> = (0..n).collect();
        let a = Permutation::from_seed(n, sa).unwrap();
        let b = Permutation::from_seed(n, sb).unwrap();
        prop_assert_eq!(
            a.then(&b).unwrap().apply(&x).unwrap(),
            b.apply(&a.apply(&x).unwrap()).unwrap()
        );
    }

    /// The permutation and the pile are the same Fisher–Yates.
    #[test]
    fn permutation__from_rng_matches_pile_shuffle(seed: u64) {
        let deck = Standard52::deck();
        let p = Permutation::from_seed(52, seed).unwrap();
        prop_assert_eq!(deck.permute(&p).unwrap(), deck.shuffled_with_seed(seed));
    }

    /// Canonical bytes round-trip for every size the kernel supports in a deck.
    #[test]
    fn permutation__canonical_roundtrip(seed: u64, n in 0_usize..=216) {
        let p = Permutation::from_seed(n, seed).unwrap();
        prop_assert_eq!(Permutation::from_canonical_bytes(&p.canonical_bytes()).unwrap(), p);
    }

    /// A blind shuffle keeps exactly the same set of names.
    #[test]
    fn slot_pile__shuffle_permutes_slot_set(seed: u64, n in 0_u16..=216) {
        let mut shoe = SlotPile::new(n);
        let before: BTreeSet<SlotId> = shoe.slots().iter().copied().collect();
        shoe.shuffle_with_seed(seed);
        let after: BTreeSet<SlotId> = shoe.slots().iter().copied().collect();
        prop_assert_eq!(before, after);
        prop_assert!(shoe.audit(usize::from(n)).is_ok());
    }

    /// Slot *i* after a blind shuffle names the card at position *i* after
    /// the same clear shuffle.
    #[test]
    fn slot_pile__shuffle_agrees_with_pile_shuffle(seed: u64) {
        let deck = Standard52::deck();
        let shuffled = deck.shuffled_with_seed(seed);
        let mut shoe = SlotPile::new(52);
        shoe.shuffle_with_seed(seed);
        for (i, slot) in shoe.slots().iter().enumerate() {
            prop_assert_eq!(shuffled.cards()[i], deck.cards()[slot.index()]);
        }
    }

    /// Plain-value payoff: a rejected operation leaves the shoe byte-identical.
    #[test]
    fn slot_pile__rejected_ops_change_nothing(n in 0_u16..=52, extra in 1_usize..=10) {
        let before = SlotPile::new(n);
        let mut after = before.clone();
        let too_many = usize::from(n) + extra;
        prop_assert_eq!(after.draw(too_many), None);
        prop_assert_eq!(after.cut(too_many), Err(CardError::InvalidCut(too_many)));
        prop_assert!(after.permute(&Permutation::identity(too_many).unwrap()).is_err());
        prop_assert_eq!(before, after);
    }

    /// The slot path is a faithful deal: shuffle a shoe and a deck from one
    /// seed, deal `n` slots, reveal them through the codebook, and the result
    /// is the clear deal. Meaningful because the slot path never held a value.
    #[test]
    fn deal__slots_then_reveal_all_equals_clear_deal(seed: u64, n in 0_usize..=52) {
        let codebook = Standard52::codebook();
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let mut shoe = SlotPile::new(52);
        shoe.shuffle_with_seed(seed);

        let clear_hand = deck.draw(n).unwrap();
        let slot_hand = shoe.draw(n).unwrap();

        // Slot i names ordinal i of the unshuffled deck (decision 7's hazard,
        // used here on purpose: the codebook IS the sealing scheme).
        let mut revealed = Revealed::<Standard52>::new();
        for slot in slot_hand.slots() {
            let card = codebook.card(Ordinal::new(slot.get())).unwrap();
            revealed.reveal(*slot, card).unwrap();
        }
        prop_assert_eq!(revealed.pile_for(slot_hand.slots()).unwrap(), clear_hand);
        prop_assert_eq!(revealed.len(), n);
    }
}
