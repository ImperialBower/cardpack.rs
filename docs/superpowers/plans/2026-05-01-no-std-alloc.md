# `no_std` + `alloc` Support (0.7.0) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make cardpack compile and link on `thumbv7em-none-eabihf` (bare-metal embedded Rust) under `--no-default-features`, while reshaping a small set of public APIs (HashMap/HashSet → BTreeMap/BTreeSet, `into_hashset` → `unique_cards`, `From<HashSet>` → `FromIterator`) for a clean 0.7.0 release.

**Architecture:** Add a default-on `std` feature; use `extern crate alloc` unconditionally; mechanically replace `std::` imports with `core::`/`alloc::` equivalents; gate `rand::rng()` callers behind `std`; ship a `no_std_smoke.rs` example whose role is to force monomorphization through the no_std code path so the linker — not the compiler — verifies the no_std story.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), `cargo`, `rand 0.9`, `serde 1.x`, `thiserror 2.x`, `itertools 0.14`, `log 0.4`. CI on GitHub Actions with `dtolnay/rust-toolchain`.

**Spec:** [`docs/superpowers/specs/2026-05-01-no-std-alloc-design.md`](../specs/2026-05-01-no-std-alloc-design.md)

---

## Conventions

- All file paths are relative to the repo root (`/Users/christoph/src/github.com/ImperialBower/cardpack.rs`).
- "Run: `cargo X`" means run from the repo root.
- "Default features green" is a precondition for every task — if it goes red mid-task, stop and diagnose before moving on.
- Every task ends with a commit. Frequent commits make bisects easy if `--no-default-features` regresses.
- Where TDD-style tests are not natural (e.g., Cargo.toml edits), the "test" is a `cargo build` invocation that should fail before and pass after — the build itself is the test.

---

## Task 1: Cargo.toml feature flags + dependency adjustments

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Verify default-features build is green (precondition)**

Run: `cargo build`
Expected: PASS — clean build under default features.

Run: `cargo test --all-features --lib`
Expected: PASS — all unit tests green at baseline.

- [ ] **Step 2: Edit `Cargo.toml` `[features]` table**

Replace the existing `[features]` block (lines 24-29) with:

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

- [ ] **Step 3: Edit dependency declarations to disable defaults**

Replace these dep lines in `[dependencies]` (lines 31-39):

```toml
itertools = { version = "0.14.0", default-features = false, features = ["use_alloc"] }
log = { version = "0.4.29", default-features = false }
rand = { version = "0.9", default-features = false }
serde = { version = "1.0.219", default-features = false, features = ["derive"], optional = true }
thiserror = { version = "2.0.18", default-features = false }
```

(Leave `colored`, `fluent-templates`, `serde_norway` unchanged — they stay opt-in via their existing features.)

- [ ] **Step 4: Verify default-features build still green**

Run: `cargo build`
Expected: PASS.

Run: `cargo build --all-features`
Expected: PASS.

Run: `cargo test --all-features --lib`
Expected: PASS — same tests as Step 1.

- [ ] **Step 5: Verify `--no-default-features` still fails (expected pre-task-2 state)**

Run: `cargo build --no-default-features`
Expected: FAIL with errors about `use std::collections::HashMap` (or similar) — proves `std` is no longer in the dependency graph but the source still references it. This is the failure we will close in Tasks 2-9.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml
git commit -m "build: introduce std/alloc features and disable upstream default-features"
```

---

## Task 2: Crate-root `no_std` attribute + `extern crate alloc`

**Files:**
- Modify: `src/lib.rs:1-19` (top-level attribute block)

- [ ] **Step 1: Verify default-features build is green**

Run: `cargo build`
Expected: PASS.

- [ ] **Step 2: Add `no_std` cfg_attr and `extern crate alloc`**

Edit `src/lib.rs`. After the existing `#![allow(dead_code)]` line (line 19) and before the existing comment block, insert:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
```

The lib.rs top should now read:

```rust
#![warn(
    clippy::all,
    // ... existing lints unchanged ...
)]
#![allow(
    // ... existing allows unchanged ...
)]
#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// ... existing comment block continues ...
```

- [ ] **Step 3: Verify default-features build is still green**

Run: `cargo build`
Expected: PASS — adding `extern crate alloc` and the cfg_attr is harmless under default features.

Run: `cargo test --lib`
Expected: PASS — no behavioral change.

- [ ] **Step 4: Verify `--no-default-features` build advances (still fails, but on later errors)**

Run: `cargo build --no-default-features 2>&1 | head -20`
Expected: FAIL. Errors should now be about specific `use std::` imports (HashMap, HashSet, fmt::Display, etc.) — the cfg_attr is now active and `std` is genuinely unavailable.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs
git commit -m "build: enable no_std at crate root with extern crate alloc"
```

---

