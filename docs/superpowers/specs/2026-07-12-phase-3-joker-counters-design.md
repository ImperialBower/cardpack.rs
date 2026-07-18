# Phase 3 (vertical slice): per-run joker counters

> Implements the first slice of **EPIC-01a Phase 3**. Adds the per-joker counter
> store, the two in-round growth events (hand played, discard), and the six
> jokers those events unblock. Shop/round-boundary events and the remaining
> counter jokers are explicitly out of scope (later slices).

**Date:** 2026-07-12 · **Branch:** `funky` · **Follows:** EPIC-01a Phase 0 (guard
+ data fixes), Phase 1a/1b (money + Bull), Phase 2a/2b (Banner + Mystic).

---

## Context

The funky scoring engine folds cards/jokers into a running `Score` across four
phases. Joker contributions resolve in `fold_jokers` (`board.rs`) to a `ScoreOp`
per joker, applied read-only — `score()` is `&self`.

A family of jokers "gains X, currently Y": their contribution is a mutable
per-run accumulator that grows on some event and is read at scoring time. The
read side is trivial; the work is the **write** side — the events and where they
fire — plus a place to keep the state.

Today nothing mutates `board.jokers` mid-round (no shop yet), and there are no
lifecycle hooks. This slice introduces both, minimally.

## Goals

- A per-joker counter store on `BuffoonBoard` that serializes and survives clone.
- Two growth events (`on_hand_played`, `on_discard`) that tick each joker's
  accumulator by its own rule.
- Six jokers wired to score their **exact Balatro value** from that accumulator,
  each with a test that fails before its arm lands.
- Keep `score()` a pure `&self` read; keep the 0b reachability guard green.

## Scope

**In:** the counter store; `on_hand_played`/`on_discard`; the read seam; the six
jokers below; their tests; probe-battery + `KNOWN_UNWIRED` updates.

**Out (later slices):** round-boundary events (Popcorn), shop events (Red Card,
Flash Card), consumable events (Constellation, Fortune Teller), card-add
(Hologram), lucky-trigger (Lucky Cat), destroy (Canio), cumulative-discard
(Yorick), and the enhancement-stripping mutation (Vampire). No shop, no round
loop, no consumable inventory.

## Jokers in this slice

| Joker (Nr) | Balatro effect | New `MPip` variant | Growth | Read |
|---|---|---|---|---|
| Green Joker (58) | +1 Mult per hand played, −1 Mult per discard | `GainMultPerHandLessDiscard(1)` | +1 on hand played; −1 on discard (per discard *action*) | `mult += rate · max(0, counter)` |
| Ramen (100) | ×2 Mult, −×0.01 per card discarded | `LoseMultTimesPerDiscard(200, 1)` | += `discarded.len()` on discard (per *card*) | `×mult = max(1.0, (200 − 1·counter) / 100)` |
| Ice Cream (—) | +100 Chips, −5 per hand played | `LoseChipsPerHand(100, 5)` | +1 on hand played | `chips += max(0, 100 − 5·counter)` |
| Square Joker (65) | +4 Chips if played hand has exactly 4 cards | `GainChipsPerCardCountHand(4, 4)` | +1 when `played.len() == 4` | `chips += rate · counter` |
| Spare Trousers (98) | +2 Mult if played hand contains Two Pair | `GainMultPerTwoPairHand(2)` | +1 when `played.has_2pair()` | `mult += rate · counter` |
| Runner (49) | +15 Chips if played hand contains a Straight | `GainChipsPerStraightHand(15)` | +1 when `played.has_straight()` | `chips += rate · counter` |

`GainChipsPerCardCountHand(rate, n)`: `n` is the required card count (4);
`rate` the chips gained (4). `LoseMultTimesPerDiscard(base, per)` /
`LoseChipsPerHand(base, per)` carry the starting value and the per-event decay.
Each joker is fully described by its **(growth, read)** pair keyed on the same
variant — no behaviour is spread across unrelated files.

## Design

### Storage — `board.joker_state: Vec<i32>`

A parallel `Vec<i32>`, index-aligned with `board.jokers`: `joker_state[i]` is the
accumulator for `jokers[i]`. Default empty; `i32` because Green Joker's net
(hands − discards) can go negative before the read floors it at 0.

**Alignment invariant:** `joker_state.len() == jokers.len()`, and index `i`
belongs to `jokers[i]`. Enforced by routing every joker mutation through helpers
that touch both vectors together:

- `push_joker(&mut self, joker: BuffoonCard)` — pushes to `jokers` and a fresh `0`
  to `joker_state`.
- `remove_joker(&mut self, i: usize) -> BuffoonCard` — removes index `i` from both.

**push_joker/remove_joker are the sanctioned path.** To stay robust to the
existing "set `board.jokers` directly" test style (and the probe battery), the
store treats a missing entry as `0`:

- **Write (event, `&mut self`):** `on_hand_played`/`on_discard` first call
  `ensure_state_len()`, which pads `joker_state` with zeros up to `jokers.len()`,
  then apply deltas by index.
- **Read (scoring, `&self`):** `fold_jokers` reads
  `self.joker_state.get(i).copied().unwrap_or(0)` — no mutation, so `score()`
  stays `&self`. A board that never fired an event reads every counter as 0.

This keeps the invariant cheap and never requires a mutable score path.

### Events — growth hooks

```rust
// board.rs, &mut self — called by the caller when the player acts.
pub fn on_hand_played(&mut self, played: &BuffoonPile);
pub fn on_discard(&mut self, discarded: &BuffoonPile);
```

