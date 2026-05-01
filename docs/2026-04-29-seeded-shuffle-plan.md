# Seeded Shuffle Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add deterministic shuffle (`shuffle_with_seed` / `shuffled_with_seed`) and a generic bring-your-own-RNG variant (`shuffle_with_rng` / `shuffled_with_rng`) to `BasicPile` and `Pile<DeckType>`, resolving the `basic_pile.rs:63` TODO and unblocking property tests (audit row #4).

**Architecture:** Eight new methods total (4 per type), structurally symmetric. `shuffle_with_seed(u64)` is sugar over `shuffle_with_rng<R>` — it constructs `StdRng::seed_from_u64(seed)` and forwards. The non-seeded `shuffle()`/`shuffled()` methods are unchanged. No new dependencies.

**Tech Stack:** Rust 2024 edition, `rand 0.9` (`rand::rngs::StdRng`, `rand::SeedableRng`, `rand::Rng`, existing `rand::seq::SliceRandom`), `cargo test --lib` for verification.

**Spec:** [`docs/2026-04-29-seeded-shuffle-design.md`](./2026-04-29-seeded-shuffle-design.md)

---

## File Structure

| File | Change | Responsibility |
|---|---|---|
| `src/basic/types/basic_pile.rs` | Modify lines 1–8 (imports), 61–74 (shuffle methods); add tests in `mod basic__types__pile_tests` (around line 382+) | Core `BasicPile` shuffle API + tests |
| `src/basic/types/pile.rs` | Modify imports (around lines 1–15), 672–682 (shuffle methods); add tests in `mod basic__types__deck_tests` (around line 959+) | `Pile<DeckType>` shuffle API + tests |
| `docs/audit-2026-04-29.md` | Modify §9, §14 third bullet, §16 row #3 | Mark resolution; reflect new API |

No new files. The spec at `docs/2026-04-29-seeded-shuffle-design.md` is already committed.

---

## Task 1: Add seeded shuffle to `BasicPile` (TDD)

**Files:**
- Modify: `src/basic/types/basic_pile.rs:1-8` (imports), `:61-74` (replace TODO + existing shuffle/shuffled methods)
- Test: `src/basic/types/basic_pile.rs` test module starting at line 382 (`mod basic__types__pile_tests`)

- [ ] **Step 1.1: Add imports**

In `src/basic/types/basic_pile.rs`, replace the existing rand imports:

```rust
use rand::prelude::SliceRandom;
use rand::rng;
```

with:

```rust
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};
```

- [ ] **Step 1.2: Write the failing test for determinism**

Append to the test module `mod basic__types__pile_tests` in `src/basic/types/basic_pile.rs` (find the closing `}` of the module — file ends around line 731):

```rust
    #[test]
    fn shuffled_with_seed__deterministic() {
        let pile = French::deck().basic_pile();
        let a = pile.shuffled_with_seed(42);
        let b = pile.shuffled_with_seed(42);
        assert_eq!(a, b, "same seed must produce identical permutation");
    }
```