## Task 3: Mechanical `std::` → `core::` import substitutions for re-exports

**Files (all modify):**
- `src/prelude.rs`
- `src/localization.rs`
- `src/basic/types/pile.rs`
- `src/basic/types/basic_pile.rs`
- `src/basic/types/combos.rs`
- `src/basic/types/basic_card.rs`
- `src/basic/types/pips.rs`
- `src/basic/types/basic.rs`
- `src/basic/types/card.rs`

**Substitution table** (apply across all listed files):

| Find | Replace |
|---|---|
| `use std::fmt::` | `use core::fmt::` |
| `use std::str::FromStr;` | `use core::str::FromStr;` |
| `use std::cmp::Ordering;` | `use core::cmp::Ordering;` |
| `use std::hash::Hash;` (and `Hasher`) | `use core::hash::Hash;` (and `Hasher`) |
| `use std::cell::Cell;` | `use core::cell::Cell;` |
| `use std::marker::PhantomData;` | `use core::marker::PhantomData;` |
| `use std::error::Error;` | `use core::error::Error;` |
| `use std::vec::IntoIter;` | `use alloc::vec::IntoIter;` |

**Do NOT touch in this task:**
- `use std::collections::{HashMap, HashSet};` — handled in Tasks 4 / 8
- `use std::fs::File;` and `use std::io::Read;` in `basic_card.rs` — these stay (file IO is already inside `cards_from_yaml_file`, which is `yaml`-gated)
- Any `std::` reference *inside* `#[cfg(feature = "std")]` or test-gated blocks — those stay
- Any `std::` reference in doctests — handled in Task 10

- [ ] **Step 1: Verify default-features build is green**

Run: `cargo build`
Expected: PASS.

- [ ] **Step 2: Apply substitutions across all listed files**

Use sed or your editor's project-wide find-and-replace. Example shell pass:

```bash
# Run from repo root. The substitutions are restricted to use-statements only.
for f in src/prelude.rs src/localization.rs src/basic/types/pile.rs \
         src/basic/types/basic_pile.rs src/basic/types/combos.rs \
         src/basic/types/basic_card.rs src/basic/types/pips.rs \
         src/basic/types/basic.rs src/basic/types/card.rs; do
  sed -i '' \
    -e 's|^use std::fmt::|use core::fmt::|' \
    -e 's|^use std::str::FromStr;|use core::str::FromStr;|' \
    -e 's|^use std::cmp::Ordering;|use core::cmp::Ordering;|' \
    -e 's|^use std::hash::Hash;|use core::hash::Hash;|' \
    -e 's|^use std::hash::{Hash, Hasher};|use core::hash::{Hash, Hasher};|' \
    -e 's|^use std::cell::Cell;|use core::cell::Cell;|' \
    -e 's|^use std::marker::PhantomData;|use core::marker::PhantomData;|' \
    -e 's|^use std::error::Error;|use core::error::Error;|' \
    -e 's|^use std::vec::IntoIter;|use alloc::vec::IntoIter;|' \
    "$f"
done
```

(macOS `sed -i ''` syntax. Linux: drop the `''`.)

- [ ] **Step 3: Manual verification of each touched file**

Run: `git diff --stat src/`
Expected: 9 files modified, ~1-3 lines changed each.

For each file, confirm no leftover `use std::fmt::`, `use std::str::FromStr`, etc. Inline `use std::` statements (inside test functions) for `FromStr` and similar in `basic_pile.rs:440`, `basic_card.rs:200`, `basic.rs:331-332` — leave those alone for now if they are inside `#[cfg(test)]` blocks (Task 8 will sweep them).

Run: `grep -rn '^use std::' src/ | grep -v collections | grep -v fs:: | grep -v io::`
Expected: empty output (everything except collections + fs/io has been migrated).

- [ ] **Step 4: Verify default-features build is still green**

Run: `cargo build`
Expected: PASS.

Run: `cargo test --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "refactor: replace std:: imports with core::/alloc:: equivalents"
```

---

## Task 4: Migrate `map_by_suit` and `map_by_rank` (HashMap → BTreeMap, atomic)

**Files:**
- Modify: `src/basic/types/pile.rs:14` (import), `pile.rs:480-490` (`map_by_suit`), `pile.rs:941-960` (the `colors()` HashMap import — leave this one)
- Modify: `src/basic/types/traits.rs:11` (import), `traits.rs:356-370` (`map_by_rank`)

**Important:** `map_by_suit` and `map_by_rank` body + signature change must happen in a single edit per file — leaving body BTreeMap with signature HashMap (or vice versa) will not compile.

- [ ] **Step 1: Run existing tests for `map_by_suit` and `map_by_rank` (baseline)**

Run: `cargo test --lib map_by_suit map_by_rank`
Expected: PASS — these tests use indexing (`mappie[&FrenchSuit::SPADES]`) which works identically against BTreeMap, so they should keep passing after our edit too.

