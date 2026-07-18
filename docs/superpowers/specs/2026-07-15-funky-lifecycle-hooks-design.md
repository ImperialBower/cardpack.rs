# EPIC-01a Phase 1c — Round Lifecycle Hooks & `+$` Payouts (Design)

**Date:** 2026-07-15 · **Branch:** `funky` · **EPIC:** [EPIC-01a Phase 1](../../EPIC-01a_Joker_Wiring_Backlog.md) (Economy / money, keystone)

## Summary

Add the round-end lifecycle hook to `BuffoonBoard` and extend the existing
event seam so jokers can move **money** and **destroy themselves**. Wire six
deterministic money/round-end jokers end-to-end, and close the 0c-mirror debt
by rolling the probabilistic joker destructions (Gros Michel, Cavendish)
through the seeded-RNG path:

| Joker | # | Fires on | Exact effect |
|---|---|---|---|
| **Golden Joker** | — | round end | +$4 |
| **Delayed Gratification** | 35 | round end | $2 × remaining discards, only if none used this round |
| **Cloud 9** | — | round end | $1 per **9** in the full deck |
| **To the Moon** | 84 | round end | $1 extra interest per $5 held, capped at $5 |
| **Faceless Joker** | 57 | discard | +$5 when ≥3 face cards discarded at once |
| **Egg** | — | round end | its own `resell_value` +$3 |
| **Gros Michel** | — | round end (RNG) | 1-in-6 chance destroyed |
| **Cavendish** | 33 | round end (RNG) | 1-in-1000 chance destroyed *(new compound variant)* |
| **Ice Cream** | — | hand played | destroyed when its chips decay to 0 |

(“—” = wiki number not recorded in-repo; verify against the wiki when wiring
rather than trusting a guess.)

Hooks must be **inert for boards without these jokers**: `on_round_end` on a
plain board changes nothing, and the existing `score()`/counter behaviour is
byte-identical throughout.

## Context (current engine)

The lifecycle seam is two-thirds built:

- `GrowthEvent` (`board.rs:14`, `HandPlayed`/`Discard`) is folded over jokers
  by the pure `growth_delta(enhancement, event, rules) -> i32` (`board.rs:775`)
  and applied by `apply_growth` (`board.rs:865`). Public hooks:
  `on_hand_played` (`board.rs:799`), `on_discard` (`board.rs:804`).
- `on_scored` (`board.rs:839`) applies Hiker's permanent card mutations.
- `BuffoonBoard.money: isize` exists (1a); **Bull** reads it (1b). No writer.
- `probability_numerator()` (`board.rs:489`) is the shared odds seam — Oops!
  All 6s doubles any probability routed through it.
- `is_face_card` (`board.rs:447`) is the face predicate — Pareidolia-aware.
- `remove_joker(index)` (`board.rs:691`) removes a joker and its counter slot.
- `MPip::MultPlusChanceDestroyed(15, 1, 6)` (Gros Michel, from 0c) documents
  that its destruction half is "data only until a round-end hook exists to
  roll it" — this design is that hook.
- The `+$` jokers above are all `MPip::Blank` today; Egg already carries
  `SellValueIncrement(3)`; Ice Cream already carries
  `LoseChipsPerHand(100, 5)`; Cavendish carries a bare `MultTimes(3)`
  (missing its destroy chance — EPIC-01a §Data fixes).

## Decisions (from brainstorming)

1. **Scope:** deterministic payouts only, plus the destruction rolls. Still
   `Blank` with a reason: To Do List & Mail-In Rebate (need a per-round random
   target), Rocket (boss blinds), Trading Card (discard destruction), Reserved
   Parking (probabilistic held-card payout — deferred, not blocked).
2. **Architecture:** extend the Phase 3 event seam (Approach A) — a `RoundEnd`
   variant on `GrowthEvent` and a `payout_delta` mirror of `growth_delta` —
   rather than ad-hoc hook bodies or the `EffectRegistry`.
3. **Destruction included**, via an `on_round_end`/`on_round_end_with_rng`
   split mirroring `score`/`score_with_rng`: no RNG → rolls skipped (like
   Lucky staying inert in pure `score()`).
