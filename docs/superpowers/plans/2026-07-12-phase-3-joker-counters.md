# Phase 3 Joker Counters (vertical slice) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a per-joker counter store plus two in-round growth events (hand played, discard) and wire the six jokers those events unblock to score their exact Balatro values.

**Architecture:** A parallel `Vec<i32>` on `BuffoonBoard` (`joker_state`), index-aligned with `board.jokers`, holds one accumulator per joker. Two `&mut self` event hooks tick each joker's accumulator via a per-variant `growth_delta`; `fold_jokers` reads the accumulator (missing → 0) and routes counter variants through a new `counter_joker_op` seam, keeping `score()` a `&self` read.

**Tech Stack:** Rust, `cargo test --features funky`, the funky scoring engine (`ScoreOp` fold pipeline).

## Global Constraints

- Feature flag: all work is behind `--features funky`; must not leak into `--no-default-features` (that build must stay green).
- Exact Balatro values, tested; every new scoring behaviour has a test that fails before its arm lands.
- Lints: `cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic` clean.
- Format: `cargo fmt --all -- --check` clean.
- Do not run state-changing git yourself if the environment forbids it; the commit steps below are the exact commands to run (or hand to the user).
- `score()` stays `&self` — counters are read during scoring, written only by event hooks.

---

### Task 1: Counter store + alignment helpers

**Files:**
- Modify: `src/funky/types/board.rs` (struct `BuffoonBoard`, `fn new`, new helpers)
- Test: `src/funky/types/board.rs` (test module `funky__types__board__buffoon_board_tests`)

**Interfaces:**
- Produces:
  - field `pub joker_state: Vec<i32>` on `BuffoonBoard`
  - `pub fn push_joker(&mut self, joker: BuffoonCard)`
  - `pub fn remove_joker(&mut self, index: usize) -> BuffoonCard`
  - `fn ensure_state_len(&mut self)`

- [ ] **Step 1: Write the failing test**

Add to the board test module (near the other helpers/tests):

```rust
#[test]
fn joker_state__push_and_remove_stay_aligned() {
    let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
    board.push_joker(card::JOKER);
    board.push_joker(card::CAVENDISH);
    assert_eq!(board.jokers.len(), 2);
    assert_eq!(board.joker_state, vec![0, 0]);

    // Grow the second joker's counter, then remove the first.
    board.joker_state[1] = 7;
    let removed = board.remove_joker(0);
    assert_eq!(removed, card::JOKER);
    assert_eq!(board.jokers.len(), 1);
    // The survivor keeps its counter, now at index 0.
    assert_eq!(board.joker_state, vec![7]);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky joker_state__push_and_remove_stay_aligned`
Expected: FAIL — `no field 'joker_state'` / `no method 'push_joker'` (compile error).

- [ ] **Step 3: Add the field and initialise it**

In `struct BuffoonBoard` add after `pub money: isize,`:

```rust
    /// One accumulator per joker, index-aligned with `jokers`: `joker_state[i]`
    /// belongs to `jokers[i]`. Signed because Green Joker's net (hands −
    /// discards) can dip negative before the read floors it at 0. Grown by the
    /// event hooks, read (never written) during scoring.
    pub joker_state: Vec<i32>,
```

In `fn new`, add `joker_state: Vec::new(),` to the struct literal (after `money: 0,`).

- [ ] **Step 4: Add the helpers**

Add to `impl BuffoonBoard` (near the jokers/scoring helpers):

```rust
/// Add a joker with a fresh (0) counter, keeping `joker_state` aligned.
pub fn push_joker(&mut self, joker: BuffoonCard) {
    self.jokers.push(joker);
    self.joker_state.push(0);
}

/// Remove the joker at `index`, dropping its counter with it.
pub fn remove_joker(&mut self, index: usize) -> BuffoonCard {
    if index < self.joker_state.len() {
        self.joker_state.remove(index);
    }
    self.jokers.remove(index)
}

/// Pad `joker_state` with zeros up to `jokers.len()`, so a board built by
/// setting `jokers` directly still has a counter slot per joker. Only grows —
/// never truncates.
fn ensure_state_len(&mut self) {
    if self.joker_state.len() < self.jokers.len() {
        self.joker_state.resize(self.jokers.len(), 0);
    }
}
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky joker_state__push_and_remove_stay_aligned`
Expected: PASS.

