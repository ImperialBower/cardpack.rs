//! Property tests for cardpack's core deck/pile invariants.
//!
//! These tests use [`proptest`] to generate random seeds and draw counts,
//! then verify invariants that should hold for *every* valid input. The
//! seeded-shuffle API (`shuffled_with_seed`) makes the otherwise-random
//! shuffle deterministic, which is what makes property tests viable here:
//! a failing case can be reproduced exactly from the failing seed.
//!
//! All tests run against `Pile<Standard52>` because Standard52 is the
//! simplest non-trivial deck (52 cards, no jokers, no duplicates).
//!
//! Skipped on `wasm32-unknown-unknown` because proptest's transitive
//! `wait-timeout` crate is unix-only. The wasm runtime tests live in
//! `tests/wasm.rs`.

#![cfg(not(target_arch = "wasm32"))]
#![allow(non_snake_case)]

use cardpack::prelude::*;
use proptest::prelude::*;

proptest! {
    /// Shuffling preserves the multiset of cards: same cards, possibly
    /// different order. This catches any shuffle bug that drops, duplicates,
    /// or mutates a card.
    #[test]
    fn shuffled_with_seed__preserves_cards(seed: u64) {
        let original = Standard52::deck();
        let shuffled = original.shuffled_with_seed(seed);
        prop_assert!(original.same(&shuffled));
    }

    /// Same seed → same permutation. This is the core determinism contract
    /// of `shuffled_with_seed`; without it, property tests would not be
    /// reproducible from a failing seed.
    #[test]
    fn shuffled_with_seed__is_deterministic(seed: u64) {
        let a = Standard52::deck().shuffled_with_seed(seed);
        let b = Standard52::deck().shuffled_with_seed(seed);
        prop_assert_eq!(a, b);
    }

    /// Shuffling preserves length. Trivially implied by `preserves_cards`,
    /// but worth asserting directly because it catches a different mutation
    /// (e.g. shuffle returning an empty pile would still pass length tests
    /// only if both sides were empty — this guards against that).
    #[test]
    fn shuffled_with_seed__preserves_length(seed: u64) {
        let original = Standard52::deck();
        let shuffled = original.shuffled_with_seed(seed);
        prop_assert_eq!(original.len(), shuffled.len());
    }

    /// `sorted()` is idempotent: sorting twice gives the same result as
    /// sorting once. Catches sort instability bugs and any sort impl that
    /// is sensitive to its starting order.
    #[test]
    fn sorted__is_idempotent(seed: u64) {
        let pile = Standard52::deck().shuffled_with_seed(seed);
        let once = pile.sorted();
        let twice = once.clone().sorted();
        prop_assert_eq!(once, twice);
    }

    /// Shuffle and sort are inverses (modulo the deck's canonical order):
    /// any shuffled deck, when sorted, equals the original sorted deck.
    /// This is a stronger statement than `preserves_cards` because it
    /// asserts a specific canonical equality, not just multiset equality.
    #[test]
    fn shuffle_then_sort__is_canonical(seed: u64) {
        let original = Standard52::deck();
        let shuffled = original.shuffled_with_seed(seed);
        prop_assert_eq!(original.sorted(), shuffled.sorted());
    }

    /// Drawing `n` cards splits the deck into two parts whose lengths
    /// sum to the original. No cards are lost, none invented.
    #[test]
    fn draw__preserves_total_count(seed: u64, n in 0usize..=52) {
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let original_len = deck.len();
        let drawn = deck.draw(n).expect("n is bounded by deck size");
        prop_assert_eq!(drawn.len() + deck.len(), original_len);
        prop_assert_eq!(drawn.len(), n);
    }

    /// Drawing more cards than the deck holds returns `None` and leaves
    /// the deck untouched. Documents the API's "all-or-nothing" contract.
    #[test]
    fn draw__too_many_returns_none(seed: u64, extra in 1usize..100) {
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let before = deck.clone();
        let n = deck.len() + extra;
        prop_assert!(deck.draw(n).is_none());
        prop_assert_eq!(deck, before);
    }

    /// Drawing zero cards is a no-op: the returned pile is empty, the
    /// source pile is unchanged.
    #[test]
    fn draw__zero_is_noop(seed: u64) {
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let before = deck.clone();
        let drawn = deck.draw(0).expect("draw(0) always succeeds");
        prop_assert_eq!(drawn.len(), 0);
        prop_assert_eq!(deck, before);
    }

    /// `pile_on(&[a, b])` produces a pile whose length is `a.len() + b.len()`.
    /// This catches any concatenation bug that would drop or duplicate cards
    /// from one of the input piles.
    #[test]
    fn pile_on__sums_lengths(
        seed1: u64,
        seed2: u64,
        n1 in 0usize..=52,
        n2 in 0usize..=52,
    ) {
        let mut d1 = Standard52::deck().shuffled_with_seed(seed1);
        let mut d2 = Standard52::deck().shuffled_with_seed(seed2);
        let a = d1.draw(n1).unwrap();
        let b = d2.draw(n2).unwrap();
        let combined = Pile::<Standard52>::pile_on(&[a.clone(), b.clone()]);
        prop_assert_eq!(combined.len(), a.len() + b.len());
    }

    /// `pile_on(&[p])` is the identity: a single-element pile_on returns
    /// the input unchanged.
    #[test]
    fn pile_on__single_is_identity(seed: u64) {
        let pile = Standard52::deck().shuffled_with_seed(seed);
        let combined = Pile::<Standard52>::pile_on(&[pile.clone()]);
        prop_assert_eq!(combined, pile);
    }
}
