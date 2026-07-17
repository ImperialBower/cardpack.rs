# EPIC-01a: Remaining Joker Wiring (JOKERS)

> **Follow-on to [`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 4.** This document
> defines the work left to give the ~43 still-`MPip::Blank` jokers real scoring
> behaviour. It groups every remaining joker by the **subsystem it needs**, gives
> its exact Balatro effect, and says where it plugs into the current pipeline.
>
> **All eight phases have landed.** Every subsystem this EPIC set out to build
> exists, and 19 of the 43 jokers are `Blank` no longer. The rest stay `Blank`
> **with a stated reason** rather than a plausible-but-wrong value, which was
> always the goal — see [Remaining `Blank`](#remaining-blank) for the full list
> and what each one is actually waiting on.

**Date:** 2026-07-12 (closed out 2026-07-16) · **Branch:** `funky`

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
| 1 — Economy / money | Bull + all `+$` jokers | **Complete** — money field + Bull (1a/1b); round-end/discard payout seam: Golden Joker, Delayed Gratification, Cloud 9, To the Moon, Faceless Joker, Egg, plus the Gros Michel/Cavendish destruction rolls and Ice Cream's melt (1c). **Rocket** landed later, with the boss blinds it was waiting on (Phase 8). Still `Blank` with reasons: To Do List, Mail-In Rebate, Trading Card, Reserved Parking |
| 2 — Round & hand state | Banner + Mystic Summit (**assigned-but-unscored → silently 0**), Burglar, Juggler, Drunkard | **Complete** — Banner + Mystic wired (2a/2b); `on_blind_selected()` recomputes `Draws` from a recorded baseline: Juggler (+1 hand size), Drunkard (+1 discard), Burglar (+3 hands, wipes all discards) (2c) |
| 3 — Per-run joker counters | Green Joker, Vampire, Constellation, Hologram, Lucky Cat, Ramen, Popcorn, Square Joker, Spare Trousers, Red Card, Fortune Teller, Flash Card, Runner | **Complete** — store + the six growth events. Wired: Green Joker, Ramen, Ice Cream, Square Joker, Spare Trousers, Runner (3a); **Popcorn**, **Yorick**, **Hologram**, **Canio**, **Vampire** (3b); **Constellation**, **Fortune Teller** (3c). Still `Blank` with reasons: Lucky Cat (needs the in-fold Lucky proc), Red Card (booster packs), Flash Card (shop rerolls) |
| 4 — Retriggers | Hack, Mime, Dusk, Sock and Buskin, Seltzer, Hanging Chad | **Complete** — retrigger loops in `fold_played_cards` (played) + `fold_held_cards` (held); **Hack**, **Sock and Buskin**, **Hanging Chad**, **Mime** (4a/4b), plus **Dusk** and **Seltzer** on the new per-round `hands_played` state (4c) |
| 5 — Deck mutation / create / consumables | DNA, Séance, Superposition, Riff-Raff, Vagabond, Sixth Sense, Hallucination, Marble Joker, Hiker, Perkeo | **Complete** — mutation seam + `on_scored` (5a/5b, **Hiker**); the consumable seam `create_consumable`/`use_consumable` with real slot limits, then **Superposition**, **Vagabond** (5c) and **Marble Joker**, **Riff-Raff** (5d). Still `Blank` (5e, subsystems outside this EPIC): DNA, Sixth Sense, Séance, Hallucination, Perkeo |
| 6 — Rule modifiers (detection hooks) | Pareidolia, Splash, Shortcut, Four Fingers, Smeared, Oops! All 6s | **Complete** — `HandRules` seam (straight/flush/smeared), face-predicate hook, and RNG odds-numerator; all six wired (Four Fingers, Shortcut, Pareidolia, Smeared, Splash [no-op], Oops! All 6s) |
| 7 — Full-deck view | Steel Joker, Stone Joker, Erosion | **Complete** — `full_deck` roster + `starting_deck_size` on the board; all three wired |
| 8 — Boss blinds | Madness, Luchador, Matador, Chicot | **Complete** — `Blind`/`BossBlind` plus the three bosses whose ability is a pure `Draws` mutation (The Needle, The Water, The Manacle); **Madness**, **Luchador**, **Chicot** wired, and **Rocket** unblocked from Phase 1. **Matador** stays `Blank`: no modelled boss ability is triggered *by a played hand*, so it has no event to read |
| 0 — Prerequisites (data fixes + guard) | Baron rarity/cost, weight uniqueness, silent-zero guard, Gros Michel, Glass | **Complete** — 0c fixed Gros Michel's missing +15 mult; 0d wired the **Glass** card and added the **card-level** silent-zero guard, which promptly found the **Stone** card scoring 0 (tracked, needs a detection hook too). **Cavendish** remains latently short its destroy chance |

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
| "…this run" (retroactive) | a run-wide statistic | a board field, **not** a counter |
| "(Must have room)" | a real slot limit | `joker_slots` / `consumable_slots` |
| "final hand of round" | per-round hand count | `hands_played` + `is_final_hand()` |
| "Sell this card to …" | a sell operation | `sell_joker()` |

**The state model, in one line each.** Three kinds of joker memory turned out to
be genuinely distinct, and picking the wrong one is silently wrong scoring:

- **Per-joker counter** (`joker_state`) — grows from when the joker arrived.
  Constellation. *Not* retroactive.
- **Run-wide statistic** (a board field) — true regardless of who is watching.
  Fortune Teller, which Balatro makes retroactive.
- **Per-round state** (a board field, reset by the round) — `hands_played`.
  Dusk's "final hand of round" resets; Seltzer's 10 hands do not, which is why
  one is a board field and the other a counter.

**When an event fires matters as much as that it fires.** `Scored` (before the
fold) and `HandPlayed` (after it) are the same hand seen twice: a counter grown
on the first is read by the hand that grew it, one grown on the second is not.
Vampire needs the first, Ice Cream the second.

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
- ~~**Erosion** and **Stone Joker** are tagged `CommonJoker` / `value: 5` and sit
  in no rarity pile; Balatro has both at **Uncommon / $6**.~~ Fixed in Phase 7 —
  see 7c.

  ~~**The sweep is still open**~~ — **swept 2026-07-16** (post-close-out): 52
  consts reconciled against balatrowiki.org, the piles now partition
  `ALL_JOKERS` (56/41/10/5), and two new guards pin it —
  `all_jokers__every_joker_is_piled_by_its_rarity` and
  `all_jokers__resell_value_is_half_cost_floored_at_one`. The sweep also caught
  **Mystic Summit piled and wrong** (Uncommon/$6 in `UNCOMMON_JOKERS`; the wiki
  has Common/$5) — a *consistently* misfiled const is invisible to both guards,
  so only source reconciliation finds it. The drift was measured rather than
  estimated before the sweep: Every joker Phases 3–8 touched needed the *same* fix — the const
  had been left at the `CommonJoker` / `value: 5` / `resell_value: 0` default and
  adrift from every pile. Nine more were corrected in passing (Hologram, Vampire,
  Constellation, Fortune Teller, Superposition, Vagabond, Riff-Raff, Madness,
  Rocket), bringing the tally to **twelve** across 0a/7c and Phases 3–8. That is
  a pattern, not a coincidence: `CommonJoker`/$5 is what an unwired const looks
  like, so **rarity drift and `MPip::Blank` are the same debt seen twice**, and
  wiring a joker is when its data gets checked. The remaining ~50 unpiled consts
  will each want the same three-line fix. Worth one sweep against the wiki rather
  than piecemeal, but the piecemeal route is at least self-correcting: nothing
  gets wired without being looked up.
- ~~**Glass card scores nothing** (`decks/tarot.rs`, the `JUSTICE` const →
  `MPip::Glass(2, 4)`).~~ Fixed in 0d.
- ~~**The silent-zero guard is joker-only**, so no *card* enhancement is
  covered.~~ Closed in 0d — `all_card_enhancements__intended_hand_scorers_are_reachable`
  is the card-level twin.
- **Stone card scores nothing** (`decks/tarot.rs`, the `TOWER` const →
  `MPip::Stone(50)`). Balatro's Stone card is **+50 chips**, with **no rank or
  suit**. Found by the 0d guard; tracked in `KNOWN_UNWIRED_CARD_ENHANCEMENTS`
  rather than wired, because the chips alone would make a Stone card count toward
  straights and flushes it should not — a silently wrong *hand type*, worse than
  the silent zero it replaces. Needs the chips **and** a detection suppression
  together; Phase 6's `HandRules` seam is the natural home for the latter.
- ~~**Cavendish** (#33) is missing its **1-in-1000 destroy chance** (`MultTimes(3)`
  carries only the mult) — the mirror image of the Gros Michel bug fixed in 0c.
  Latent, not live: scoring is correct today and nothing drives destruction yet.
  Give it `MultTimesChanceDestroyed` when the round-end hook lands, rather than
  minting a compound variant now for a system that does not exist.~~ Fixed in
  1c, exactly as planned: `MultTimesChanceDestroyed(3, 1, 1000)`, rolled by
  `on_round_end_with_rng`.
- ~~**Hiker** (#56) is tagged `CommonJoker` / `value: 5` and sits in no rarity
  pile. Balatro is believed to have it at **Uncommon / $5** — unverified against
  the wiki, so deliberately *not* fixed on a guess.~~ Verified and fixed in the
  2026-07-16 sweep: the wiki confirms **Uncommon / $5**, exactly as believed.

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
  - ~~*Not catchable by this guard — separate data bug:* **Gros Michel** encodes
    only `ChanceDestroyed(1, 6)`; its **+15 mult is missing from the const
    entirely**. The guard classifies by the *variant*, so a joker using a
    non-scoring variant when it should score is invisible to it. Needs a compound
    `chance + mult` representation.~~ **Fixed** — see 0c.

- [x] **0c.** **Gros Michel** fixed, via the compound representation 0b called
  for: new `MPip::MultPlusChanceDestroyed(15, 1, 6)` (+15 mult, 1-in-6 destroyed
  at end of round), const flipped from `ChanceDestroyed(1, 6)`, scored through a
  `builtin_joker_op` arm as a flat `AddMult`. Classified **scoring**; the joker
  was reachable immediately, so no probe board was needed. Tests, both failing
  before the const flips (the joker scored `mult: 1` — literally zero):
  `score__gros_michel_adds_mult_regardless_of_its_destruction_chance` and
  `score__gros_michel_mult_is_not_a_probabilistic_effect` (the +15 is flat, so
  all 16 probed seeds agree with the pure `score()` — it must not drift onto the
  RNG path like Lucky). `score__cavendish_still_scores_its_x3_beside_its_sibling`
  pins the sibling against regression.

  The destruction half is **data only** — end of round has no hook yet — but is
  recorded as an explicit numerator/denominator so it routes through
  `probability_numerator` and inherits Oops! All 6s free when a round-end lands
  (the Phase 6e prediction). `ChanceDestroyed` now has no users; it is kept but
  documented as a footgun, since encoding only the destruction half is precisely
  what hid the mult.

  **Audit finding — Gros Michel is one of three, not one.** The same "a card
  that both scores and can be destroyed" shape is encoded three different ways,
  each broken differently:
  - **Gros Michel** — `ChanceDestroyed(1, 6)`: had the destruction, lost the
    mult. *Live wrong-scoring.* **Fixed here.**
  - **Cavendish** (#33) — `MultTimes(3)`: the exact mirror image. Has the mult,
    lost its **1-in-1000 destroy chance**. *Latent only* — scoring is correct
    today, and the missing half has no system to drive it, so it is left alone
    rather than given a second compound variant on spec.
  - **Glass card** (tarot `JUSTICE`) — `MPip::Glass(2, 4)`: carries **both**
    halves and **scores neither**. A Glass King scores identically to a plain
    King (verified: 40/1 both). *Live wrong-scoring*, and worse than Gros Michel
    in reach — Glass is a card enhancement, not a single joker. It was invisible
    to the reachability guard for a structural reason: the guard iterates
    `ALL_JOKERS`, and Glass is a **card**. **Fixed in 0d**, along with the guard
    gap itself.

- [x] **0d.** The **card-level silent-zero guard** — the structural gap 0c
  surfaced — plus the two bugs it exists to catch.

  **Glass card wired.** `MPip::Glass(mult, _)` → `ScoreOp::TimesMult` in
  `fold_played_cards`'s special match, beside Lucky. It belongs there and not on
  the additive `calculate_plus` path because ×mult is **order-sensitive**: Glass
  scales the score accumulated up to *its own card*. Reclassified **scoring** in
  `scores_hand`. Two tests, both failing before the arm lands:
  `score__glass_card_multiplies_mult_when_scored` (a Glass King keeps its 10
  chips and doubles the mult, 40/1 → 40/2) and
  `score__glass_card_multiplies_at_its_own_position_in_the_hand` (same two cards,
  swapped order → mult 6 vs 10; this is what stops the arm drifting back onto the
  additive path, where the ordering would be silently lost). The 1-in-4
  destruction half stays data-only, pending the same round-end hook as Gros
  Michel's.

  **The guard** — `all_card_enhancements__intended_hand_scorers_are_reachable`,
  the twin of the joker guard, against the **same** `scores_hand` intent oracle,
  so there is one definition of "intends to score" for the whole crate. Two
  design choices worth keeping:
  - The registry is **derived, not hand-listed**: stamp every tarot onto a plain
    card via `enhance` and read back what stuck (rank/suit mutators and run-level
    tarots leave it `Blank` and drop out). A new tarot joins the guard
    automatically; a hand-written list would quietly fall behind the deck — the
    same rot that produced these bugs.
  - It probes the card **played *and* held**, because the two run through
    different folds. A Bonus card's chips land in phase 2, a Steel card's ×1.5
    only in phase 3, and an enhancement wired to the wrong fold scores nothing
    where it counts.

  Verified the guard bites rather than passing vacuously: reverting the Glass arm
  makes it fail naming `Glass(2, 4)`.

  **It immediately found a third bug, independently: the Stone card.**
  `MPip::TOWER` = `Stone(50)` should be +50 chips when scored and currently adds
  **nothing** (confirmed by emptying `KNOWN_UNWIRED_CARD_ENHANCEMENTS` and
  watching the guard report `Stone(50)`). It is listed in
  `KNOWN_UNWIRED_CARD_ENHANCEMENTS` rather than wired, deliberately: a Stone card
  also has **no rank or suit** for detection, so wiring only the chips would
  trade a silent zero for a silently *wrong hand type* — a strictly worse bug.
  Both halves should land together, behind a detection hook (Phase 6's
  `HandRules` seam is the natural home).

### Phase 1 — Economy / money  *(keystone)*

- [x] **1a.** Added `BuffoonBoard.money: isize` (`board.rs`), default 0.
- [x] **1b.** `MPip::ChipsPerDollar(usize)` + `builtin_joker_op` arm; **Bull**
  wired; test `score__bull_scales_with_money`.
- [x] **1c.** Round lifecycle hooks landed (design:
  [`2026-07-15-funky-lifecycle-hooks-design.md`](./superpowers/specs/2026-07-15-funky-lifecycle-hooks-design.md)).
  The Phase 3 event seam grew a `RoundEnd` variant and a **money mirror** of
  `growth_delta` — `payout_delta(&self, …) -> isize`, applied by
  `apply_payouts` as one sum computed from the pre-event board, so joker
  order cannot matter and To the Moon reads the balance *before* this
  round's payouts (Balatro's cash-out screen semantics). Plus
  `BuffoonBoard.discards_used` (reset each round end; `draws` stays static
  config until 2c) and the `on_round_end` / `on_round_end_with_rng` split
  mirroring `score`/`score_with_rng` — no RNG, no destruction rolls, like
  Lucky staying inert in the pure `score()`. Six jokers wired, one test per
  behaviour, each verified to fail without its arm:
  - **Golden Joker** → `CashOnRoundEnd(4)` — pays the $4 that 0b's audit
    found mislabelled as `Chips(4)`;
  - **Delayed Gratification** → `CashPerDiscardIfNoneUsed(2)` — $2 per
    remaining discard, forfeited by any discard this round;
  - **Cloud 9** → `CashPerFullDeckRank(1, '9')` — reads the roster, so a
    destroyed 9 stops paying (pinned via 5a's `destroy_deck_card`);
  - **To the Moon** → `ExtraInterest(1)` — `min(money/5, 5)`, the base
    interest cap; base-game interest itself is an economy rule, not a
    joker, and stays out of scope;
  - **Faceless Joker** → `CashOnFacesDiscarded(5, 3)` — fires on
    `on_discard`, classified through `is_face_card`, so Pareidolia makes
    any three discarded cards pay (interaction test);
  - **Egg** — its `SellValueIncrement(3)` now grows its own `resell_value`
    in place each round end.

  The destruction pass closes 0c's deferred debt: **Gros Michel** rolls its
  1-in-6, and **Cavendish** gained its missing 1-in-1000 via the new
  compound `MultTimesChanceDestroyed(3, 1, 1000)` (the §Data fixes mirror
  bug; its ×3 keeps scoring through `joker_x_mult`, pinned by the existing
  Cavendish tests). Both roll through `probability_numerator`, so Oops!
  All 6s doubles them — pinned: three Oops make Gros Michel's destruction
  certain on every seed. Bare `ChanceDestroyed` rolls too, so the 0c
  footgun variant can no longer hide a destruction. **Ice Cream** melts on
  the hand that empties its chips (`on_hand_played`, where its counter
  already grows — exact Balatro timing; the 19th hand leaves it, the 20th
  removes it). All hooks are inert on a board without these jokers
  (`on_round_end__is_inert_on_a_plain_board`).

  Still `Blank` with reasons (the design's scope cut): **To Do List** and
  **Mail-In Rebate** (need a per-round random target), ~~**Rocket** (boss
  blinds)~~, **Trading Card** (discard destruction), **Reserved Parking**
  (probabilistic held-card payout — deferred, not blocked).

  **Rocket** is wired as of Phase 8, exactly as this item predicted: the boss
  blinds were the only thing it needed. See item 8.

### Phase 2 — Round & hand state

- [x] **2a.** `builtin_joker_op` arm for `MultPlusOnZeroDiscards` reading
  `self.draws.discards`; **Mystic Summit** no longer scores 0; test
  `score__mystic_summit_adds_mult_only_when_no_discards`.
- [x] **2b.** `builtin_joker_op` arm for `ChipsPerRemainingDiscard`; **Banner**
  no longer scores 0; test `score__banner_adds_chips_per_remaining_discard`.
- [x] **2c.** `on_blind_selected()` landed, completing Phase 2. `Draws` gained
  a `hand_size` field (base 8, Balatro's default — the concept did not exist
  anywhere on the board), and `BuffoonBoard` records `starting_draws` — the
  `starting_deck_size` pattern — so the hook **recomputes** the round's
  `draws` from the baseline instead of mutating in place. That makes it
  idempotent (a second blind select never stacks a bonus) and self-cleaning
  (a sold joker takes its bonus with it at the next blind). It also resets
  `discards_used`, since a new blind is a new round for Delayed
  Gratification. Three jokers wired, each flipping an existing `Blank` const,
  all classified non-scoring (they change what the round *grants*, not the
  hand score):
  - **Juggler** (#87) → `MPip::HandSizeIncrement(1)`;
  - **Drunkard** (#88) → `MPip::DiscardIncrement(1)` — interaction test:
    Banner reads the recomputed discards (+30 × 4);
  - **Burglar** (#47) → `MPip::GainHandsLoseDiscardsWhenBlindSelected(3)` —
    the discard wipe lands **after** every increment, so it erases Drunkard's
    +1 regardless of joker order (pinned), and it switches Mystic Summit on
    (interaction test: +15 mult at zero discards).

  Six tests, each failing before its wiring (spot-verified by reverting the
  Burglar const), plus the inert-plain-board guard for exit criterion 2.

### Phase 3 — Per-run joker counters

- [x] **3a.** Added the per-joker counter store to `BuffoonBoard`, plus the two
  in-round growth events `on_hand_played` and `on_discard`. Six counter jokers
  wired end-to-end (store → event → read-side score), one test each: **Green
  Joker** (+1 mult/hand, −1/discard), **Ramen** (×2 mult, decaying −×0.01 per
  card discarded), **Ice Cream** (+100 chips, decaying −5/hand played — also
  removed from `KNOWN_UNWIRED`), **Square Joker** (+4 chips per 4-card hand),
  **Spare Trousers** (+2 mult per two-pair hand), **Runner** (+15 chips per
  straight hand). Remaining counter jokers (Vampire, Constellation, Hologram,
  Lucky Cat, Popcorn, Red Card, Fortune Teller, Flash Card) still await their
  triggering events — round-end, shop, consumables, etc. — before they can be
  wired.
- [x] **3b.** The growth seam grew three events — `CardAdded` (fired by
  `add_card_to_deck`), `CardDestroyed` (by `destroy_deck_card`) and `Scored`
  (by `on_scored`) — and `growth_delta` took `&self`, the `payout_delta` shape,
  so growth can read the board. Five jokers wired, one exact-value test each:
  - **Popcorn** → `LoseMultPerRound(20, 4)` (+20 mult, −4 a round). Ice Cream's
    decay on the mult side, which generalised `melt_emptied_jokers` into a shared
    `is_decayed_to_nothing` now called from `on_round_end` too — a decaying joker
    is destroyed **by the event that empties it**, and each is only ever emptied
    by its own event, so the other hook's call is a no-op for it.
  - **Yorick** → `GainMultTimesPerDiscardedCards(100, 23)`, counting **cards**
    discarded rather than discard *actions*; the distinguishing test discards 23
    in one action and expects ×2.
  - **Hologram** → `GainMultTimesPerCardAdded(25)`. Additive (`1 + 0.25×n`), the
    Steel Joker rule — pinned at ×2 on four cards, which compounding (×0.25⁴)
    would fail. A paired negative test holds `replace_deck_card` to *not* being
    an add, so Hiker's per-card bump cannot silently feed it.
  - **Canio** → `GainMultTimesPerFaceDestroyed(100)`, classified through
    `is_face_card` — so Pareidolia makes every destroyed card feed it, the same
    amplification it already gives Faceless Joker (interaction test).
  - **Vampire** → `GainMultTimesPerEnhancedPlayed(10)`. The wiki settles the
    ordering question this was blocked on: *"Vampire removes Enhancements before
    their effect occurs, meaning that the enhancement will not affect your hand's
    score."* That is exactly what `on_scored` already does, so both halves live
    there — it grows on the new `Scored` event (which is why its ×mult lands on
    the hand it ate, unlike a `HandPlayed` counter) and strips as it goes.
    Pinned by the headline Glass interaction: two Glass cards give mult 4 alone
    and mult 2 once Vampire eats them.

  *Finding — the Wild caveat does not apply here.* The wiki notes Vampire cannot
  retroactively change a hand type, since eating a Wild leaves the hand a Flush.
  This engine never wires `MPip::Wild` into detection at all (it appears only on
  the tarot const and in the non-scoring classification), so there is no
  detection effect for Vampire to remove. That is a separate pre-existing gap,
  not one Vampire introduces.

  *Data fix (the Baron/Erosion pattern, 4th and 5th instance):* **Hologram** and
  **Vampire** were tagged Common / $5 and adrift from every rarity pile; Balatro
  has both at **Uncommon / $7** (sell $3). Test
  `hologram_and_vampire__are_uncommon_dollar_seven_and_piled`.

- [x] **3c.** The last two counter jokers, once 5c's consumable seam gave them an
  event — and they are the phase's most interesting pair, because they are the
  *same* effect shape with opposite state models:
  - **Constellation** → `GainMultTimesPerPlanetUsed(10)`, a plain `joker_state`
    counter. Balatro's does **not** scale retroactively, and
    `score__constellation_does_not_scale_retroactively` pins that: five Planets
    spent before it arrives are worth nothing to it.
  - **Fortune Teller** → `MultPlusPerTarotUsedThisRun(1)`, which is deliberately
    **not** a counter. Balatro's is retroactive — it reads a run-wide statistic —
    so it reads the new `BuffoonBoard.tarots_used` through `builtin_joker_op`
    like any other board reader. A per-joker counter would start it at zero and
    be wrong; `score__fortune_teller_is_retroactive` is the test that would catch
    that.

  Data fixes: **Constellation** Common/$5 → **Uncommon / $6**; **Fortune
  Teller**'s cost $5 → **$6**.

  Still `Blank` with reasons: **Lucky Cat** (needs the Lucky proc, which happens
  *inside* the pure `&self` fold — the gap `on_scored` already characterizes for
  Hiker), **Red Card** (booster packs), **Flash Card** (shop rerolls).

### Phase 4 — Retriggers

- [x] **4a.** Per-card retrigger count landed in `fold_played_cards`: each played
  card at `index` is scored `1 + played_retriggers(index, card)` times, re-running
  its full contribution (so a retriggered Lucky card rolls again). All three pure
  per-card retrigger jokers wired, one exact-value test each (each fails before
  its arm lands):
  - **Hack** — variant `MPip::RetriggerPlayedRanks(1, ['2','3','4','5'])`
    (retrigger each played 2-5); test
    `score__hack_retriggers_played_two_through_five`.
  - **Sock and Buskin** — new const (Uncommon / $6 / 🧦, weight 803,
    `UNCOMMON_JOKERS` 12→13), variant `MPip::RetriggerPlayedFaces(1)` matching
    K/Q/J; test `score__sock_and_buskin_retriggers_played_faces`.
  - **Hanging Chad** — new const (Common / $4 / 🗳, weight 804, `COMMON_JOKERS`
    22→23), variant `MPip::RetriggerFirstPlayed(2)` gated on `index == 0`
    (the positional case — `played_retriggers` now takes the card's index);
    test `score__hanging_chad_retriggers_first_played_card_twice`.

  `ALL_JOKERS` grew 105→107, all three reclassified scoring in `scores_hand`.

- [x] **4b.** Held-card retriggers: **Mime** wired. New `fold_held_cards` loop
  applies each held card's op `1 + held_retriggers()` times (a retriggered Steel
  card gives ×1.5 twice); `held_retriggers` sums `MPip::RetriggerCardsInHand`
  across jokers (card-independent — Mime retriggers all held cards alike). Mime
  reclassified scoring in `scores_hand`; a Steel-held probe board added so the
  reachability guard exercises it. Test `score__mime_retriggers_held_steel_card`
  (held Steel King, 8→12→18; fails before the arm lands).

- [x] **4c.** Round-state retriggers, on one new field: `BuffoonBoard.hands_played`,
  the per-**round** count of hands *completed* (reset by `on_blind_selected` and
  `on_round_end`). It counts hands behind the board rather than the one in front
  of it — the convention every counter joker already follows, since Ice Cream
  scores its full +100 on the first hand and only then decays.
  - **Dusk** — the const already carried `RetriggerPlayedCardsInFinalRound` and
    was classified **non-scoring**, i.e. one silent zero away from the
    Banner/Mystic Summit class; reclassified and wired to a new `is_final_hand()`
    (`hands_played + 1 >= draws.hands_to_play`). It reads the round's *granted*
    allowance rather than a hardcoded four, so Burglar's +3 hands pushes the
    final hand out — pinned by
    `score__dusk_follows_the_hand_allowance_rather_than_a_fixed_count`.
  - **Seltzer** — new const (Uncommon / $6 / 🥤, weight 808), variant
    `RetriggerAllPlayedForHands(1, 10)`. Its counter is hands *completed*, which
    is exactly what makes the tenth hand still retrigger and the joker die
    straight after it; counting hands *started* would silently give nine. It
    joins `is_decayed_to_nothing` as a third instance of the same shape — a
    resource spent at a fixed rate per event, destroyed at 0 — alongside Ice
    Cream's chips and Popcorn's mult.

  *Guard change:* `played_retriggers` now walks jokers with their `joker_state`
  slot, since Seltzer's retrigger reads a counter.

### Phase 6 — Rule modifiers (detection hooks)

- [x] **6a.** `HandRules` seam landed (`buffoon_pile.rs`): a `{straight_distance,
  straight_connectors, flush_len}` struct (default = vanilla Balatro) threaded
  into rule-aware detection variants — `determine_hand_type_with`,
  `has_straight_with`, `has_flush_with` (+ the composed straight-flush / royal /
  flush-five / flush-house). The board derives it once from its jokers
  (`BuffoonBoard::hand_rules`) and passes it into phase-1 hand typing, the
  straight/flush ×mult conditionals (The Order, The Tribe), and Runner's growth
  condition; the no-arg detection methods delegate with `HandRules::default()`,
  so the pure path is unchanged. Two jokers wired, no new consts needed:
  - **Four Fingers** — `MPip::FourFlushAndStraight` (already on the const) →
    `straight_connectors 4→3`, `flush_len 5→4`; test
    `score__four_fingers_makes_four_card_straight_flush` + interaction test
    `score__four_fingers_enables_the_order_on_four_card_straight`.
  - **Shortcut** — new `MPip::GappedStraight` (const flipped from `Blank`) →
    `straight_distance 1→2`; test `score__shortcut_makes_one_gap_straight`.

  Both reclassified scoring in `scores_hand`; two probe boards added (a bare
  four-card straight flush, a one-gap straight) so the reachability guard
  exercises them. Each test fails before the seam threads its rule.

- [x] **6b.** **Pareidolia** wired: new `MPip::AllCardsAreFaces` (const flipped
  from `Blank`); a board face-predicate hook `is_face_card`/`all_cards_are_faces`
  replaces the two inline `matches!(index, K|Q|J)` sites (Scary Face's
  `ChipsPlusPerScoredFace`, Sock and Buskin's `RetriggerPlayedFaces`). Pareidolia
  has no standalone score — it only amplifies those jokers — so it is
  intentionally classified **non-scoring** in `scores_hand` (the reachability
  guard correctly ignores it) and covered instead by two interaction tests:
  `score__pareidolia_makes_every_card_a_face_for_scary_face` (2→5 faces) and
  `score__pareidolia_retriggers_every_card_under_sock_and_buskin`. Both fail
  before the hook lands.

- [x] **6c.** **Smeared** wired: new `MPip::SmearedSuits` + new const (Uncommon /
  $7 / 🖍, weight 806, `UNCOMMON_JOKERS` 13→14, `ALL_JOKERS` 107→108). Extends the
  `HandRules` seam with a `smeared` flag; `count_largest_same_suit_with` counts
  the largest *merged* colour group (Hearts+Diamonds, Spades+Clubs) so a mixed
  red/black hand can flush. Reclassified scoring (changes the base hand type
  standalone); a five-red probe board added. Tests
  `score__smeared_joker_merges_suits_for_flush` (5 red → Flush, and a 4-red hand
  stays High Card) and `score__smeared_enables_the_tribe_on_red_flush`
  (interaction); both fail before the flag threads.

- [x] **6d.** **Splash** wired as a **verified no-op**: new
  `MPip::AllPlayedCardsScore` (const flipped from `Blank`), classified
  non-scoring. This engine's phase-2 fold already scores *every* played card —
  there is no scoring-vs-kicker split — so "all played cards score" is inert, not
  a silent-zero bug (confirmed: no scoring-subset logic anywhere). Characterization
  test `score__splash_is_inert_because_all_played_cards_already_score` asserts a
  Pair hand scores all five card pips (kickers included) and that Splash leaves
  the score unchanged. *(If the engine ever adds a scoring-subset, Splash needs
  real wiring.)*

- [x] **6e.** **Oops! All 6s** wired: new `MPip::DoubleOdds` + new const (Uncommon
  / $4 / 🎲, weight 807, `UNCOMMON_JOKERS` 14→15, `ALL_JOKERS` 108→109). Board
  helper `probability_numerator` returns `2^(Oops count)`; the Lucky roll in
  `fold_played_cards` now wins on outcomes `0..min(numerator, odds)` (was `== 0`,
  i.e. numerator 1 — backward-compatible). Classified non-scoring: it only moves
  on the seeded-RNG path, so the pure-score reachability guard can't (and
  shouldn't) see it. Seeded test `score__oops_all_6s_doubles_lucky_odds_to_certainty`
  — a 1-in-2 Lucky floors on some seeds normally, but with Oops (2-in-2) procs on
  every seed; fails before the numerator doubles.

  **Phase 6 complete.** Not modelled (need their own subsystems, tracked
  elsewhere): Smiley Face / Photograph (face-readers with no const yet — the
  `is_face_card` hook is ready for them); Oops's doubling currently reaches the
  one wired 1-in-N effect (Lucky) — Business Card / Bloodstone / etc. inherit it
  free once they route through `probability_numerator`.

### Phase 7 — Full-deck view  *(Complete)*

- [x] **7a.** Full-deck view built. `BuffoonBoard` gains `full_deck: BuffoonPile`
  (every card the run owns) and `starting_deck_size: usize`, both seeded from the
  deck in `new()`. Test `full_deck__starts_as_the_whole_deck_and_records_its_size`.

  *Design finding:* the EPIC assumed the full deck could be **computed** as
  `deck ∪ in_hand ∪ played ∪ discarded`. It can't — the board conserves no deal
  invariant (nothing draws; `board.played = …` leaves the card in `deck` too, so a
  union double-counts), and there is no discard pile at all. A stored **roster** is
  both correct here and what Balatro actually models: the full deck is a stable
  list that drawing doesn't shrink, while `deck` stays the *undealt remainder*
  (Blue Joker's `ChipsPerDeckCard` keeps reading that). Only deck **mutation**
  (Phase 5) should write `full_deck`.

- [x] **7b.** All three jokers wired, each flipping an existing `Blank` const
  (no new consts, so no new weights/piles):
  - **Steel Joker** (#32) → `MPip::MultTimesPlusPerFullDeckSteel(2)`, via
    `joker_x_mult`. Factor is **additive**, `1 + 0.2×count` — not compounding like
    the neighbouring per-card ×jokers; `score__steel_joker_x_mult_grows_additively_per_full_deck_steel`
    pins ×1.8 at four Steel (mult 15), which ×1.2⁴ (17) would fail.
    `score__steel_joker_counts_the_deck_not_the_hand` guards the roster-vs-location
    split: a held Steel *card* moves phase 3 only.
  - **Stone Joker** (#89) → `MPip::ChipsPerFullDeckStone(25)`, via
    `builtin_joker_op`. Test `score__stone_joker_adds_chips_per_full_deck_stone`.
  - **Erosion** (#81) → `MPip::MultPlusPerMissingDeckCard(4)`, via
    `builtin_joker_op`, scoring `starting_deck_size.saturating_sub(full_deck.len())`.
    Test `score__erosion_adds_mult_per_card_below_starting_deck_size`, including a
    grown deck scoring 0 rather than wrapping.

  All three classified **scoring**; the guard duly caught all three as silent
  zeros until `probe_boards()` gained a worn-deck board (2 Steel + 2 Stone
  enhanced, 3 cards destroyed).

- [x] **7c.** Data fix, the Baron pattern from 0a applied to the two consts this
  phase touched: **Erosion** and **Stone Joker** were tagged `CommonJoker` /
  `value: 5` and adrift from every rarity pile; Balatro has both at **Uncommon /
  $6**. Both re-tagged and added to `UNCOMMON_JOKERS` (size 15→17). Test
  `erosion_and_stone_joker__are_uncommon_dollar_six_and_piled`. `ALL_JOKERS`
  is unchanged at 109 — they were already in it, only adrift from their rarity
  pile. `resell_value` left at 0, matching what the Baron fix did (resell is a
  separate dimension, still 0 across all the adrift consts).

  ~~*Still open:* ~59 defined-but-unpiled consts remain, with rarity/cost/pile
  unreconciled against the wiki.~~ Closed by the 2026-07-16 sweep — see §Data
  fixes.

### Phase 5 — Deck mutation / create / consumables

- [x] **5a.** The deck-mutation seam landed on `BuffoonBoard` — the legitimate
  writer of `full_deck` that Phase 7 deferred to this phase. Three primitives,
  each keeping the roster and the undealt remainder coherent:
  `add_card_to_deck` (the run owns one more, undealt — and deliberately does
  *not* bump `starting_deck_size`, so a grown deck leaves Erosion at 0 rather
  than going negative), `destroy_deck_card(index) -> Option<BuffoonCard>`, and
  `replace_deck_card(index, card)` (the seam every permanent card mutation goes
  through), plus a `full_deck_index_of` lookup. `BuffoonPile` gained `insert`,
  matching its existing `Vec`-passthrough style. Tests:
  `add_card_to_deck__grows_the_roster_and_the_undealt_remainder`,
  `destroy_deck_card__removes_the_undealt_copy_but_tolerates_a_dealt_one`,
  `replace_deck_card__swaps_the_card_in_both_piles_and_keeps_its_slot`, and
  `score__erosion_moves_through_real_deck_mutation` — Erosion now moves through
  real destruction rather than a poked `full_deck`. The Phase 7 tests' private
  `enhance_in_full_deck` helper was rewritten onto `replace_deck_card`, so
  Steel/Stone Joker are exercised through the seam too.

  *Design finding:* a `BuffoonCard` is a `Copy` value type with no identity, so
  locating a card's undealt copy has to be a value match. That is exact, not a
  compromise: two value-equal cards are interchangeable, so removing or
  replacing either leaves the same multiset — and the moment Hiker fattens one
  of a duplicate pair, they stop being value-equal and the match distinguishes
  them again. Roster-only cards (already dealt, played, or held) are tolerated,
  since the board conserves no deal invariant (the 7a finding).

- [x] **5b.** **Hiker** (#56) wired — the phase's only joker that touches a hand
  score. New `MPip::GainChipsOnScored(4)` (const flipped from `Blank`; no new
  const, so no new weight or pile) and a new `on_scored()` lifecycle hook — one
  of the three item 1c already anticipates. Unlike its `Gain*` neighbours it is
  not a counter: nothing accumulates on the joker, the growth lives on the
  *cards*. Every card in `played` gains +4 chips, applied to the played card and
  persisted to its roster copy through 5a's `replace_deck_card`.

  Chips ride on the card's **base rank value** (`add_base_chips`), which is
  orthogonal to `enhancement` — so a Steel card collects Hiker chips and keeps
  its ×1.5 — and leaves rank `weight` untouched, which is what `distance`
  (and therefore straight/flush detection) keys off. Hiker cannot silently
  break hand detection; `on_scored__leaves_rank_weight_alone_so_detection_is_unaffected`
  pins that.

  Classified **scoring** in `scores_hand`, and `is_reachable` now fires
  `on_scored()` — verified the guard genuinely covers Hiker by removing that
  call and watching it report Hiker as a silent zero. Four tests, each failing
  before the const flips: `score__hiker_permanently_adds_chips_to_every_scored_card`
  (5 cards → +20, and stacking on a second scoring, which is what "permanently"
  means), `on_scored__persists_the_chips_onto_the_run_roster`,
  `on_scored__stacks_with_an_enhancement_rather_than_clobbering_it`, and the
  detection guard above. Plus `on_scored__is_inert_without_hiker` for exit
  criterion 2.

  **Known gap, characterized not hidden:** Balatro fires Hiker per scoring
  *trigger*, so a Hack-retriggered card gains +4 twice and scores the second
  trigger already fattened. `on_scored` runs before the pure `&self` fold and
  cannot interleave, so it bumps once per hand. Every board without a retrigger
  joker is exact; the deviation is pinned by
  `on_scored__bumps_once_per_hand_even_when_a_card_is_retriggered` (84/3 where
  Balatro gives 96/3), which will fail the day scoring becomes mutating and the
  gap can be closed properly.

- [x] **5c.** The consumable seam, plus **real slot limits**.

  *Finding:* `consumables` was a **dead field** — declared on the board and
  neither read nor written anywhere, so there was no "use a consumable" path at
  all. That, not counters, was what blocked Constellation and Fortune Teller.
  And `BuffoonPile::new_with_capacity(2)` was never a cap: it is a `Vec`
  reallocation hint that bounds nothing — the very thing `KNOWN_UNWIRED` had
  flagged for Joker Stencil ("Vec capacity is not the game's 5-slot rule").

  So the board gained `joker_slots` (5) and `consumable_slots` (2) as real
  limits, and:
  - `create_consumable(card) -> bool` — refuses when full, which is the
    "(Must have room)" every creator card carries: it does not queue and does not
    evict.
  - `use_consumable(index, targets) -> Option<BuffoonCard>` — a **Planet** levels
    its hand type through the existing `PokerHands::increment`; a **Tarot**
    enhances the roster cards named by `targets` through `BuffoonCard::enhance`
    and 5a's `replace_deck_card`. Either way it fires the `ConsumableUsed` event.
  - `BuffoonBoard::default()` is now hand-written, delegating to `new()`. The
    derived one would have given the slot fields `0` — a board with room for
    nothing, which is a trap rather than a neutral starting point.

  Two jokers wired, both on the seeded-RNG path (`on_scored_with_rng`, the split
  that leaves them inert in the pure `on_scored` the way Lucky is inert in
  `score()`): **Superposition** → `CreateTarotOnAceStraight` (both conditions
  required; it reads the straight through `HandRules`, so Four Fingers and
  Shortcut widen it) and **Vagabond** → `CreateTarotOnLowMoney(4)`.

  Data fixes: **Superposition** cost $5 → **$4**; **Vagabond** Common/$5 →
  **Rare / $8** (1.0.1f moved it, and the wiki's update history reads
  newest-first, which is easy to misread as the threshold reverting to $3 — it is
  $4, confirmed against the infobox, the in-game text, and the source config).

  *Known gap, characterized:* `use_consumable` applies the **card-enhancing**
  tarots. The run-level ones (Death, Judgement, The Hermit, The Wheel of Fortune)
  pass through `enhance` unchanged, so they count as used — correctly, for
  Fortune Teller — while their real effects stay 5e's business. Using one is a
  no-op rather than a wrong effect.

- [x] **5d.** Blind-select creators, on 2c's `on_blind_selected()`:
  - **Marble Joker** — the const already carried
    `AddCardTypeWhenBlindSelected(BCardType::Stone)`; wired via a new
    `mint_card`, which returns `None` for every type but Stone rather than
    minting an arbitrary stand-in. Its Stone card is a new
    `basic::card::STONE_CARD` whose **both pips are blank** (`PipType::Blank`,
    weight 0, value 0) — "no rank and no suit" is the whole of what a Stone card
    is, and borrowing some arbitrary rank would have let it connect straights it
    must not. Worth something today because the roster count behind **Stone
    Joker** is wired: `on_blind_selected__marble_joker_feeds_stone_joker_and_hologram`
    pins +25 chips and Hologram's ×1.25 off one selection.
  - **Riff-Raff** → `CreateJokersWhenBlindSelected(2, BCardType::CommonJoker)`,
    drawing from `Joker::COMMON_JOKERS` in the new
    `on_blind_selected_with_rng`. Room is checked **per joker**, not
    all-or-nothing: a board with one free slot gets one of the two, a full board
    gets none. Data fix: cost $5 → **$6**.

- [ ] **5e.** Blocked on subsystems outside this EPIC, each with its reason:
  **Sixth Sense** (#54) and **Séance** (#66) need **Spectral cards, which do not
  exist** — `BCardType::Spectral` is a bare tag with no deck (EPIC-01 Story 3).
  **Hallucination** (#85) needs booster packs. **Perkeo** (#150) needs the shop
  and Negative editions. **DNA** (#51) needs "first hand of round" state and a
  draw step. All stay `Blank` rather than take a plausible-but-wrong value.

### Phase 8 — Boss blinds

- [x] **8.** Boss blinds, at the **smallest honest** size.

  *Design finding — "identity only" does not work.* The obvious minimum is a
  blind enum and nothing else: enough for Madness (which only asks *not a boss?*)
  and Rocket (*is a boss?*). But it cannot carry the other three, and the reason
  is structural: **Luchador, Chicot and Matador all act on boss *abilities***.
  With no abilities modelled, "disable the current Boss Blind" sets a flag
  nothing reads — a no-op that *looks* wired, which is a worse failure than a
  silent zero, because no guard can see it. (Splash is a genuine no-op: the
  engine really does already score every played card. This would not have been.)

  So the model is identity **plus the boss abilities that are pure `Draws`
  mutations** — new `src/funky/types/blind.rs`, with `Blind::{Small, Big,
  Boss(BossBlind)}` and three bosses that ride 2c's recompute and need no new
  machinery at all:
  - **The Needle** — play only 1 hand;
  - **The Water** — start with 0 discards;
  - **The Manacle** — −1 hand size.

  These pay for themselves as tests: The Needle makes the first hand the *final*
  hand, so **Dusk** always triggers under it (the pairing the wiki calls out),
  and The Water switches **Mystic Summit** on. Both are real interactions, not
  contrivances. The rest of the ~20-boss roster stays out, and the omission is
  the point: The Wall needs a blind score target and the suit-debuff bosses need
  debuffs threaded into scoring — neither exists, and half of either is silently
  wrong scoring.

  The load-bearing distinction is **identity vs ability**:
  - `Blind::is_boss()` — identity. A boss disabled by Luchador or Chicot is still
    a boss, so Madness still refuses to grow on it and Rocket still counts it
    defeated.
  - `BuffoonBoard::boss_ability_active()` — identity **and** not disabled **and**
    no Chicot held.

  Four jokers wired:
  - **Madness** → `GainMultTimesOnNonBossBlindDestroyingJoker(50)`. The halves
    are split across the `_with_rng` seam on purpose, because Balatro splits
    them: the ×0.5 is deterministic and lands in the pure hook (it is granted
    **whether or not anything was destroyed** — coupling the two is the easy bug,
    pinned by `madness_gains_even_with_nothing_to_destroy`), while the random
    destruction lives in `on_blind_selected_with_rng`. It never destroys itself,
    and never fires on a boss. The destruction resolves against the board's
    *original* slots and applies removals once, so a Madness eaten by an earlier
    Madness does not still get a turn.
  - **Luchador** → new const (Uncommon / $5 / 🤼, weight 809),
    `DisableBossBlindOnSell`, on a new `sell_joker`. The disable is *observable*
    because selling recomputes the round's draws and the boss's grip lifts —
    which also, for free, makes selling Juggler or Drunkard take its bonus with
    it, exactly as 2c's recompute-from-baseline design intends. The round's own
    counters are left alone: a sale happens mid-round.
  - **Chicot** → `DisablesAllBossBlinds`, read **live** from `jokers` rather than
    through a flag — it disables by being held, so selling it hands the boss back
    automatically.
  - **Rocket** → `CashOnRoundEndGrowingOnBossDefeat(1, 2)`, the last of Phase 1's
    boss-blocked `+$` jokers. Its increment lands **before** the payout of the
    round that earned it, so the boss round itself pays the raised amount — which
    is why `on_round_end` now grows *then* pays. Nothing else reads a counter to
    pay, so nothing else notices; payouts still precede destruction.

  Data fixes: **Madness** Common/$5 → **Uncommon / $7**; **Rocket** Common/$5 →
  **Uncommon / $6**.

  **`KNOWN_UNWIRED` is now empty, and 0b's last debt is paid.** Its sole
  remaining entry was **Joker Stencil**, whose stated blocker was "needs a real
  joker-slot *limit* on the board (Vec capacity is not the game's 5-slot rule)".
  5c added `joker_slots` for Riff-Raff's "(Must have room)" — which unblocked
  Stencil for free and, more to the point, made its recorded reason **false**.
  A stale reason is the same rot this EPIC keeps finding, so it was wired rather
  than left:

  `MultTimesOnEmptyJokerSlots(1)` → ×1 per empty slot, "Joker Stencil included".
  Verified against the game source rather than the wiki, whose prose explanation
  of this one is self-contradictory (its ×5 result is right; its reasoning is
  not). The rule is `(slots − jokers) + (number of Stencils)` — the "included"
  clause is **+1 per Stencil on the board, not +1 for self**, so it reduces to
  **`slots − (jokers that are not Stencils)`**: alone it is ×5, and two Stencils
  are ×5 *each* (→ ×25), not ×4. The trigger gate is on **literally** empty slots
  while the value uses the inclusive count; the two only disagree on a full board
  holding ≥2 Stencils, where the game displays a factor it does not apply. The
  gate is what is implemented — a full board contributes nothing, never ×0.
  Reading `joker_slots` live rather than a hardcoded 5 is also faithful:
  `score__joker_stencil_reads_the_current_slot_limit` pins it.

  Keep the list, and keep it empty: every joker that *intends* to score now does,
  so a new entry is a bug with a reason attached. Jokers that legitimately do not
  score belong in `scores_hand`, never here.

  **Matador** stays `Blank`, and `matador__stays_blank_because_no_boss_ability_fires_on_a_played_hand`
  characterizes why: it pays "if played hand triggers the Boss Blind ability",
  and every boss modelled here applies its ability once at blind select — none is
  triggered *by a hand*. There is no event to read, and paying on every boss hand
  would be the plausible-but-wrong value this EPIC exists to keep out. (The wiki
  also records that Matador is inconsistent in the real game "due to an
  oversight", without enumerating which bosses fail.) The const is minted anyway
  (Uncommon / $7 / 🐂, weight 811) so the joker exists and is piled.

---

## Remaining `Blank`

Fourteen jokers are still `MPip::Blank`, every one of them **blocked, with a
stated reason** — the EPIC's goal was always "wire it correctly or say why not", and
these are the "why not".

Those reasons now live in the code as **data**, in `BLANK_WITH_REASON`
(`src/funky/decks/joker.rs`), and are checked by
`all_jokers__every_blank_joker_has_a_stated_reason`. It is the third guard, and
the one the other two structurally could not be: `KNOWN_UNWIRED` and
`KNOWN_UNWIRED_CARD_ENHANCEMENTS` catch a card that *intends* to score and does
not, but a `Blank` joker scores nothing **correctly** — so reachability has
nothing to catch it with. Without this guard, "haven't got to it" and "waiting on
spectral cards" are indistinguishable in the source. The guard fails three ways:
a `Blank` joker missing from the list, a listed joker that got wired, and a
reason too thin to be a reason. The table below is the same set, grouped by
blocker:

| Waiting on | Jokers | Notes |
|---|---|---|
| **Spectral cards** (do not exist — `BCardType::Spectral` is a bare tag with no deck) | Sixth Sense, Séance | EPIC-01 Story 3 |
| **Booster packs** | Hallucination, Red Card | Red Card is "+3 mult per pack skipped" |
| **The shop** | Perkeo (also needs Negative editions), Flash Card (rerolls) | |
| **A per-round random target** | To Do List, Mail-In Rebate | 1c's scope cut |
| **Card destruction on discard** | Trading Card | |
| **A probabilistic held-card payout** | Reserved Parking | Deferred, not blocked — it could be wired on the existing seeded-RNG path |
| **A draw step** | DNA | Its other half, "first hand of round", *is* now expressible via 4c's `hands_played` |
| **Tags** (and the shop, for its sell trigger) | Diet Cola | It creates a Double Tag — there is no object to create |
| **In-fold effects** (the pure `&self` scoring fold cannot grow a counter mid-fold) | Lucky Cat | The gap `on_scored` already characterizes for Hiker. Closing it means making scoring mutating — its own phase |
| **Per-hand boss abilities** | Matador | See item 8; the bosses modelled here apply once at blind select |

### The untriaged three — resolved

Closing this EPIC out found three jokers that were `Blank` by **omission** rather
than decision: **Card Sharp**, **Diet Cola** and **Ancient Joker** were in no
phase's scope and mentioned nowhere in this document. They had sat through all
eight phases not because anything blocked them, but because nobody had looked.

Triaging them made the point better than the guard does: **two of the three were
never blocked at all.**

- **Card Sharp** → `MultTimesOnRepeatedHandThisRound(3)`. ×3 mult if the played
  hand type was already played **this round** — new board state
  `hands_by_type_this_round`, the per-type twin of `hands_played`. (Distinct from
  `PokerHands::times_played`, which counts the whole *run* and never resets, so
  it cannot answer the question.) Data fix: Common/$5 → **Uncommon / $6**.

  *The trap, and why the source mattered.* Balatro bumps its per-round tally at
  the **top** of `evaluate_play`, *before* jokers evaluate — hence its `> 1`
  test. This engine's `on_hand_played` fires **after** a hand scores (Ice Cream's
  convention), so the correct test here is `>= 1`. Copying Balatro's comparison
  verbatim would be correct-looking, silently wrong, and would fire Card Sharp on
  the round's *first* hand. Pinned: flipping `>= 1` to `> 1` fails three tests.

- **Ancient Joker** → `MultTimesPerScoredAncientSuit(15)`. ×1.5 per played card
  of the run's current suit, compounding (three Hearts = ×3.375), with the suit
  re-rolled at every round end. Data fix: Common/$5 → **Rare / $8**.

  The suit is **run state** (`ancient_suit`), not a per-joker counter — which is
  *why* two Ancient Jokers agree on it in Balatro: one shared field, not a sync
  rule. `None` until the first roll, which is what makes that roll a 1-in-4 while
  every later one is a 1-in-3 excluding the current, so it can never repeat back
  to back. It honours `HandRules::smeared`, so Smeared widens what it pays for
  exactly as it widens a flush.

  *Deliberate deviation:* the re-roll is **gated on holding one**. Balatro rolls
  every round regardless, but rolling unconditionally would consume RNG on every
  board and shift every other seeded roll downstream (Gros Michel, Cavendish).
  Nothing but Ancient Joker reads the suit, so the gate is unobservable.

- **Diet Cola** stays `Blank`, now with a real reason: it creates a **Double
  Tag**, and **Tags do not exist** — there is no object to create, and Double
  Tag's own effect ("copy the next selected Tag") needs the skip-blind
  tag-selection flow besides. Its `selling_self` trigger also wants the shop.
  Tags first. (Worth noting it is filed under "Food Jokers" on the wiki and is
  **not** one mechanically — no timer, no counter, no round decrement. Modelling
  it as a countdown joker would be wrong.)

**The lesson is the finding.** Two of three needed nothing that did not already
exist; they were invisible purely because no document named them. That is the
gap `BLANK_WITH_REASON` closes — not by tracking work, but by making *"nobody
looked"* impossible to express. `blank_jokers__every_reason_names_a_blocker`
enforces it: a reason has to name what is missing, and "TODO"/"untriaged" are
rejected outright.

## The round loop  *(follow-on; closes EPIC-01 Story 7's last item)*

This EPIC built a complete lifecycle — `on_blind_selected`, `on_scored`,
`on_hand_played`, `on_discard`, `on_round_end` — and **nothing ever ran it as a
life**. Each hook was tested in isolation; the only thing that drove a sequence
was the reachability probe, in a deliberately unrealistic order. Four ordering
rules had been discovered separately and pinned separately:

1. growth before payouts (Rocket's boss increment);
2. payouts before destruction (a joker that dies still pays);
3. the boss ability after every joker draw modifier;
4. `Scored` before the fold, `HandPlayed` after it.

The loop is what proves they compose. `round_loop__the_lifecycle_hooks_compose_in_order`
drives all four on one board in one round, and they do — **the answer was yes**,
but it was not knowable before.

**Two bugs, both found by building it, both fixed:**

- **Banner and Mystic Summit read `draws.discards`** — what the round *granted* —
  where both say "**remaining**". The board's own model is explicit that these
  differ (`discards_used` is documented as "kept separate from `draws`, which
  counts what the round *grants*, not what it has consumed"), but nothing
  subtracted one from the other. So Banner kept paying +30 for discards already
  spent, and Mystic Summit never fired once they were all spent — the exact two
  jokers 2a/2b were written to rescue from silent-zero, wrong again in the other
  direction.

  It was invisible because **the tests faked it**: they set `draws.discards = 0`
  directly rather than discarding, which is only the same thing when nothing has
  been consumed. Nothing had ever driven a real discard. Fixed with
  `discards_remaining()` (granted − used, floored), which Delayed Gratification
  now also reads — equivalent there, but it leaves the trap field with no
  "remaining" readers left to imitate.

  *This is the loop's whole argument, arriving before the loop did.*

- **`BuffoonPile::draw(n)` loses cards.** It pops one at a time and returns
  `None` if it cannot supply the full count — dropping the cards it already
  popped. Ask a 3-card deck for 5 and the deck ends up empty with the 3 gone.
  Sidestepped rather than fixed: `deal_to_hand_size` pops until the hand is full
  or the deck is dry, which is Balatro's behaviour anyway (you are dealt what
  there is). `draw` is left alone as a separate defect — see TECHNICAL_DEBT.

**The deal invariant now holds — inside the loop.** Item 7a found that the board
"conserves no deal invariant" (`board.played = …` left the card in `deck` too),
which is *why* `full_deck` had to be a stored roster rather than a union of the
location piles. The loop is the first thing that can hold the line, and
`round_loop__conserves_every_card_the_run_owns` pins it: through a round of
playing and discarding, `deck + in_hand + discarded == full_deck`, all 52. It
catches both directions — a leak *and* a duplication (dealing without removing).
`full_deck` stays a roster regardless: the invariant holds for boards driven
through the loop, and boards that poke `played` directly are still legal.

**Not modelled, deliberately:** ante progression, so nothing supplies a
`blind_target` — the caller sets it, and `0` means "run until the hands are
spent". The mechanism is here; the ante table is a set of numbers this engine has
no way to check.

## The Stone card  *(follow-on; the last card-level debt)*

The Stone card is +50 chips with **no rank or suit**, and it sat in
`KNOWN_UNWIRED_CARD_ENHANCEMENTS` from 0d onwards for a good reason: wiring the
chips alone would have traded a silent zero for a silently *wrong hand type*.
Both halves are now in, and the entry is gone — **all three guards are empty.**

**The plan was wrong, and the source said so.** The intended fix was "have
`enhance` blank both pips", i.e. model no-rank-no-suit as absent data. Balatro
does the opposite: Stone is a **mask at the accessor layer** over a fully
preserved base. That is observable rather than academic — Vampire strips the
enhancement and the original rank and suit come straight back, which is
impossible if they were erased. So detection filters on the **enhancement**
(`BuffoonCard::is_stone`), never on the pips.

The distinction has a specific bug attached. Blanking the pips makes every Stone
card *identical*, so **two Stone cards pair with each other** — they must never
pair, not with a rank and not with one another. This was live: the `STONE_CARD`
built for Marble Joker in 5d used blank pips, and two Marble Joker blinds put two
of them in the deck.

**A second live bug, self-inflicted.** `Pip::default()` weighs 0 — and **so does
a Deuce**. `connectors` compares raw weights, so a blank-ranked Stone card
connected straights *exactly as if it were a 2*: `3-4-5-6` + Stone read as a
Straight. It was unreachable until the round loop, because nothing could draw a
Stone out of the deck; the loop made the whole chain live (Marble Joker adds to
the deck → the loop deals → you play it). Pinned end-to-end through that path.

**What landed:**
- `BuffoonCard::is_stone` — the enhancement flag the accessors consult.
- `BuffoonCard::get_chips` — a Stone card gives its flat 50, *replacing* the rank
  value rather than adding to it (a Stone Ace is 50, not 61).
- `BuffoonPile::detectable` — the one seam that drops Stones from classification,
  used by `connectors`, `count_largest_same_suit(_with)` and `has_x_of_a_kind`.
  A Stone still consumes a played slot, so the hand is classified from four cards
  rather than five — which is exactly Balatro's behaviour, and means **Four
  Fingers rescues a Stone-shortened straight or flush for free**.
- `STONE_CARD` now carries a real, masked base. *Deliberate divergence:* Balatro
  randomises a created Stone's base and this one is fixed, since
  `on_blind_selected` has no RNG and the base is unobservable while the
  enhancement is on.

**A test was pinning the bug.** `enhance__tarot__tower` asserted a Stone-enhanced
Seven scored **7** — its base rank, with the +50 lost. It encoded the silent zero
rather than catching it, which is why the guard listing was the only thing that
knew. Both are now correct, and the mask-don't-erase model has its own test.

**Also fixed:** `connectors`' doctest fence was never closed — the ``` opened and
ran to the end of the comment, so any prose added below it would have been
compiled as Rust. Closed, and the Stone case is now a doctest.

### The reachability guard, hardened  *(found while wiring Phases 3–8)*

Two latent weaknesses in `all_jokers__intended_hand_scorers_are_reachable`
surfaced only once the new events were driven through it. Both are fixed, and
both are worth knowing about before adding the next probe:

1. **The baseline must be driven too.** The guard took its baseline *before*
   firing any event, then compared against a board with the joker **and** the
   events. That silently assumed every event is score-neutral on its own. Using a
   Planet is not — it levels a hand type, moving phase 1's base for *everyone* —
   so the moment 3c's `use_consumable` joined the probe, **every** joker looked
   reachable and the guard passed vacuously for all 112. Now both sides run the
   identical `drive_events` sequence and only the probed joker differs, so the
   marginal is attributable to it and nothing else. Any future event that moves
   the score is safe by construction.
2. **The resident probe joker must read only the played hand.** Each probe board
   carries one Uncommon joker so Baseball Card has something to count. It was
   **Mystic Summit** (+15 mult at zero discards) — which reads *round state*. As
   soon as Phase 8's probe selected a blind, Burglar and Drunkard moved the score
   *through* it and the guard reported them as misclassified, when both are
   modifiers of exactly Pareidolia's kind. Swapped to **Fibonacci**, which reads
   only `played`. A resident that reads round state turns every draw modifier
   into a false positive.

The guard bit correctly and repeatedly through this work — it caught Vampire as a
silent zero (no probe board *played* an enhanced card, so it had nothing to eat;
fixed with an `enhanced_played` board) and refused to compile until each of the
14 new `MPip` variants was classified.

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

**All eight phases met all three.** Final state: **618 lib tests** (501 at the
start of this EPIC, 525 before Phases 3–8, 586 before the round-loop and
Stone-card follow-ons), clippy pedantic **0 warnings**,
`no_std` clean, `fmt` clean, docs **0 warnings**. Every arm landed in this EPIC
was spot-verified by reverting it and watching its test fail — including the
negative guards, which pass vacuously by design (they assert an *absence*, so
they are regression guards rather than Gold Standard tests, and are called out as
such where they appear).

**One process note worth carrying forward.** Every exact effect wired in Phases
3–8 was checked against the Balatro wiki and, where the wiki was ambiguous, the
game's own localization strings and source config. That was not ceremony — it
changed the outcome four times:

- **Vampire** was believed blocked on the in-fold ordering gap. The wiki settles
  it ("removes Enhancements before their effect occurs"), and that is exactly
  what `on_scored` already did — so it wired cleanly instead of being deferred.
- **Fortune Teller** is retroactive. A `joker_state` counter is the obvious
  implementation and is **silently wrong**; no test anyone would think to write
  catches it, because it only differs on a joker acquired mid-run.
- **Madness**'s ×0.5 is granted whether or not it destroys anything. Coupling the
  two is the natural reading of "gain X0.5 Mult and destroy a random Joker".
- **Joker Stencil**'s own wiki page reasons incorrectly to a correct number; only
  the source settles that "included" means +1 *per Stencil on the board*, which
  is what makes two Stencils ×25 rather than ×16.

None of these would have been caught by the guards, the type system, or review.
They are exactly the class of bug this EPIC exists to prevent, and the only thing
that caught them was reading the source of truth before writing the value.