Note: `ensure_state_len` is unused until Task 3; add `#[allow(dead_code)]` above it if clippy complains at this task, and remove that attribute in Task 3 when it is called.

- [ ] **Step 6: Commit**

```bash
git add src/funky/types/board.rs
git commit -m "Phase 3: add per-joker counter store + push/remove helpers"
```

---

### Task 2: Six counter MPip variants + Display + classification

**Files:**
- Modify: `src/funky/types/mpip.rs` (enum `MPip`, `impl Display`)
- Modify: `src/funky/decks/joker.rs` (`fn scores_hand` in the test module)
- Test: `src/funky/types/mpip.rs` (test module)

**Interfaces:**
- Produces (new `MPip` variants):
  - `GainMultPerHandLessDiscard(usize)`
  - `LoseMultTimesPerDiscard(usize, usize)`
  - `LoseChipsPerHand(usize, usize)`
  - `GainChipsPerCardCountHand(usize, usize)`
  - `GainMultPerTwoPairHand(usize)`
  - `GainChipsPerStraightHand(usize)`

- [ ] **Step 1: Write the failing test**

Find the mpip test module (`grep -n "mod .*mpip.*tests\|#\[cfg(test)\]" src/funky/types/mpip.rs`). Add:

```rust
#[test]
fn display__counter_variants() {
    assert_eq!(MPip::GainMultPerHandLessDiscard(1).to_string(), "GainMultPerHandLessDiscard(1)");
    assert_eq!(MPip::LoseMultTimesPerDiscard(200, 1).to_string(), "LoseMultTimesPerDiscard(200, 1)");
    assert_eq!(MPip::LoseChipsPerHand(100, 5).to_string(), "LoseChipsPerHand(100, 5)");
    assert_eq!(MPip::GainChipsPerCardCountHand(4, 4).to_string(), "GainChipsPerCardCountHand(4, 4)");
    assert_eq!(MPip::GainMultPerTwoPairHand(2).to_string(), "GainMultPerTwoPairHand(2)");
    assert_eq!(MPip::GainChipsPerStraightHand(15).to_string(), "GainChipsPerStraightHand(15)");
}
```

If no test module exists in `mpip.rs`, create one at the end of the file:

```rust
#[cfg(test)]
mod funky__types__mpip_tests {
    use super::*;

    // (test above goes here)
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky display__counter_variants`
Expected: FAIL — `no variant 'GainMultPerHandLessDiscard'` (compile error).

- [ ] **Step 3: Add the variants**

In `enum MPip`, add (place next to the other counter-ish variants, e.g. after `ChipsPerDollar(usize),`):

```rust
    /// Green Joker: +n mult per hand played, −n per discard; the accumulator is
    /// the net (hands − discards), the read floors it at 0.
    GainMultPerHandLessDiscard(usize),
    /// Ramen: starts at `base`/100 ×mult and loses `per`/100 per card discarded;
    /// the accumulator counts cards discarded. Read floors at ×1.
    LoseMultTimesPerDiscard(usize, usize),
    /// Ice Cream: starts at `base` chips and loses `per` per hand played; the
    /// accumulator counts hands played. Read floors at 0.
    LoseChipsPerHand(usize, usize),
    /// Square Joker: +`rate` chips per hand played with exactly `n` cards; the
    /// accumulator counts qualifying hands.
    GainChipsPerCardCountHand(usize, usize),
    /// Spare Trousers: +`rate` mult per hand played containing a Two Pair.
    GainMultPerTwoPairHand(usize),
    /// Runner: +`rate` chips per hand played containing a Straight.
    GainChipsPerStraightHand(usize),
```

