# Funky Round Lifecycle Hooks (EPIC-01a Phase 1c) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `on_round_end` / `on_round_end_with_rng` to `BuffoonBoard`, extend `on_discard`/`on_hand_played`, and wire six deterministic money jokers plus the probabilistic joker destructions (Gros Michel, Cavendish) and Ice Cream's melt.

**Architecture:** Extend the Phase 3 event seam: `GrowthEvent` gains `RoundEnd`; a new `payout_delta(&self, enhancement, event) -> isize` mirrors `growth_delta` for money; `apply_payouts` mirrors `apply_growth`. Destruction is a separate pass inside `on_round_end_with_rng` (the `score`/`score_with_rng` split), rolled through `probability_numerator()` so Oops! All 6s doubles odds for free.

**Tech Stack:** Rust 2024, `--features funky` (std-only), seeded `StdRng` for destruction tests.

**Spec:** [`docs/superpowers/specs/2026-07-15-funky-lifecycle-hooks-design.md`](../specs/2026-07-15-funky-lifecycle-hooks-design.md)

## Global Constraints

- **Git commits:** this repo's owner runs every state-changing git command. When a step says "commit", print the exact command for the owner to run — never run it yourself.
- **Feature gate:** all code is `#[cfg(feature = "funky")]` (the whole `src/funky/` tree already is). Never touch `src/basic/`.
- **Exhaustive matches:** `MPip::Display` (`src/funky/types/mpip.rs`) and the `scores_hand` oracle (`src/funky/decks/joker.rs`) have no `_` arm — every new variant MUST get an arm in both or the crate/tests won't compile.
- **Exact values only:** every payout test asserts the exact Balatro dollar amount (EPIC-00f Gold Standard); each test must be observed failing before its arm lands.
- **Inertness invariant:** `on_round_end` on a board without these jokers changes nothing; existing `score__*` tests stay green at every step.
- **Verification (run after each task):**
  ```bash
  cargo test --features funky
  cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
  cargo build --no-default-features
  cargo fmt --all -- --check
  ```

## Payout semantics (reference for all tasks)

| Joker | Variant (data change) | Event | Exact payout |
|---|---|---|---|
| Golden Joker | `Blank` → `CashOnRoundEnd(4)` | round end | +$4 |
| Delayed Gratification | `Blank` → `CashPerDiscardIfNoneUsed(2)` | round end | $2 × `draws.discards` iff `discards_used == 0` |
| Cloud 9 | `Blank` → `CashPerFullDeckRank(1, '9')` | round end | $1 × 9s in `full_deck` |
| To the Moon | `Blank` → `ExtraInterest(1)` | round end | $1 × `min(money/5, 5)` |
| Faceless Joker | `Blank` → `CashOnFacesDiscarded(5, 3)` | discard | +$5 iff ≥3 faces in the discarded pile (via `is_face_card`, Pareidolia-aware) |
| Egg | `SellValueIncrement(3)` (exists) | round end | own `resell_value` += 3 |
| Gros Michel | `MultPlusChanceDestroyed(15, 1, 6)` (exists) | round end + RNG | destroyed on 1-in-6 |
| Cavendish | `MultTimes(3)` → `MultTimesChanceDestroyed(3, 1, 1000)` | round end + RNG | destroyed on 1-in-1000; **still scores ×3** |
| Ice Cream | `LoseChipsPerHand(100, 5)` (exists) | hand played | removed when `100 − 5×hands == 0` (the 20th hand) |

## File Structure

- `src/funky/types/mpip.rs` — 6 new variants + `Display` arms.
- `src/funky/types/board.rs` — `discards_used` field; `GrowthEvent::RoundEnd`; `payout_delta`, `apply_payouts`, `cash`; `on_round_end`, `on_round_end_with_rng`, `roll_destructions`, `grow_sell_values`, `melt_spent_jokers`; extended `on_discard`/`on_hand_played`; all new tests in `mod funky__types__board__buffoon_board_tests`.
- `src/funky/decks/joker.rs` — 6 const flips; oracle arms (5 non-scoring, 1 scoring); Cavendish regression test lives in `board.rs` with the others.
- `docs/EPIC-01a_Joker_Wiring_Backlog.md` — status flips (final task).

---

### Task 1: New `MPip` variants + `Display` arms + oracle arms

**Files:**
- Modify: `src/funky/types/mpip.rs` (enum ~line 148 after `ChipsPerDollar`; `Display` ~line 356)
- Modify: `src/funky/decks/joker.rs` (`scores_hand` oracle: non-scoring arm ends ~line 2111)

**Interfaces:**
- Produces: `MPip::CashOnRoundEnd(usize)`, `MPip::CashPerDiscardIfNoneUsed(usize)`, `MPip::CashPerFullDeckRank(usize, char)`, `MPip::ExtraInterest(usize)`, `MPip::CashOnFacesDiscarded(usize, usize)`, `MPip::MultTimesChanceDestroyed(usize, usize, usize)` — all later tasks consume these.

