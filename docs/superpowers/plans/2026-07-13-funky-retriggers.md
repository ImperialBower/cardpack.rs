# Funky Per-Card Retriggers (EPIC-01a Phase 4a) Implementation Plan

> ⚠️ **SUPERSEDED — do not execute this plan.** The feature it plans is already
> shipped: a parallel implementation from `origin/funky` (`26db1db` + `c5b7b0e`)
> won at merge `6d3ac11`, wiring all four jokers (Hack, Sock and Buskin, Hanging
> Chad, Mime) with a different `MPip` vocabulary —
> `RetriggerPlayedRanks(n, ranks)` / `RetriggerPlayedFaces(n)` /
> `RetriggerFirstPlayed(n)` instead of the variants named below — and
> `played_retriggers` / `held_retriggers` as the count helpers. Only the
> stacking test (`score__stacked_retriggers_are_additive`) survived from this
> plan's implementation. See the matching superseded note in
> [the design spec](../specs/2026-07-13-funky-retriggers-design.md).

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make a played or held card's scoring contribution apply `1 + n` times, where retrigger jokers grant `n`, and wire four jokers (Hack, Sock and Buskin, Hanging Chad, Mime) end-to-end.

**Architecture:** A per-card retrigger-count pre-pass in the two existing card folds (`fold_played_cards`, `fold_held_cards`). Each fold wraps its per-card body in a `0..=extra` loop; `extra` is summed across the board's jokers. Retrigger jokers carry data-only `MPip` variants read by the count helpers — they never produce a `ScoreOp`. Score is byte-identical when no retrigger jokers are present (count is 0 → one pass).

**Tech Stack:** Rust 2024, `--features funky` (std-only), `rstest` not required here, seeded `StdRng` for the fresh-Lucky-roll-per-trigger path.

**Spec:** [`docs/superpowers/specs/2026-07-13-funky-retriggers-design.md`](../specs/2026-07-13-funky-retriggers-design.md)

## Global Constraints

- **Feature gate:** all code is under `#[cfg(feature = "funky")]`; funky is std-only. Never import funky types into `basic`/no_std modules.
- **Git commits:** this repo's owner runs every state-changing git command. When a step says "commit", surface the exact command for the owner to run — do not run it yourself.
- **Gold Standard (EPIC-00f):** every new joker gets a test asserting its exact Balatro value, and that test MUST be observed failing before its scoring arm lands.
- **Exhaustive matches:** `MPip`'s `Display` (`src/funky/types/mpip.rs`) and the test oracle `scores_hand` (`src/funky/decks/joker.rs`) are exhaustive `match`es with no `_` arm — every new variant MUST be added to both or the crate/tests won't compile.
- **Compatibility invariant:** `BuffoonBoard::score()` for a board with no retrigger jokers must stay byte-identical. The existing `score__*` suite is the regression guard; keep it green at every step.
- **Verification (run after each task):**
  ```bash
  cargo test --features funky
  cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
  cargo build --no-default-features
  cargo fmt --all -- --check
  ```

## Retrigger semantics (reference for all tasks)

Chip values (Balatro, confirmed in-repo): `2→2, 5→5, 7→7, 8→8, T/J/Q/K→10`. A plain card's retrigger re-applies its `get_chips()` (no mult). Held Steel (`MPip::STEEL = MultTimes1Dot(15)`) applies `TimesMult(1.5)`; `multi_mult` uses `.ceil()`.

Retrigger counts **stack additively** across all jokers and copies. Rules:

| Joker | Variant | Extra triggers granted |
|---|---|---|
| Hack | `RetriggerScoredRanks(['2','3','4','5'])` | +1 to each played card whose rank ∈ set |
| Sock and Buskin | `RetriggerScoredFaces` | +1 to each played face (K/Q/J) |
| Hanging Chad | `RetriggerFirstScored(2)` | +2 to the first played card (index 0) only |
| Mime | `RetriggerCardsInHand(1)` *(already exists)* | +1 to each held card |

## File Structure

