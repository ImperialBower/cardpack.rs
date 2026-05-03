# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Pip::new` is now a `const fn`, enabling compile-time `Pip` literals
  via the constructor (callers previously had to use struct literals).
  (audit row #14)
- `BasicCard::new(suit: Pip, rank: Pip) -> Self` — `const fn`
  constructor that's more ergonomic than the existing struct-literal
  pattern at deck-card definition sites. (audit row #14)
- `Pile<T>::from_slice(&[BasicCard]) -> Self` — ergonomic non-const
  helper for building piles from `const` arrays. `Pile` wraps `Vec`
  and is intentionally not const-constructible. (audit row #14)

### Internal

- Criterion benchmarks under `benches/draw.rs` covering shuffle, draw,
  `pile_on`, and `combos` against the 108-card Canasta deck. New
  `make bench` target. (audit row #8)

## [0.7.0] — 2026-05-01

### Breaking
- `Pile::into_hashset()` renamed to `Pile::unique_cards()`; return type
  changed from `HashSet<Card<DeckType>>` to `BTreeSet<Card<DeckType>>`.
- `From<HashSet<Card<DeckType>>> for Pile<DeckType>` removed; replaced
  with `impl FromIterator<Card<DeckType>> for Pile<DeckType>`,
  enabling `.collect::<Pile<_>>()` from any iterator.
  Migration: `Pile::from(my_hashset)` becomes
  `my_hashset.into_iter().collect::<Pile<_>>()`. Note that `HashSet`
  iteration order is non-deterministic; if the previous code relied on
  the `From<HashSet>` impl's internal `.sorted()` call for stable output,
  add an explicit `.sorted()` after the `.collect()`, or iterate over a
  `BTreeSet` instead.
- `Pile::map_by_suit()` and `Ranged::map_by_rank()` return `BTreeMap`
  instead of `HashMap` (deterministic iteration order; identical
  lookup semantics).
- `Pile::shuffle()` / `Pile::shuffled()` (no-arg) and
  `BasicPile::shuffle()` / `BasicPile::shuffled()` (no-arg) now require
  the `std` feature (still default-on). Under `--no-default-features`,
  use `shuffle_with_seed(u64)` or `shuffle_with_rng(&mut R)`. Same gate
  applies to `Pile::draw_random()` and `BasicPileCell::shuffle/shuffled`.

### Added
- `no_std` support via `extern crate alloc`. Build with
  `--no-default-features` for an alloc-only build that targets embedded
  Rust (`thumbv7em-none-eabihf` and similar).
- New `std` feature (default-on). Existing features (`i18n`,
  `colored-display`, `yaml`) implicitly require `std`; `serde` implies
  `alloc`.
- New `alloc` feature (mostly internal plumbing for `serde?/alloc`).
- CI gates on `cargo build --no-default-features --target
  thumbv7em-none-eabihf`.
- `examples/no_std_smoke.rs` — bare-metal compile + link smoke binary
  with a minimal static bump allocator.
- New Make targets: `make no-std`, `make no-std-thumbv7`.

### Internal
- `Ranged::extract_pips` switched from `HashSet` → `BTreeSet`,
  removing a redundant sort step.
- `Ranged::combos` switched from `HashSet<BasicPile>` → `BTreeSet<BasicPile>`;
  the post-collect `.sort()` is now redundant and removed.
- `HashMap` imports in `pile.rs` and `traits.rs` are gated on
  `colored-display` (the only feature that uses HashMap return types).
- Library source migrated from `std::*` to `core::*`/`alloc::*` for
  re-exported types (`fmt`, `str::FromStr`, `cmp::Ordering`, `hash`,
  `cell::Cell`, `marker::PhantomData`, `error::Error`, `vec::IntoIter`).
- `#[macro_use] extern crate alloc;` brings `format!`/`vec!` macros
  into scope crate-wide for no_std builds.
- Dev-deps that require `std` (clap, ckc-rs, env_logger, rstest,
  term-table, criterion, proptest) are gated off
  `cfg(target_os = "none")` so cargo doesn't try to compile them when
  building examples for bare-metal targets.

## [0.6.12] - 2026-04-30

This release closes most of the gap analysis tracked in
[`docs/audit-2026-04-29.md`](docs/audit-2026-04-29.md). Default-features
behavior is unchanged from 0.6.11 — every new opt-out feature flag is on
by default, so existing downstream code keeps compiling.

### Added

- **Cargo features** — `i18n`, `colored-display`, `yaml`, `serde`. All
  enabled in `default`. Lets consumers trim transitive deps:
  ```toml
  cardpack = { version = "0.6", default-features = false, features = ["serde"] }
  ```
  `yaml` implies `serde`. See `Cargo.toml` and the README's "Cargo
  features" section.
- **Deterministic shuffle** — `Pile::shuffle_with_seed(u64)`,
  `Pile::shuffled_with_seed(u64)`, and generic-RNG variants
  `Pile::shuffle_with_rng<R>` / `Pile::shuffled_with_rng<R>` on both
  `BasicPile` and `Pile<T>`. Use the seeded variants for reproducible
  game state.
- **`DeckKind` enum** — non-generic registry of every shipped deck.
  Re-exported from `prelude`. `DeckKind::all()` returns
  `&'static [DeckKind]`; `DeckKind::base_vec()`, `deck_name()`,
  `fluent_deck_key()`, and `demo()` dispatch to the underlying typed
  deck. `Razz` variant is gated on the `yaml` feature.
- **WebAssembly support** — cardpack compiles cleanly to
  `wasm32-unknown-unknown` across all feature combinations.
  `getrandom = { features = ["wasm_js"] }` is wired in as a target-gated
  dep so consumers don't need to add it themselves. See
  [`docs/wasm.md`](docs/wasm.md) and [`examples/wasm.rs`](examples/wasm.rs).
- **New locale drafts** — `fr` (French), `la` (Latin), `tlh` (Klingon).
  All marked as DRAFT pending native-speaker / KLI review; per-file
  confidence notes live in each locale's `README.md`.
- **`FluentName` locale constants** — `FRANCAIS`, `LATINA`, `TLHINGAN`,
  matching the existing `US_ENGLISH` / `DEUTSCH` pattern.
- **`examples/poker_eval.rs`** — wires `ckc-rs` to `Pile<Standard52>`,
  enumerating each player's 21 5-card combos and selecting the best
  hand via `ckc_rs::evaluate::five_cards`.
- **`examples/demo.rs --all` flag** — iterates `DeckKind::all()` to
  show every shipped deck.
- **Verbose demo output** — five-locale side-by-side table with column
  headers (English / German / French / Latin / Klingon).

### Changed

- **`fluent_connector` is locale-aware** — French uses `" de "`, Latin
  and Klingon use `" "` (space), German remains `" "`. Previously
  every non-DEUTSCH locale silently got the English `" of "`.
- `Pinochle::fluent_deck_key` and `Canasta::fluent_deck_key` now return
  the French key. Removes the silent FTL fallback for those decks.
  Public constants `FLUENT_KEY_BASE_NAME_PINOCHLE` and
  `FLUENT_KEY_BASE_NAME_CANASTA` are retained but no longer referenced
  internally.
- `FluentName::new_with_weight` no longer panics. It now discards the
  weight (since `FluentName` has no weight field) and delegates to
  `FluentName::new`. Behavior is documented on the impl.
- README updates — feature-flag table, WebAssembly section, fixed
  broken links to locale files, updated `serde_yml` references to
  `serde_norway`, removed the unimplemented "playability verification"
  bullet.

### Fixed

- **`de/tarot.ftl` schema** — Major Arcana keys renamed from
  `name-rank-tarot-{0..l}` to `name-rank-tarot-special-{0..l}` to match
  the en-US schema. The 14 missing Minor Arcana entries (Ass, König,
  Königin, Ritter, Bube, Zehn..Zwei) were added. Both Major and Minor
  Arcana now resolve in German instead of silently falling back to
  English. Guarded by `german_tarot_resolves_correctly` in
  `src/localization.rs`.

### Documentation

- `docs/audit-2026-04-29.md` — full gap analysis with status icons per
  item.
- `docs/wasm.md` — consumer setup guide, recommended feature combos,
  runtime gotchas.
- Locale READMEs (`fr/`, `la/`, `tlh/`) — confidence levels and
  reviewer profiles needed before promotion.

### Internal

- 10 property tests in `tests/properties.rs` (proptest) covering
  shuffle / sort / draw / `pile_on` invariants.
- 5 wasm runtime tests in `tests/wasm.rs` (wasm-bindgen-test,
  node-headless).
- New CI jobs: `doc` (rustdoc -D warnings), `wasm-build`, `wasm-test`.
- Makefile gained `make build-wasm`, `make test-wasm`,
  `make install-wasm-bindgen-cli`.
- `[package.metadata.docs.rs] all-features = true` so docs.rs matches
  the new CI doc gate.

## [0.6.11]

The starting point for the work tracked above. See git history for
prior changes.

[Unreleased]: https://github.com/ImperialBower/cardpack.rs/compare/v0.6.12...HEAD
[0.6.12]: https://github.com/ImperialBower/cardpack.rs/compare/v0.6.11...v0.6.12
[0.6.11]: https://github.com/ImperialBower/cardpack.rs/releases/tag/v0.6.11
