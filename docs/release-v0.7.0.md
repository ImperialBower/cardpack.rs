# cardpack v0.7.0 — Release Notes

**Released:** 2026-05-02
**Range covered:** v0.6.8 → v0.7.0 (inclusive of v0.6.9, v0.6.10, v0.6.11, v0.6.12)
**Previous release notes:** see [`CHANGELOG.md`](../CHANGELOG.md) for the
authoritative per-version changelog. This document is a single narrative
summary of everything that happened across the v0.6.9–v0.7.0 series.

---

## TL;DR

The v0.6.9 → v0.7.0 series moves cardpack from a single-target,
std-only library into a configurable, embeddable, internationally
localized one:

- **`no_std` support** (v0.7.0) — alloc-only builds work on bare-metal
  targets such as `thumbv7em-none-eabihf`, gated behind a default-on
  `std` feature.
  - **Cargo features** (v0.6.12) — `i18n`, `colored-display`, `yaml`,
    `serde`, `std`, `alloc` let consumers trim transitive deps.
  - **WebAssembly** (v0.6.12) — clean `wasm32-unknown-unknown` builds
    across all feature combinations.
  - **Deterministic shuffle** (v0.6.12) — `shuffle_with_seed(u64)` and
    `shuffle_with_rng<R>` for reproducible game state.
  - **Three new locale drafts** (v0.6.12) — `fr` (French), `la` (Latin),
    `tlh` (Klingon), plus a German Tarot schema fix.
  - **Serde for piles** (v0.6.9) — `Serialize`/`Deserialize` derives on
    `BasicPile` and `Pile<DeckType>`.
  - **Test-coverage hardening** (v0.6.11) — ~70 new unit tests closed the
    gaps surfaced by `cargo-mutants`.
  - **Tooling** (v0.6.10) — repo-wide `Makefile`, audit-driven cleanup,
    flaky-test fix.