- [ ] **Step 4: Add the Display arms**

In `impl Display for MPip`, add arms (place near `ChipsPerDollar`):

```rust
            Self::GainMultPerHandLessDiscard(n) => write!(f, "GainMultPerHandLessDiscard({n})"),
            Self::LoseMultTimesPerDiscard(base, per) => {
                write!(f, "LoseMultTimesPerDiscard({base}, {per})")
            }
            Self::LoseChipsPerHand(base, per) => write!(f, "LoseChipsPerHand({base}, {per})"),
            Self::GainChipsPerCardCountHand(rate, n) => {
                write!(f, "GainChipsPerCardCountHand({rate}, {n})")
            }
            Self::GainMultPerTwoPairHand(n) => write!(f, "GainMultPerTwoPairHand({n})"),
            Self::GainChipsPerStraightHand(n) => write!(f, "GainChipsPerStraightHand({n})"),
```

- [ ] **Step 5: Classify them in the reachability guard**

In `src/funky/decks/joker.rs`, in `fn scores_hand`, add the six variants to the `true` group (the big `|`-joined list), e.g. after `| MPip::ChipsPerDollar(_)`:

```rust
            | MPip::GainMultPerHandLessDiscard(_)
            | MPip::LoseMultTimesPerDiscard(_, _)
            | MPip::LoseChipsPerHand(_, _)
            | MPip::GainChipsPerCardCountHand(_, _)
            | MPip::GainMultPerTwoPairHand(_)
            | MPip::GainChipsPerStraightHand(_)
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test --features funky display__counter_variants`
Expected: PASS.

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS (the new variants are classified but not yet attached to any joker, so the guard is unaffected).

- [ ] **Step 7: Commit**

```bash
git add src/funky/types/mpip.rs src/funky/decks/joker.rs
git commit -m "Phase 3: add six counter MPip variants (Display + classification)"
```

---

### Task 3: Growth + read plumbing (inert skeleton)

**Files:**
- Modify: `src/funky/types/board.rs` (`GrowthEvent`, `growth_delta`, `on_hand_played`, `on_discard`, `counter_joker_op`, `fold_jokers`)
- Modify: `src/funky/decks/joker.rs` (`fn is_reachable` in the test module)
- Test: `src/funky/types/board.rs` (test module)

**Interfaces:**
- Consumes: `MPip` counter variants (Task 2), `joker_state` + `ensure_state_len` (Task 1).
- Produces:
  - `enum GrowthEvent<'a> { HandPlayed(&'a BuffoonPile), Discard(&'a BuffoonPile) }`
  - `fn growth_delta(enhancement: MPip, event: &GrowthEvent) -> i32` (assoc fn on `BuffoonBoard`)
  - `pub fn on_hand_played(&mut self, played: &BuffoonPile)`
  - `pub fn on_discard(&mut self, discarded: &BuffoonPile)`
  - `fn counter_joker_op(&self, joker: &BuffoonCard, counter: i32) -> Option<ScoreOp>`

- [ ] **Step 1: Write the failing test**

Add to the board test module:

```rust
#[test]
fn on_hand_played__is_inert_without_counter_jokers() {
    // A board with only a non-counter joker scores identically before and
    // after events fire; the plumbing exists but does nothing yet.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::CAVENDISH); // MultTimes(3): 40/1 -> 40/3
    let before = board.score();

    board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
    board.on_discard(&bcards!("2C 3C"));

    assert_eq!(board.score(), before, "no counter jokers -> events change nothing");
    assert_eq!(board.joker_state, vec![0], "Cavendish is not a counter joker");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky on_hand_played__is_inert_without_counter_jokers`
Expected: FAIL — `no method 'on_hand_played'` (compile error).

- [ ] **Step 3: Add the event enum, growth_delta, and hooks**

Add to `board.rs` (module scope, near `BuffoonBoard`):

```rust
/// An in-round event that can grow a joker's counter.
enum GrowthEvent<'a> {
    HandPlayed(&'a BuffoonPile),
    Discard(&'a BuffoonPile),
}
```