- [ ] **Step 1: Add the six variants** to the `MPip` enum in `src/funky/types/mpip.rs`, directly after `ChipsPerDollar(usize)`:

```rust
    /// Golden Joker: earn +$n at end of round. Data only until `on_round_end`
    /// applies it — never a scoring arm.
    CashOnRoundEnd(usize),
    /// Delayed Gratification: earn $n per remaining discard at end of round,
    /// but only if no discard was used this round (`discards_used == 0`).
    CashPerDiscardIfNoneUsed(usize),
    /// Cloud 9: earn $n per card of `rank` in the run's **full deck** at end
    /// of round — `CashPerFullDeckRank(1, '9')`.
    CashPerFullDeckRank(usize, char),
    /// To the Moon: $n of extra interest per $5 held at end of round, capped
    /// at the base-game interest ceiling of 5 increments ($5 for n = 1).
    ExtraInterest(usize),
    /// Faceless Joker: earn `$cash` when at least `min` face cards are
    /// discarded at once — `CashOnFacesDiscarded(5, 3)`. Face-ness goes
    /// through the board's face predicate, so Pareidolia amplifies it.
    CashOnFacesDiscarded(usize, usize),
    /// Cavendish: ×mult unconditionally, **and** a `numerator`-in-`denominator`
    /// chance the joker is destroyed at end of round —
    /// `MultTimesChanceDestroyed(3, 1, 1000)`. The ×mult twin of Gros Michel's
    /// [`MultPlusChanceDestroyed`](Self::MultPlusChanceDestroyed); compound for
    /// the same reason (one card is one `MPip`; splitting the halves is what
    /// hid Gros Michel's mult).
    MultTimesChanceDestroyed(usize, usize, usize),
```

- [ ] **Step 2: Add the `Display` arms**, next to `Self::ChipsPerDollar`:

```rust
            Self::CashOnRoundEnd(value) => write!(f, "CashOnRoundEnd({value})"),
            Self::CashPerDiscardIfNoneUsed(value) => {
                write!(f, "CashPerDiscardIfNoneUsed({value})")
            }
            Self::CashPerFullDeckRank(value, rank) => {
                write!(f, "CashPerFullDeckRank({value}, {rank})")
            }
            Self::ExtraInterest(value) => write!(f, "ExtraInterest({value})"),
            Self::CashOnFacesDiscarded(cash, min) => {
                write!(f, "CashOnFacesDiscarded({cash}, {min})")
            }
            Self::MultTimesChanceDestroyed(mult, numerator, denominator) => {
                write!(f, "MultTimesChanceDestroyed({mult}, {numerator}, {denominator})")
            }
```

- [ ] **Step 3: Add the oracle arms** in `scores_hand` (`src/funky/decks/joker.rs`). The five cash variants are **non-scoring** — add to the `false` arm's list (alphabetically near `MPip::Credit(_)`):

```rust
            | MPip::CashOnRoundEnd(_)
            | MPip::CashPerDiscardIfNoneUsed(_)
            | MPip::CashPerFullDeckRank(_, _)
            | MPip::CashOnFacesDiscarded(_, _)
            | MPip::ExtraInterest(_)
```

`MultTimesChanceDestroyed` is **scoring** — add to the `true` arm directly under `MPip::MultPlusChanceDestroyed(_, _, _)` with the comment:

```rust
            // Cavendish: the ×3 scores now; the destruction half rolls at
            // round end, exactly like Gros Michel's + half above.
            | MPip::MultTimesChanceDestroyed(_, _, _)
```

- [ ] **Step 4: Verify** — `cargo test --features funky` → all pass (no behaviour change yet; the compiler enforced both matches). Run the clippy/fmt/no_std gates.

- [ ] **Step 5: Commit** — print for the owner:

```bash
git add src/funky/types/mpip.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: add lifecycle MPip variants (5 cash + Cavendish compound)"
```

---

### Task 2: Cavendish → compound variant, still scores ×3

**Files:**
- Modify: `src/funky/decks/joker.rs:977` (`CAVENDISH` const)
- Modify: `src/funky/types/board.rs:513` (`joker_x_mult` match)
- Test: `src/funky/types/board.rs` (`mod funky__types__board__buffoon_board_tests`)

**Interfaces:**
- Consumes: `MPip::MultTimesChanceDestroyed` (Task 1).
- Produces: `CAVENDISH` carrying `MultTimesChanceDestroyed(3, 1, 1000)` — Task 8's destruction pass matches on it.

- [ ] **Step 1: Write the failing test** (in `board.rs` tests, near the other `score__` joker tests):