A small number of public-API breakages ship in v0.7.0 (collection types
and shuffle gating). See **[Migration guide](#migration-guide-v06x--v070)**
below — most consumers using default features will see no impact beyond
a `cargo update`.

---

## Migration guide (v0.6.x → v0.7.0)

Default features remain on, so most code keeps compiling. The breaking
edges are:

### 1. `Pile::into_hashset()` → `Pile::unique_cards()`

```rust
// before
let set: HashSet<Card<Standard52>> = pile.into_hashset();

// after
let set: BTreeSet<Card<Standard52>> = pile.unique_cards();
```

The return type changed from `HashSet` to `BTreeSet` (deterministic
iteration; identical lookup semantics).

### 2. `From<HashSet<Card<DeckType>>> for Pile<DeckType>` removed

Replaced with `impl FromIterator<Card<DeckType>> for Pile<DeckType>`,
which works for any iterator:

```rust
// before
let pile = Pile::from(my_hashset);

// after
let pile: Pile<_> = my_hashset.into_iter().collect();
```

`HashSet` iteration order is non-deterministic. The old `From<HashSet>`
impl called `.sorted()` internally; if you were relying on that, append
`.sorted()` after `.collect()`, or iterate over a `BTreeSet` instead.

### 3. `map_by_suit()` / `map_by_rank()` return `BTreeMap`

```rust
// before: HashMap<Pip, BasicPile>
// after:  BTreeMap<Pip, BasicPile>
```

Lookup semantics are identical; iteration is now deterministic.

### 4. No-arg `shuffle` / `shuffled` / `draw_random` require `std`

The convenience shuffles still work with default features
(`std` is on by default). Under `--no-default-features`:

```rust
// no_std-friendly variants
pile.shuffle_with_seed(42);
pile.shuffle_with_rng(&mut my_rng);
```

This applies to `Pile`, `BasicPile`, and `BasicPileCell`.

---

## Release timeline

| Version | Date | PR | Theme |
|---|---|---|---|
| [v0.6.9](#v069--serde-for-piles)  | 2026-02-27 | [#66](https://github.com/ImperialBower/cardpack.rs/pull/66) | Serde derives on piles |
| [v0.6.10](#v0610--audit-driven-cleanup--tooling) | 2026-04-09 | — | Audit-driven cleanup, Makefile |
| [v0.6.11](#v0611--mutation-test-coverage-hardening) | 2026-04-13 | [#67](https://github.com/ImperialBower/cardpack.rs/pull/67) | Mutation-testing coverage gaps |
| [v0.6.12](#v0612--features-locales-wasm-seeded-shuffle) | 2026-05-01 | [#68](https://github.com/ImperialBower/cardpack.rs/pull/68) | Cargo features, locales, wasm, seeded shuffle |
| [v0.7.0](#v070--no_std-and-deterministic-collections) | 2026-05-02 | [#69](https://github.com/ImperialBower/cardpack.rs/pull/69) | `no_std` support, deterministic collections |

---

## v0.6.9 — Serde for piles

PR [#66](https://github.com/ImperialBower/cardpack.rs/pull/66) ·
2026-02-27

### Added
- `Serialize` / `Deserialize` derives on `BasicPile` and
  `Pile<DeckType>`. Decks and hands can now round-trip through any
  serde format (JSON, bincode, postcard, etc.) without bespoke
  conversion code.

This release was a single, focused change — no other behavioral diffs.
The serde dependency became a hard dep here; in v0.6.12 it was moved
behind an opt-out `serde` feature flag.

---

## v0.6.10 — Audit-driven cleanup & tooling

2026-04-09 (no GitHub PR — direct merges)

This release was the implementation pass against
[`docs/audit-2026-04-09.md`](audit-2026-04-09.md). Default-features
behavior is unchanged.

### Added
- **`Makefile`** — repository-wide build/test/lint targets, modeled on
  the sibling `pkcore` repo's Makefile. Replaces ad-hoc shell scripts
  in `bin/`.
  - **`docs/audit-2026-04-09.md`** — full audit report capturing the
    state of the codebase prior to the cleanup pass.

### Changed
- **`combos.rs`** — `connectors()` rewritten from a double-collect
  (filter → collect → map → collect) into a single chained iterator
  pass.
  - **`pile.rs`** — two broken `txt`-fenced doc blocks (referencing
    removed APIs) replaced with one compiled, working example exercising
    `ranks_index`, `suit_symbol_index`, `suits_index`, `draw`, and `len`.
  - **`basic.rs`** — added a struct-level doc comment to `BasicPileCell`
    explaining why `Cell<T>` is used and the take → mutate → set pattern.
  - **`razz.rs`** — added `log::error!` to the YAML load failure path so
    load failures aren't silent. Cleaned up a dangling
    `"This is an"` doc fragment.
  - **`canasta.rs`** — removed `.shuffled()` from `ranks_index` and
    `suits_index` tests (the shuffle had no observable effect, since
    `extract_pips` deduplicates and sorts). Replaced
    `// WTF??!!`–style comments with an explanation of the 11-suit output.
  - **`localization.rs`** — replaced the
    `// should we remove fluent-templates?` TODO with an inline note
    explaining that fluent-templates is intentionally retained for
    i18n extensibility.

### Deprecated
- **`Pile::piles_to_string()`** marked `#[deprecated]`. (Still callable
  in this release; replacement guidance lands with the function's
  removal.)

### Removed
- **`bin/ayce`** and **`bin/README.md`** — replaced by `Makefile`
  targets.

### Fixed
- **Non-deterministic canasta test** — a test in `canasta.rs` was
  failing intermittently because of a shuffle-then-compare against an
  unsorted expected value. Fix landed before the cleanup pass and is
  the reason this release exists.

### Removed (stale state)
- TODO comment on `BasicCard::ckc_suit_number()` (the function was
  already in the right place inside `impl CKCRevised for BasicCard`).

---

## v0.6.11 — Mutation-test coverage hardening

PR [#67](https://github.com/ImperialBower/cardpack.rs/pull/67) ·
2026-04-13

A focused pass driven by `cargo-mutants`. ~70 new unit tests were added
to close every catchable mutation-testing gap. No behavioral or API
changes.

### Internal — new tests by file
- **`src/common/utils.rs`** — 4 tests for `ckc_bits`, `ckc_prime`,
  `ckc_shift8`, `strip_suit_flags`.
  - **`src/basic/types/pips.rs`** — 1 `Display` test.
  - **`src/basic/types/basic_card.rs`** — 2 `is_blank` half-blank tests.
  - **`src/basic/decks/{standard52,french,canasta,euchre24,euchre32,
    pinochle,razz,short,skat,spades,tarot,tiny}.rs`** — `colors`,
    `deck_name`, `fluent_deck_key` tests per deck.
  - **`src/basic/types/basic_pile.rs`** — 7 tests for `draw`,
    `draw_first`, `extend`, `get`, `is_empty`, `pop`.
  - **`src/basic/types/basic.rs`** — 13 tests for every
    `BasicPileCell` operation.
  - **`src/basic/types/combos.rs`** — 10 tests for accessor / iterator
    methods.
  - **`src/localization.rs`** — 8 tests, including a new
    `WeightedName` helper struct for parameterized assertions.
  - **`src/basic/types/traits.rs`** — new test module, 5 tests.
  - **`src/basic/types/pile.rs`** — 10 tests for previously-untested
    methods and trait impls.
  - **`src/basic/types/card.rs`** — 4 tests, including all 13 `Color`
    variants exercised through macro-generated deck types.

### Documented as irreducible / expected mutants
The remaining `cargo-mutants` survivors are not catchable by tests:

- `ckc_rank_number` — `|` → `^` (equivalent mutant: bits don't overlap).
  - `validate` — `→ true` and `&& → ||` (would require a deck the
    validator already rejects).
  - `demo` body — `→ ()` (no output verification — `demo` only prints).

---

## v0.6.12 — Features, locales, wasm, seeded shuffle

PR [#68](https://github.com/ImperialBower/cardpack.rs/pull/68) ·
2026-05-01

The implementation pass against
[`docs/audit-2026-04-29.md`](audit-2026-04-29.md). Default features are
unchanged from v0.6.11 — every new opt-out flag is on by default.

### Added
- **Cargo features** — `i18n`, `colored-display`, `yaml`, `serde`. All
  enabled in `default`.

  ```toml
  cardpack = { version = "0.6", default-features = false, features = ["serde"] }
  ```

  `yaml` implies `serde`. See `Cargo.toml` and the README's
  "Cargo features" section.

  - **Deterministic shuffle** — `Pile::shuffle_with_seed(u64)`,
    `Pile::shuffled_with_seed(u64)`, and generic-RNG variants
    `shuffle_with_rng<R>` / `shuffled_with_rng<R>`, on both `BasicPile`
    and `Pile<T>`. Use the seeded variants for reproducible game state.

  - **`DeckKind` enum** — non-generic registry of every shipped deck.
    Re-exported from `prelude`. `DeckKind::all()` returns
    `&'static [DeckKind]`; `DeckKind::base_vec()`, `deck_name()`,
    `fluent_deck_key()`, and `demo()` dispatch to the underlying typed
    deck. The `Razz` variant is gated on the `yaml` feature.

  - **WebAssembly support** — cardpack compiles cleanly to
    `wasm32-unknown-unknown` across all feature combinations.
    `getrandom = { features = ["wasm_js"] }` is wired in as a
    target-gated dep so consumers don't need to add it themselves. See
    [`docs/wasm.md`](wasm.md) and [`examples/wasm.rs`](../examples/wasm.rs).

  - **New locale drafts** — `fr` (French), `la` (Latin),
    `tlh` (Klingon). All marked DRAFT pending native-speaker / KLI
    review; per-file confidence notes live in each locale's `README.md`.

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
  and Klingon use `" "` (space), German remains `" "`. Previously every
  non-DEUTSCH locale silently fell back to the English `" of "`.

  - **`Pinochle::fluent_deck_key`** and **`Canasta::fluent_deck_key`**
    now return the French key. Removes the silent FTL fallback for those
    decks. The public constants `FLUENT_KEY_BASE_NAME_PINOCHLE` and
    `FLUENT_KEY_BASE_NAME_CANASTA` are retained but no longer referenced
    internally.

  - **`FluentName::new_with_weight`** no longer panics. It now discards
    the weight (since `FluentName` has no weight field) and delegates to
    `FluentName::new`. Behavior is documented on the impl.

  - **README** — feature-flag table, WebAssembly section, fixed broken
    links to locale files, updated `serde_yml` references to
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
- [`docs/audit-2026-04-29.md`](audit-2026-04-29.md) — full gap analysis
  with per-item status icons.
  - [`docs/wasm.md`](wasm.md) — consumer setup guide, recommended feature
    combos, runtime gotchas.
  - [`docs/2026-04-29-seeded-shuffle-design.md`](2026-04-29-seeded-shuffle-design.md)
    and [`docs/2026-04-29-seeded-shuffle-plan.md`](2026-04-29-seeded-shuffle-plan.md) —
    design and plan for the deterministic shuffle work.
  - [`docs/2026-04-29-la-tlh-locales-design.md`](2026-04-29-la-tlh-locales-design.md) —
    Latin and Klingon locale design notes.
  - Locale READMEs (`fr/`, `la/`, `tlh/`) — confidence levels and
    reviewer profiles needed before promotion.

### Internal
- 10 property tests in `tests/properties.rs` (proptest) covering
  shuffle / sort / draw / `pile_on` invariants.
  - 5 wasm runtime tests in `tests/wasm.rs` (wasm-bindgen-test,
    node-headless).
  - New CI jobs: `doc` (rustdoc with `-D warnings`), `wasm-build`,
    `wasm-test`.
  - Makefile gained `make build-wasm`, `make test-wasm`,
    `make install-wasm-bindgen-cli`.
  - `[package.metadata.docs.rs] all-features = true` so docs.rs matches
    the new CI doc gate.

---

## v0.7.0 — `no_std` and deterministic collections

PR [#69](https://github.com/ImperialBower/cardpack.rs/pull/69) ·
2026-05-02

Adds `no_std` support and migrates internal/return collection types to
deterministic (`BTreeSet` / `BTreeMap`) variants. Default features are
unchanged.

### Breaking
- **`Pile::into_hashset()` renamed to `Pile::unique_cards()`**;
  return type changed from `HashSet<Card<DeckType>>` to
  `BTreeSet<Card<DeckType>>`.
  - **`From<HashSet<Card<DeckType>>> for Pile<DeckType>` removed**;
    replaced with `impl FromIterator<Card<DeckType>> for Pile<DeckType>`,
    enabling `.collect::<Pile<_>>()` from any iterator. See
    [Migration guide §2](#2-fromhashsetcarddecktype-for-piledecktype-removed).
  - **`Pile::map_by_suit()` and `Ranged::map_by_rank()`** return
    `BTreeMap` instead of `HashMap` (deterministic iteration; identical
    lookup semantics).
  - **No-arg `Pile::shuffle()` / `shuffled()`,
    `BasicPile::shuffle()` / `shuffled()`,
    `BasicPileCell::shuffle()` / `shuffled()`,
    and `Pile::draw_random()`** now require the `std` feature
    (still default-on). Under `--no-default-features`, use
    `shuffle_with_seed(u64)` or `shuffle_with_rng(&mut R)`.

### Added
- **`no_std` support** via `extern crate alloc`. Build with
  `--no-default-features` for an alloc-only build that targets embedded
  Rust (`thumbv7em-none-eabihf` and similar).
  - **New `std` feature** (default-on). Existing features (`i18n`,
    `colored-display`, `yaml`) implicitly require `std`; `serde` implies
    `alloc`.
  - **New `alloc` feature** (mostly internal plumbing for
    `serde?/alloc`).
  - **CI** gates on
    `cargo build --no-default-features --target thumbv7em-none-eabihf`.
  - **`examples/no_std_smoke.rs`** — bare-metal compile + link smoke
    binary with a minimal static bump allocator.
  - **New Make targets** — `make no-std`, `make no-std-thumbv7`.

### Internal
- `Ranged::extract_pips` switched from `HashSet` → `BTreeSet`,
  removing a redundant sort step.
  - `Ranged::combos` switched from `HashSet<BasicPile>` →
    `BTreeSet<BasicPile>`; the post-collect `.sort()` is now redundant
    and removed.
  - `HashMap` imports in `pile.rs` and `traits.rs` are gated on
    `colored-display` (the only feature that uses `HashMap` return
    types).
  - Library source migrated from `std::*` to `core::*` / `alloc::*` for
    re-exported types (`fmt`, `str::FromStr`, `cmp::Ordering`, `hash`,
    `cell::Cell`, `marker::PhantomData`, `error::Error`,
    `vec::IntoIter`).
  - `#[macro_use] extern crate alloc;` brings `format!` / `vec!` macros
    into scope crate-wide for `no_std` builds.
  - Dev-deps that require `std` (`clap`, `ckc-rs`, `env_logger`,
    `rstest`, `term-table`, `criterion`, `proptest`) are gated off
    `cfg(target_os = "none")` so cargo doesn't try to compile them when
    building examples for bare-metal targets.

### Documentation
- [`docs/superpowers/specs/2026-05-01-no-std-alloc-design.md`](superpowers/specs/2026-05-01-no-std-alloc-design.md) —
  design.
  - [`docs/superpowers/plans/2026-05-01-no-std-alloc.md`](superpowers/plans/2026-05-01-no-std-alloc.md) —
    implementation plan.

---

## Compatibility matrix

| Target / config | v0.6.8 | v0.7.0 |
|---|---|---|
| `cargo build` (default features) | ✅ | ✅ |
| `cargo build --no-default-features` | ✅ (std-only) | ✅ (alloc-only, embeddable) |
| `wasm32-unknown-unknown` | ❌ (broken on some flag combos) | ✅ (CI-gated) |
| `thumbv7em-none-eabihf` (bare metal) | ❌ | ✅ (CI-gated) |
| Deterministic shuffle | ❌ | ✅ (`shuffle_with_seed`) |
| Deterministic `unique_cards` / `map_by_*` | ❌ (`HashSet` / `HashMap`) | ✅ (`BTreeSet` / `BTreeMap`) |
| Serde for `Pile` / `BasicPile` | ❌ | ✅ (opt-out `serde` feature) |
| Locales | en-US, de | en-US, de, fr (DRAFT), la (DRAFT), tlh (DRAFT) |

---

## Links

- Source: <https://github.com/ImperialBower/cardpack.rs>
  - Compare view: <https://github.com/ImperialBower/cardpack.rs/compare/v0.6.8...v0.7.0>
  - crates.io: <https://crates.io/crates/cardpack>
  - docs.rs: <https://docs.rs/cardpack>
  - Changelog: [`CHANGELOG.md`](../CHANGELOG.md)