Add to `impl BuffoonBoard`:

```rust
/// How much a joker's counter changes for one growth event. The write-side
/// mirror of `counter_joker_op`; both switch on the same enhancement. Returns
/// 0 for every non-counter joker.
fn growth_delta(enhancement: MPip, event: &GrowthEvent) -> i32 {
    match (enhancement, event) {
        (MPip::GainMultPerHandLessDiscard(_), GrowthEvent::HandPlayed(_)) => 1,
        (MPip::GainMultPerHandLessDiscard(_), GrowthEvent::Discard(_)) => -1,
        (MPip::LoseMultTimesPerDiscard(_, _), GrowthEvent::Discard(d)) => {
            i32::try_from(d.len()).unwrap_or(i32::MAX)
        }
        (MPip::LoseChipsPerHand(_, _), GrowthEvent::HandPlayed(_)) => 1,
        (MPip::GainChipsPerCardCountHand(_, n), GrowthEvent::HandPlayed(p)) if p.len() == n => 1,
        (MPip::GainMultPerTwoPairHand(_), GrowthEvent::HandPlayed(p)) if p.has_2pair() => 1,
        (MPip::GainChipsPerStraightHand(_), GrowthEvent::HandPlayed(p)) if p.has_straight() => 1,
        _ => 0,
    }
}

/// Grow every joker's counter for a played hand.
pub fn on_hand_played(&mut self, played: &BuffoonPile) {
    self.apply_growth(&GrowthEvent::HandPlayed(played));
}

/// Grow every joker's counter for a discard.
pub fn on_discard(&mut self, discarded: &BuffoonPile) {
    self.apply_growth(&GrowthEvent::Discard(discarded));
}

fn apply_growth(&mut self, event: &GrowthEvent) {
    self.ensure_state_len();
    let deltas: Vec<i32> = self
        .jokers
        .iter()
        .map(|j| Self::growth_delta(j.enhancement, event))
        .collect();
    for (slot, delta) in self.joker_state.iter_mut().zip(deltas) {
        *slot += delta;
    }
}
```

Remove the `#[allow(dead_code)]` from `ensure_state_len` if it was added in Task 1 (it is now used by `apply_growth`).

- [ ] **Step 4: Add the read seam (returns None for everything for now)**

Add to `impl BuffoonBoard`:

```rust
/// The scoring contribution of a counter joker given its accumulator, or
/// `None` if `joker` is not a counter joker (so scoring falls through to
/// `builtin_joker_op`). Arms are added per joker in later tasks.
fn counter_joker_op(&self, joker: &BuffoonCard, counter: i32) -> Option<ScoreOp> {
    let _ = counter;
    match joker.enhancement {
        _ => None,
    }
}
```

Note: keep the explicit `match` (not just `None`) so later tasks add arms above the `_ => None`. If clippy flags the single-arm match, use `#[allow(clippy::match_single_binding)]` on the function for this task; it goes away once real arms land.

- [ ] **Step 5: Route counter jokers in `fold_jokers`**

In `fn fold_jokers`, change the loop header from `for joker in &self.jokers {` to:

```rust
        for (index, joker) in self.jokers.iter().enumerate() {
```

and change the final `_ =>` arm of the inner `match joker.enhancement` from `_ => self.builtin_joker_op(joker),` to:

```rust
                _ => {
                    let counter = self.joker_state.get(index).copied().unwrap_or(0);
                    self.counter_joker_op(joker, counter)
                        .unwrap_or_else(|| self.builtin_joker_op(joker))
                }
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test --features funky on_hand_played__is_inert_without_counter_jokers`
Expected: PASS.

- [ ] **Step 7: Make the reachability guard drive events**

In `src/funky/decks/joker.rs`, replace `fn is_reachable` with a version that fires the in-round events so counter jokers accumulate before scoring:

```rust
    /// A joker is *reachable* if adding it to some probe board changes the score.
    /// In-round events are fired after the joker is added so counter jokers
    /// (Green Joker, Ramen, …) accumulate before the score is read.
    fn is_reachable(joker: BuffoonCard) -> bool {
        // Hands that satisfy every growth condition across the slice: a 4-card
        // straight (Square + Runner), a two-pair hand (Spare Trousers), and a
        // generic hand (Green Joker, Ice Cream).
        let growth_hands = [
            bcards!("2C 3C 4C 5C"),
            bcards!("KH KS QD QC 4S"),
            bcards!("AH KH QH JH TH"),
        ];
        probe_boards().into_iter().any(|mut board| {
            let baseline = board.score();
            board.jokers.push(joker);
            for hand in &growth_hands {
                board.on_hand_played(hand);
            }
            board.on_discard(&bcards!("2C 3C 4C"));
            board.score() != baseline
        })
    }
```

- [ ] **Step 8: Run the full guard + suite to verify still green**

Run: `cargo test --features funky all_jokers__ && cargo test --features funky`
Expected: PASS (no counter jokers attached yet, so the event-driving is inert; all other tests unaffected).

- [ ] **Step 9: Commit**

```bash
git add src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 3: growth-event + counter-read plumbing (inert skeleton)"
```

---

### Task 4: Wire Green Joker