```rust
    #[test]
    fn score__cavendish_times_3_survives_the_compound_variant() {
        // High Card 2,5,8,T,K: 5 base + 35 pips = 40 chips, 1 mult.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CAVENDISH);

        // ×3 mult, unconditional: 40 x 3. The 1-in-1000 destruction half is
        // data for on_round_end_with_rng and must not gate the mult.
        assert_eq!(board.score(), Score::new(40, 3));
    }
```

- [ ] **Step 2: Flip the const** in `joker.rs` (`CAVENDISH`, ~line 977):

```rust
        enhancement: MPip::MultTimesChanceDestroyed(3, 1, 1000),
```

- [ ] **Step 3: Run the test — verify it fails**

Run: `cargo test --features funky score__cavendish -- --nocapture`
Expected: FAIL — score is `40 x 1` (the new variant falls through `joker_x_mult` to the additive path, which scores nothing). If it *passes*, stop: something else is handling the variant.

- [ ] **Step 4: Add the `joker_x_mult` arm** in `board.rs`, directly under `MPip::MultTimes(n) => n as f32,`:

```rust
            // Cavendish: ×n unconditionally; the destruction half rolls in
            // on_round_end_with_rng, not here.
            MPip::MultTimesChanceDestroyed(n, _, _) => n as f32,
```

- [ ] **Step 5: Run tests — verify all pass**

Run: `cargo test --features funky`
Expected: PASS, including `all_jokers__intended_hand_scorers_are_reachable` (Cavendish scores on the probe boards, proving the compound variant is reachable).

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 1c: Cavendish carries its 1-in-1000 destroy chance, still x3 (0c mirror fix)"
```

---

### Task 3: `discards_used` round state

**Files:**
- Modify: `src/funky/types/board.rs` (struct ~line 54, `new` ~line 74, `on_discard` ~line 804)
- Test: same file, tests module

**Interfaces:**
- Produces: `BuffoonBoard.discards_used: usize` — Task 4's `payout_delta` and `on_round_end` (reset) consume it.

- [ ] **Step 1: Write the failing test:**

```rust
    #[test]
    fn on_discard__counts_discards_used_this_round() {
        let mut board = board_playing("2S 5D 8C TS KH");
        assert_eq!(board.discards_used, 0);

        board.on_discard(&bcards!("3C 4D"));
        board.on_discard(&bcards!("7H"));

        assert_eq!(board.discards_used, 2, "one per discard action, not per card");
    }
```

- [ ] **Step 2: Run it — verify it fails to compile** (no such field):

Run: `cargo test --features funky on_discard__counts`
Expected: compile error "no field `discards_used`".

- [ ] **Step 3: Add the field.** In the `BuffoonBoard` struct, after `starting_deck_size`:

```rust
    /// Discard actions taken this round. Written by
    /// [`on_discard`](Self::on_discard), reset by
    /// [`on_round_end`](Self::on_round_end); read by Delayed Gratification's
    /// payout ($2 per remaining discard only when this is still 0).
    pub discards_used: usize,
```

In `new()`, after `joker_state: Vec::new(),`:

```rust
            discards_used: 0,
```

In `on_discard`, add as the last line of the body:

```rust
        self.discards_used += 1;
```

- [ ] **Step 4: Run tests — verify pass:** `cargo test --features funky` → PASS (the struct derives `Default`, so `board_playing` boards start at 0). Run all gates.

- [ ] **Step 5: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs
git commit -m "Phase 1c: track discards_used on the board"
```

---

### Task 4: Payout seam + Golden Joker

**Files:**
- Modify: `src/funky/types/board.rs` (`GrowthEvent` ~line 14; new methods next to `apply_growth` ~line 865)
- Modify: `src/funky/decks/joker.rs` (`GOLDEN_JOKER` const, ~line 1291)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: `MPip::CashOnRoundEnd` (Task 1), `discards_used` (Task 3).
- Produces: `GrowthEvent::RoundEnd`; `fn payout_delta(&self, enhancement: MPip, event: &GrowthEvent) -> isize`; `fn apply_payouts(&mut self, event: &GrowthEvent)`; `pub fn on_round_end(&mut self)`; `fn cash(n: usize) -> isize`. Tasks 5–9 add arms/behaviour to these; Task 10 wraps `on_round_end`.

- [ ] **Step 1: Write the failing tests:**

```rust
    #[test]
    fn on_round_end__golden_joker_pays_4() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::GOLDEN_JOKER);

        board.on_round_end();

        assert_eq!(board.money, 4);
    }

    #[test]
    fn on_round_end__is_inert_on_a_plain_board() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::JOKER); // +4 mult, no lifecycle behaviour
        let before = board.clone();

        board.on_round_end();

        // Only discards_used may reset (it is already 0 here), so the whole
        // board is unchanged — the inertness invariant.
        assert_eq!(board, before);
    }
```