(Note: use whichever construction pattern is established in the existing tests in this file. From line 514: `Pile::<French>::basic_pile().shuffled()`. Adjust if the existing tests use `French::deck().into_basic_pile()` or another form — match what's there.)

- [ ] **Step 1.3: Run test — verify it fails**

Run:
```sh
cargo test --lib basic_pile::basic__types__pile_tests::shuffled_with_seed__deterministic 2>&1 | tail -20
```

Expected: compile error (`shuffled_with_seed` not found on `BasicPile`).

- [ ] **Step 1.4: Implement seed methods (inline StdRng, will refactor in Step 2)**

Replace lines 61–74 of `src/basic/types/basic_pile.rs` (current `shuffle` + `shuffled` block including the TODO comment):

```rust
    /// Shuffles the `BasicPile` in place using the process default RNG
    /// (`rand::rng()`). For deterministic shuffling, use
    /// [`shuffle_with_seed`](Self::shuffle_with_seed).
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    /// Returns a new shuffled version of the `BasicPile` using the process
    /// default RNG. For deterministic shuffling, use
    /// [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the `BasicPile` in place deterministically from a `u64` seed.
    ///
    /// Uses `rand::rngs::StdRng` internally. Same seed produces the same
    /// permutation **within one `rand` major version**; a `rand` upgrade may
    /// change the result. For long-lived replay logs or cross-version
    /// reproducibility, pass a portable RNG (e.g., `ChaCha8Rng` from
    /// `rand_chacha`) to [`shuffle_with_rng`](Self::shuffle_with_rng) instead.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile();
    /// let a = pile.shuffled_with_seed(42);
    /// let b = pile.shuffled_with_seed(42);
    /// assert_eq!(a, b);
    /// ```
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new `BasicPile` shuffled deterministically from a `u64` seed.
    ///
    /// See [`shuffle_with_seed`](Self::shuffle_with_seed) for the
    /// portability caveat.
    #[must_use]
    pub fn shuffled_with_seed(&self, seed: u64) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_seed(seed);
        pile
    }

    /// Shuffles the `BasicPile` in place using the caller's RNG.
    ///
    /// Generic over any `R: Rng + ?Sized`. The seed-based methods are sugar
    /// over this primitive — pass your own RNG (e.g., `ChaCha8Rng`) for
    /// algorithm-stable reproducibility across `rand` major-version bumps.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// Returns a new `BasicPile` shuffled using the caller's RNG.
    ///
    /// See [`shuffle_with_rng`](Self::shuffle_with_rng) for the rationale
    /// of the generic variant.
    #[must_use]
    pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_rng(rng);
        pile
    }
```

(This deletes the `TODO: I would like to be able to pass in a seed to the shuffle function.` comment that was on the line above the old `shuffle` definition. The new `shuffle_with_seed` resolves that TODO.)

- [ ] **Step 1.5: Run the test — verify it passes**

Run:
```sh
cargo test --lib basic_pile::basic__types__pile_tests::shuffled_with_seed__deterministic 2>&1 | tail -10
```

Expected: 1 passed.

- [ ] **Step 1.6: Add the remaining two tests**

Append to the same test module:

```rust
    #[test]
    fn shuffled_with_seed__different_seeds_differ() {
        let pile = French::deck().basic_pile();
        let a = pile.shuffled_with_seed(1);
        let b = pile.shuffled_with_seed(2);
        assert_ne!(
            a, b,
            "different seeds should almost always produce different orderings"
        );
    }

    #[test]
    fn shuffled_with_seed__same_cards() {
        let pile = French::deck().basic_pile();
        let mut shuffled = pile.shuffled_with_seed(0xC0FFEE);
        let mut original = pile.clone();
        // Both should contain the same multiset of cards.
        // Convert to sorted vectors for equality check.
        let mut s_vec = shuffled.v().clone();
        let mut o_vec = original.v().clone();
        s_vec.sort();
        o_vec.sort();
        assert_eq!(o_vec, s_vec, "shuffle must permute, not transform");
        assert_eq!(pile.len(), shuffled.len());
    }
```

(Adjust the way you obtain the inner `Vec<BasicCard>` if `BasicPile` doesn't expose `v()` returning `&Vec<BasicCard>` — from the file's line 16, `v()` is the public accessor.)

- [ ] **Step 1.7: Run all three new tests**

```sh
cargo test --lib basic_pile::basic__types__pile_tests::shuffled_with_seed 2>&1 | tail -15
```

Expected: 3 passed.

- [ ] **Step 1.8: Run full test suite — confirm no regressions**

```sh
cargo test --lib 2>&1 | tail -5
```

Expected: 288 passed (was 285 → +3 new tests).

- [ ] **Step 1.9: Run clippy — confirm clean**

```sh
cargo clippy 2>&1 | tail -5
```

Expected: `Finished` line, no warnings.

- [ ] **Step 1.10: Commit (suggest to user)**

The user runs commits manually. Suggest:

```bash
git add src/basic/types/basic_pile.rs
git commit -m "Add seeded + generic-RNG shuffle to BasicPile