- `src/funky/types/mpip.rs` — add 3 new variants (`RetriggerScoredRanks`, `RetriggerScoredFaces`, `RetriggerFirstScored`) + their `Display` arms. `RetriggerCardsInHand` already exists.
- `src/funky/types/board.rs` — `played_retrigger_count`, `held_retrigger_count`; the `0..=extra` loops in `fold_played_cards` and `fold_held_cards`; all `score__*` retrigger tests + the retrigger guard.
- `src/funky/decks/joker.rs` — wire `HACK`; new `SOCK_AND_BUSKIN` + `HANGING_CHAD` consts; pile arrays (`COMMON_JOKERS`, `UNCOMMON_JOKERS`) + `_SIZE` consts + `ALL_JOKERS`; add new variants to the `scores_hand` oracle's `false` arm.
- `docs/EPIC-01a_Joker_Wiring_Backlog.md` — flip the Phase 4 status row.

---

### Task 1: Played-retrigger mechanism + Hack

**Files:**
- Modify: `src/funky/types/mpip.rs` (add `RetriggerScoredRanks` + Display)
- Modify: `src/funky/types/board.rs` (`played_retrigger_count`, loop in `fold_played_cards`)
- Modify: `src/funky/decks/joker.rs` (`HACK` enhancement; `scores_hand` arm)
- Test: `src/funky/types/board.rs` (`score__hack_retriggers_low_cards`)

**Interfaces:**
- Produces: `BuffoonBoard::played_retrigger_count(&self, card: &BuffoonCard, index: usize) -> usize` (Tasks 2 & 3 extend its match).
- Produces: `MPip::RetriggerScoredRanks([char; 4])`.

- [ ] **Step 1: Write the failing test** — add to the `#[cfg(test)] mod tests` in `src/funky/types/board.rs`, next to `score__scholar_*`:

```rust
#[test]
fn score__hack_retriggers_low_cards() {
    // Hack retriggers each played card ranked 2/3/4/5. Only the 2S qualifies
    // here, so its 2 chips are added one extra time; mult unchanged.
    let mut board = board_playing("2S 8D TC JS KH"); // High Card, one low card
    let base = board.score();

    board.jokers.push(card::HACK);
    let scored = board.score();

    assert_eq!(scored.chips, base.chips + 2, "one extra trigger of the 2");
    assert_eq!(scored.mult, base.mult, "retrigger adds no mult here");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__hack_retriggers_low_cards`
Expected: FAIL — `scored.chips == base.chips` (Hack is `MPip::Blank`, no retrigger yet).

- [ ] **Step 3: Add the `MPip` variant + Display** in `src/funky/types/mpip.rs`. Add near the other `Retrigger*` variants (after `RetriggerCardsInHand(usize)`):

```rust
    /// Retrigger each played card whose rank index is in the set (Hack: 2/3/4/5).
    RetriggerScoredRanks([char; 4]),
```

and in the `Display` impl, next to the other `Retrigger*` arms:

```rust
            Self::RetriggerScoredRanks(ranks) => {
                write!(f, "RetriggerScoredRanks({ranks:?})")
            }
```

- [ ] **Step 4: Add the count helper + loop** in `src/funky/types/board.rs`. Add the helper as a private method on `impl BuffoonBoard` near the folds:

```rust
    /// Extra times each played card scores, summed across every joker on the
    /// board (retriggers stack). `0` when no retrigger joker is present, so the
    /// played fold runs each card exactly once and `score()` is unchanged.
    fn played_retrigger_count(&self, card: &BuffoonCard, index: usize) -> usize {
        let _ = index; // used by Task 3 (Hanging Chad)
        self.jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::RetriggerScoredRanks(ranks) if ranks.contains(&card.rank.index) => 1,
                _ => 0,
            })
            .sum()
    }
```

Then wrap the per-card body of `fold_played_cards` in a `0..=extra` loop. Replace the `for card in &self.played { ... }` body so it reads:

```rust
        for (index, card) in self.played.iter().enumerate() {
            let extra = self.played_retrigger_count(card, index);
            for _ in 0..=extra {
                // Built-in chips/mult first, then the special (probabilistic /
                // custom) effect resolves in card order. A retrigger re-runs
                // both, so a Lucky card rolls fresh each pass.
                score = Self::builtin_played_op(card).apply(score);

                let special = match card.enhancement {
                    MPip::Lucky(mult_odds, _) if mult_odds > 0 => {
                        rng.as_deref_mut().map_or(ScoreOp::Nothing, |rng| {
                            if rng.random_range(0..mult_odds) == 0 {
                                ScoreOp::AddMult(LUCKY_MULT)
                            } else {
                                ScoreOp::Nothing
                            }
                        })
                    }
                    MPip::Custom(id) => self.custom_op(*card, id, registry),
                    _ => ScoreOp::Nothing,
                };
                score = special.apply(score);
            }
        }
```