- [ ] **Step 2: Run — verify compile failure** (`on_round_end` not defined):

Run: `cargo test --features funky on_round_end__golden`
Expected: compile error "no method named `on_round_end`".

- [ ] **Step 3: Implement the seam** in `board.rs`. Add `RoundEnd` to the event enum:

```rust
/// An in-round event that can grow a joker's counter or pay out money.
enum GrowthEvent<'a> {
    HandPlayed(&'a BuffoonPile),
    Discard(&'a BuffoonPile),
    RoundEnd,
}
```

Add next to `apply_growth`:

```rust
    /// A `usize` dollar amount as the signed money delta, saturating (money
    /// magnitudes are tiny; this only satisfies pedantic cast rules).
    fn cash(n: usize) -> isize {
        isize::try_from(n).unwrap_or(isize::MAX)
    }

    /// Money a joker pays out for one lifecycle event — the money mirror of
    /// [`growth_delta`](Self::growth_delta). Takes `&self` because payouts
    /// read board state (full deck, money, discards). Returns 0 for every
    /// non-cash joker, so boards without them see no money movement.
    fn payout_delta(&self, enhancement: MPip, event: &GrowthEvent) -> isize {
        match (enhancement, event) {
            (MPip::CashOnRoundEnd(n), GrowthEvent::RoundEnd) => Self::cash(n),
            _ => 0,
        }
    }

    /// Apply every joker's payout for `event` to the board's money.
    fn apply_payouts(&mut self, event: &GrowthEvent) {
        let total: isize = self
            .jokers
            .iter()
            .map(|joker| self.payout_delta(joker.enhancement, event))
            .sum();
        self.money += total;
    }

    /// Round-end lifecycle, deterministic half: pays the round-end `+$`
    /// jokers and resets [`discards_used`](Self::discards_used). Probabilistic
    /// round-end effects (joker destruction rolls) need an RNG and live in
    /// [`on_round_end_with_rng`](Self::on_round_end_with_rng) — calling this
    /// plain version skips them, exactly as pure [`score`](Self::score) keeps
    /// Lucky inert.
    pub fn on_round_end(&mut self) {
        self.apply_payouts(&GrowthEvent::RoundEnd);
        self.discards_used = 0;
    }
```

- [ ] **Step 4: Flip Golden Joker's const** in `joker.rs` (~line 1291): `enhancement: MPip::Blank,` → `enhancement: MPip::CashOnRoundEnd(4),`

- [ ] **Step 5: Run tests — verify pass:** `cargo test --features funky` → PASS (both new tests; the reachability guard ignores Golden Joker because `CashOnRoundEnd` is a `false` oracle arm). Run all gates.

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: payout seam (RoundEnd event + payout_delta) + Golden Joker pays $4"
```

---

### Task 5: Delayed Gratification

**Files:**
- Modify: `src/funky/types/board.rs` (`payout_delta`), `src/funky/decks/joker.rs` (`DELAYED_GRATIFICATION`, ~line 588)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: seam from Task 4; `discards_used` from Task 3.

- [ ] **Step 1: Write the failing tests** (`board_playing` uses `Draws::new(4, 3)`, so 3 discards remain):

```rust
    #[test]
    fn on_round_end__delayed_gratification_pays_2_per_remaining_discard() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DELAYED_GRATIFICATION);

        board.on_round_end();

        assert_eq!(board.money, 6, "$2 x 3 remaining discards, none used");
    }

    #[test]
    fn on_round_end__delayed_gratification_pays_0_after_a_discard() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::DELAYED_GRATIFICATION);
        board.on_discard(&bcards!("3C"));

        board.on_round_end();

        assert_eq!(board.money, 0, "any discard this round voids the payout");
    }
```

- [ ] **Step 2: Run — verify both fail** (money stays 0 / passes vacuously?): the first test FAILs (pays 0, expects 6); the second passes trivially — that's fine, it's the guard rail for the arm you're about to add.

Run: `cargo test --features funky delayed_gratification`
Expected: 1 FAIL (`pays_2_per`), 1 PASS.

- [ ] **Step 3: Add the arm** to `payout_delta`, after the `CashOnRoundEnd` arm:

```rust
            (MPip::CashPerDiscardIfNoneUsed(n), GrowthEvent::RoundEnd)
                if self.discards_used == 0 =>
            {
                Self::cash(n * self.draws.discards)
            }
```

- [ ] **Step 4: Flip the const** (`DELAYED_GRATIFICATION`): `MPip::Blank` → `MPip::CashPerDiscardIfNoneUsed(2)`.

- [ ] **Step 5: Run tests — verify all pass:** `cargo test --features funky` → PASS. Run all gates.

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: Delayed Gratification pays $2 per unused discard"
```

---