**Files:**
- Modify: `src/funky/decks/joker.rs` (`GREEN_JOKER` const)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs` (test module)

**Interfaces:**
- Consumes: `on_hand_played`, `on_discard`, `counter_joker_op`, `MPip::GainMultPerHandLessDiscard`.

- [ ] **Step 1: Write the failing test**

Add to the board test module:

```rust
#[test]
fn score__green_joker_gains_mult_per_hand_less_discard() {
    // Green Joker: +1 Mult per hand played, −1 per discard; floors at 0.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::GREEN_JOKER);

    // 3 hands, 1 discard -> net +2 mult.
    let hand = bcards!("2S 5D 8C TS KH");
    board.on_hand_played(&hand);
    board.on_hand_played(&hand);
    board.on_hand_played(&hand);
    board.on_discard(&bcards!("9C"));
    assert_eq!(board.score(), Score::new(40, 3)); // 1 + 2

    // More discards than hands -> floored at +0 mult, not negative.
    board.on_discard(&bcards!("9C"));
    board.on_discard(&bcards!("9C"));
    board.on_discard(&bcards!("9C"));
    assert_eq!(board.score(), Score::new(40, 1));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__green_joker`
Expected: FAIL — `GREEN_JOKER` is still `Blank`, scores 40/1 both times (the `40/3` assertion fails).

- [ ] **Step 3: Flip the const**

In `src/funky/decks/joker.rs`, find `pub const GREEN_JOKER` and set its enhancement:

```rust
        // Green Joker: +1 Mult per hand played, −1 per discard (net, floored ≥0).
        enhancement: MPip::GainMultPerHandLessDiscard(1),
```

- [ ] **Step 4: Add the read arm**

In `board.rs` `fn counter_joker_op`, add above `_ => None`:

```rust
        MPip::GainMultPerHandLessDiscard(rate) => {
            let net = counter.max(0);
            #[allow(clippy::cast_sign_loss)]
            Some(ScoreOp::AddMult(rate * net as usize))
        }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky score__green_joker`
Expected: PASS.

- [ ] **Step 6: Verify the guard still passes**

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS (Green Joker now grows on the guard's driven events → reachable).

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Green Joker (+1 mult/hand, -1/discard)"
```

---

### Task 5: Wire Square Joker

**Files:**
- Modify: `src/funky/decks/joker.rs` (`SQUARE_JOKER` const)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__square_joker_gains_chips_per_four_card_hand() {
    // Square Joker: +4 chips for each hand played with exactly 4 cards.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::SQUARE_JOKER);

    // Two 4-card hands -> +8 chips.
    board.on_hand_played(&bcards!("2S 5D 8C TC"));
    board.on_hand_played(&bcards!("3S 6D 9C JC"));
    assert_eq!(board.score(), Score::new(48, 1));

    // A 5-card hand does not qualify -> no further gain.
    board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
    assert_eq!(board.score(), Score::new(48, 1));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__square_joker`
Expected: FAIL — `SQUARE_JOKER` is `Blank`, scores 40/1.

- [ ] **Step 3: Flip the const**

```rust
        // Square Joker: +4 chips for each hand played with exactly 4 cards.
        enhancement: MPip::GainChipsPerCardCountHand(4, 4),
```

- [ ] **Step 4: Add the read arm** (above `_ => None` in `counter_joker_op`)

```rust
        MPip::GainChipsPerCardCountHand(rate, _n) => {
            #[allow(clippy::cast_sign_loss)]
            Some(ScoreOp::AddChips(rate * counter.max(0) as usize))
        }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky score__square_joker`
Expected: PASS.

- [ ] **Step 6: Verify the guard**

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Square Joker (+4 chips per 4-card hand)"
```

---

### Task 6: Wire Spare Trousers

**Files:**
- Modify: `src/funky/decks/joker.rs` (`SPARE_TROUSERS` const)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__spare_trousers_gains_mult_per_two_pair_hand() {
    // Spare Trousers: +2 Mult for each hand played containing a Two Pair.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::SPARE_TROUSERS);

    // Two two-pair hands -> +4 mult.
    board.on_hand_played(&bcards!("2S 2D 5C 5H 8C"));
    board.on_hand_played(&bcards!("3S 3D 6C 6H 9C"));
    assert_eq!(board.score(), Score::new(40, 5));

    // A no-two-pair hand does not qualify.
    board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
    assert_eq!(board.score(), Score::new(40, 5));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__spare_trousers`
Expected: FAIL — `SPARE_TROUSERS` is `Blank`, scores 40/1.

- [ ] **Step 3: Flip the const**

```rust
        // Spare Trousers: +2 Mult for each hand played containing a Two Pair.
        enhancement: MPip::GainMultPerTwoPairHand(2),
```

- [ ] **Step 4: Add the read arm** (above `_ => None`)

```rust
        MPip::GainMultPerTwoPairHand(rate) => {
            #[allow(clippy::cast_sign_loss)]
            Some(ScoreOp::AddMult(rate * counter.max(0) as usize))
        }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky score__spare_trousers`
Expected: PASS.

- [ ] **Step 6: Verify the guard**

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Spare Trousers (+2 mult per two-pair hand)"
```

---

### Task 7: Wire Runner

**Files:**
- Modify: `src/funky/decks/joker.rs` (`RUNNER` const)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__runner_gains_chips_per_straight_hand() {
    // Runner: +15 chips for each hand played containing a Straight.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::RUNNER);

    // Two straight hands -> +30 chips.
    board.on_hand_played(&bcards!("2S 3D 4C 5H 6C"));
    board.on_hand_played(&bcards!("5S 6D 7C 8H 9C"));
    assert_eq!(board.score(), Score::new(70, 1));

    // A non-straight hand does not qualify.
    board.on_hand_played(&bcards!("2S 5D 8C TS KH"));
    assert_eq!(board.score(), Score::new(70, 1));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__runner`
Expected: FAIL — `RUNNER` is `Blank`, scores 40/1.

- [ ] **Step 3: Flip the const**

```rust
        // Runner: +15 chips for each hand played containing a Straight.
        enhancement: MPip::GainChipsPerStraightHand(15),
```

- [ ] **Step 4: Add the read arm** (above `_ => None`)

```rust
        MPip::GainChipsPerStraightHand(rate) => {
            #[allow(clippy::cast_sign_loss)]
            Some(ScoreOp::AddChips(rate * counter.max(0) as usize))
        }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky score__runner`
Expected: PASS.

- [ ] **Step 6: Verify the guard**

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Runner (+15 chips per straight hand)"
```

---

### Task 8: Wire Ramen (decaying ×mult)

**Files:**
- Modify: `src/funky/decks/joker.rs` (`RAMEN` const)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs`

- [ ] **Step 1: Write the failing test**

The engine applies a ×mult factor via `Score::multi_mult`, which does
`(self.mult as f32 * factor).ceil() as usize`. At base mult 1 that means any
factor in `(1.0, 2.0]` ceils to mult 2, and the exact ×1.00 floor boundary is
reached at `(200 − cards)/100 == 1.0` → **100 cards discarded**. Test the rate
precisely at that boundary:

```rust
#[test]
fn score__ramen_loses_x_mult_per_card_discarded() {
    // Ramen: ×2 Mult, −×0.01 for each card discarded; floors at ×1.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::RAMEN);

    // No discards -> ×2.00 -> mult 1 * 2.00 = 2.
    assert_eq!(board.score(), Score::new(40, 2));

    // 99 cards discarded -> ×1.01 -> ceil(1 * 1.01) = 2.
    for _ in 0..33 {
        board.on_discard(&bcards!("2C 3C 4C")); // 33 * 3 = 99 cards
    }
    assert_eq!(board.score(), Score::new(40, 2), "x1.01 ceils to mult 2");

    // 100th card discarded -> exactly ×1.00 -> mult 1 (the floor boundary).
    board.on_discard(&bcards!("2C"));
    assert_eq!(board.score(), Score::new(40, 1), "x1.00 at the floor boundary");

    // Further discards stay floored at ×1.
    board.on_discard(&bcards!("2C 3C 4C 5C 6C"));
    assert_eq!(board.score(), Score::new(40, 1), "floored at x1");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__ramen`
Expected: FAIL — `RAMEN` is `Blank`, scores 40/1 (the ×2 assertion fails).

- [ ] **Step 3: Flip the const**

```rust
        // Ramen: ×2 Mult, loses ×0.01 per card discarded; floors at ×1.
        enhancement: MPip::LoseMultTimesPerDiscard(200, 1),
```

- [ ] **Step 4: Add the read arm** (above `_ => None`)

```rust
        MPip::LoseMultTimesPerDiscard(base, per) => {
            #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
            let raw = (base as f32 - per as f32 * counter.max(0) as f32) / 100.0;
            Some(ScoreOp::TimesMult(raw.max(1.0)))
        }
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test --features funky score__ramen`
Expected: PASS.

- [ ] **Step 6: Verify the guard**

Run: `cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Ramen (x2 mult, -x0.01 per card discarded)"
```

---

### Task 9: Wire Ice Cream (decaying chips) + trim KNOWN_UNWIRED

**Files:**
- Modify: `src/funky/decks/joker.rs` (`ICE_CREAM` const, `KNOWN_UNWIRED`)
- Modify: `src/funky/types/board.rs` (`counter_joker_op` arm)
- Test: `src/funky/types/board.rs`

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__ice_cream_loses_chips_per_hand_played() {
    // Ice Cream: +100 chips, −5 for each hand played; floors at 0.
    let mut board = board_playing("2S 5D 8C TS KH"); // High Card 40/1
    board.push_joker(card::ICE_CREAM);

    // No hands played yet -> +100 chips.
    assert_eq!(board.score(), Score::new(140, 1));

    // Two hands played -> +90 chips.
    let hand = bcards!("2S 5D 8C TS KH");
    board.on_hand_played(&hand);
    board.on_hand_played(&hand);
    assert_eq!(board.score(), Score::new(130, 1));

    // 20+ hands -> floored at +0 chips.
    for _ in 0..30 {
        board.on_hand_played(&hand);
    }
    assert_eq!(board.score(), Score::new(40, 1));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__ice_cream`
Expected: FAIL — `ICE_CREAM` is `Chips(100)` (unwired), scores 40/1.

- [ ] **Step 3: Flip the const**

Replace Ice Cream's `enhancement: MPip::Chips(100),` with:

```rust
        // Ice Cream: +100 chips, −5 per hand played; floors at 0.
        enhancement: MPip::LoseChipsPerHand(100, 5),
```

- [ ] **Step 4: Add the read arm** (above `_ => None`)

```rust
        MPip::LoseChipsPerHand(base, per) => {
            #[allow(clippy::cast_sign_loss)]
            let hands = counter.max(0) as usize;
            Some(ScoreOp::AddChips(base.saturating_sub(per * hands)))
        }
```

- [ ] **Step 5: Remove Ice Cream from KNOWN_UNWIRED**

In `src/funky/decks/joker.rs`, change `KNOWN_UNWIRED` from length 2 to 1, dropping the `card::ICE_CREAM` entry (and its comment). Result:

```rust
    const KNOWN_UNWIRED: [BuffoonCard; 1] = [
        // MultTimesOnEmptyJokerSlots(1): ×1 per empty joker slot needs a real
        // joker-slot *limit* on the board (Vec capacity is not the game's 5-slot
        // rule), so it waits on that Phase 3/8 state.
        card::JOKER_STENCIL,
    ];
```

- [ ] **Step 6: Run test + guard to verify they pass**

Run: `cargo test --features funky score__ice_cream && cargo test --features funky all_jokers__intended_hand_scorers_are_reachable`
Expected: PASS (Ice Cream is now wired and reachable; the guard would fail if it were still listed in `KNOWN_UNWIRED` — that is the self-cleaning check).

- [ ] **Step 7: Commit**

```bash
git add src/funky/decks/joker.rs src/funky/types/board.rs
git commit -m "Phase 3: wire Ice Cream (+100 chips, -5/hand); trim KNOWN_UNWIRED"
```

---

### Task 10: Full verification + EPIC status update

**Files:**
- Modify: `docs/EPIC-01a_Joker_Wiring_Backlog.md` (Status table + Phase 3 work items)

- [ ] **Step 1: Run the full gate**

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo fmt --all -- --check
cargo build --no-default-features
```
Expected: all green; test count up by the 6 new score tests + the store/skeleton tests + the Display test.

- [ ] **Step 2: Update the EPIC**

In `docs/EPIC-01a_Joker_Wiring_Backlog.md`:
- Status table row `3 — Per-run joker counters`: change `Planned` to `In progress — store + hand-played/discard events; Green Joker, Ramen, Ice Cream, Square Joker, Spare Trousers, Runner wired`.
- Work Items Phase 3: check `3a` (store landed) and add a sub-bullet listing the six wired jokers and the two events, and note the remaining counter jokers still await their events (round-end, shop, consumables, etc.).

- [ ] **Step 3: Commit**

```bash
git add docs/EPIC-01a_Joker_Wiring_Backlog.md
git commit -m "Phase 3 slice: update EPIC-01a status (store + 6 counter jokers)"
```

---

## Notes for the implementer

- **`TimesMult` rounding (Task 8):** resolved — `Score::multi_mult` computes `(mult as f32 * factor).ceil() as usize`. Ramen's test uses the exact ×1.00 floor boundary (100 cards discarded) to pin the −×0.01/card rate. Every other joker in this slice is additive integer chips/mult, unaffected by rounding.
- **Serialization:** the repo has no `serde_json` dev-dependency and no JSON round-trip test pattern, so there is no explicit serde test. `BuffoonBoard` already derives `Serialize`/`Deserialize`; adding a `Vec<i32>` field extends both derives automatically (guaranteed to compile). Task 1's clone/alignment test is the coverage for the new field.
- **Guard order matters:** each per-joker task re-runs `all_jokers__intended_hand_scorers_are_reachable`. If it fails with the joker in the "silent-zero" list, the read arm or the const flip is missing; if it fails with the joker in "now wired — remove from KNOWN_UNWIRED", that only applies to Ice Cream (Task 9).
- **`counter_joker_op` arm placement:** always add new arms *above* the `_ => None` line.
- **`board_playing` / `bcards!` / `Draws` / `Deck`** are already imported in the board test module; `card::` and `MPip` are in scope there via `use super::*` and the existing `use crate::funky::types::mpip::MPip;`.