shuffle_with_seed(u64) and shuffled_with_seed(u64) provide deterministic
shuffling via rand::rngs::StdRng; shuffle_with_rng<R> and
shuffled_with_rng<R> let callers bring their own Rng for cross-version-
portable seeding (e.g., ChaCha8Rng).

Resolves the basic_pile.rs:63 TODO. Three new tests cover determinism,
seed-sensitivity, and permutation-correctness.

Audit row #3 / §9 / §14 third bullet — to be marked done in a follow-up
commit covering Pile<T> + audit doc."
```

---

## Task 2: Add seeded shuffle to `Pile<DeckType>` (mirror)

**Files:**
- Modify: `src/basic/types/pile.rs:1-15` (imports), `:672-682` (replace shuffle/shuffled methods)
- Test: `src/basic/types/pile.rs` test module starting at line 959 (`mod basic__types__deck_tests`)

- [ ] **Step 2.1: Add imports**

In `src/basic/types/pile.rs`, find the existing `use rand::...` lines (around lines 8–9):

```rust
use rand::seq::SliceRandom;
use rand::{Rng, rng};
```

Replace with:

```rust
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};
```

(If the existing import is `use rand::seq::SliceRandom`, keep that exact path — don't change to `prelude::`. Match what's already there. The actual existing imports per the file's line 8: `use rand::seq::SliceRandom;` and line 9: `use rand::{Rng, rng};` — adjust to add `SeedableRng` and `StdRng`.)

- [ ] **Step 2.2: Write a failing determinism test**

Append to the test module `mod basic__types__deck_tests` in `src/basic/types/pile.rs`:

```rust
    #[test]
    fn shuffled_with_seed__deterministic() {
        let deck = Pile::<Standard52>::deck();
        let a = deck.shuffled_with_seed(42);
        let b = deck.shuffled_with_seed(42);
        assert_eq!(a, b, "same seed must produce identical permutation");
    }
```

- [ ] **Step 2.3: Run test — verify it fails**

```sh
cargo test --lib pile::basic__types__deck_tests::shuffled_with_seed__deterministic 2>&1 | tail -20
```

Expected: compile error (method not found).

- [ ] **Step 2.4: Replace the existing `shuffle` / `shuffled` block (around lines 672–682) with the four-method block**

Locate in `src/basic/types/pile.rs`:

```rust
    /// `shuffled` feels so much better. Nice and succinct.
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }
```

Replace with:

```rust
    /// Shuffles the `Pile` in place using the process default RNG
    /// (`rand::rng()`). For deterministic shuffling, use
    /// [`shuffle_with_seed`](Self::shuffle_with_seed).
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    /// Returns a new shuffled `Pile` using the process default RNG.
    ///
    /// `shuffled` feels so much better. Nice and succinct. For deterministic
    /// shuffling, use [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the `Pile` in place deterministically from a `u64` seed.
    ///
    /// Uses `rand::rngs::StdRng` internally. Same seed produces the same
    /// permutation within one `rand` major version; a `rand` upgrade may
    /// change the result. For cross-version reproducibility, pass a portable
    /// RNG (e.g., `ChaCha8Rng` from `rand_chacha`) to
    /// [`shuffle_with_rng`](Self::shuffle_with_rng).
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let deck = Pile::<Standard52>::deck();
    /// let a = deck.shuffled_with_seed(42);
    /// let b = deck.shuffled_with_seed(42);
    /// assert_eq!(a, b);
    /// ```
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new `Pile` shuffled deterministically from a `u64` seed.
    ///
    /// See [`shuffle_with_seed`](Self::shuffle_with_seed) for the
    /// portability caveat.
    #[must_use]
    pub fn shuffled_with_seed(&self, seed: u64) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_seed(seed);
        pile
    }

    /// Shuffles the `Pile` in place using the caller's RNG.
    ///
    /// Generic over any `R: Rng + ?Sized`. The seed-based methods are
    /// sugar over this primitive — pass your own RNG for algorithm-stable
    /// reproducibility across `rand` major-version bumps.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// Returns a new `Pile` shuffled using the caller's RNG.
    #[must_use]
    pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_rng(rng);
        pile
    }