### Task 6: Cloud 9

**Files:**
- Modify: `src/funky/types/board.rs` (`payout_delta`), `src/funky/decks/joker.rs` (`CLOUD_9`, ~line 1137)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: seam from Task 4; `full_deck` roster + `destroy_deck_card`/`full_deck_index_of` (Phase 5a/7, exist).

- [ ] **Step 1: Write the failing test:**

```rust
    #[test]
    fn on_round_end__cloud_9_pays_1_per_nine_in_full_deck() {
        // A fresh 52-card deck holds four 9s.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::CLOUD_9);

        board.on_round_end();
        assert_eq!(board.money, 4, "$1 x four 9s");

        // Destroy a 9 from the run: the payout reads the live roster.
        let nine = *bcards!("9S").iter().next().unwrap();
        let slot = board.full_deck_index_of(nine).unwrap();
        board.destroy_deck_card(slot);

        board.on_round_end();
        assert_eq!(board.money, 4 + 3, "$1 x three 9s after the destruction");
    }
```

- [ ] **Step 2: Run — verify it fails:** `cargo test --features funky cloud_9` → FAIL (money 0, expects 4).

- [ ] **Step 3: Add the arm** to `payout_delta`:

```rust
            (MPip::CashPerFullDeckRank(n, rank), GrowthEvent::RoundEnd) => {
                let count = self
                    .full_deck
                    .iter()
                    .filter(|card| card.rank.index == rank)
                    .count();
                Self::cash(n * count)
            }
```

- [ ] **Step 4: Flip the const** (`CLOUD_9`): `MPip::Blank` → `MPip::CashPerFullDeckRank(1, '9')`.

- [ ] **Step 5: Run tests — verify all pass**, then all gates.

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: Cloud 9 pays $1 per 9 in the full deck"
```

---

### Task 7: To the Moon

**Files:**
- Modify: `src/funky/types/board.rs` (`payout_delta`), `src/funky/decks/joker.rs` (`TO_THE_MOON`, ~line 1207)
- Test: `board.rs` tests module

- [ ] **Step 1: Write the failing tests:**

```rust
    #[test]
    fn on_round_end__to_the_moon_pays_1_per_5_dollars() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.money = 23; // 4 full $5 increments
        board.push_joker(card::TO_THE_MOON);

        board.on_round_end();

        assert_eq!(board.money, 23 + 4);
    }

    #[test]
    fn on_round_end__to_the_moon_extra_interest_caps_at_5() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.money = 100; // 20 increments, but the interest cap is 5
        board.push_joker(card::TO_THE_MOON);

        board.on_round_end();

        assert_eq!(board.money, 100 + 5);
    }
```

- [ ] **Step 2: Run — verify both fail:** `cargo test --features funky to_the_moon` → 2 FAIL (no payout).

- [ ] **Step 3: Add the arm** to `payout_delta`:

```rust
            (MPip::ExtraInterest(n), GrowthEvent::RoundEnd) => {
                // $n per full $5 held, capped at the base game's 5 interest
                // increments (no cap-raising vouchers exist yet). Debt earns
                // nothing.
                let increments = usize::try_from(self.money).unwrap_or(0) / 5;
                Self::cash(n * increments.min(5))
            }
```

- [ ] **Step 4: Flip the const** (`TO_THE_MOON`): `MPip::Blank` → `MPip::ExtraInterest(1)`.

- [ ] **Step 5: Run tests — verify all pass**, then all gates. (Payout order note: `apply_payouts` sums all deltas against the *pre-payout* `money` snapshot in one pass, so Golden Joker money landing the same round does **not** compound into To the Moon's interest — matching Balatro, where interest reads the money you ended the round with.)

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: To the Moon pays $1 extra interest per $5, capped at $5"
```

---

### Task 8: Faceless Joker (discard payout)

**Files:**
- Modify: `src/funky/types/board.rs` (`payout_delta`, `on_discard`), `src/funky/decks/joker.rs` (`FACELESS_JOKER`, ~line 909)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: seam (Task 4), `is_face_card` (`board.rs:447`, Pareidolia-aware).

- [ ] **Step 1: Write the failing tests:**

```rust
    #[test]
    fn on_discard__faceless_joker_pays_5_on_three_faces() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);

        board.on_discard(&bcards!("KC QD JH 2S"));

        assert_eq!(board.money, 5);
    }

    #[test]
    fn on_discard__faceless_joker_pays_0_on_two_faces() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);

        board.on_discard(&bcards!("KC QD 2S 3H"));

        assert_eq!(board.money, 0);
    }

    #[test]
    fn on_discard__pareidolia_makes_any_three_cards_faces_for_faceless() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::FACELESS_JOKER);
        board.push_joker(card::PAREIDOLIA);

        board.on_discard(&bcards!("2C 3D 4H"));

        assert_eq!(board.money, 5, "Pareidolia turns every discard into a face");
    }
```