(The `rng`/`registry`/`score` bindings are the existing ones from `fold_played_cards`; only the loop nesting and `enumerate()` are new.)

- [ ] **Step 5: Wire Hack + classify the variant.** In `src/funky/decks/joker.rs`, change the `HACK` const's `enhancement: MPip::Blank` to:

```rust
        enhancement: MPip::RetriggerScoredRanks(['2', '3', '4', '5']),
```

and add the new variant to the `false` (non-scoring) arm of `scores_hand`, next to `MPip::RetriggerCardsInHand(_)`:

```rust
            | MPip::RetriggerScoredRanks(_)
```

- [ ] **Step 6: Run the test to verify it passes**

Run: `cargo test --features funky score__hack_retriggers_low_cards`
Expected: PASS.

- [ ] **Step 7: Run the full suite + lints** (compatibility invariant)

Run: `cargo test --features funky && cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: all green — the `score__*` regression suite confirms retrigger-free boards are unchanged.

- [ ] **Step 8: Commit** (surface for the owner to run)

```bash
git add src/funky/types/mpip.rs src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 4a: played-card retrigger mechanism; wire Hack (2/3/4/5)"
```

---

### Task 2: Sock and Buskin (retrigger played faces)

**Files:**
- Modify: `src/funky/types/mpip.rs` (`RetriggerScoredFaces` + Display)
- Modify: `src/funky/types/board.rs` (extend `played_retrigger_count`)
- Modify: `src/funky/decks/joker.rs` (new `SOCK_AND_BUSKIN` const; `UNCOMMON_JOKERS`+size; `ALL_JOKERS`; `scores_hand`)
- Test: `src/funky/types/board.rs` (`score__sock_and_buskin_retriggers_faces`)

**Interfaces:**
- Consumes: `played_retrigger_count` (Task 1).
- Produces: `MPip::RetriggerScoredFaces`; `card::SOCK_AND_BUSKIN`.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__sock_and_buskin_retriggers_faces() {
    // Retrigger each played face card (K/Q/J). Only KH qualifies -> +10 chips.
    let mut board = board_playing("2S 8D 5C 7H KH"); // High Card, one face
    let base = board.score();

    board.jokers.push(card::SOCK_AND_BUSKIN);
    let scored = board.score();

    assert_eq!(scored.chips, base.chips + 10, "one extra trigger of the King");
    assert_eq!(scored.mult, base.mult);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features funky score__sock_and_buskin_retriggers_faces`
Expected: FAIL to **compile** — `card::SOCK_AND_BUSKIN` does not exist yet. That is the expected red state.

- [ ] **Step 3: Add the variant + Display** in `src/funky/types/mpip.rs`:

```rust
    /// Retrigger each played face card (K/Q/J) — Sock and Buskin.
    RetriggerScoredFaces,
```

Display arm:

```rust
            Self::RetriggerScoredFaces => write!(f, "RetriggerScoredFaces"),
```

- [ ] **Step 4: Extend the count helper.** In `played_retrigger_count` (board.rs), add a match arm before `_ => 0`:

```rust
                MPip::RetriggerScoredFaces if matches!(card.rank.index, 'K' | 'Q' | 'J') => 1,
```

- [ ] **Step 5: Create the const.** In `src/funky/decks/joker.rs`, add next to the other Uncommon joker consts (mirror the `HACK` shape):

```rust
    pub const SOCK_AND_BUSKIN: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 1091,
            pip_type: PipType::Joker,
            index: '🎭',
            symbol: '🎭',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::RetriggerScoredFaces,
        resell_value: 3,
        debuffed: false,
    };
```

- [ ] **Step 6: Add it to the piles + registry + oracle.**
  - In `UNCOMMON_JOKERS`, add `card::SOCK_AND_BUSKIN,` and bump `UNCOMMON_JOKERS_SIZE` from `12` to `13`.
  - In `ALL_JOKERS`, add `card::SOCK_AND_BUSKIN,` and change its array length `[BuffoonCard; 105]` to `[BuffoonCard; 106]`.
  - In `scores_hand`, add to the `false` arm: `| MPip::RetriggerScoredFaces`.

- [ ] **Step 7: Run the test + pile/weight guards to verify pass**