4. **No base-game interest** — that is an economy rule, not a joker. To the
   Moon's *extra* interest is exact and self-contained: `min(money/5, 5) × $1`
   (the base interest cap; vouchers that raise it don't exist yet).
5. **Money mutates in place** (`self.money`), like counters. No payout-summary
   struct until something (a cash-out screen) needs one.
6. **Ice Cream melts on the hand that empties it**, not at round end — the
   check rides `on_hand_played` where its counter already grows. Exact
   Balatro timing.

## Design

### New board state

```rust
// BuffoonBoard
pub discards_used: usize,   // incremented by on_discard, reset by on_round_end
```

`draws.discards` is static config today (2c owns its mutators), so Delayed
Gratification needs its own "was any discard used this round" signal.

### New `MPip` variants (+ `Display` arms + oracle arms)

```rust
CashOnRoundEnd(usize),               // Golden Joker: CashOnRoundEnd(4)
CashPerDiscardIfNoneUsed(usize),     // Delayed Gratification: (2)
CashPerFullDeckRank(usize, char),    // Cloud 9: (1, '9')
ExtraInterest(usize),                // To the Moon: (1)
CashOnFacesDiscarded(usize, usize),  // Faceless Joker: (5, 3) = $5 at ≥3 faces
MultTimesChanceDestroyed(usize, usize, usize), // Cavendish: (3, 1, 1000)
```

Both exhaustive matches (`MPip::Display`, the `scores_hand` oracle in
`joker.rs`) get arms — the five cash variants and the destruction data are
non-scoring (`false` in the oracle); `MultTimesChanceDestroyed` is **scoring**
(`true`) and gets an arm in the ×mult scoring path so Cavendish keeps its ×3
(the Gros Michel compound-variant pattern, on the ×mult side).

### Payout seam

```rust
GrowthEvent::RoundEnd                       // new event variant

/// Money a joker pays for one lifecycle event. The money mirror of
/// `growth_delta`; returns 0 for every non-cash joker.
fn payout_delta(&self, enhancement: MPip, event: &GrowthEvent) -> isize
```

Unlike `growth_delta`, `payout_delta` takes `&self`: Cloud 9 reads
`full_deck`, To the Moon reads `money`, Delayed Gratification reads
`discards_used` + `draws.discards`, Faceless Joker reads the discarded pile
through `is_face_card` (so Pareidolia amplifies it, as in Balatro).

### Hooks

```rust
pub fn on_discard(&mut self, discarded: &BuffoonPile)   // existing, extended:
    // apply_growth(Discard) [unchanged] + apply_payouts(Discard) + discards_used += 1

pub fn on_hand_played(&mut self, played: &BuffoonPile)  // existing, extended:
    // apply_growth(HandPlayed) [unchanged] + melt check: remove any
    // LoseChipsPerHand joker whose base − per × counter has reached 0

pub fn on_round_end(&mut self)                          // new, deterministic:
    // apply_payouts(RoundEnd) → self.money
    // Egg: bump resell_value on each SellValueIncrement joker, in place
    // discards_used = 0

pub fn on_round_end_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R)  // new:
    // everything on_round_end does, then the destruction pass:
    // for each joker with (Mult…|MultTimes…)ChanceDestroyed(_, num, den):
    //   destroyed if rng.random_range(0..den) < probability_numerator-scaled num
    //   (routing through probability_numerator() so Oops! All 6s doubles it)
    // collect indices first, remove_joker() in reverse order
```

Ordering inside `on_round_end_with_rng`: payouts before destruction — a Gros
Michel destroyed this round still pays nothing (it has no payout), but a
Golden Joker must pay before any hypothetical future self-destroyer removes
it; paying first is also Balatro's cash-out-then-cleanup order.

### Error handling

No panics, no new `Result`s: payouts saturate on `isize` arithmetic in the
practical range (money magnitudes are tiny); destruction indexes are collected
before removal so no iterator invalidation; `remove_joker` keeps
`joker_state` aligned. The hooks touch no I/O and stay `std`-only behind the
`funky` feature — `basic`/no_std is untouched.

## Testing

One test per behaviour, exact values, negative cases included
(`mod funky__types__board__buffoon_board_tests`):

- `on_round_end__golden_joker_pays_4`
- `on_round_end__delayed_gratification_pays_2_per_remaining_discard`
- `on_round_end__delayed_gratification_pays_0_after_a_discard`
- `on_round_end__cloud_9_pays_1_per_nine_in_full_deck` (worn-deck board: also
  proves destroyed 9s stop paying)
- `on_round_end__to_the_moon_pays_1_per_5_dollars_capped_at_5`
- `on_discard__faceless_joker_pays_5_on_three_faces` + `_pays_0_on_two_faces`
  + `_pareidolia_makes_any_three_cards_faces`
- `on_round_end__egg_grows_resell_value` (3 → 6)
- `on_round_end__is_inert_on_a_plain_board` (money, jokers, resell all unchanged)
- Destruction (seeded `StdRng`, seeds picked per outcome):
  `on_round_end_with_rng__gros_michel_survives_and_dies`,
  `…__cavendish_1_in_1000`, `…__oops_doubles_gros_michel_odds`
- `on_hand_played__ice_cream_melts_at_zero_chips` (20th hand removes it;
  19th does not)
- `score__cavendish_still_times_3` — the compound-variant regression guard;
  reachability guards keep covering Cavendish via the probe boards.

**Gates (all must stay green):** `cargo test --features funky` · clippy
`-Dclippy::all -Dclippy::pedantic --all-targets --features funky` ·
`cargo build --no-default-features` · `cargo fmt --all -- --check` ·
`cargo doc`. The six newly wired jokers come off the `KNOWN_UNWIRED` ledger;
EPIC-01a's Status table flips 1c (and the Cavendish row in §Data fixes) on
completion.

## Out of scope

Base-game interest and the round cash-out (blind reward, $-per-hand-left);
`Draws` mutation (2c); To Do List / Mail-In Rebate / Rocket / Trading Card /
Reserved Parking (reasons above); retrigger-aware Hiker (tracked on
`on_scored`); vouchers that lift the interest cap.