```

- [ ] **Step 2.5: Run test — verify it passes**

```sh
cargo test --lib pile::basic__types__deck_tests::shuffled_with_seed__deterministic 2>&1 | tail -10
```

Expected: 1 passed.

- [ ] **Step 2.6: Add the remaining two tests**

Append to the same test module:

```rust
    #[test]
    fn shuffled_with_seed__different_seeds_differ() {
        let deck = Pile::<Standard52>::deck();
        assert_ne!(
            deck.shuffled_with_seed(1),
            deck.shuffled_with_seed(2),
            "different seeds should almost always produce different orderings"
        );
    }

    #[test]
    fn shuffled_with_seed__same_cards() {
        let deck = Pile::<Standard52>::deck();
        let shuffled = deck.shuffled_with_seed(0xC0FFEE);
        assert_eq!(deck.len(), shuffled.len());
        let mut o = deck.cards().clone();
        let mut s = shuffled.cards().clone();
        o.sort();
        s.sort();
        assert_eq!(o, s, "shuffle must permute, not transform");
    }
```

- [ ] **Step 2.7: Run all six seeded tests across both files**

```sh
cargo test --lib shuffled_with_seed 2>&1 | tail -10
```

Expected: 6 passed (3 in `basic_pile`, 3 in `pile`).

- [ ] **Step 2.8: Run full test suite + clippy**

```sh
cargo test --lib 2>&1 | tail -5 && cargo clippy 2>&1 | tail -5
```

Expected: 291 tests passed; clippy clean.

- [ ] **Step 2.9: Commit (suggest to user)**

```bash
git add src/basic/types/pile.rs
git commit -m "Mirror seeded + generic-RNG shuffle to Pile<DeckType>

Same four-method API as BasicPile (Task 1): shuffle_with_seed,
shuffled_with_seed, shuffle_with_rng, shuffled_with_rng. Three new tests
exercise determinism, seed-sensitivity, and permutation correctness on
Pile<Standard52>.