Each iterates jokers with their index and applies a per-joker delta:

```rust
fn growth_delta(enhancement: MPip, event: GrowthEvent<'_>) -> i32
```

where `GrowthEvent` is a small internal enum `{ HandPlayed(&BuffoonPile),
Discard(&BuffoonPile) }`. `growth_delta` matches the enhancement + event to an
`i32` (0 for non-counter jokers). The hook adds it to `joker_state[i]`. This is
the write-side mirror of the read-side match — both switch on the same variant.

Growth per variant:

- `GainMultPerHandLessDiscard(_)` → `HandPlayed` `+1`, `Discard` `−1`.
- `LoseMultTimesPerDiscard(_, _)` → `Discard` `+discarded.len()`.
- `LoseChipsPerHand(_, _)` → `HandPlayed` `+1`.
- `GainChipsPerCardCountHand(_, n)` → `HandPlayed` `+1` iff `played.len() == n`.
- `GainMultPerTwoPairHand(_)` → `HandPlayed` `+1` iff `played.has_2pair()`.
- `GainChipsPerStraightHand(_)` → `HandPlayed` `+1` iff `played.has_straight()`.

### Read — `counter_joker_op`

`fold_jokers` gains `.enumerate()`. Before falling through to `builtin_joker_op`,
it matches the counter variants and routes to a new seam that receives the
accumulator:

```rust
fn counter_joker_op(&self, joker: &BuffoonCard, counter: i32) -> Option<ScoreOp>
```

Returns `Some(op)` for a counter variant, `None` otherwise (so the existing
`builtin_joker_op` / `joker_x_mult` path is untouched for every other joker).
Reads per the table above. Sub-integer ×mult (Ramen) is computed as `f32` in the
same style as `MultTimes1Dot` (there tenths; here hundredths), floored at `1.0`.
`ScoreOp::TimesMult(f32)` already exists for the multiplicative case;
additive reads use `AddChips` / `AddMult` with `max(0, …)` flooring.

`score()` stays `&self`: the counter is read, never written, during scoring.

### Data flow

```
player plays hand ─► board.on_hand_played(&played) ─► joker_state[i] += growth_delta(…)
player discards   ─► board.on_discard(&discarded)  ─► joker_state[i] += growth_delta(…)
score()           ─► fold_jokers: for (i, joker)    ─► counter_joker_op(joker, joker_state.get(i).unwrap_or(0))
                                    └─ else ─────────► builtin_joker_op(joker)
```

## Reachability guard interaction (0b)

The six variants are classified `scores_hand = true`. With counter `0` they add
nothing, so on a fresh probe board they would look unreachable and trip the
guard. **Resolution:** the probe battery calls `on_hand_played` (and, on one
board, `on_discard`) a few times before scoring, so every counter joker holds a
nonzero accumulator and is genuinely reachable. `KNOWN_UNWIRED` drops Ice Cream
(and this slice adds no new entries); Joker Stencil remains (needs a slot limit).

## Testing

- Per joker, TDD (each fails before its arm exists):
  - Green Joker: 3× `on_hand_played`, 1× `on_discard` → `mult += 2`.
  - Ramen: discard 3 cards → `×mult = (200 − 3)/100 = ×1.97`; floor check with a
    large discard.
  - Ice Cream: 2 hands → `chips += 90`; 20+ hands → floored at 0.
  - Square Joker: two exactly-4-card hands → `chips += 8`; a 5-card hand adds 0.
  - Spare Trousers: two two-pair hands → `mult += 4`.
  - Runner: two straight hands → `chips += 30`.
- Store invariant: `push_joker`/`remove_joker` keep `joker_state` aligned; a
  removed joker's counter goes with it and the survivors keep theirs.
- Serialization round-trip of a board with non-zero counters.
- The 0b guard stays green (probe battery drives the events).

## Key files

| File | Change |
|---|---|
| `src/funky/types/board.rs` | `joker_state` field; `push_joker`/`remove_joker`/`ensure_state_len`; `on_hand_played`/`on_discard`; `growth_delta`; `counter_joker_op`; `fold_jokers` enumerate; tests |
| `src/funky/types/mpip.rs` | six new `MPip` variants + `Display` arms |
| `src/funky/decks/joker.rs` | flip the six jokers' `enhancement`; `scores_hand` classification; probe battery drives events; `KNOWN_UNWIRED` 2→1 |

## Reuse (do NOT recreate)

- The `ScoreOp` currency + `apply` — counter reads return existing `AddChips` /
  `AddMult` / `TimesMult`, no new op kind.
- Hand predicates `has_2pair` / `has_straight` / `len` (`buffoon_pile.rs`).
- The fixed-point ×mult convention of `MultTimes1Dot`.
- `ALL_JOKERS` registry + the 0b guard from Phase 0.

## Compatibility

New state defaults to empty/zero and is inert until an event fires, so `score()`
for any board that never calls the hooks is byte-identical to today. Adds board
state + `MPip` variants + two methods. Breaks nothing; the `Serialize` derive
extends cleanly with the new `Vec` field.

## Out of scope / follow-ups

- Round-boundary event (`on_round_end`) → Popcorn.
- Shop events (reroll, pack skip) → Flash Card, Red Card.
- Consumable events (planet, tarot used) → Constellation, Fortune Teller.
- Card-added → Hologram; lucky-trigger → Lucky Cat; destroy → Canio; cumulative
  discard → Yorick; enhancement strip → Vampire.
- Wiring `push_joker`/`remove_joker` into a real shop/round loop.