- [ ] **Step 2: Edit `src/basic/types/pile.rs`**

At the `use std::collections::{HashMap, HashSet};` line (line 14), split the imports:

```rust
use alloc::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
```

(`HashMap` is still needed for `colors()` returning `HashMap<Pip, Color>` — that's `colored-display`-gated and stays. `HashSet` will be removed in Task 7.)

Replace the `map_by_suit` body (lines 480-490) with:

```rust
    #[must_use]
    pub fn map_by_suit(&self) -> BTreeMap<Pip, Self> {
        let mut map: BTreeMap<Pip, Self> = BTreeMap::new();

        for card in &self.0 {
            let suit = card.base_card.suit;
            let entry = map.entry(suit).or_default();
            entry.push(*card);
        }

        map
    }
```

Update the doctest above `map_by_suit` (lines 463-478): the `mappie[&FrenchSuit::SPADES]` indexing is unchanged for BTreeMap. No edit needed in the doctest body.

- [ ] **Step 3: Edit `src/basic/types/traits.rs`**

At the `use std::collections::{HashMap, HashSet};` line (line 11), split:

```rust
use alloc::collections::{BTreeMap, BTreeSet};
use std::collections::{HashMap, HashSet};
```

(`BTreeSet` is needed for Task 5's `extract_pips` change. `HashMap`/`HashSet` are still referenced — `HashMap` for `colors()`, `HashSet` for `extract_pips` until Task 5. Leave them.)

Replace the `map_by_rank` body (lines 356-370 — preserve the trait method signature with `BTreeMap<Pip, BasicPile>` instead of `HashMap`):

```rust
    fn map_by_rank(&self) -> BTreeMap<Pip, BasicPile> {
        let mut mappy: BTreeMap<Pip, BasicPile> = BTreeMap::new();

        for card in &self.my_basic_pile() {
            let rank = card.rank;

            if let alloc::collections::btree_map::Entry::Vacant(e) = mappy.entry(rank) {
                let pile = BasicPile::from(vec![*card]);
                e.insert(pile);
            } else if let Some(pile) = mappy.get_mut(&rank) {
                pile.push(*card);
            }
        }
        mappy
    }
```

(Note: the original ends with returning `mappy` — confirm the closing of the function matches what was there.)

- [ ] **Step 4: Run map_by_suit and map_by_rank tests**

Run: `cargo test --lib map_by_suit map_by_rank`
Expected: PASS.

- [ ] **Step 5: Run full lib test suite under default features**

Run: `cargo test --lib`
Expected: PASS — no regressions.

- [ ] **Step 6: Verify `--no-default-features` still progresses**

Run: `cargo build --no-default-features 2>&1 | head -10`
Expected: FAIL with remaining errors about `HashSet` (Task 5/7) or `rand::rng()` (Task 9), NOT about HashMap. Confirms the HashMap migration is done.

- [ ] **Step 7: Commit**

```bash
git add src/basic/types/pile.rs src/basic/types/traits.rs
git commit -m "refactor: map_by_suit and map_by_rank return BTreeMap (was HashMap)"
```

---

## Task 5: `extract_pips` internal swap to `BTreeSet`

**Files:**
- Modify: `src/basic/types/traits.rs:345-354`

- [ ] **Step 1: Run existing tests for `extract_pips`**

Run: `cargo test --lib extract_pips`
Expected: PASS — baseline.

- [ ] **Step 2: Replace `extract_pips` body**

Replace lines 345-354 of `src/basic/types/traits.rs`:

```rust
    fn extract_pips<F>(&self, f: F) -> Vec<Pip>
    where
        F: Fn(&BasicCard) -> Pip,
    {
        // BTreeSet iterates in sorted order, so the explicit sort/reverse
        // step the previous HashSet-based implementation needed is gone.
        let set: BTreeSet<Pip> = self.my_basic_pile().iter().map(f).collect();
        set.into_iter().rev().collect()
    }
```

(The reverse is preserved — original sorted then reversed, giving high-to-low order. BTreeSet sorts ascending, so `.rev()` keeps the original behavior.)

- [ ] **Step 3: Run extract_pips tests + everything that depends on it**

Run: `cargo test --lib`
Expected: PASS — `extract_pips` is used by `ranks_index`, `suits_index`, etc. Any change in ordering would break those tests.

- [ ] **Step 4: Verify `--no-default-features` build progresses**

Run: `cargo build --no-default-features 2>&1 | head -10`
Expected: FAIL on the *remaining* `HashSet` reference (in `into_hashset` — Task 6/7) or on `rand::rng()` (Task 9), NOT in `extract_pips`.

- [ ] **Step 5: Commit**

```bash
git add src/basic/types/traits.rs
git commit -m "refactor: extract_pips uses BTreeSet, drops redundant sort step"
```

---

## Task 6: Rename `into_hashset` → `unique_cards`, return `BTreeSet`

**Files:**
- Modify: `src/basic/types/pile.rs:14` (import), `pile.rs:406-424` (method + doctest), `pile.rs:1170-1175` (test)

- [ ] **Step 1: Write the failing test**

Add the following test to the test module in `src/basic/types/pile.rs` (find the `#[cfg(test)] mod tests` block; if there isn't a clean place, add at the end of the existing test module):

```rust
    #[test]
    fn pile__unique_cards__returns_btreeset() {
        use alloc::collections::BTreeSet;
        use crate::prelude::*;

        let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
        let mut expected: BTreeSet<Card<Standard52>> = BTreeSet::new();
        expected.insert(card!(2S));
        expected.insert(card!(8S));
        expected.insert(card!(4S));

        assert_eq!(pile.unique_cards(), expected);
    }
```

- [ ] **Step 2: Run the test, confirm it fails**

Run: `cargo test --lib pile__unique_cards__returns_btreeset 2>&1 | tail -10`
Expected: FAIL with `no method named 'unique_cards' found for struct 'Pile'`.

- [ ] **Step 3: Replace the method**

Edit `src/basic/types/pile.rs` lines 406-424. Replace:

```rust
    /// Returns the `Pile` as a `HashSet`, an unordered collection of each unique [`Card`].
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut hs: HashSet<Card<Standard52>> = HashSet::new();
    ///
    /// hs.insert(card!(2S));
    /// hs.insert(card!(8S));
    /// hs.insert(card!(4S));
    ///
    /// assert_eq!(pile.into_hashset(), hs);
    /// ```
    #[must_use]
    pub fn into_hashset(&self) -> HashSet<Card<DeckType>> {
        self.0.iter().copied().collect()
    }
```

With:

```rust
    /// Returns the `Pile` as a `BTreeSet`, an ordered collection of each unique [`Card`].
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut set: BTreeSet<Card<Standard52>> = BTreeSet::new();
    ///
    /// set.insert(card!(2S));
    /// set.insert(card!(8S));
    /// set.insert(card!(4S));
    ///
    /// assert_eq!(pile.unique_cards(), set);
    /// ```
    #[must_use]
    pub fn unique_cards(&self) -> BTreeSet<Card<DeckType>> {
        self.0.iter().copied().collect()
    }
```

Make sure the `use alloc::collections::BTreeMap;` import added in Task 4 is upgraded to:

```rust
use alloc::collections::{BTreeMap, BTreeSet};
```

- [ ] **Step 4: Update the existing `into_hashset` test**

The existing test at `src/basic/types/pile.rs:1168-1177` is:

```rust
    #[test]
    fn into_hashset() {
        let five_deck = French::decks(5);

        let hashset: HashSet<Card<French>> = five_deck.into_hashset();
        let deck = Pile::<French>::from(hashset);

        assert_eq!(five_deck.len(), 270);
        assert_eq!(deck, French::deck());
    }
```

Replace it with:

```rust
    #[test]
    fn unique_cards() {
        let five_deck = French::decks(5);

        let btreeset: alloc::collections::BTreeSet<Card<French>> = five_deck.unique_cards();
        // Bridge via Vec because From<BTreeSet> doesn't exist; Task 7 will
        // replace this with `let deck: Pile<French> = btreeset.into_iter().collect();`
        // once FromIterator is in place.
        let deck = Pile::<French>::from(btreeset.into_iter().collect::<Vec<_>>());

        assert_eq!(five_deck.len(), 270);
        assert_eq!(deck, French::deck());
    }
```

This both renames the test and rewrites the body to bypass `From<HashSet>` (which still exists at this point but is being phased out) using the existing `From<Vec<Card>>` impl.

- [ ] **Step 5: Run the new test + the full lib suite**

Run: `cargo test --lib pile__unique_cards__returns_btreeset`
Expected: PASS.

Run: `cargo test --lib`
Expected: PASS — no regressions.

- [ ] **Step 6: Run doctests**

Run: `cargo test --doc`
Expected: PASS — the new doctest under `unique_cards` runs cleanly.

- [ ] **Step 7: Commit**

```bash
git add src/basic/types/pile.rs
git commit -m "refactor!: rename Pile::into_hashset to unique_cards, return BTreeSet"
```

---

## Task 7: Replace `From<HashSet<Card>>` with `FromIterator<Card>`

**Files:**
- Modify: `src/basic/types/pile.rs:14` (HashSet import — remove now), `pile.rs:971-977` (impl block)

- [ ] **Step 1: Write a failing test for `FromIterator` collect**

Add to the test module:

```rust
    #[test]
    fn pile__from_iterator__collects_from_card_iter() {
        use crate::prelude::*;

        let cards = vec![card!(AS), card!(KH), card!(QC)];
        let pile: Pile<Standard52> = cards.into_iter().collect();

        assert_eq!(pile.len(), 3);
    }
```

- [ ] **Step 2: Run the test, confirm it fails**

Run: `cargo test --lib pile__from_iterator__collects_from_card_iter 2>&1 | tail -10`
Expected: FAIL with `the trait bound 'Pile<Standard52>: FromIterator<...>' is not satisfied`.

- [ ] **Step 3: Replace the `From<HashSet>` impl with `FromIterator`**

Edit `src/basic/types/pile.rs`. Find lines 971-977:

```rust
impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<HashSet<Card<DeckType>>>
    for Pile<DeckType>
{
    fn from(cards: HashSet<Card<DeckType>>) -> Self {
        Self(cards.into_iter().collect()).sorted()
    }
}
```

Replace with:

```rust
impl<DeckType: DeckedBase + Default + Ord + Copy> FromIterator<Card<DeckType>>
    for Pile<DeckType>
{
    fn from_iter<I: IntoIterator<Item = Card<DeckType>>>(iter: I) -> Self {
        Self(iter.into_iter().collect()).sorted()
    }
}
```

Note: `Hash` bound is dropped — only needed for the HashSet receiver.

- [ ] **Step 4: Simplify the `unique_cards` test to use `FromIterator`**

The test rewritten in Task 6 used a Vec bridge (`btreeset.into_iter().collect::<Vec<_>>()`) because `From<HashSet>`/`From<BTreeSet>` was the only conversion path. Now that `FromIterator` is in place, simplify the test body. Find the test in `src/basic/types/pile.rs` (the `unique_cards` test, ~line 1168 area) and replace its body with:

```rust
    #[test]
    fn unique_cards() {
        let five_deck = French::decks(5);

        let btreeset: alloc::collections::BTreeSet<Card<French>> = five_deck.unique_cards();
        let deck: Pile<French> = btreeset.into_iter().collect();

        assert_eq!(five_deck.len(), 270);
        assert_eq!(deck, French::deck());
    }
```

- [ ] **Step 5: Remove the now-unused `HashSet` import**

In `src/basic/types/pile.rs:14`, change:

```rust
use std::collections::{HashMap, HashSet};
```

To:

```rust
use std::collections::HashMap;
```

(`HashMap` still used for `colors()`, which is `colored-display`-gated.)

- [ ] **Step 6: Run the new test + full suite**

Run: `cargo test --lib pile__from_iterator__collects_from_card_iter`
Expected: PASS.

Run: `cargo test --lib`
Expected: PASS.

Run: `cargo test --doc`
Expected: PASS.

- [ ] **Step 7: Verify `--no-default-features` build**

Run: `cargo build --no-default-features 2>&1 | head -10`
Expected: FAIL only on `rand::rng()` callers (Task 9) and possibly on `HashMap`/`HashSet` references in test code or remaining files (Task 8). The Pile.rs HashSet should be gone.

- [ ] **Step 8: Commit**

```bash
git add src/basic/types/pile.rs
git commit -m "refactor!: replace From<HashSet> impl with FromIterator on Pile"
```

---

## Task 8: Audit and fix remaining `std::collections` references

**Files:** discovered via grep; expected hits include test code in `basic_pile.rs`, `basic_card.rs`, `basic.rs`, and any remaining tests in `pile.rs` / `traits.rs`.

- [ ] **Step 1: Find remaining hits**

Run: `grep -rn 'std::collections' src/ | grep -v ' *//'`
Expected: a small list (≤10) — `traits.rs:11` will still have `HashMap, HashSet` for the `colors()` method (HashMap stays; HashSet is now only used internally — confirm whether it's still referenced after Task 5).

If `traits.rs` still imports `HashSet` but no body still uses it, drop `HashSet` from that import:

```rust
use std::collections::HashMap;
```

- [ ] **Step 2: For each remaining hit, decide**

For each line in the grep output:

- If it's a non-test reference to `HashMap` for `colors()` → leave it (gated on `colored-display`).
- If it's a test or doctest reference → either swap to `alloc::collections::BTreeMap`/`BTreeSet`, or wrap the test in `#[cfg(feature = "std")]` if the test is specifically about HashMap behavior.
- If it's an internal use of `HashSet`/`HashMap` that's no longer reachable from public API → delete the import.

For `src/basic/types/basic.rs:331`:

```rust
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
```

This is in a test that hashes a `BasicPileCell` to verify `Hash` is implemented. Wrap the entire test (`#[test] fn ...`) in `#[cfg(feature = "std")]`:

```rust
    #[cfg(feature = "std")]
    #[test]
    fn basic_pile_cell__hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        // ... existing body unchanged ...
    }
```

- [ ] **Step 3: Confirm grep is clean for non-stay items**

Run: `grep -rn 'std::collections' src/ | grep -v 'colors\|HashMap' | grep -v cfg`
Expected: empty (everything either uses HashMap inside `colors()` or is `cfg`-gated).

- [ ] **Step 4: Run full suite under default features**

Run: `cargo test --lib`
Expected: PASS.

Run: `cargo test --all-features`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/
git commit -m "refactor: gate remaining std::collections references behind cfg(feature = std)"
```

---

## Task 9: Gate no-arg `shuffle()` / `shuffled()` on `feature = "std"`

**Files:**
- Modify: `src/basic/types/pile.rs:9-11` (rand imports), `pile.rs:717-733` (`shuffled` and `shuffle`)
- Modify: `src/basic/types/basic_pile.rs` (`shuffle` and `shuffled` no-arg variants — find the equivalent lines)

- [ ] **Step 1: Verify default-features build is green**

Run: `cargo build`
Expected: PASS.

- [ ] **Step 2: Edit `src/basic/types/pile.rs`**

At lines 9-11, change the `rand` imports to gate `rng` on std:

```rust
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
#[cfg(feature = "std")]
use rand::rng;
```

At lines 717-733, gate the no-arg methods:

```rust
    /// `shuffled` feels so much better. Nice and succinct.
    ///
    /// For deterministic shuffling, use
    /// [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[cfg(feature = "std")]
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the `Pile` in place using the process default RNG
    /// (`rand::rng()`). For deterministic shuffling, use
    /// [`shuffle_with_seed`](Self::shuffle_with_seed).
    #[cfg(feature = "std")]
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }
```

- [ ] **Step 3: Apply equivalent edit to `src/basic/types/basic_pile.rs`**

Run: `grep -n 'pub fn shuffle\|pub fn shuffled' src/basic/types/basic_pile.rs`

For each no-arg `shuffle` and `shuffled`, prepend `#[cfg(feature = "std")]`. Same for the `use rand::rng;` import — if it exists, gate it.

Leave `shuffle_with_seed`, `shuffled_with_seed`, `shuffle_with_rng`, `shuffled_with_rng` unconditional.

- [ ] **Step 4: Verify default-features build still green**

Run: `cargo build`
Expected: PASS.

Run: `cargo test --lib`
Expected: PASS — `shuffle()` no-arg is still available under default features.

- [ ] **Step 5: Verify `--no-default-features` build is green**

Run: `cargo build --no-default-features`
Expected: PASS — this is the milestone! Crate now builds cleanly without `std`.

- [ ] **Step 6: Run lib tests under `--no-default-features`**

Run: `cargo test --no-default-features --lib 2>&1 | tail -20`
Expected: Some tests skip (those that use no-arg shuffle, or `i18n`/`colored-display` features), but the suite passes (no failures, only fewer tests).

- [ ] **Step 7: Commit**

```bash
git add src/basic/types/pile.rs src/basic/types/basic_pile.rs
git commit -m "feat!: gate no-arg shuffle()/shuffled() behind std feature"
```

---

## Task 10: Doctest sweep — replace HashMap/HashSet usages

**Files:** any source file containing doctests with `std::collections::HashMap` or `HashSet`.

- [ ] **Step 1: Find doctests referencing the migrated types**

Run: `grep -rn 'std::collections::Hash' src/`
Expected: a list of ~10-20 lines, all inside `///` doctest blocks.

- [ ] **Step 2: For each hit, edit the doctest**

For each occurrence:

- `use std::collections::HashMap;` in a doctest that calls `map_by_suit` or `map_by_rank` → either change to `use std::collections::BTreeMap;` (the doctest still has std available because doctests run with default features), or delete the import if `BTreeMap` indexing is the only operation (`mappie[&FrenchSuit::SPADES]` works without explicit import).
- `use std::collections::HashSet;` in any doctest that referenced `into_hashset` → already updated in Task 6. Re-confirm none remain.

Specifically check:
- `src/lib.rs:196,219` — doctest comments (look for "use std::collections::HashMap")
- `src/basic/types/pile.rs` — `map_by_suit` doctest at line 463 (no edit if it doesn't import HashMap)
- Any deck file (`french.rs`, etc.) — they implement `colors() -> HashMap`; if any doctest constructs the map by hand, leave it (still under `colored-display`).

- [ ] **Step 3: Run all doctests**

Run: `cargo test --doc --all-features`
Expected: PASS.

Run: `cargo test --doc`  (default features)
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/
git commit -m "docs: update doctests to BTreeMap/BTreeSet (was HashMap/HashSet)"
```

---

## Task 11: `examples/no_std_smoke.rs` smoke binary

**Files:**
- Create: `examples/no_std_smoke.rs`
- Modify: `Cargo.toml` (`[[example]]` registration)

- [ ] **Step 1: Verify default-features build is green**

Run: `cargo build`
Expected: PASS.

- [ ] **Step 2: Create `examples/no_std_smoke.rs`**

```rust
//! No-std compile + link smoke binary.
//!
//! Built only under `--no-default-features` against
//! `--target thumbv7em-none-eabihf` to verify cardpack monomorphizes
//! cleanly without `std`. The `std`-feature build is a no-op `main()`
//! so `cargo build --example no_std_smoke` doesn't break locally.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use cardpack::prelude::*;
    use rand::{SeedableRng, rngs::StdRng};

    let deck: Pile<Standard52> = Pile::<Standard52>::deck();
    let _ = deck.len();
    let _ = deck.unique_cards();
    let _ = deck.map_by_suit();

    let mut rng = StdRng::seed_from_u64(42);
    let _ = deck.shuffled_with_rng(&mut rng);

    loop {}
}

#[cfg(feature = "std")]
fn main() {
    // Host build is a no-op; the real verification is the no_std target build.
}
```

- [ ] **Step 3: Register the example in `Cargo.toml`**

Append to the `[[example]]` block list in `Cargo.toml`:

```toml
[[example]]
name = "no_std_smoke"
required-features = []
```

- [ ] **Step 4: Verify host build (`--example` against default features)**

Run: `cargo build --example no_std_smoke`
Expected: PASS — produces a host binary that just calls `main` (no-op).

- [ ] **Step 5: Verify embedded target build**

Run: `rustup target add thumbv7em-none-eabihf`
Expected: PASS (or "target already installed").

Run: `cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke`
Expected: PASS — the linker resolves all symbols against `core` + `alloc` only. **This is the C-level verification milestone.**

If this fails: read the error carefully. Common causes:
- A transitive dep pulls `std` (look for "no `std`" or "could not find `std`" in the error). Fix: gate that dep behind `std`.
- A `#[panic_handler]` collision (some dep already defines one). Unlikely, but check the error message.
- A symbol like `_ZN5alloc...` is unresolved. Means we forgot `extern crate alloc`. Re-check `src/lib.rs`.

- [ ] **Step 6: Commit**

```bash
git add examples/no_std_smoke.rs Cargo.toml
git commit -m "feat: add no_std_smoke example, verify thumbv7em-none-eabihf builds"
```

---

## Task 12: CI jobs — `no-std-build` and `no-std-thumbv7em`

**Files:**
- Modify: `.github/workflows/CI.yaml`

- [ ] **Step 1: Read current CI structure**

Run: `cat .github/workflows/CI.yaml`
Expected: review existing jobs (`test`, `clippy`, `fmt`, etc.) — pick a job to model the new ones after.

- [ ] **Step 2: Append two new jobs to `.github/workflows/CI.yaml`**

Add at the end of the `jobs:` block (preserving the file's existing indentation conventions):

```yaml
  no-std-build:
    name: no_std host build
    runs-on: ubuntu-latest
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --no-default-features
      - run: cargo build --no-default-features --features serde
      - run: cargo test --no-default-features --lib

  no-std-thumbv7em:
    name: no_std bare-metal target
    runs-on: ubuntu-latest
    timeout-minutes: 20
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: thumbv7em-none-eabihf
      - run: cargo build --no-default-features --target thumbv7em-none-eabihf
      - run: cargo build --no-default-features --features serde --target thumbv7em-none-eabihf
      - run: cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke
```

- [ ] **Step 3: Validate YAML locally**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/CI.yaml'))"`
Expected: no output (parses cleanly).

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/CI.yaml
git commit -m "ci: add no-std-build and no-std-thumbv7em jobs"
```

---

## Task 13: Makefile targets

**Files:**
- Modify: `Makefile` (add `.PHONY` entries and target bodies)

- [ ] **Step 1: Add `.PHONY` entries**

In the existing `.PHONY:` declaration at the top of `Makefile`, append `no-std no-std-thumbv7`. The line should now read:

```makefile
.PHONY: clean build test test-unit test-doc test-wasm build-wasm coverage bench build_test fmt clippy create_docs ayce default help docs test-nightly clippy-nightly nightly miri mutants tree tree-duplicates deny audit unused-deps install-tools install-nextest install-mutants install-wasm-bindgen-cli install-llvm-cov watch install-watch no-std no-std-thumbv7
```

- [ ] **Step 2: Add target bodies**

Append to the end of `Makefile`:

```makefile
no-std:
	cargo build --no-default-features
	cargo build --no-default-features --features serde

no-std-thumbv7:
	rustup target add thumbv7em-none-eabihf
	cargo build --no-default-features --target thumbv7em-none-eabihf
	cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke
```

(Use TAB indentation, not spaces — Makefile syntax requirement.)

- [ ] **Step 3: Add help-text entries**

Find the `help:` target in `Makefile` and add two lines before the "Nightly:" section:

```makefile
	@echo "  make no-std          - Build the lib with --no-default-features"
	@echo "  make no-std-thumbv7  - Build for thumbv7em-none-eabihf (bare-metal)"
```

- [ ] **Step 4: Test the targets**

Run: `make no-std`
Expected: PASS.

Run: `make no-std-thumbv7`
Expected: PASS (after `rustup target add thumbv7em-none-eabihf` succeeds).

- [ ] **Step 5: Commit**

```bash
git add Makefile
git commit -m "build: add make no-std and make no-std-thumbv7 targets"
```

---

## Task 14: CHANGELOG.md 0.7.0 entry

**Files:**
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Read existing CHANGELOG**

Run: `head -40 CHANGELOG.md`
Expected: see the existing format (Keep a Changelog 1.1).

- [ ] **Step 2: Insert 0.7.0 section above the most recent entry**

Add this section above the `## [0.6.12]` (or whatever the most recent version is) entry in `CHANGELOG.md`:

```markdown
## [0.7.0] — YYYY-MM-DD

### Breaking
- `Pile::into_hashset()` renamed to `Pile::unique_cards()`; return type
  changed from `HashSet<Card<DeckType>>` to `BTreeSet<Card<DeckType>>`.
- `From<HashSet<Card<DeckType>>> for Pile<DeckType>` removed; replaced
  with `impl FromIterator<Card<DeckType>> for Pile<DeckType>`,
  enabling `.collect::<Pile<_>>()` from any iterator.
- `Pile::map_by_suit()` and `Ranged::map_by_rank()` return `BTreeMap`
  instead of `HashMap` (deterministic iteration order, identical
  lookup semantics).
- `Pile::shuffle()` / `Pile::shuffled()` (no-arg) and
  `BasicPile::shuffle()` / `BasicPile::shuffled()` (no-arg) now require
  the `std` feature (still default-on). Under `--no-default-features`,
  use `shuffle_with_seed(u64)` or `shuffle_with_rng(&mut R)`.

### Added
- `no_std` support via `extern crate alloc`. Build with
  `--no-default-features` for an alloc-only build that targets embedded
  Rust (`thumbv7em-none-eabihf` and similar).
- New `std` feature (default-on). Existing features (`i18n`,
  `colored-display`, `yaml`) implicitly require `std`.
- New `alloc` feature for fine-grained dep control (mostly internal
  plumbing for `serde?/alloc`).
- CI gates on `cargo build --no-default-features --target
  thumbv7em-none-eabihf`.
- `examples/no_std_smoke.rs` — bare-metal compile + link smoke binary.
- New Make targets: `make no-std`, `make no-std-thumbv7`.

### Internal
- `Ranged::extract_pips` switched from `HashSet` → `BTreeSet`,
  removing a redundant sort step.
```

(Replace the date placeholder with today's date when committing.)

- [ ] **Step 3: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs: add 0.7.0 CHANGELOG entry"
```

---

## Task 15: Bump version to 0.7.0

**Files:**
- Modify: `Cargo.toml:4`

- [ ] **Step 1: Run `make ayce` (full umbrella check, baseline)**

Run: `make ayce`
Expected: PASS.

- [ ] **Step 2: Edit `Cargo.toml`**

Change:

```toml
version = "0.6.12"
```

To:

```toml
version = "0.7.0"
```

- [ ] **Step 3: Update `Cargo.lock` (auto)**

Run: `cargo build`
Expected: PASS — Cargo updates `Cargo.lock` to reflect the new version.

- [ ] **Step 4: Final verification suite**

Run: `cargo build` (default)
Run: `cargo build --no-default-features`
Run: `cargo build --no-default-features --features serde`
Run: `cargo build --all-features`
Run: `cargo test --all-features`
Run: `cargo test --no-default-features --lib`
Run: `cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke`
Run: `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings`
Run: `make ayce`

All expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.7.0"
```

---

## Verification Checklist (mirrors spec §11)

- [ ] `cargo build` (default features) — green
- [ ] `cargo build --no-default-features` — green
- [ ] `cargo build --no-default-features --features serde` — green
- [ ] `cargo build --all-features` — green
- [ ] `cargo test --all-features` — full test count (291+, possibly +2 for new tests in Tasks 6 and 7)
- [ ] `cargo test --no-default-features --lib` — reduced count, no failures
- [ ] `rustup target add thumbv7em-none-eabihf && cargo build --no-default-features --target thumbv7em-none-eabihf` — green
- [ ] `cargo build --no-default-features --target thumbv7em-none-eabihf --example no_std_smoke` — green
- [ ] `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings` — green
- [ ] `make ayce` umbrella — green

When every box is checked, 0.7.0 is ready to publish.