Run: `cargo test --features funky score__sock_and_buskin_retriggers_faces all_jokers`
Expected: PASS — including `all_jokers__is_superset_of_every_pile` and `all_jokers__weights_are_unique`. If weight `1091` collides, pick another unused value near it and re-run.

- [ ] **Step 8: Full suite + lints**

Run: `cargo test --features funky && cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: green.

- [ ] **Step 9: Commit**

```bash
git add src/funky/types/mpip.rs src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 4a: wire Sock and Buskin (retrigger played faces)"
```

---

### Task 3: Hanging Chad (retrigger first played card) + stacking

**Files:**
- Modify: `src/funky/types/mpip.rs` (`RetriggerFirstScored` + Display)
- Modify: `src/funky/types/board.rs` (extend `played_retrigger_count`)
- Modify: `src/funky/decks/joker.rs` (new `HANGING_CHAD` const; `COMMON_JOKERS`+size; `ALL_JOKERS`; `scores_hand`)
- Test: `src/funky/types/board.rs` (`score__hanging_chad_retriggers_first_card_thrice`, `score__stacked_retriggers_are_additive`)

**Interfaces:**
- Consumes: `played_retrigger_count` (uses the `index` param, live from Task 1).
- Produces: `MPip::RetriggerFirstScored(usize)`; `card::HANGING_CHAD`.

- [ ] **Step 1: Write the failing tests**

```rust
#[test]
fn score__hanging_chad_retriggers_first_card_thrice() {
    // Retrigger the FIRST played card 2 additional times (3 total). First card
    // is 2S (2 chips) -> +4 chips; a later card is untouched.
    let mut board = board_playing("2S 8D 5C 7H KH");
    let base = board.score();

    board.jokers.push(card::HANGING_CHAD);
    let scored = board.score();

    assert_eq!(scored.chips, base.chips + 4, "2 extra triggers of the first card (2)");
    assert_eq!(scored.mult, base.mult);
}

#[test]
fn score__stacked_retriggers_are_additive() {
    // Hack (+1 to the 2) and Hanging Chad (+2 to the first card, also the 2)
    // stack: the 2 scores 1 + 1 + 2 = 4 times -> 3 extra -> +6 chips.
    let mut board = board_playing("2S 8D TC JS KH"); // only the first card is low
    let base = board.score();

    board.jokers.push(card::HACK);
    board.jokers.push(card::HANGING_CHAD);
    let scored = board.score();

    assert_eq!(scored.chips, base.chips + 6, "3 extra triggers of the 2");
    assert_eq!(scored.mult, base.mult);
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test --features funky retriggers_are_additive hanging_chad`
Expected: FAIL to compile — `card::HANGING_CHAD` undefined.

- [ ] **Step 3: Add the variant + Display** in `src/funky/types/mpip.rs`:

```rust
    /// Retrigger the first played card `n` additional times — Hanging Chad.
    RetriggerFirstScored(usize),
```

Display arm:

```rust
            Self::RetriggerFirstScored(n) => write!(f, "RetriggerFirstScored({n})"),
```

- [ ] **Step 4: Extend the count helper.** In `played_retrigger_count`, add before `_ => 0` and remove the `let _ = index;` line (it is now used):

```rust
                MPip::RetriggerFirstScored(n) if index == 0 => n,
```

- [ ] **Step 5: Create the const.** In `src/funky/decks/joker.rs`, add next to the Common joker consts:

```rust
    pub const HANGING_CHAD: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 1151,
            pip_type: PipType::Joker,
            index: '🗳',
            symbol: '🗳',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::RetriggerFirstScored(2),
        resell_value: 2,
        debuffed: false,
    };
```

- [ ] **Step 6: Piles + registry + oracle.**
  - In `COMMON_JOKERS`, add `card::HANGING_CHAD,` and bump `COMMON_JOKERS_SIZE` from `22` to `23`.
  - In `ALL_JOKERS`, add `card::HANGING_CHAD,` and change `[BuffoonCard; 106]` to `[BuffoonCard; 107]`.
  - In `scores_hand`, add to the `false` arm: `| MPip::RetriggerFirstScored(_)`.

- [ ] **Step 7: Run the tests + guards**

Run: `cargo test --features funky hanging_chad retriggers_are_additive all_jokers`
Expected: PASS (weight `1151` unique; if not, pick another and re-run).

- [ ] **Step 8: Full suite + lints**

Run: `cargo test --features funky && cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: green.

