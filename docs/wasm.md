# Using cardpack on WebAssembly

cardpack compiles cleanly to `wasm32-unknown-unknown` (browser WASM) with
every feature combination — verified against the matrix in §"Feature
combinations" below. This guide covers the consumer-side setup that
makes it work and the runtime gotchas to watch for.

## TL;DR

Add to your consuming crate's `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

Then in your code, prefer the seeded shuffle:

```rust
use cardpack::prelude::*;

let deck = Standard52::deck().shuffled_with_seed(seed);
```

That's it. cardpack's wasm dependency on `getrandom`'s `wasm_js` feature
is set automatically (see `Cargo.toml` `[target.'cfg(target_arch = "wasm32")'.dependencies]`),
so you don't need to add `getrandom` to your own `[dependencies]`.

## Why the cfg flag is needed

cardpack uses `rand 0.9`, which uses `rand_core 0.9`, which depends on
`getrandom 0.3`. On `wasm32-unknown-unknown`, `getrandom` requires the
consumer to **pick a backend** — there's no default because the right
choice depends on the runtime (browser vs WASI vs custom embedder).

Without the cfg flag, the build fails with:

```
error: The wasm32-unknown-unknown targets are not supported by default;
       you may need to enable the "wasm_js" configuration flag.
```

The `wasm_js` backend pulls entropy from `crypto.getRandomValues()` in
the browser. Other valid choices include:

- **WASI** (`wasm32-wasip1`/`wasm32-wasip2`): use `wasm32-wasi*` targets
  instead — `getrandom` provides a backend automatically. No cfg flag
  needed.
- **Cloudflare Workers / Edge**: same as browser, use `wasm_js`.
- **Custom embedder with no entropy source**: implement
  [`getrandom::register_custom_getrandom!`](https://docs.rs/getrandom/0.3/getrandom/macro.register_custom_getrandom.html).

## Recommended feature combos

cardpack's `default` features (`i18n`, `colored-display`, `yaml`,
`serde`) all compile to wasm, but a few are **wasted bytes** in browser
builds:

| Feature | Browser-friendly? | Notes |
|---|---|---|
| `i18n` (fluent-templates) | ✅ | Locale resources are compiled in; runtime reads them from memory. |
| `colored-display` | ⚠️ harmless | TTY detection always returns false in browsers, so output is plain. Drop it to save ~50 KB. |
| `yaml` (serde_norway) | ⚠️ partial | `cards_from_yaml_str` works fine. `cards_from_yaml_file` calls `std::fs::File::open` which **panics** on `wasm32-unknown-unknown`. |
| `serde` | ✅ | Pure logic, no platform deps. |

A typical browser build:

```toml
[dependencies]
cardpack = { version = "0.6", default-features = false, features = ["i18n", "serde"] }
```

If you need YAML loading, parse the YAML in your JS host and pass the
string to `cards_from_yaml_str`:

```rust
use cardpack::prelude::*;

let yaml = "...";  // ferried from JS
let cards = BasicCard::cards_from_yaml_str(yaml).unwrap();
```

## Runtime gotchas

### `Pile::shuffle()` and `Pile::shuffled()` (no-arg)

These call `rand::rng()` (the thread-local RNG), which on
`wasm32-unknown-unknown` will pull entropy via `getrandom`'s `wasm_js`
backend on first use. **It works**, but:

- It calls into JS each shuffle (slow if you shuffle in a hot loop).
- It's non-deterministic — bad for testing, unfair-feel for game
  state-syncing across clients.

**Prefer `shuffle_with_seed(seed: u64)` and `shuffled_with_seed(seed)`**
when you can. Same shuffle quality (it's still ChaCha8), zero JS
round-trips, deterministic across runs.

### `Pile::demo_cards()`

Calls `println!`, which on `wasm32-unknown-unknown` writes to nothing
useful. Use the `Pile` API directly and ferry results back to JS via
`wasm-bindgen` or your bridge of choice.

### `cards_from_yaml_file(path)`

Panics on browsers (no filesystem). Use `cards_from_yaml_str` instead
and let JS load the file.

## A working example

[`examples/wasm.rs`](../examples/wasm.rs) compiles to
`wasm32-unknown-unknown` and demonstrates the recommended patterns:

```sh
# From this repo:
cargo build --target wasm32-unknown-unknown --example wasm
```

The repo's `.cargo/config.toml` sets the `getrandom_backend` cfg flag,
so this just works.

## Verifying wasm correctness

The repo includes [`tests/wasm.rs`](../tests/wasm.rs), a small
runtime test suite that compiles to wasm and runs under a real wasm
runtime (node-headless) via `wasm-bindgen-test`. It verifies:

- `shuffled_with_seed` is deterministic on wasm
- `Pile::draw` works correctly under wasm
- `Pile::shuffle()` (no-arg) works — i.e. the `getrandom` `wasm_js`
  backend successfully sources entropy from the host
- `DeckKind::all()` and the deck registry work on wasm
- `cards_from_yaml_str` parses without panicking under wasm

To run locally:

```sh
cargo install wasm-bindgen-cli
cargo test --target wasm32-unknown-unknown --test wasm
```

The repo's `.cargo/config.toml` configures
`runner = "wasm-bindgen-test-runner"`, so `cargo test` dispatches
automatically. CI runs these tests on every push (see the `wasm-test`
job in `.github/workflows/CI.yaml`) along with a `wasm-build`
compile-only gate.

## Feature combinations verified

All of these compile clean against `wasm32-unknown-unknown` as of
2026-04-30:

```sh
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --no-default-features
cargo build --target wasm32-unknown-unknown --no-default-features --features i18n
cargo build --target wasm32-unknown-unknown --no-default-features --features serde
cargo build --target wasm32-unknown-unknown --all-features
```

## Going further: real browser integration

This guide covers compiling cardpack for wasm. To actually run it in a
browser, you need:

- [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/) for
  ergonomic JS interop, or
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) for a full
  bundle pipeline.

cardpack itself stays JS-agnostic — the integration lives in your
consumer crate. A skeleton looks like:

```rust
use cardpack::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn deal_hand(seed: u64, n: usize) -> Vec<String> {
    let mut deck = Standard52::deck().shuffled_with_seed(seed);
    let hand = deck.draw(n).unwrap_or_default();
    hand.cards().iter().map(|c| c.to_string()).collect()
}
```

That's the same shape as `deal_hand` in `examples/wasm.rs`, just with
`#[wasm_bindgen]` and `Vec<String>` returns instead of a primitive.
