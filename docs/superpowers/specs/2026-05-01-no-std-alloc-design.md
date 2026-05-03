# `no_std` + `alloc` Support — 0.7.0 Design

**Date:** 2026-05-01
**Branch:** main (feature branch to be created at implementation time)
**Target version:** 0.7.0
**Audit reference:** [`docs/audit-2026-04-29.md` §5, §16 row 13b](../../audit-2026-04-29.md)
**Status:** Draft pending implementation

---

## 1. Goal

Resolve audit item 13b: gate cardpack's alloc-only core behind a feature so the
crate compiles and links on bare-metal Rust targets (e.g.,
`thumbv7em-none-eabihf`). The wasm half of the original "no_std/wasm" audit
item was resolved in 13a (2026-04-30); this spec covers the embedded-Rust
half.

This is shipped as **0.7.0** — a deliberate breaking-change release. Audit §4
already deferred a 0.7.0-shaped break for default features; bundling those
decisions saves downstream from two consecutive migrations.

## 2. Non-goals (out of scope)

- **Pure no_std (no alloc).** `Pile` wraps `Vec`; would require an
  architectural rewrite to a fixed-size backing store. Not pursued.
- **QEMU runtime test of the smoke binary.** Compile/link on
  `thumbv7em-none-eabihf` is the agreed verification level (Q2: rigor "C"
  interpreted as "real-target compile + link smoke binary").
- **Embedded HAL / `defmt` integration.** Consumer concern, not cardpack's.
- **`Vec → VecDeque` for O(1) draw** (audit §15 / `pile.rs:597`). Separate
  effort. Not bundled.
- **Removing `colored::Color` from `DeckedBase`.** Already gated on
  `colored-display`; redesigning the trait is a separate semver decision.
- **`fluent-templates` replacement.** Audit §2 deferred; no_std does not change
  that decision (`i18n` simply implies `std`).

## 3. Decisions locked in (from brainstorming Q&A)

| # | Question | Answer |
|---|---|---|
| 1 | Breaking change appetite | **0.7.0 release; breaking changes acceptable** |
| 2 | Verification rigor | **C** — real-target compile + link smoke binary on `thumbv7em-none-eabihf`; **no QEMU runtime** |
| 3 | API rename for `into_hashset` | **B (semantic)** — `unique_cards` + `FromIterator` |
| 4 | Feature-flag layout | **Layout 2** — `BTreeMap`/`BTreeSet` is the public type unconditionally; `std` feature gates only `rand::rng()` callers |

## 4. Architecture

### 4.1 Cargo.toml `[features]` table

```toml
[features]
default = ["std", "i18n", "colored-display", "yaml", "serde"]
std = ["alloc", "rand/std", "rand/std_rng", "serde?/std", "log/std"]
alloc = ["serde?/alloc"]
i18n = ["std", "dep:fluent-templates"]
colored-display = ["std", "dep:colored"]
yaml = ["std", "dep:serde_norway", "serde"]
serde = ["dep:serde"]
```

Notes:
- `alloc` is non-optional in practice (`Pile` wraps `Vec`), but is exposed as a
  feature so `serde?/alloc` chains correctly without needing `std`.
- `rand/std_rng` is requested separately from `rand/std` because `StdRng` is
  used by `shuffle_with_seed` even under no_std; only `rand::rng()`
  (thread-local) is `std`-gated.
- All four pre-existing features (`i18n`, `colored-display`, `yaml`, `serde`)
  imply `std`, so toggling any of them off does not break the no_std story.

### 4.2 Dependency adjustments

```toml
itertools = { version = "0.14.0", default-features = false, features = ["use_alloc"] }
serde = { version = "1.0.219", default-features = false, features = ["derive"], optional = true }
log = { version = "0.4.29", default-features = false }
thiserror = { version = "2.0.18", default-features = false }
```