- [ ] **Step 9: Commit**

```bash
git add src/funky/types/mpip.rs src/funky/types/board.rs src/funky/decks/joker.rs
git commit -m "Phase 4a: wire Hanging Chad (retrigger first played card); stacking test"
```

---

### Task 4: Mime (retrigger held-card abilities)

**Files:**
- Modify: `src/funky/types/board.rs` (`held_retrigger_count`; loop in `fold_held_cards`)
- Test: `src/funky/types/board.rs` (`score__mime_retriggers_held_steel`)

**Note:** `MIME` already carries `MPip::RetriggerCardsInHand(1)` and that variant is already in the `scores_hand` `false` arm — no joker.rs data change is needed, only the scoring path.

**Interfaces:**
- Produces: `BuffoonBoard::held_retrigger_count(&self, card: &BuffoonCard, index: usize) -> usize`.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn score__mime_retriggers_held_steel() {
    // Mime retriggers held-card abilities: a held Steel card applies its x1.5
    // mult twice. Build a held Steel King (enhancement is a public field).
    let mut board = board_playing("2S 8D TC JS KH");
    let mut steel = *bcards!("KH").iter().next().unwrap();
    steel.enhancement = MPip::STEEL; // MultTimes1Dot(15) = x1.5
    board.in_hand = BuffoonPile::from(vec![steel]);

    let without = board.score(); // Steel applied once
    board.jokers.push(card::MIME);
    let with = board.score(); // Steel applied twice

    assert_eq!(with.chips, without.chips, "retrigger of a held card adds no chips");
    assert_eq!(
        with.mult,
        (f64::from(u32::try_from(without.mult).unwrap()) * 1.5).ceil() as usize,
        "one extra x1.5 application"
    );
    assert!(with.mult > without.mult, "Mime must increase mult");
}
```

- [ ] **Step 2: Run to verify it fails**

Run: `cargo test --features funky score__mime_retriggers_held_steel`
Expected: FAIL — `with.mult == without.mult` (held retriggers not applied yet).

- [ ] **Step 3: Add the held count helper + loop.** In `src/funky/types/board.rs`, add:

```rust
    /// Extra times each held card's ability fires, summed across every joker.
    /// `0` unless a held-retrigger joker (Mime) is present.
    fn held_retrigger_count(&self, _card: &BuffoonCard, _index: usize) -> usize {
        self.jokers
            .iter()
            .map(|joker| match joker.enhancement {
                MPip::RetriggerCardsInHand(n) => n,
                _ => 0,
            })
            .sum()
    }
```

Then wrap the per-card body of `fold_held_cards` in a `0..=extra` loop:

```rust
        for (index, card) in self.in_hand.iter().enumerate() {
            let extra = self.held_retrigger_count(card, index);
            let op = match card.enhancement {
                MPip::Custom(id) => self.custom_op(*card, id, registry),
                _ => Self::builtin_held_op(card),
            };
            for _ in 0..=extra {
                score = op.apply(score);
            }
        }
```

(`score`/`registry` are the existing `fold_held_cards` bindings; only the loop, `enumerate()`, and the count call are new. `op` is `Copy`-cheap — `ScoreOp` — so recomputing it once and applying in the loop is correct.)

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --features funky score__mime_retriggers_held_steel`
Expected: PASS.

- [ ] **Step 5: Full suite + lints**