- [ ] **Step 2: Run — verify the two payout tests fail** (`pays_0` passes trivially): `cargo test --features funky faceless` → 2 FAIL, 1 PASS. (Also `pareidolia_makes` FAILs.)

- [ ] **Step 3: Add the arm and the hook call.** In `payout_delta`:

```rust
            (MPip::CashOnFacesDiscarded(cash, min_faces), GrowthEvent::Discard(discarded)) => {
                let faces = discarded
                    .iter()
                    .filter(|card| self.is_face_card(card))
                    .count();
                if faces >= min_faces {
                    Self::cash(cash)
                } else {
                    0
                }
            }
```

In `on_discard`, add the payout pass between the growth call and the `discards_used` increment, so the full body reads:

```rust
    pub fn on_discard(&mut self, discarded: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::Discard(discarded));
        self.apply_payouts(&GrowthEvent::Discard(discarded));
        self.discards_used += 1;
    }
```

- [ ] **Step 4: Flip the const** (`FACELESS_JOKER`): `MPip::Blank` → `MPip::CashOnFacesDiscarded(5, 3)`.

- [ ] **Step 5: Run tests — verify all pass**, then all gates. (Ramen's `LoseMultTimesPerDiscard` counter tests must stay green — `apply_growth` is untouched.)

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 1c: Faceless Joker pays $5 on 3+ discarded faces (Pareidolia-aware)"
```

---

### Task 9: Egg grows its sell value at round end

**Files:**
- Modify: `src/funky/types/board.rs` (`on_round_end` + new `grow_sell_values`)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: `MPip::SellValueIncrement(usize)` (exists, `mpip.rs:188`); `EGG` const already carries `SellValueIncrement(3)` — **no joker.rs change in this task**.
- Produces: `fn grow_sell_values(&mut self)` (private).

- [ ] **Step 1: Write the failing test:**

```rust
    #[test]
    fn on_round_end__egg_grows_its_own_resell_value() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::EGG);
        let before = card::EGG.resell_value;

        board.on_round_end();
        board.on_round_end();

        let egg = board.jokers.iter().next().copied().unwrap();
        assert_eq!(egg.resell_value, before + 6, "+$3 per round end, twice");
        assert_eq!(board.money, 0, "Egg moves sell value, not money");
    }
```

- [ ] **Step 2: Run — verify it fails:** `cargo test --features funky egg_grows` → FAIL (resell unchanged).

- [ ] **Step 3: Implement.** Add next to `apply_payouts` (same remove/insert mutation pattern `on_scored` uses):

```rust
    /// Grow the sell value of every `SellValueIncrement` joker (Egg: +$3 per
    /// round). Mutates the joker card in place in the pile.
    fn grow_sell_values(&mut self) {
        for index in 0..self.jokers.len() {
            let Some(joker) = self.jokers.get(index).copied() else {
                continue;
            };
            if let MPip::SellValueIncrement(n) = joker.enhancement {
                let grown = BuffoonCard {
                    resell_value: joker.resell_value + n,
                    ..joker
                };
                self.jokers.remove(index);
                self.jokers.insert(index, grown);
            }
        }
    }
```

Call it from `on_round_end`, between the payouts and the reset:

```rust
    pub fn on_round_end(&mut self) {
        self.apply_payouts(&GrowthEvent::RoundEnd);
        self.grow_sell_values();
        self.discards_used = 0;
    }
```

(`BuffoonPile::get` is the same accessor `on_scored` uses at `board.rs:853`.)

- [ ] **Step 4: Run tests — verify all pass**, then all gates. `on_round_end__is_inert_on_a_plain_board` (Task 4) proves non-Egg jokers are untouched.

- [ ] **Step 5: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs
git commit -m "Phase 1c: Egg grows its resell value $3 per round end"
```

---

### Task 10: Destruction pass — `on_round_end_with_rng`

