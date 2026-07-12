# EPIC-01a: Remaining Joker Wiring (JOKERS)

> **Follow-on to [`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 4.** This document
> defines the work left to give the ~43 still-`MPip::Blank` jokers real scoring
> behaviour. It groups every remaining joker by the **subsystem it needs**, gives
> its exact Balatro effect, and says where it plugs into the current pipeline.
> It is a work definition, not an implementation — nothing here has landed yet.

**Date:** 2026-07-12 · **Branch:** `funky`

---

## Context

The funky scoring engine is complete and unified. A hand scores through a
four-phase pipeline, each phase folding into one running `Score`:

- `BuffoonBoard::fold_played_cards` (phase 2) — `src/funky/types/board.rs:81`
- `BuffoonBoard::fold_held_cards` (phase 3) — `board.rs:156`
- `BuffoonBoard::fold_jokers` (phase 4) — `board.rs:214`

Joker effects resolve to a `ScoreOp` (`src/funky/types/effect.rs`) and are applied
via `ScoreOp::apply`. A built-in joker's op comes from two helpers:

- `BuffoonBoard::builtin_joker_op` (`board.rs:240`) — additive / board-reading
  effects, returns a `ScoreOp`;
- `BuffoonBoard::joker_x_mult` (`board.rs:276`) — the ×mult factor for
  multiplicative jokers.

**What already works (do NOT redo).** Every joker whose effect is a *deterministic
pure function of the current board* is wired (EPIC-01 Story 4): the hand-conditional
`+`/`×mult` families, the per-scored-rank family (Fibonacci/Even Steven/Odd Todd),
Triboulet (per-scored-rank ×mult), and the eight board-state readers — Cavendish,
Abstract Joker (`MultPlusPerJoker`, reads `self.jokers`), Blue Joker
(`ChipsPerDeckCard`, reads `self.deck`), Baron (`MultTimesPerHeldRank`, reads
`self.in_hand`), Scary Face (`ChipsPlusPerScoredFace`), Walkie Talkie
(`ChipsMultPlusPerScoredRanks`), Blackboard (`MultTimesIfHeldAllSuits`), Baseball
Card (`MultTimesPerUncommonJoker`). All new `MPip` variants live in
`src/funky/types/mpip.rs`.

**What this EPIC does NOT do.** It does not model the full Balatro game loop, the
shop UI, run/ante progression, or blind selection as playable systems. It builds
only the *minimum state and hooks* each remaining joker needs to **score
correctly**, and stops there. It does not touch the mod seam
(`Effect`/`EffectRegistry`, `board.rs:421`) — mods can already express any of
these via `MPip::Custom`; this is about the *built-in* jokers.

The blocker in every case below is the same: the joker's effect is **not** a pure
function of the current board — it needs money, a persistent counter, a retrigger
pass, a card-mutation op, a detection-rule change, a full-deck view, or a blind.
Faking any of them re-introduces silent-wrong scoring (the failure mode EPIC-01
was careful to avoid), so each is gated behind building the real mechanism.

---

## Status

| Subsystem (phase) | Unblocks (declared Blank jokers) | Status |
|---|---|---|
| 1 — Economy / money | Bull + all `+$` jokers | **1a/1b done** (money field + Bull); 1c planned |
| 2 — Round & hand state | Banner + Mystic Summit (**assigned-but-unscored → silently 0**), Burglar, Juggler, Drunkard | **2a/2b done** (Banner + Mystic wired); 2c planned |
| 3 — Per-run joker counters | Green Joker, Vampire, Constellation, Hologram, Lucky Cat, Ramen, Popcorn, Square Joker, Spare Trousers, Red Card, Fortune Teller, Flash Card, Runner | Planned |
| 4 — Retriggers | Hack, Mime, Dusk, Sock and Buskin, Seltzer, Hanging Chad | Planned |
| 5 — Deck mutation / create / consumables | DNA, Séance, Superposition, Riff-Raff, Vagabond, Sixth Sense, Hallucination, Marble Joker, Hiker, Perkeo | Planned |
| 6 — Rule modifiers (detection hooks) | Pareidolia, Splash, Shortcut, Four Fingers, Smeared, Oops! All 6s | Planned |
| 7 — Full-deck view | Steel Joker, Stone Joker, Erosion | Planned |
| 8 — Boss blinds | Madness, Luchador, Matador, Chicot | Planned |
| 0 — Prerequisites (data fixes + guard) | Baron rarity/cost, weight uniqueness, silent-zero guard | **Complete** |

---

## Goals

- Give every remaining joker a **correct** scoring effect, or leave it `Blank`
  with a one-line reason — never a plausible-but-wrong value.
- Build each **subsystem** as the smallest board state + hook that lets its jokers
  score, reusing the existing `ScoreOp` / fold pattern rather than special-casing.
- Order the work so the **keystone (economy)** — which vouchers, the shop, and
  several jokers all depend on — lands first.
- Every wired joker gets one test asserting it scores its **exact Balatro value**.

## Scope

- A joker is only wired when its effect is deterministic given the board state the
  subsystem adds. Probabilistic effects (Bloodstone, Business Card…) go through the
  existing seeded-RNG path (`score_with_rng`, `board.rs`), not a fresh mechanism.
- Each subsystem adds fields to `BuffoonBoard` (`board.rs:14`) and/or a hook, plus
  the `MPip` variants its jokers need (`mpip.rs`), wired in `builtin_joker_op` /
  `joker_x_mult`.
- "Gains …" jokers (permanent per-run growth) are **state**, not scoring — they
  need Phase 3's counter store, not a match arm.

---

## Domain map

| Balatro term (wiki) | What it needs | funky construct to add |
|---|---|---|
| `+$` / "earn $N" | money on the board | `BuffoonBoard.money: isize` |
| "Currently +X" (grows) | per-run joker state | counter store keyed by joker |
| "Retrigger" | re-score a card | a retrigger count in the played-card fold |
| "Create" / "Add" / "Destroy" | card ops | deck/hand mutation methods |
| "all cards are face cards", "with 4 cards" | detection override | flags read by hand detection |
| "in full deck" | every card ever | a full-deck accessor |
| "Boss Blind" | blind state | `BuffoonBoard.blind` |
| "When Blind is selected" | a lifecycle hook | `on_blind_selected()` |

---

## Design

Each subsystem below lists **what to build**, the **jokers it unblocks** (exact
wiki effect), and **where it plugs in**. `Nr` = Balatro joker number.

### Phase 1 — Economy / money  *(keystone)*

Add money to the board; it is read by scoring jokers and (later) written by `+$`
jokers, the shop, and interest.

```rust
// BuffoonBoard, board.rs:14
pub money: isize,   // signed: Credit Card allows debt to -$20
```

- **Bull** (#93) — "+2 Chips for each $1 you have." *Scoring, blocked only on the
  field.* New `MPip::ChipsPerDollar(usize)`; handle in `builtin_joker_op` →
  `ScoreOp::AddChips(2 * self.money.max(0) as usize)`.
- The `+$` jokers themselves (**Delayed Gratification** #35, **Faceless Joker**
  #57, **To Do List** #60, **Rocket** #74, **Mail-In Rebate** #83, **To the Moon**
  #84, **Reserved Parking** #82, **Trading Card** #95) do not affect *hand* score —
  they mutate `money` at round/discard/scored events. They need the same lifecycle
  hooks as Phase 2 and are only "done" once those events exist. Wire the field now;
  wire each payout with its trigger.

**Why first:** money is the single most-depended-on piece (Bull, every `+$` joker,
the shop, vouchers, Gift Card, interest). Everything else composes on top.

### Phase 2 — Round & hand state

`BuffoonBoard.draws` already exists (`board.rs:15`, `Draws { hands_to_play,
discards }`). The two scoring jokers here are **not `Blank`** — they already carry
draws-reading variants — **but no scoring path handles those variants, so they
silently score 0 today** (the exact silent-wrong failure EPIC-01 warns about;
verified: `grep` for both variants in `board.rs`/`buffoon_card.rs`/
`buffoon_pile.rs` returns nothing). This phase adds the missing scoring arms, then
adds a "when blind selected" hook for the game-state jokers.

- **Banner** (#22) — "+30 Chips for each remaining discard." Const already set to
  `MPip::ChipsPerRemainingDiscard(30)` (`mpip.rs:36`, `Display` at `:171`).
  **Unhandled → scores 0.** Add a `builtin_joker_op` arm reading
  `self.draws.discards`: `ScoreOp::AddChips(30 * self.draws.discards)`.
- **Mystic Summit** (#23) — "+15 Mult when 0 discards remaining." Const already set
  to `MPip::MultPlusOnZeroDiscards(15)`. **Unhandled → scores 0.** Add a
  `builtin_joker_op` arm: `AddMult(15)` iff `self.draws.discards == 0`.
- Hand-size / discard-count jokers (**Juggler** #87 +1 hand size, **Drunkard** #88
  +1 discard, **Burglar** #47 +3 hands/−discards on blind select) change *how many
  draws you get*, not the hand score. They need `Draws` mutated at the right
  lifecycle event; no scoring hook.

### Phase 3 — Per-run joker counters

The largest bucket. Each of these "gains X, currently Y" jokers carries **mutable
per-run state** that grows on some event and is read at scoring. Build one counter
store rather than a field per joker.

```rust
// A parallel Vec<usize> aligned to self.jokers, or a small map keyed by slot.
pub joker_state: Vec<usize>,   // per-joker accumulator
```

Then a growth hook per trigger event (hand played, discard, card sold, planet used,
enhanced card scored, card added to deck, pack skipped, reroll). Scoring reads the
accumulator.

Jokers (exact effect): **Green Joker** (#58, +1/hand −1/discard), **Vampire**
(#68, ×0.1 per enhanced card played, strips enhancement — also mutation),
**Constellation** (#55, ×0.1 per planet used), **Hologram** (#70, ×0.25 per card
added), **Lucky Cat** (#91, ×0.25 per Lucky trigger), **Ramen** (#100, ×2 −0.01
per discard), **Popcorn** (#97, +20 −4/round), **Square Joker** (#65, +4 chips if 4
cards), **Spare Trousers** (#98, +2 if two pair), **Red Card** (#63, +3 per pack
skipped), **Fortune Teller** (#86, +1 per tarot used), **Flash Card** (#96, +2 per
reroll), **Runner** (#49, +15 chips if straight). Legendaries **Canio** (×1 per face
destroyed) and **Yorick** (×1 per 23 discarded) also live here.

The *read* side is trivial once the counter exists — `builtin_joker_op` /
`joker_x_mult` return the accumulated value. The work is the **write** side: the
event hooks and where they fire.

### Phase 4 — Retriggers

A retrigger re-scores a played (or held) card, re-running all its scoring. In this
engine that means the played-card fold (`fold_played_cards`, `board.rs:81`) applies
a card's contribution `1 + retriggers` times. Compute a per-card retrigger count
from the jokers before/within the fold.

Jokers: **Hack** (#36, retrigger played 2/3/4/5), **Dusk** (#28, retrigger in final
hand — needs Phase 2 "final hand" state), **Sock and Buskin** (#109, retrigger
played face), **Seltzer** (#102, retrigger all for 10 hands — Phase 3 counter),
**Hanging Chad** (#115, retrigger first card 2×), **Mime** (#19, retrigger *held*
card abilities — affects phase 3).

### Phase 5 — Deck mutation / create / consumables

Add card-level operations to the board: add a card to the deck, destroy a card,
create a consumable/joker. Most of these are **not scoring effects** — they change
future state — so they only matter once there is a round loop and consumable
inventory. Low scoring priority.

Jokers: **DNA** (#51), **Sixth Sense** (#54), **Superposition** (#59), **Séance**
(#66), **Riff-Raff** (#67), **Vagabond** (#71), **Hallucination** (#85), **Marble
Joker** (#24), **Hiker** (#56, permanently mutates scored cards' chips). Legendary
**Perkeo** (#150). Depends conceptually on the spectral/consumable systems in
EPIC-01 Story 3.

### Phase 6 — Rule modifiers (detection hooks)

These change *how hands and cards are classified* — they must be consulted inside
hand detection (`BuffoonPile::determine_hand_type` / the `has_*` predicates) and
face-card checks, not added as a `ScoreOp`. Thread a small "active rules" set from
the board's jokers into detection.

Jokers: **Pareidolia** (#37, all cards are face cards — changes Scary/Smiley/
Photograph inputs), **Splash** (#52, all played cards score — our model already
scores all played cards, so verify/no-op), **Shortcut** (#69, gapped straights —
`connectors(distance)` groundwork already exists), **Four Fingers** (#18, 4-card
flush/straight — `connectors` groundwork exists), **Smeared Joker** (#113, merge
suit pairs), **Oops! All 6s** (#126, doubles the odds used by the RNG path).

### Phase 7 — Full-deck view

"in full deck" counts every card the run owns (drawn, played, discarded, held),
not just `self.deck` (the undealt remainder). Add a full-deck accessor.

Jokers: **Steel Joker** (#32, ×0.2 per Steel in full deck), **Stone Joker** (#89,
+25 chips per Stone), **Erosion** (#81, +4 mult per card below starting deck size —
also needs the starting size recorded).

### Phase 8 — Boss blinds

Add blind state and effects. Mostly `!!` effect jokers, minimal scoring impact
except via disabling debuffs.

Jokers: **Madness** (#64, ×0.5 on blind select + destroy joker — also Phase 3),
**Luchador** (#77, sell to disable boss), **Matador** (#129, $8 if boss triggers —
economy), Legendary **Chicot** (#149, disables all boss blinds).

### Data fixes (no subsystem needed)

- **Baron** is tagged `BCardType::CommonJoker` with cost `value: 5`
  (`joker.rs`, the `BARON` const) but is **Rare / $8** in Balatro. Fix rarity +
  cost; it then belongs in `RARE_JOKERS`, not adrift.
- **Blackboard** shares `weight: 895` with **Abstract Joker** (`joker.rs`). Weights
  are display/sort order and should be unique; re-weight one.

---

## Work Items

Phases are ordered by leverage. Within a phase: build the state/hook, then wire
each joker + its test. Track completion by flipping the Status table.

### Phase 0 — Prerequisites  *(Complete)*

- [x] **0a.** Data fixes landed. **Baron** → Rare / $8, added to `RARE_JOKERS`
  (size 5→6); test `baron__is_rare_dollar_eight_and_piled`. **Weights** made
  unique across all 105 joker consts (test `all_jokers__weights_are_unique`).
  *Finding:* the Blackboard/Abstract `895` clash the EPIC flagged was 1 of **14**
  collisions (all among the 61 defined-but-unpiled consts; `weight` is a
  cosmetic sort key, never a lookup key). All 16 duplicate consts were re-weighted
  to unique nearby values; the 44 piled jokers were already unique.
- [x] **0b.** Silent-zero guard landed: a test-only `ALL_JOKERS` registry (all
  105 consts, kept a superset of the four piles by
  `all_jokers__is_superset_of_every_pile`), an exhaustive `scores_hand(MPip)`
  intent oracle, a probe-board battery, and
  `all_jokers__intended_hand_scorers_are_reachable`, which fails if any joker
  that *intends* to score adds nothing. The exhaustive match makes it crate-wide:
  a new `MPip` variant will not compile until classified.

  **Audit findings** (jokers that were silently scoring 0, beyond Banner/Mystic):
  - *Wrong-variant data bug, fixed here:* **Golden Joker** carried `Chips(4)` but
    is an economy joker ("earn $4 at round end") → set to `Blank` like its `+$`
    siblings, to be paid out in Phase 1c.
  - *Pure-function scorers, wired here* (removed from `KNOWN_UNWIRED`):
    **Scholar** `MultPlusChipsOnRank(4, 20, 'A')` → +20 chips & +4 mult per played
    Ace (`builtin_joker_op`, like Walkie Talkie); test
    `score__scholar_adds_chips_and_mult_per_played_ace`. **Raised Fist**
    `MultPlusXOnLowestRankInHand(2)` → +2× the lowest held card's value to mult;
    test `score__raised_fist_adds_double_lowest_held_rank_to_mult`.
  - *Still unwired — genuinely need per-run state* (`KNOWN_UNWIRED`): **Ice Cream**
    `Chips(100)` decays −5 per hand played (needs the Phase 3 hands counter; a flat
    +100 would score wrong), and **Joker Stencil** `MultTimesOnEmptyJokerSlots`
    needs a real joker-slot *limit* on the board (Vec capacity ≠ the 5-slot rule).
  - *Not catchable by this guard — separate data bug:* **Gros Michel** encodes
    only `ChanceDestroyed(1, 6)`; its **+15 mult is missing from the const
    entirely**. The guard classifies by the *variant*, so a joker using a
    non-scoring variant when it should score is invisible to it. Needs a compound
    `chance + mult` representation.

### Phase 1 — Economy / money  *(keystone)*

- [x] **1a.** Added `BuffoonBoard.money: isize` (`board.rs`), default 0.
- [x] **1b.** `MPip::ChipsPerDollar(usize)` + `builtin_joker_op` arm; **Bull**
  wired; test `score__bull_scales_with_money`.
- [ ] **1c.** Lifecycle hooks (`on_round_end`, `on_discard`, `on_scored`) that pay
  out the `+$` jokers; one test per payout. *(Can trail — not scoring.)*

### Phase 2 — Round & hand state

- [x] **2a.** `builtin_joker_op` arm for `MultPlusOnZeroDiscards` reading
  `self.draws.discards`; **Mystic Summit** no longer scores 0; test
  `score__mystic_summit_adds_mult_only_when_no_discards`.
- [x] **2b.** `builtin_joker_op` arm for `ChipsPerRemainingDiscard`; **Banner**
  no longer scores 0; test `score__banner_adds_chips_per_remaining_discard`.
- [ ] **2c.** `on_blind_selected()` hook + `Draws` mutators for Burglar/Juggler/
  Drunkard (game state, not scoring).

### Phase 3 — Per-run joker counters

- [ ] **3a.** Add the per-joker counter store to `BuffoonBoard`.
- [ ] **3b.** Growth hooks per trigger event; **read**-side `MPip` variants +
  `joker_x_mult`/`builtin_joker_op` arms.
- [ ] **3c.** Wire the 13 counter jokers + Canio/Yorick; one test each.

### Phase 4 — Retriggers

- [ ] **4a.** Per-card retrigger count in `fold_played_cards`; **Hack**, **Sock and
  Buskin**, **Hanging Chad** first (pure per-card), then the state-dependent ones.

### Phase 5–8

- [ ] **5–8.** Deck mutation, rule modifiers, full-deck view, boss blinds — each a
  self-contained sub-EPIC; see Design. Wire jokers as each mechanism lands.

---

## Test Plan

- `score__bull_scales_with_money` — Bull with `money = N` adds `2·N` chips.
- `score__mystic_summit_no_discards` — +15 mult only when `draws.discards == 0`.
- One `score__<joker>_*` per newly wired joker asserting its **exact wiki value**,
  following the existing board-test pattern (`board.rs`, the `score__*` tests).
- Extend `assert_rarity_pile` after the Baron data fix.
- Gold Standard (EPIC-00f): each new joker's test must *fail* before its arm lands.

## Key Files

| File | Role |
|---|---|
| `src/funky/types/board.rs` | new board state (money, counters, blind) + `builtin_joker_op`/`joker_x_mult` arms + lifecycle hooks |
| `src/funky/types/mpip.rs` | new `MPip` variants (+ `Display`) |
| `src/funky/decks/joker.rs` | flip `enhancement: MPip::Blank` → the wired variant; data fixes |
| `src/funky/types/effect.rs` | `ScoreOp` — unchanged; new additive effects reuse `AddChips`/`AddMult`/`Add` |

## Reuse (do NOT recreate)

- The fold pattern — `fold_played_cards`/`fold_held_cards`/`fold_jokers`
  (`board.rs:81,156,214`). New jokers add a match arm, never a new loop.
- `builtin_joker_op` (`board.rs:240`) for additive/board-reading, `joker_x_mult`
  (`board.rs:276`) for ×mult — the two seams. Board-reading examples: Abstract/
  Blue/Baron already show reading `self.jokers`/`self.deck`/`self.in_hand`.
- `ScoreOp` + `apply` (`effect.rs`) — the single application currency.
- Seeded RNG path — `score_with_rng` (`board.rs`) for any probabilistic joker;
  do not hand-roll randomness.
- `MPip::Custom` + `EffectRegistry` (`board.rs:421`) — if a joker is a one-off, a
  mod-style custom effect may be cheaper than a built-in variant.
- `connectors(distance)` (`buffoon_pile.rs`) — groundwork for Four Fingers /
  Shortcut straight-detection changes.

## Compatibility

- **Preserves** the pure `score()` and every existing wired joker — new state
  defaults to inert (money 0, counters 0, no blind), so scores are unchanged until
  a joker reads them. **Adds** board fields + `MPip` variants. **Breaks** nothing.

## Dependencies

- **Blocks:** EPIC-01 Story 7 (shop/economy) shares the `money` field built here;
  vouchers/blinds build on Phases 1 & 8.
- **Built on:** EPIC-01 (four-phase pipeline, `ScoreOp`, rarity piles) and the
  effect-registry design (`docs/2026-07-11-effect-registry-design.md`).
- **Related:** EPIC-01 Story 3 (spectral/consumables) underlies Phase 5.

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features            # funky must not leak into no_std
cargo fmt --all -- --check
```

Exit criteria (per phase, not the whole EPIC):

1. Every joker wired in a phase has a test asserting its exact Balatro value, and
   that test failed before the arm landed.
2. `score()` for a board with none of the phase's jokers is byte-identical to
   before (new state is inert by default).
3. The Status table row flips to **Complete** only with cited, tested code.