Run: `cargo test --features funky && cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: green (held folds with no Mime are unchanged → `score__*` still pass).

- [ ] **Step 6: Commit**

```bash
git add src/funky/types/board.rs
git commit -m "Phase 4a: wire Mime (retrigger held-card abilities)"
```

---

### Task 5: Retrigger silent-zero guard + EPIC status

**Files:**
- Test: `src/funky/types/board.rs` (`retrigger_jokers_actually_retrigger`)
- Modify: `docs/EPIC-01a_Joker_Wiring_Backlog.md` (status row)

**Rationale:** the `scores_hand` oracle classifies retrigger variants as non-scoring, so the existing `all_jokers__intended_hand_scorers_are_reachable` guard does not cover them. This adds the parallel guard for the retrigger class — the protection that caught Banner/Mystic, applied to retriggers.

**Interfaces:**
- Consumes: all four wired jokers + `played_retrigger_count`/`held_retrigger_count`.

- [ ] **Step 1: Write the guard test**

```rust
#[test]
fn retrigger_jokers_actually_retrigger() {
    // Each retrigger joker must strictly increase the score of a board that has
    // a matching card. A silently-inert retrigger joker (the Banner/Mystic
    // failure mode) fails here.

    // Hack over a played 2.
    let mut b = board_playing("2S 8D TC JS KH");
    let base = b.score();
    b.jokers.push(card::HACK);
    assert!(b.score().chips > base.chips, "Hack must retrigger the 2");

    // Sock and Buskin over a played King.
    let mut b = board_playing("2S 8D 5C 7H KH");
    let base = b.score();
    b.jokers.push(card::SOCK_AND_BUSKIN);
    assert!(b.score().chips > base.chips, "Sock and Buskin must retrigger the King");

    // Hanging Chad over the first played card.
    let mut b = board_playing("2S 8D 5C 7H KH");
    let base = b.score();
    b.jokers.push(card::HANGING_CHAD);
    assert!(b.score().chips > base.chips, "Hanging Chad must retrigger the first card");

    // Mime over a held Steel card.
    let mut b = board_playing("2S 8D TC JS KH");
    let mut steel = *bcards!("KH").iter().next().unwrap();
    steel.enhancement = MPip::STEEL;
    b.in_hand = BuffoonPile::from(vec![steel]);
    let base = b.score();
    b.jokers.push(card::MIME);
    assert!(b.score().mult > base.mult, "Mime must retrigger the held Steel");
}
```

- [ ] **Step 2: Run to verify it passes** (all four are wired by now)

Run: `cargo test --features funky retrigger_jokers_actually_retrigger`
Expected: PASS. (Sanity: temporarily revert one joker's enhancement to `MPip::Blank` and confirm this test fails, then restore — proves the guard bites.)

- [ ] **Step 3: Update the EPIC status.** In `docs/EPIC-01a_Joker_Wiring_Backlog.md`, change the Phase 4 (Retriggers) status-table row from `Planned` to a done note, and update the `- [ ] 4a.` work item to `- [x] 4a.` with the wired jokers (Hack, Sock and Buskin, Hanging Chad, Mime) and a note that Dusk/Seltzer remain (need final-round / counter state).

- [ ] **Step 4: Full suite + lints + no_std + fmt**

Run:
```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features
cargo fmt --all -- --check
```
Expected: all green.

- [ ] **Step 5: Commit**

```bash
git add src/funky/types/board.rs docs/EPIC-01a_Joker_Wiring_Backlog.md
git commit -m "Phase 4a: retrigger silent-zero guard; EPIC-01a status update"
```

---

## Self-Review

**Spec coverage:**
- Retrigger unit (full op + fresh Lucky roll) → Task 1 Step 4 loop re-runs `builtin_played_op` + the Lucky/Custom `special` each pass. ✓
- Played retriggers Hack/Sock/Chad → Tasks 1–3. ✓
- Held retrigger Mime → Task 4 (reuses existing `RetriggerCardsInHand`, a spec refinement — the spec's invented `RetriggerHeld` is dropped in favour of the variant Mime already carries). ✓
- New `MPip` variants + Display → Tasks 1–3 Step 3. ✓
- `scores_hand` classification (exhaustive) → Tasks 1/2/3 Step 5/6. ✓
- New consts, pile + `ALL_JOKERS` 105→107 + `_SIZE` bumps → Tasks 2 & 3. ✓
- Silent-zero parallel guard → Task 5. ✓
- Byte-identical `score()` for retrigger-free boards → count helpers return 0 → single pass; regression suite run every task. ✓
- Exact-value tests failing first → each Task's Steps 1–2. ✓

**Placeholder scan:** no TBD/TODO; all code shown; expected outputs given. ✓

**Type consistency:** `played_retrigger_count(&self, &BuffoonCard, usize) -> usize` and `held_retrigger_count(&self, &BuffoonCard, usize) -> usize` used consistently; `RetriggerScoredRanks([char; 4])`, `RetriggerScoredFaces`, `RetriggerFirstScored(usize)`, `RetriggerCardsInHand(usize)` consistent across mpip/board/joker; `MPip::STEEL`, `BuffoonPile::from(Vec<_>)`, `card::*` names match the codebase. ✓

**Deviation from spec (intentional):** Mime uses the pre-existing `RetriggerCardsInHand(1)` rather than a new `RetriggerHeld` variant — fewer variants, and Mime's const already carries it, so Task 4 is scoring-path-only.
