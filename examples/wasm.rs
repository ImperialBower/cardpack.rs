//! Minimal cardpack example targeting wasm32-unknown-unknown.
//!
//! Demonstrates wasm-friendly API patterns:
//! - Use `shuffle_with_seed(seed)` instead of `shuffle()` to keep
//!   randomness deterministic and avoid runtime panics from
//!   thread-local RNG initialization in browsers.
//! - Avoid `cards_from_yaml_file` (calls `std::fs` — panics in browsers
//!   with no filesystem). Use `cards_from_yaml_str` if you need YAML.
//! - `colored` output is silently disabled by browsers (no TTY).
//! - `demo_cards()` calls `println!` which goes nowhere useful in the
//!   browser; use the Pile API directly and ferry results to JS.
//!
//! ## Build
//!
//! ```sh
//! cargo build --target wasm32-unknown-unknown --example wasm
//! ```
//!
//! The repo's `.cargo/config.toml` sets the required
//! `--cfg getrandom_backend="wasm_js"` flag for the wasm target. See
//! `docs/wasm.md` for the consumer-side setup.

use cardpack::prelude::*;

#[cfg_attr(target_arch = "wasm32", unsafe(no_mangle))]
pub extern "C" fn deal_hand(seed: u64, n: usize) -> usize {
    // The classic "shuffle and deal" flow, all in pure deterministic land:
    let mut deck = Standard52::deck().shuffled_with_seed(seed);
    let hand = deck.draw(n).unwrap_or_default();
    hand.len()
}

fn main() {
    // Native-build smoke test. On wasm the `deal_hand` function above is
    // the actual exported entry point; the host JS would call it directly.
    let dealt = deal_hand(42, 5);
    assert_eq!(dealt, 5);

    // Demonstrate the deterministic-seed pattern:
    let a = Standard52::deck().shuffled_with_seed(0);
    let b = Standard52::deck().shuffled_with_seed(0);
    assert_eq!(a, b, "same seed should produce identical shuffles");
    assert_eq!(a.len(), 52);
}