Audit row #3 / §9 / §14 third bullet now closeable — see follow-up commit
for doc updates."
```

---

## Task 3: Update audit doc (`docs/audit-2026-04-29.md`)

**Files:**
- Modify: `docs/audit-2026-04-29.md` §9 (paragraph after the property test code), §14 (third bullet "Shuffle seed"), §16 (row #3)

- [ ] **Step 3.1: Update §9**

In `docs/audit-2026-04-29.md`, find the paragraph that begins with "Note: this also exposes a missing API". Replace:

```markdown
Note: this also exposes a missing API — `shuffled_with_seed` doesn't exist (per `basic_pile.rs:63` TODO "Shuffle seed parameter"). Adding deterministic shuffling unblocks property testing.
```

with:

```markdown
Note (✅ resolved 2026-04-29): seeded shuffle (`shuffle_with_seed` /
`shuffled_with_seed`) plus generic-RNG variants (`shuffle_with_rng` /
`shuffled_with_rng`) now exist on both `BasicPile` and `Pile<T>`. The
`basic_pile.rs:63` TODO is removed. Property testing (row #4) is
unblocked.
```

- [ ] **Step 3.2: Update §14 third bullet**

Find:

```markdown
- **Shuffle seed**: `basic_pile.rs:63` TODO. Without it, no determinism for tests, replays, or networked games.
```

Replace with:

```markdown
- **Shuffle seed**: ✅ done 2026-04-29. `shuffle_with_seed(u64)` /
  `shuffled_with_seed(u64)` plus generic `shuffle_with_rng<R>` /
  `shuffled_with_rng<R>` now ship on `BasicPile` and `Pile<T>`. The
  generic variant is the escape hatch for callers who need cross-`rand`-
  version reproducibility (pass their own `ChaCha8Rng` etc.).
```

- [ ] **Step 3.3: Update §16 row #3**

Find:

```markdown
| 3  | **M**    | Add deterministic shuffle (`shuffle_with_seed`, `shuffled_with_seed`)                    | 1 hr     | `basic_pile.rs:63` already flagged                                                           |
```

Replace with:

```markdown
| 3  | ✅ done  | Add deterministic shuffle (`shuffle_with_seed`, `shuffled_with_seed`)                    | 1 hr     | resolved 2026-04-29 — `BasicPile` + `Pile<T>` got 4 methods each (seed sugar + generic `_with_rng` for portability); 6 tests cover determinism, sensitivity, permutation; TODO at `basic_pile.rs:63` removed. |
```

- [ ] **Step 3.4: Verify the audit doc still renders cleanly**

```sh
head -1 docs/audit-2026-04-29.md
grep -n "✅ done\|🟡 drafted" docs/audit-2026-04-29.md | head -10
```

Expected: row #1 (README), #3 (this), 6a, 6d are ✅ done; 6b, 6e, 6f are 🟡 drafted.

- [ ] **Step 3.5: Commit (suggest to user)**

```bash
git add docs/audit-2026-04-29.md
git commit -m "Mark audit row #3 (seeded shuffle) resolved

§9, §14 third bullet, §16 row #3 updated to reflect the seeded shuffle
landed in the prior two commits."
```

---

## Task 4: Final verification

- [ ] **Step 4.1: Full umbrella check**

```sh
make ayce 2>&1 | tail -10
```

Expected: build clean; 291 tests pass; 0 clippy warnings; doc generation succeeds.

- [ ] **Step 4.2: Confirm git status reflects only intended files**

```sh
git status
```

Expected: working tree clean (after the three commits in Tasks 1–3).

- [ ] **Step 4.3: Spot-check rustdoc**

```sh
cargo doc --no-deps --open 2>&1 | tail -5
```

Or equivalent: confirm the four new methods on `BasicPile` and `Pile<DeckType>` show up in the generated docs with the expected ordering (seed methods first, generic methods second).

---

## Self-review notes

**Spec coverage (against `docs/2026-04-29-seeded-shuffle-design.md`):**

- §2 API (8 methods): Tasks 1.4 + 2.4 implement all 4 methods on each type.
- §3 RNG choice (StdRng for seed sugar, generic for power users): Tasks 1.4 + 2.4.
- §4 Tests (3 properties × 2 types = 6 tests): Tasks 1.2/1.6 + 2.2/2.6.
- §5 No impact on existing API: Tasks 1.4 + 2.4 keep `shuffle()`/`shuffled()` semantics.
- §6 Imports: Tasks 1.1 + 2.1.
- §7 Audit doc updates: Task 3.
- §8 Verification: Task 4.

**Placeholder check:** All steps have concrete code, paths, expected output, and commit messages. No TBD/TODO patterns.

**Type/name consistency:** Method names (`shuffle_with_seed`, `shuffled_with_seed`, `shuffle_with_rng`, `shuffled_with_rng`) match across all tasks. `&mut self` for mutating, `&self -> Self` for cloning variants. Type bound `R: Rng + ?Sized` consistent.

**Open: deck-construction idiom in tests.** Step 1.2 / 1.6 use `French::deck().basic_pile()`; if the existing tests in `basic_pile.rs` use a different form (`Pile::<French>::basic_pile()` etc.), match the file's local idiom. Same for Pile<T> tests in Step 2.2 / 2.6.
