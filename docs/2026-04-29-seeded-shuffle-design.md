# Design: Deterministic Shuffle (`shuffle_with_seed` / `shuffle_with_rng`)

**Date:** 2026-04-29
**Branch:** gapped
**Audit reference:** [`docs/audit-2026-04-29.md`](./audit-2026-04-29.md) §9, §14, §16 row #3

---

## 1. Purpose

Resolve the `TODO: I would like to be able to pass in a seed to the shuffle function`
at `src/basic/types/basic_pile.rs:63`. Add deterministic shuffling to
`BasicPile` and `Pile<DeckType>`. Unblocks property testing (audit row #4),
test fixtures with reproducible state, and replay-style use cases.

## 2. API

Eight new methods total. Four on `BasicPile`, four on `Pile<DeckType>`,
structurally identical.

### Per type:

```rust
/// Shuffle in place deterministically from a `u64` seed.
pub fn shuffle_with_seed(&mut self, seed: u64);

/// Return a new shuffled copy from a `u64` seed.
#[must_use]
pub fn shuffled_with_seed(&self, seed: u64) -> Self;

/// Shuffle in place using the caller's RNG.
pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R);

/// Return a new shuffled copy using the caller's RNG.
#[must_use]
pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self;
```

### Implementation hierarchy

`shuffle_with_seed` is sugar over `shuffle_with_rng`:

```rust
pub fn shuffle_with_seed(&mut self, seed: u64) {
    self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
}
```

`shuffle_with_rng` is the leaf — it forwards to `SliceRandom::shuffle`, the
same Fisher-Yates implementation the existing `shuffle()` method uses.
`shuffled_with_*` clone-then-shuffle, mirroring the existing `shuffled()`
pattern.

### Source order in rustdoc

Seed variants documented first (most discoverable for the common case), then
the generic `_with_rng` variants. The generic methods are the
implementation primitive but the seed methods are the user-facing entry
point.

## 3. RNG choice

Internal RNG for `shuffle_with_seed` is `rand::rngs::StdRng`, available
without extra feature flags in `rand 0.9`. Construction:
`StdRng::seed_from_u64(seed)`.

### Tradeoff: portability across `rand` major versions

`StdRng`'s output is documented as **not portable** across `rand` major-
version bumps — same seed may produce different shuffles after a `rand`
upgrade. This is acceptable for:

- Tests (regenerate fixtures on `rand` bump).
- Short-lived replay (within one release).
- Property tests (deterministic within a test run; fixtures not persisted).

Not acceptable for:

- Long-lived game replay logs persisted across releases.
- Cross-platform seed exchange where peers may run different `rand`
  versions.

**Escape hatch**: `shuffle_with_rng` accepts any `R: Rng + ?Sized`. Users
who care can pass a fixed-algorithm RNG — e.g., `ChaCha8Rng` from
`rand_chacha` — and get full reproducibility. The library does not pull
`rand_chacha` as a default dep; users add it if they need it.

This split (sugar API for the common case, generic API for the
specialized case) is the canonical Rust idiom and lets the library serve
both audiences without dictating one.

## 4. Tests

Three new tests per type — six total. Cover three properties:

| Property | Test name | Assertion |
|---|---|---|
| Determinism | `shuffled_with_seed__deterministic` | same seed → identical order |
| Sensitivity | `shuffled_with_seed__different_seeds_differ` | different seeds → different orderings (with overwhelming probability) |
| Permutation | `shuffled_with_seed__same_cards` | shuffled set equals original set after sort |

The third property guards against accidental data corruption (e.g., if
the shuffle were ever rewritten to drop or duplicate cards).

`_with_rng` variants are exercised transitively by the seed tests
(seed sugar forwards into them); no direct test needed.

## 5. Existing API impact

None. The non-seeded `shuffle()` and `shuffled()` methods continue to use
`rng()` (process `ThreadRng`). The new methods are purely additive. Public
API surface grows by 8 methods (4 per type).

The `TODO` comment at `basic_pile.rs:63` is removed (resolved).

## 6. Imports

`basic_pile.rs` and `pile.rs` each gain:

```rust
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
```

The existing `use rand::rng;` and `use rand::prelude::SliceRandom;` (or
`use rand::seq::SliceRandom;`) stay.

## 7. Audit doc updates

- **§9** — strike "*this also exposes a missing API — `shuffled_with_seed`
  doesn't exist*"; replace with "✅ resolved 2026-04-29: seeded shuffle
  shipped on `BasicPile` and `Pile<T>`; property tests (row #4) are now
  unblocked."
- **§14** third bullet — "**Shuffle seed**: `basic_pile.rs:63` TODO" → "✅
  done 2026-04-29".
- **§16 row #3** — `**M**` → `✅ done` with resolution note.

## 8. Verification

```sh
cargo build         # confirms imports compile
cargo test --lib    # 285 → 291 tests (6 new); all pass
cargo clippy        # remains clean
```

End-to-end success criteria:

1. ✅ `cargo build` succeeds.
2. ✅ All 6 new tests pass; existing 285 unaffected.
3. ✅ `cargo clippy` reports no new warnings (the prior `sort_by_key` fix
   keeps it clean).
4. ✅ Audit doc accurately reflects row #3 resolution.
5. ✅ TODO at `basic_pile.rs:63` is removed.

## 9. Out of scope

- Property tests (audit row #4). Unblocked but separate.
- `rand_chacha` as a default dep. Users add it ad-hoc.
- Same-seed-cross-`rand`-version reproducibility tests. Wrong tool for
  `StdRng`; portability-conscious users use `_with_rng` with their own RNG.
- Seeded variants on derived APIs (e.g., `Pile::pile_up`,
  `Pile::demo_cards`). Callers compose with the new methods if needed.

## 10. Trajectory

After this lands:

- Audit row #4 (property tests) is the obvious follow-up; the seeded
  shuffle is its prerequisite.
- A future minor-bump may want to expose a `RandomSource` trait or similar
  abstraction if more APIs grow seeded variants. Not warranted for one
  shuffle.
