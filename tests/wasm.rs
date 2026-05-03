//! Wasm runtime tests.
//!
//! These tests compile to `wasm32-unknown-unknown` and run under a real
//! wasm runtime (via `wasm-bindgen-test`), unlike the rest of the test
//! suite which runs natively. They guard against regressions where:
//!
//! - A new code path takes a hard dep on `std::fs` or `std::env` and
//!   compiles cleanly but panics at wasm runtime.
//! - The `getrandom` `wasm_js` backend stops working with `rand`'s
//!   default RNG.
//!
//! Default test mode is node-headless (no browser install required).
//! To run locally:
//!
//! ```sh
//! cargo install wasm-bindgen-cli   # one-time
//! cargo test --target wasm32-unknown-unknown --test wasm
//! ```
//!
//! `.cargo/config.toml` sets the `runner` to `wasm-bindgen-test-runner`,
//! so `cargo test` dispatches automatically.

#![cfg(target_arch = "wasm32")]

use cardpack::prelude::*;
use wasm_bindgen_test::*;

/// Confirms that `shuffled_with_seed` produces identical output for the
/// same seed under wasm. This is the foundational determinism property
/// that makes proptest possible — if the wasm RNG were non-deterministic
/// despite the seed, every other property in `tests/properties.rs` would
/// silently lose its guarantees on wasm targets.
#[wasm_bindgen_test]
fn shuffled_with_seed_is_deterministic() {
    let a = Standard52::deck().shuffled_with_seed(42);
    let b = Standard52::deck().shuffled_with_seed(42);
    assert_eq!(a, b);
    assert_eq!(a.len(), 52);
}

/// Confirms `Pile::draw` works under wasm — exercises the same code path
/// users hit when implementing card games in the browser.
#[wasm_bindgen_test]
fn deal_a_hand() {
    let mut deck = Standard52::deck().shuffled_with_seed(0);
    let hand = deck.draw(5).expect("52-card deck has 5 to draw");
    assert_eq!(hand.len(), 5);
    assert_eq!(deck.len(), 47);
}

/// Confirms that the no-arg `Pile::shuffle()` works under wasm — i.e.
/// that the `getrandom` `wasm_js` backend successfully sources entropy
/// from the host (node's `crypto.getRandomValues` polyfill). If this
/// test panics, the wasm getrandom plumbing is broken.
#[wasm_bindgen_test]
fn nondeterministic_shuffle_works() {
    let deck = Standard52::deck();
    let shuffled = deck.shuffled();
    assert_eq!(shuffled.len(), 52);
    assert!(deck.same(&shuffled), "shuffle must preserve cards");
}

/// Confirms `DeckKind::all()` enumerates all decks under wasm. Trivial,
/// but it's the public API entry point for "list available decks" and
/// the registry uses `&'static [Self]` which must work on the wasm
/// memory model.
#[wasm_bindgen_test]
fn deck_kind_registry_works() {
    let kinds = DeckKind::all();
    assert!(!kinds.is_empty());
    assert!(kinds.contains(&DeckKind::Standard52));
    assert_eq!(DeckKind::Standard52.base_vec().len(), 52);
}

/// Confirms YAML string parsing works under wasm. (`cards_from_yaml_file`
/// would panic on wasm because there's no filesystem; we test the
/// `_str` variant which stays in memory.)
#[cfg(feature = "yaml")]
#[wasm_bindgen_test]
fn yaml_str_parser_works() {
    // Round-trip a small YAML through the parser. Exact format isn't
    // important — we just want to confirm the parser doesn't panic
    // calling into wasm.
    let yaml = r#"
- suit: { weight: 4, pip_type: Suit, index: S, symbol: "♠", value: 4 }
  rank: { weight: 13, pip_type: Rank, index: A, symbol: A, value: 14 }
"#;
    let cards = BasicCard::cards_from_yaml_str(yaml);
    assert!(cards.is_ok(), "yaml_str parser must not panic on wasm");
    assert_eq!(cards.unwrap().len(), 1);
}
