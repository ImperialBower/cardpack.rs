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
| 3 — Per-run joker counters | Green Joker, Vampire, Constellation, Hologram, Lucky Cat, Ramen, Popcorn, Square Joker, Spare Trousers, Red Card, Fortune Teller, Flash Card, Runner | **In progress** — store + hand-played/discard events; Green Joker, Ramen, Ice Cream, Square Joker, Spare Trousers, Runner wired |
| 4 — Retriggers | Hack, Mime, Dusk, Sock and Buskin, Seltzer, Hanging Chad | **In progress** — retrigger loops in `fold_played_cards` (played) + `fold_held_cards` (held); **Hack**, **Sock and Buskin**, **Hanging Chad**, **Mime** wired; only round-state ones (Dusk final-round, Seltzer 10-hand counter) remain |
| 5 — Deck mutation / create / consumables | DNA, Séance, Superposition, Riff-Raff, Vagabond, Sixth Sense, Hallucination, Marble Joker, Hiker, Perkeo | **In progress** — mutation seam (`add_card_to_deck` / `destroy_deck_card` / `replace_deck_card`) + `on_scored`; **Hiker** wired (the phase's only scoring joker). The rest need consumables, packs, blinds, or the shop |
| 6 — Rule modifiers (detection hooks) | Pareidolia, Splash, Shortcut, Four Fingers, Smeared, Oops! All 6s | **Complete** — `HandRules` seam (straight/flush/smeared), face-predicate hook, and RNG odds-numerator; all six wired (Four Fingers, Shortcut, Pareidolia, Smeared, Splash [no-op], Oops! All 6s) |
| 7 — Full-deck view | Steel Joker, Stone Joker, Erosion | **Complete** — `full_deck` roster + `starting_deck_size` on the board; all three wired |
| 8 — Boss blinds | Madness, Luchador, Matador, Chicot | Planned |
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
  see 7c. The remaining ~59 defined-but-unpiled consts still want one reconciling
  sweep across rarity/cost/pile.
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
- **Cavendish** (#33) is missing its **1-in-1000 destroy chance** (`MultTimes(3)`
  carries only the mult) — the mirror image of the Gros Michel bug fixed in 0c.
  Latent, not live: scoring is correct today and nothing drives destruction yet.
  Give it `MultTimesChanceDestroyed` when the round-end hook lands, rather than
  minting a compound variant now for a system that does not exist.
- **Hiker** (#56) is tagged `CommonJoker` / `value: 5` and sits in no rarity pile.
  Balatro is believed to have it at **Uncommon / $5** — unverified against the
  wiki, so deliberately *not* fixed on a guess (the in-repo catalog at
  `joker.rs` only covers #96–150). Fold into the reconciling sweep above.

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
- [ ] **3b.** Growth hooks per trigger event; **read**-side `MPip` variants +
  `joker_x_mult`/`builtin_joker_op` arms.
- [ ] **3c.** Wire the 13 counter jokers + Canio/Yorick; one test each.

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

- [~] **4b.** Held-card retriggers: **Mime** wired. New `fold_held_cards` loop
  applies each held card's op `1 + held_retriggers()` times (a retriggered Steel
  card gives ×1.5 twice); `held_retriggers` sums `MPip::RetriggerCardsInHand`
  across jokers (card-independent — Mime retriggers all held cards alike). Mime
  reclassified scoring in `scores_hand`; a Steel-held probe board added so the
  reachability guard exercises it. Test `score__mime_retriggers_held_steel_card`
  (held Steel King, 8→12→18; fails before the arm lands).

- [ ] **4c.** Round-state retriggers, still blocked: **Dusk** (final-round — needs
  Phase 2 "final hand" state), **Seltzer** (retrigger all played for 10 hands —
  needs a Phase 3 per-run counter + round-end decrement).

### Phase 6 — Rule modifiers (detection hooks)

- [~] **6a.** `HandRules` seam landed (`buffoon_pile.rs`): a `{straight_distance,
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

- [~] **6b.** **Pareidolia** wired: new `MPip::AllCardsAreFaces` (const flipped
  from `Blank`); a board face-predicate hook `is_face_card`/`all_cards_are_faces`
  replaces the two inline `matches!(index, K|Q|J)` sites (Scary Face's
  `ChipsPlusPerScoredFace`, Sock and Buskin's `RetriggerPlayedFaces`). Pareidolia
  has no standalone score — it only amplifies those jokers — so it is
  intentionally classified **non-scoring** in `scores_hand` (the reachability
  guard correctly ignores it) and covered instead by two interaction tests:
  `score__pareidolia_makes_every_card_a_face_for_scary_face` (2→5 faces) and
  `score__pareidolia_retriggers_every_card_under_sock_and_buskin`. Both fail
  before the hook lands.

- [~] **6c.** **Smeared** wired: new `MPip::SmearedSuits` + new const (Uncommon /
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

  *Still open:* ~59 defined-but-unpiled consts remain, with rarity/cost/pile
  unreconciled against the wiki. Worth one sweep rather than piecemeal fixes;
  logged under Data fixes.

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

- [ ] **5c.** Consumable create: a `create_consumable` path honouring the cap-2
  slot, then **Superposition** (#59, Tarot on an Ace + Straight) and **Vagabond**
  (#71, Tarot on a hand played with ≤$4 — money already exists from Phase 1).
  Both draw from the existing 22-card `MajorArcana::DECK` via the seeded-RNG path.

- [ ] **5d.** Blind-select creators, shared with 2c/Phase 8's `on_blind_selected()`:
  **Marble Joker** (#24, add a Stone card to the deck — 5a's `add_card_to_deck`)
  and **Riff-Raff** (#67, create 2 Common Jokers).

- [ ] **5e.** Blocked on subsystems outside this EPIC, each with its reason:
  **Sixth Sense** (#54) and **Séance** (#66) need **Spectral cards, which do not
  exist** — `BCardType::Spectral` is a bare tag with no deck (EPIC-01 Story 3).
  **Hallucination** (#85) needs booster packs. **Perkeo** (#150) needs the shop
  and Negative editions. **DNA** (#51) needs "first hand of round" state and a
  draw step. All stay `Blank` rather than take a plausible-but-wrong value.

### Phase 8

- [ ] **8.** Boss blinds — a self-contained sub-EPIC; see Design. Wire jokers as
  the mechanism lands.

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