`thiserror 2.x` already uses `core::error::Error` (stable since Rust 1.81;
cardpack's MSRV 1.85 is comfortable). `colored`, `fluent-templates`, and
`serde_norway` remain unchanged — they live behind their existing
std-implying features.

### 4.3 Crate-root attributes (`src/lib.rs`)

```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
```

Lint attributes already present (`clippy::pedantic`, etc.) are unaffected.

### 4.4 MSRV

Stays at **1.85**. No bump required.

## 5. Public API changes (the user-visible 0.7.0 surface)

### 5.1 Renamed / reshaped

| Before | After | Reason |
|---|---|---|
| `Pile::into_hashset(&self) -> HashSet<Card<DeckType>>` | `Pile::unique_cards(&self) -> BTreeSet<Card<DeckType>>` | Semantic, type-agnostic name |
| `impl From<HashSet<Card<DeckType>>> for Pile<DeckType>` | `impl FromIterator<Card<DeckType>> for Pile<DeckType>` | Stronger contract; enables `.collect::<Pile<_>>()` |
| `Pile::map_by_suit(&self) -> HashMap<Pip, Self>` | `Pile::map_by_suit(&self) -> BTreeMap<Pip, Self>` | Name unchanged; deterministic iteration order is an upgrade |
| `Ranged::map_by_rank(&self) -> HashMap<Pip, BasicPile>` | `Ranged::map_by_rank(&self) -> BTreeMap<Pip, BasicPile>` | Same |

### 5.2 Trait-bound relaxation

The `Hash` bound on the old `From<HashSet>` impl is removed (no longer needed
once the impl is `FromIterator`-based). Existing `Hash` impls on `Card`,
`BasicCard`, `Pip` remain — downstream users (poker engines, replay systems)
hash card identities into their own collections; removing those impls would
break them with no benefit.

### 5.3 Methods gated behind `feature = "std"`

These produce a clean compile error under `--no-default-features` rather than
silent failure:

- `Pile::shuffle()` (no-arg) — `pile.rs:731`
- `Pile::shuffled()` (no-arg) — `pile.rs:722`
- `BasicPile::shuffle()` / `BasicPile::shuffled()` (no-arg)

(All seeded variants — `shuffle_with_seed`, `shuffle_with_rng`, etc. — work
under no_std unchanged. They were added in audit item #3 / §9.)

`DeckedBase::colors() -> HashMap<Pip, colored::Color>` is already gated on
`colored-display` (which now implies `std`), so the trait method itself
keeps its `HashMap` return type. No changes to the 12 deck `colors()`
implementations.

### 5.4 Migration cheat-sheet (for downstream users)

```rust
// 1. into_hashset → unique_cards
- let s: HashSet<Card<Standard52>> = pile.into_hashset();
+ let s: BTreeSet<Card<Standard52>> = pile.unique_cards();
// or, to keep HashSet specifically:
+ let s: HashSet<Card<Standard52>> = pile.iter().copied().collect();

// 2. From<HashSet> → FromIterator
- let pile = Pile::<Standard52>::from(my_hashset);
+ let pile: Pile<Standard52> = my_hashset.into_iter().collect();

// 3. HashMap returns from map_by_suit / map_by_rank
//    Call sites unchanged; iteration order is now sorted by Pip.
```

## 6. Internal mechanical migration

### 6.1 Import substitutions (repo-wide)

```rust
// std → core (re-exports; zero behavioral change)
use std::fmt::{Display, Formatter, Debug};   →  use core::fmt::...
use std::str::FromStr;                       →  use core::str::FromStr;
use std::cmp::Ordering;                      →  use core::cmp::Ordering;
use std::hash::Hash;                         →  use core::hash::Hash;
use std::cell::Cell;                         →  use core::cell::Cell;
use std::marker::PhantomData;                →  use core::marker::PhantomData;
use std::error::Error;                       →  use core::error::Error;

// std → alloc
use std::vec::IntoIter;                      →  use alloc::vec::IntoIter;
use std::collections::{HashMap, HashSet};    →  use alloc::collections::{BTreeMap, BTreeSet};
```

### 6.2 std-gated holdouts

Genuine std dependencies that stay behind their existing feature gates (no
new gating required):

- `src/basic/types/basic_card.rs:13–15` — `std::fs::File`, `std::io::Read`
  for YAML file loading. Already inside `cards_from_yaml_file`, gated under
  `yaml` (which now implies `std`).
- `src/basic/types/basic.rs:331` (test) — `DefaultHasher`. Wrap test in
  `#[cfg(feature = "std")]`.

### 6.3 Internal HashMap/HashSet swaps

- `traits.rs:349` — `extract_pips`: `HashSet<Pip>` → `BTreeSet<Pip>`. The
  trailing `.sort(); .reverse()` becomes `.into_iter().rev().collect()`
  (BTreeSet iterates in sorted order). Net cleaner code.
- `traits.rs:357` — `map_by_rank` body: `HashMap` → `BTreeMap`,
  `hash_map::Entry::Vacant` → `btree_map::Entry::Vacant`.
- `pile.rs:481` — `map_by_suit` body: same.

### 6.4 Doctests

~20 doctests reference `std::collections::{HashMap, HashSet}`. Each gets
updated to `BTreeMap`/`BTreeSet` or rewritten to use `unique_cards()`
directly. Mechanical edit; CI's doctest run catches misses.

### 6.5 Prelude

`src/prelude.rs:42`: `pub use std::str::FromStr;` → `pub use core::str::FromStr;`
(canonical trait; std's is a re-export).

## 7. Verification (CI)

### 7.1 Smoke binary — `examples/no_std_smoke.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use cardpack::prelude::*;
    use rand::{rngs::StdRng, SeedableRng};

    let deck: Pile<Standard52> = Pile::<Standard52>::deck();
    let _ = deck.len();
    let _ = deck.unique_cards();
    let _ = deck.map_by_suit();

    let mut rng = StdRng::seed_from_u64(42);
    let _ = deck.shuffled_with_rng(&mut rng);

    loop {}
}

#[cfg(feature = "std")]
fn main() {}  // host build is a no-op
```

Cargo.toml registration:

```toml
[[example]]
name = "no_std_smoke"
required-features = []
```

The binary's value is at the **link step**: monomorphizing
`Pile::<Standard52>::deck()`, `unique_cards`, `map_by_suit`, and
`shuffled_with_rng` forces the no_std code path through the linker. If any
internal path silently pulls in `std`, the
`thumbv7em-none-eabihf` build fails because `libstd` is not in the
sysroot. We do not run the binary; the exit code of `cargo build` is the
verification.

### 7.2 New CI jobs (`.github/workflows/CI.yaml`)

#### `no-std-build` (host gate)

```yaml
no-std-build:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo build --no-default-features
    - run: cargo build --no-default-features --features serde
    - run: cargo test  --no-default-features --lib
```

#### `no-std-thumbv7em` (real-target link gate)

```yaml
no-std-thumbv7em:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: thumbv7em-none-eabihf
    - run: cargo build --no-default-features --target thumbv7em-none-eabihf
    - run: cargo build --no-default-features --features serde --target thumbv7em-none-eabihf
    - run: cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke
```

### 7.3 Make targets

```makefile
no-std:
	cargo build --no-default-features
	cargo build --no-default-features --features serde

no-std-thumbv7:
	rustup target add thumbv7em-none-eabihf
	cargo build --no-default-features --target thumbv7em-none-eabihf
	cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke
```

Optionally fold both into the existing `make ayce` umbrella.

### 7.4 Feature matrix

The audit-§4 matrix already runs `default`, `--no-default-features`,
single-feature, and `--all-features`. The new combinations to add:

- `--no-default-features --features serde`
- `--no-default-features --features alloc` (mostly redundant; documents intent)
- `--no-default-features --features std`

## 8. Risk register

| Risk | Likelihood | Mitigation |
|---|---|---|
| Transitive dep silently pulls `std` despite `default-features = false` | Medium | The `thumbv7em` CI job is the mitigation — link failure surfaces it pre-release. Worst case: gate the dep on `std`. |
| Downstream user has hard `From<HashSet>` dependency | Low–Medium | Migration note + one-line `FromIterator` replacement. |
| Caller relied on `HashMap` ordering (which was undefined) | Very low | Note in CHANGELOG; ship. |
| Stale doctest with `std::collections::HashMap` | Low | All updated in same PR; CI doctest run catches misses. |

## 9. CHANGELOG.md entry (draft)

```markdown
## [0.7.0] — 2026-05-XX

### Breaking
- `Pile::into_hashset()` renamed to `Pile::unique_cards()`; return type
  changed from `HashSet<Card<DeckType>>` to `BTreeSet<Card<DeckType>>`.
- `From<HashSet<Card<DeckType>>> for Pile<DeckType>` removed; replaced
  with `impl FromIterator<Card<DeckType>> for Pile<DeckType>`.
- `Pile::map_by_suit()` and `Ranged::map_by_rank()` return `BTreeMap`
  instead of `HashMap` (deterministic iteration order; identical
  lookup semantics).
- `Pile::shuffle()` / `Pile::shuffled()` (no-arg) and
  `BasicPile::shuffle()` / `BasicPile::shuffled()` (no-arg) now require
  the `std` feature (still default-on). Under `--no-default-features`,
  use `shuffle_with_seed(u64)` or `shuffle_with_rng(&mut R)`.

### Added
- `no_std` support via `extern crate alloc`. Build with
  `--no-default-features` for an alloc-only build that targets embedded
  Rust (`thumbv7em-none-eabihf` and similar).
- New `std` feature (default-on); existing features (`i18n`,
  `colored-display`, `yaml`) implicitly require `std`.
- New `alloc` feature for fine-grained dep control.
- CI gates on `cargo build --no-default-features --target
  thumbv7em-none-eabihf`.
- `examples/no_std_smoke.rs` — bare-metal compile + link smoke binary.

### Internal
- `Ranged::extract_pips` switched from `HashSet` → `BTreeSet`,
  removing a redundant sort step.
```

## 10. Build sequence (suggested order for the implementation plan)

Each step leaves the codebase compiling; reorderings that cross the boundary
between "internal collection swap" and "public-API return-type change" must
be done together to avoid an intermediate broken state.

1. Cargo.toml feature edits + `default-features = false` on core deps.
2. Crate-root `#![cfg_attr(not(feature = "std"), no_std)]` + `extern crate alloc`.
3. Mechanical `std::` → `core::` import substitutions for re-exports
   (Display, FromStr, Ordering, Hash, Cell, PhantomData, Error,
   `vec::IntoIter`). Default-features build remains green; under
   `--no-default-features` the build still fails on the
   `std::collections` imports — that's the next step's job.
4. **`map_by_suit` / `map_by_rank` migration in one atomic step:** swap
   `use std::collections::HashMap` → `use alloc::collections::BTreeMap`,
   change function bodies (replace `HashMap::new()` /
   `hash_map::Entry` with `BTreeMap::new()` / `btree_map::Entry`), and
   update return types in the same edit. After this step both
   functions compile under both feature configurations.
5. **`extract_pips` internal swap:** `HashSet<Pip>` → `BTreeSet<Pip>`;
   drop redundant `.sort(); .reverse()` in favor of
   `.into_iter().rev()`.
6. **`unique_cards`:** rename `Pile::into_hashset` → `Pile::unique_cards`,
   change return type to `BTreeSet<Card<DeckType>>`. Single-file edit.
7. **`From<HashSet>` → `FromIterator`:** remove the `From<HashSet<Card>>`
   impl, add `impl FromIterator<Card<DeckType>> for Pile<DeckType>`.
8. Audit any remaining `use std::collections::{HashMap, HashSet}` import
   sites. Test code (e.g., `pile.rs:1172` referencing `HashSet`) gets
   either updated to `BTreeSet` or wrapped in `#[cfg(feature = "std")]`,
   whichever preserves test intent.
9. `#[cfg(feature = "std")]` gates on no-arg `Pile::shuffle()` /
   `Pile::shuffled()` and on `BasicPile::shuffle()` /
   `BasicPile::shuffled()`. (Seeded variants stay unconditional.)
10. Doctest updates (~20 mechanical edits to use `BTreeMap`/`BTreeSet`/
    `unique_cards`).
11. `examples/no_std_smoke.rs` + Cargo.toml `[[example]]` registration.
12. CI jobs (`no-std-build`, `no-std-thumbv7em`).
13. Make targets (`no-std`, `no-std-thumbv7`).
14. CHANGELOG.md 0.7.0 entry.
15. Bump `version = "0.7.0"` in Cargo.toml.

Verifiability: `cargo build` (default features) should be green after every
step; `cargo build --no-default-features` becomes green at step 4 and
should stay green thereafter.

## 11. Verification checklist

- [ ] `cargo build` (default features) — green
- [ ] `cargo build --no-default-features` — green
- [ ] `cargo build --no-default-features --features serde` — green
- [ ] `cargo build --all-features` — green
- [ ] `cargo test --all-features` — full test count (291+)
- [ ] `cargo test --no-default-features --lib` — reduced count, no failures
- [ ] `rustup target add thumbv7em-none-eabihf && cargo build
      --no-default-features --target thumbv7em-none-eabihf` — green
- [ ] `cargo build --no-default-features --target thumbv7em-none-eabihf
      --example no_std_smoke` — green
- [ ] `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings` — green
- [ ] `make ayce` umbrella — green