**Files:**
- Modify: `src/funky/types/board.rs` (new `on_round_end_with_rng`, `roll_destructions`)
- Test: `board.rs` tests module (`StdRng` is already imported at `board.rs:6`; `SeedableRng` comes with it — check the existing seeded tests' `use` lines and mirror them)

**Interfaces:**
- Consumes: `on_round_end` (Task 4/9), `probability_numerator()` (`board.rs:489`), `remove_joker` (`board.rs:691`), `MPip::MultPlusChanceDestroyed` (Gros Michel, exists) + `MPip::MultTimesChanceDestroyed` (Task 2).
- Produces: `pub fn on_round_end_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R)`.

- [ ] **Step 1: Write the failing tests:**

```rust
    #[test]
    fn on_round_end_with_rng__gros_michel_dies_about_1_in_6() {
        // Deterministic over a fixed seed range: some seeds destroy, most
        // don't, and the destruction rate sits in a generous 1-in-6 band.
        let destroyed = (0..100u64)
            .filter(|&seed| {
                let mut board = board_playing("2S 5D 8C TS KH");
                board.push_joker(card::GROS_MICHEL);
                board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
                board.jokers.is_empty()
            })
            .count();
        assert!((5..=40).contains(&destroyed), "1-in-6 out of band: {destroyed}/100");
    }

    #[test]
    fn on_round_end_with_rng__cavendish_1_in_1000_almost_never_fires() {
        let destroyed = (0..100u64)
            .filter(|&seed| {
                let mut board = board_playing("2S 5D 8C TS KH");
                board.push_joker(card::CAVENDISH);
                board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
                board.jokers.is_empty()
            })
            .count();
        assert!(destroyed <= 2, "1-in-1000 fired {destroyed} times in 100 seeds");
    }

    #[test]
    fn on_round_end_with_rng__oops_doubles_gros_michel_destruction_odds() {
        let destroyed_count = |with_oops: bool| {
            (0..200u64)
                .filter(|&seed| {
                    let mut board = board_playing("2S 5D 8C TS KH");
                    board.push_joker(card::GROS_MICHEL);
                    if with_oops {
                        board.push_joker(card::OOPS_ALL_6S);
                    }
                    board.on_round_end_with_rng(&mut StdRng::seed_from_u64(seed));
                    !board
                        .jokers
                        .iter()
                        .any(|j| matches!(j.enhancement, MPip::MultPlusChanceDestroyed(..)))
                })
                .count()
        };
        assert!(
            destroyed_count(true) > destroyed_count(false),
            "Oops! All 6s must double the destroy odds via probability_numerator"
        );
    }

    #[test]
    fn on_round_end_with_rng__still_pays_the_deterministic_payouts() {
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::GOLDEN_JOKER);

        board.on_round_end_with_rng(&mut StdRng::seed_from_u64(0));

        assert_eq!(board.money, 4, "the rng path must include on_round_end");
    }
```

- [ ] **Step 2: Run — verify compile failure** (`on_round_end_with_rng` not defined).

- [ ] **Step 3: Implement**, next to `on_round_end`:

```rust
    /// Round-end lifecycle, full version: everything
    /// [`on_round_end`](Self::on_round_end) does, then the probabilistic
    /// destruction rolls (Gros Michel 1-in-6, Cavendish 1-in-1000). Payouts
    /// land **before** destruction — Balatro's cash-out-then-cleanup order.
    pub fn on_round_end_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.on_round_end();
        self.roll_destructions(rng);
    }

    /// Roll each self-destroying joker's end-of-round chance and remove the
    /// losers. Odds route through
    /// [`probability_numerator`](Self::probability_numerator), so Oops! All 6s
    /// doubles them, capped at certainty. Indices are collected first and
    /// removed in reverse so removal never shifts an unprocessed slot.
    fn roll_destructions<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let scale = self.probability_numerator();
        let mut doomed: Vec<usize> = Vec::new();
        for (index, joker) in self.jokers.iter().enumerate() {
            let (numerator, denominator) = match joker.enhancement {
                MPip::MultPlusChanceDestroyed(_, n, d)
                | MPip::MultTimesChanceDestroyed(_, n, d) => (n, d),
                _ => continue,
            };
            if denominator == 0 {
                continue;
            }
            let wins = (numerator * scale).min(denominator);
            if rng.random_range(0..denominator) < wins {
                doomed.push(index);
            }
        }
        for index in doomed.into_iter().rev() {
            self.remove_joker(index);
        }
    }
```

- [ ] **Step 4: Run tests — verify all pass**, then all gates. (Note the Oops test counts by enhancement, not emptiness, because Oops! All 6s itself stays on the board.)

- [ ] **Step 5: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs
git commit -m "Phase 1c: round-end destruction rolls (Gros Michel, Cavendish), Oops-aware"
```

---

### Task 11: Ice Cream melts on the hand that empties it

**Files:**
- Modify: `src/funky/types/board.rs` (`on_hand_played` + new `melt_spent_jokers`)
- Test: `board.rs` tests module

**Interfaces:**
- Consumes: `on_hand_played` (`board.rs:799`), `joker_state` counters, `MPip::LoseChipsPerHand(100, 5)` (Ice Cream, exists), `remove_joker`.

- [ ] **Step 1: Write the failing test:**

```rust
    #[test]
    fn on_hand_played__ice_cream_melts_when_its_chips_reach_zero() {
        // Ice Cream: 100 chips, −5 per hand played → empty on the 20th hand.
        let mut board = board_playing("2S 5D 8C TS KH");
        board.push_joker(card::ICE_CREAM);

        for _ in 0..19 {
            board.on_hand_played(&bcards!("2S"));
        }
        assert_eq!(board.jokers.len(), 1, "19 hands: 5 chips left, not melted");

        board.on_hand_played(&bcards!("2S"));
        assert!(board.jokers.is_empty(), "20th hand: 0 chips, melted");
    }
```

- [ ] **Step 2: Run — verify it fails:** `cargo test --features funky ice_cream_melts` → FAIL (joker still present after 20 hands).

- [ ] **Step 3: Implement.** Extend `on_hand_played` and add the helper:

```rust
    /// Grow every joker's counter for a played hand, then melt any decaying
    /// joker the hand just emptied (Ice Cream at 0 chips) — Balatro removes it
    /// on the hand that empties it, not at round end.
    pub fn on_hand_played(&mut self, played: &BuffoonPile) {
        self.apply_growth(&GrowthEvent::HandPlayed(played));
        self.melt_spent_jokers();
    }

    /// Remove every `LoseChipsPerHand` joker whose decayed chips have reached
    /// 0 — the destruction half of Ice Cream, deterministic (no RNG). Mirrors
    /// `counter_joker_op`'s read: `base − per × hands`, floored at 0.
    fn melt_spent_jokers(&mut self) {
        let mut doomed: Vec<usize> = Vec::new();
        for (index, joker) in self.jokers.iter().enumerate() {
            if let MPip::LoseChipsPerHand(base, per) = joker.enhancement {
                let hands =
                    usize::try_from(self.joker_state.get(index).copied().unwrap_or(0).max(0))
                        .unwrap_or(0);
                if base.saturating_sub(per * hands) == 0 {
                    doomed.push(index);
                }
            }
        }
        for index in doomed.into_iter().rev() {
            self.remove_joker(index);
        }
    }
```

- [ ] **Step 4: Run tests — verify all pass**, then all gates. The existing Ice Cream scoring test (`score__ice_cream…` from Phase 3) must stay green — melting only happens through the hook, and that test plays fewer than 20 hands.

- [ ] **Step 5: Commit** — print for the owner:

```bash
git add src/funky/types/board.rs
git commit -m "Phase 1c: Ice Cream melts on the hand that empties its chips"
```

---

### Task 12: Docs — flip EPIC-01a status, close the loop

**Files:**
- Modify: `docs/EPIC-01a_Joker_Wiring_Backlog.md` (Status table row 1; Work Item 1c ~line 408; §Data fixes Cavendish bullet ~line 270)
- Modify: `docs/superpowers/specs/2026-07-15-funky-lifecycle-hooks-design.md` (one stale line)

- [ ] **Step 1: Flip Work Item 1c** to checked, recording what landed:

```markdown
- [x] **1c.** Lifecycle hooks: `on_round_end` / `on_round_end_with_rng` (payouts,
  Egg sell-value growth, destruction rolls), payout pass on `on_discard`, melt
  check on `on_hand_played`. Wired: Golden Joker, Delayed Gratification, Cloud 9,
  To the Moon, Faceless Joker, Egg; Gros Michel + Cavendish destruction;
  Ice Cream melt. Still Blank with reasons: To Do List / Mail-In Rebate
  (per-round random target), Rocket (boss blinds), Trading Card (discard
  destruction), Reserved Parking (probabilistic held payout — deferred).
```

- [ ] **Step 2: Update the Status table** row `1 — Economy / money`: replace with `**Complete for deterministic payouts** — hooks live; remaining +$ jokers blocked on shop/blinds/round-targets (see 1c)`.

- [ ] **Step 3: Strike the Cavendish bullet** in §Data fixes (~line 270) the way fixed items above it are struck (`~~…~~ Fixed in 1c — carries MultTimesChanceDestroyed(3, 1, 1000), rolled by on_round_end_with_rng.`).

- [ ] **Step 4: Correct the spec's ledger line.** In the spec's Testing section, the sentence "The six newly wired jokers come off the `KNOWN_UNWIRED` ledger" is wrong — those jokers were `Blank`, never on the ledger (it holds only Joker Stencil). Replace with: "`KNOWN_UNWIRED` is untouched — the cash jokers are non-scoring by design and never sat on the ledger."

- [ ] **Step 5: Run the full gate battery one final time:**

```bash
cargo test --features funky && \
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic && \
cargo build --no-default-features && \
cargo fmt --all -- --check && \
cargo doc --no-deps --features funky
```
Expected: all green; test count ≥ 516 (502 + ~14 new).

- [ ] **Step 6: Commit** — print for the owner:

```bash
git add docs/EPIC-01a_Joker_Wiring_Backlog.md docs/superpowers/specs/2026-07-15-funky-lifecycle-hooks-design.md
git commit -m "docs: EPIC-01a 1c complete (lifecycle hooks + 6 payouts + destructions)"
```
