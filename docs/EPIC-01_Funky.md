# EPIC-01: Funky — Balatro-Style Cards

> **Status: ✅ Complete (with named deferrals) — closed out 2026-07-18 at tip
> `e50fdd0`.** All five child EPICs are closed: [01a Joker
> Wiring](./EPIC-01a_Joker_Wiring_Backlog.md) (2026-07-16), [01b
> Shop](./EPIC-01b_Shop.md), [01c Vouchers](./EPIC-01c_Vouchers.md), [01d
> Editions](./EPIC-01d_Editions.md) (all 2026-07-17), and [01e Spectral
> Cards](./EPIC-01e_Spectral_Cards.md) (2026-07-18, seal spectrals deferred).
> Both stated goals are structurally met: the solver path (`score()` /
> `score_with_seed()`, four phases, never panics, seeded shuffle) and the
> modding path (the `Effect`/`EffectRegistry` seam). Everything still open below
> is a **named deferral onto a future EPIC** (Seals, Decks, Antes/Bosses, Tags,
> Serde), not in-flight work — see the [Implementation
> corrigendum](#implementation-corrigendum) for the design-vs-actual deltas and
> the deferral register. Verified at close-out: 769 lib + 10 integration tests +
> 101 doctests green, clippy `-Dpedantic --all-targets` clean, no_std build
> clean, doc gate clean, `examples/buffoon.rs` scores 11254.
>
> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Checked boxes reflect work already landed on `origin/funky` as of 2026-07-05 (tip `cc1595d`, the merge of main's 0.7.0 no_std work into funky). See the companion status document [`EPIC-01_Funky_Progress.md`](EPIC-01_Funky_Progress.md) for the quality evaluation behind these checkmarks.
>
> **2026-07-11 hardening pass** (uncommitted on local `funky`): funky is now gated in CI (`cargo test --features funky`; `cargo clippy --features funky --lib --tests` at `-Dpedantic`); debug `println!`s removed from scoring paths; the two `MPip` Display bugs fixed; the 6 funky-lib clippy warnings cleared; **the whole crate made clippy-pedantic-clean at `--all-targets`** (lib, all tests, all examples, benches) — `unwrap`/`expect` allowed under `cfg(test)` in `src/lib.rs`, plus mechanical fixes in core `basic` test code and API-level fixes in the examples (`bridge.rs` pass-by-value/`# Panics`/`# Errors` docs/`render` rename, `range.rs`/`demo.rs`/`poker_eval.rs` cleanups); `Draws`/`Toggle`/`ToggleCard` exported; data-invariant tests added for `decks/basic.rs` and `decks/joker.rs` (+12 tests → 395 green); and `examples/buffoon.rs` rewritten to demonstrate phase-4 joker scoring end-to-end. CI now gates `cargo clippy --features funky --all-targets`. **Deferred by design:** scoring phases 1–3 (`todo!()`), the ~54 silently-zero `MPip` variants, the mod/effect-registry redesign, and the remaining Balatro subsystems.

**Goal:** Model [Balatro](https://www.playbalatro.com/)-style cards — jokers with scoring effects, planet cards that level up poker hands, tarot/spectral consumables, enhanced decks, and chips × mult scoring — well enough to (a) power a **Balatro solver** and (b) enable **dynamic creation of custom Balatro mods**.

**Architecture:** A `std`-only, feature-gated module (`funky = ["std"]`, `#[cfg(feature = "funky")] pub mod funky;` at `src/lib.rs:298`) that layers on top of the crate's core `basic` engine rather than forking it. `BuffoonCard` (`src/funky/types/buffoon_card.rs:76`) embeds core `Pip`s for suit/rank and adds Balatro state: `card_type: BCardType`, `enhancement: MPip`, `resell_value`, `debuffed`. Effects are **data, not code**: a 69-variant `MPip` enum (`src/funky/types/mpip.rs`) attached to each card const, interpreted at scoring time by `calculate_plus_*` match arms. `BuffoonPile` implements the core `Ranged` trait (`buffoon_pile.rs:460`) so combinatorics and hand detection delegate to the mature `basic` engine.

**Tech Stack:** Rust 2024 edition, std-only behind the `funky` feature (deliberately outside the 0.7.0 no_std core), `rand` (seeded shuffle + probabilistic scoring), rstest for case tables.

---

## Domain map (Balatro → funky)

| Balatro concept | funky construct | Status |
|---|---|---|
| Playing card | `BuffoonCard` + `BCardType::Basic` | ✅ done |
| Joker | `BCardType::{Common,Uncommon,Rare,Legendary}Joker` + `MPip` effect | 🟢 112 declared (`ALL_JOKERS`, `joker.rs:2057`), 4 rarity piles assembled; 104 wired with exact-wiki-value tests; 8 stay `Blank` with a test-enforced reason (`BLANK_WITH_REASON`, `joker.rs:2844` — blocked on draw step / seals / tags / per-hand boss abilities) |
| Planet card | `src/funky/decks/planet.rs` (12 cards) + `PokerHands::increment` | ✅ done |
| Tarot card | `src/funky/decks/tarot.rs` (22 Major Arcana) | 🟢 card-enhancing tarots apply + score via `enhance` (tested); run-level ones deferred |
| Spectral card | `src/funky/decks/spectral.rs` (18 cards) + Sixth Sense/Séance creators | 🟢 deck + create-path + 14 of 18 effects done (EPIC-01e Phases 0–3); the four seal spectrals (Talisman, Deja Vu, Trance, Medium) deferred to a Seals EPIC |
| Voucher | `Voucher` enum + `redeem_shop_voucher`; live readers (draws/slots/economy/weights) | 🟡 20 in scope wired (EPIC-01c); edition/ante/pack-content vouchers deferred |
| Booster pack | `PackKind` + `BoosterPack`; `skip_pack`/`open_pack_with_rng` | 🟢 Buffoon/Arcana/Celestial buy/skip/open (EPIC-01b Phase 4; contents-choosing is caller's by design); a Spectral pack kind deferred |
| Enhancements (Glass/Steel/Gold/Stone…) | `MPip` variants + `enhance()` | 🟡 partial |
| Editions (foil/holo/polychrome/negative) | `Edition` enum on `BuffoonCard`; folded into played-card & joker scoring; Negative = live slot exemption | 🟡 model + scoring + slots done (EPIC-01d); edition *sourcing* (shop rolls, frequency vouchers) deferred |
| Seals | — | ❌ absent — deferred to a future Seals EPIC (blocks the 4 seal spectrals, red-seal retriggers, and seal scoring contributions) |
| Decks (Red, Checkered, Abandoned…) | `basic.rs`: Basic 52, Abandoned 40, Checkered | 🟡 3 of ~16 |
| Poker hands + levels | `HandType`/`PokerHand`/`PokerHands` (`hands.rs`) | ✅ done, incl. FiveOfAKind/FlushHouse/FlushFive |
| Chips × mult scoring | `Score` (`score.rs`), 4-phase `BuffoonBoard` scoring + `score()` aggregate | ✅ all 4 phases done (base + played cards + held ×mult + jokers L→R); additive, ×mult, retrigger, and edition contributions all fold in |
| Blinds / antes / boss blinds | `types/blind.rs`: blind state + The Needle / The Water / The Manacle | 🟡 blind state + 3 `Draws`-mutating bosses (EPIC-01a Phase 8); ante progression + the rest of the ~20-boss roster absent |
| Shop / economy | `money`, cash-out, `Shop` (stock + `buy_stock` + rerolls + packs + voucher slot), `sell_joker` | 🟢 cash-out, buying, selling, rerolls, packs, vouchers all done (EPIC-01b + 01c complete); only the edition/ante voucher subset deferred |
| Hand/discard counts | `Draws` (`draws.rs`) | ✅ done, exported via `preludes/funky.rs:18` |
| Card selection | `ToggleCard` (`toggle.rs`, `RefCell<bool>`) | ✅ done, exported via `preludes/funky.rs:25` |

---

## Story 1: Core card model & vocabulary

- [x] `BuffoonCard` struct: core `Pip` suit/rank + `card_type` + `enhancement` + `resell_value` + `debuffed` (`buffoon_card.rs:76`)
- [x] `BCardType` enum: Basic, Stone, 4 joker rarities, Planet, Spectral, Tarot, Voucher (`buffoon_card.rs:14`)
- [x] `MPip` effect enum — 69 data-driven effect descriptors (`mpip.rs`)
- [x] `bcard!` / `bcards!` literal macros (`src/funky/macros.rs`, 638 lines)
- [x] `Score { chips, mult }` with `Add`, `multi_mult(f32)`, `score()` (`score.rs`)
- [x] Remove debug `print!`/`println!` from `get_chips`/`enhance`/`enhance_swap` (`buffoon_card.rs`)
- [x] Fix `MPip` Display bugs ("ChipsOn2Straight" label → `ChipsOnStraight`; stray paren on `MultPlusOnHandPlays`)
- [x] Decide fate of experimental `FIntPip` function-pointer pip (`fpips.rs`) — **superseded and removed.** The `Effect`/`EffectRegistry` seam replaces it (a fn-pointer effect can't be serde/`Eq`/`Hash`; see the [effect-registry design](./2026-07-11-effect-registry-design.md) §2, §5); the file and its module are deleted

## Story 2: Decks

- [x] Basic 52-card Buffoon deck (`decks/basic.rs`)
- [x] Abandoned deck — 40 cards, no face cards (`basic.rs:70`)
- [x] Checkered deck — 52 cards, two suits (`basic.rs:115`)
- [ ] Remaining Balatro decks (Red, Blue, Yellow, Green, Black, Magic, Nebula, Ghost, Erratic, Painted, Anaglyph, Plasma, Zodiac)
- [x] Data-invariant tests for `decks/basic.rs` (deck sizes, all-basic, full-French-52, Abandoned has no face cards, Checkered is two-suits-each-twice)

## Story 3: Consumables — planets, tarot, spectral, vouchers

- [x] 12 planet cards with `ChipsMultPlusOnHand` effects (`decks/planet.rs`)
- [x] Hand leveling: `PokerHands::increment` applies planet upgrades (`hands.rs:106`)
- [x] 22 Major Arcana tarot cards with mapped `MPip` effects (`decks/tarot.rs`)
- [~] Score/apply tarot effects — the **card-enhancing** tarots apply and score
  through `BuffoonCard::enhance` (Glass/Steel/Bonus/Mult/Lucky/Wild/Stone, plus
  Strength's rank bump and the four suit-changers), pinned by
  `funky__decks__tarot_tests`; applied in play via `BuffoonBoard::use_consumable`.
  The **run-level** tarots (Death, Judgement, Hermit, Wheel, Emperor, High
  Priestess, Hanged Man) are no-ops on a card — their effects belong to the
  consumable/economy subsystems
- [~] Spectral cards (18 in Balatro) — **14 of 18 are fully wired** (EPIC-01e
  Phases 0–3, closed 2026-07-18): the deck (`decks/spectral.rs`), the Sixth
  Sense / Séance create-path, the six run-level spectrals (Black Hole, The
  Soul, Wraith, Ectoplasm, Hex, Ankh), and the eight hand/in-hand spectrals
  (Aura, Sigil, Ouija, Immolate, Familiar, Grim, Incantation, Cryptid) via the
  `in_hand` mutation seam. The four seal spectrals (Talisman, Deja Vu, Trance,
  Medium — `spectral.rs:106,144,152,153`) stay `MPip::Blank`, **deferred to a
  future Seals EPIC**.
- [~] Vouchers (32 in Balatro) — **the `Voucher` enum and shop $10 slot landed**
  (EPIC-01c): 20 in-scope vouchers wired at exact wiki values across draws,
  slots, economy, and shop weights, with the base→upgrade prerequisite enforced.
  The edition/ante/pack-content vouchers (12) stay deferred on their subsystems.
- [x] Tests for `decks/planet.rs` (10) and `decks/tarot.rs` (8) — data-invariant
  + effect-application coverage. The planet suite found and fixed a data gap:
  `Planet::DECK` was missing Mercury (the Pair planet), so the base deck now
  covers all nine base poker hands (`DECK_SIZE` 8→9)

## Story 4: Jokers

- [x] 112 joker consts with rarity, cost, and (for 104) a wired `MPip` effect (`ALL_JOKERS`, `decks/joker.rs:2057`)
- [x] `COMMON_JOKERS` pile assembly (22 jokers, `joker.rs:8`)
- [~] Wire effects for the jokers carrying `MPip::Blank` — **all the deterministic pure-board-state ones are now wired** (8): Cavendish (`MultTimes(3)`), Abstract Joker (`MultPlusPerJoker(3)`), Blue Joker (`ChipsPerDeckCard(2)` → +104 fresh deck), Baron (`MultTimesPerHeldRank(15,'K')`, compounds), Scary Face (`ChipsPlusPerScoredFace(30)`), Walkie Talkie (`ChipsMultPlusPerScoredRanks(10,4,['T','4'])`), Blackboard (`MultTimesIfHeldAllSuits(3,['S','C'])`, vacuous-true when hand empty), Baseball Card (`MultTimesPerUncommonJoker(15)`, compounds). Each reads only current board state (`jokers`/`deck`/`in_hand`/`played`), exact wiki values, one test each. **The remaining ~43 were driven out by [`EPIC-01a_Joker_Wiring_Backlog.md`](./EPIC-01a_Joker_Wiring_Backlog.md), closed out 2026-07-16:** its eight subsystem phases (economy, round & hand state, per-run counters, retriggers, deck mutation/consumables, detection-rule hooks, full-deck view, boss blinds) wired 29 more jokers, each with an exact-wiki-value test. **14 stayed `Blank` with a stated, test-enforced reason** (`BLANK_WITH_REASON`, enforced by `all_jokers__every_blank_joker_has_a_stated_reason`), blocked on subsystems outside that EPIC's scope. EPICs 01b–01e then wired six more (Flash Card, Red Card, Hallucination via the shop; Perkeo via editions; Sixth Sense and Séance via spectrals), so **8 remain `Blank` at close-out** (`decks/joker.rs:2844`): DNA (draw step), To Do List / Mail-In Rebate (per-round random targets), Trading Card (destruction on discard), Reserved Parking (deferred, not blocked), Lucky Cat (mutating scoring), Matador (per-hand boss triggers), Diet Cola (Tags). The live silent-zero bug it flagged (Banner/Mystic Summit) is fixed — first by the scoring arms (2a/2b), then again by `discards_remaining()` when the round loop caught both reading granted rather than remaining discards
- [~] Implement jokers 96–150 (mostly still a commented-out catalog). **Done: the 5 Rare jokers** (The Duo/Trio/Family/Order/Tribe, #131–135) and **the 5 Legendary jokers** (#146–150) are declared. **Triboulet is scored** (`MultTimesPerScoredRank(2, ['K','Q'])` = ×2 per played King/Queen, compounding); **Canio, Yorick and Chicot are now wired too** (EPIC-01a Phases 3b/8); **Perkeo is now wired too** (EPIC-01d Phase 4: `CreateNegativeConsumableCopy` at round end) — so **every Legendary joker now scores or acts, and no joker remains `Blank` for want of editions**
- [x] Uncommon/Rare/Legendary pile assemblies — **all done**: `UNCOMMON_JOKERS` (12), `RARE_JOKERS` (5), `LEGENDARY_JOKERS` (5), mirroring `COMMON_JOKERS`, with `pile_uncommon()`/`pile_rare()`/`pile_legendary()`
- [x] Data-invariant tests for `decks/joker.rs` — a shared `assert_rarity_pile` helper checks size / all-jokers / correct-rarity / distinct across all four rarity piles, plus a cross-pile no-duplicate check (44 jokers). Per-card cost checks still open

## Story 5: Hand detection & hand levels

- [x] Full detection ladder `HighCard → FlushFive` incl. Balatro-only FiveOfAKind, FlushHouse, FlushFive (`BuffoonPile::determine_hand_type`, `buffoon_pile.rs:133`)
- [x] 4-card straights via `connectors(distance)` — groundwork for Four Fingers / Shortcut jokers
- [x] Base chips/mult table per Balatro, level and times-played tracking (`hands.rs:58-93`)
- [x] Well tested: 37 tests + 20 rstest cases on `buffoon_pile.rs`
- [x] Resolved the "TODO: HACKY" markers on `determine_hand_type` (documented as
  an ordered strongest-first cascade — that ordering *is* the classification) and
  `has_royal_flush` (rewritten to test for the Ace **and** King anchors, which is
  order-independent and robust; the engine has no Ace-low straights, so a straight
  flush holding both can only be A-K-Q-J-10)
- [x] Joker-modified hand detection — the `HandRules` seam (Four Fingers,
  Shortcut, Smeared suit-merging) landed in EPIC-01a Phase 6

## Story 6: Scoring engine

- [x] Per-card effect scoring: `BuffoonCard::calculate_plus/_chips/_mult` (`buffoon_card.rs:125-168`)
- [x] Per-pile hand-conditional scoring with `funky_num(n, has_flush)` (`buffoon_pile.rs:32-75`)
- [x] Phase 4 joker scoring: `BuffoonBoard::scoring_phase4_joker_scoring(running: Score)` — folds each joker into the running score **left-to-right**, so additive (`+mult`) and multiplicative (`MultTimes(n)` = ×n, `MultTimes1Dot(n)` = ×n/10) jokers compose in the correct order (order-sensitivity is tested both ways). Previously additive-only
- [x] Phase 1 pre-scoring: `BuffoonBoard::scoring_phase1_pre_scoring` — base chips/mult from the played hand's type & level (Royal Flush normalizes to Straight Flush, matching Balatro). Also fixed a `FlushFive` table-entry typo in `hands.rs`
- [x] Phase 2 played-hand scoring: `BuffoonBoard::scoring_phase2_dealt_hand_scoring` — each played card's `get_chips()` (base rank + flat `Chips`) plus per-card `calculate_plus` effects (disjoint `MPip` variants, no double-count)
- [x] Phase 3 held-card effects: `BuffoonBoard::scoring_phase3_effects_in_hand` — applies held ×mult (Steel = `MultTimes1Dot(15)` = ×1.5, `MultTimes(n)` = ×n) to the running score. **All four phases now implemented**; `BuffoonBoard::score()` folds them into one running score in Balatro order (base → cards → held ×mult → jokers L→R) and never panics; 23 board tests total (`board.rs`)
- [~] Handle the `MPip` variants that fall through to `_ => 0`. **Done: the per-card rank family** — `MultPlusOn5Ranks` (Fibonacci +8, Even Steven +4) at both card and pile level, and pile-level summing of `ChipsPlusOn5Ranks` (Odd Todd +31), which was a latent silent bug: its card-level test passed but board scoring goes through the pile, which never summed it, so Odd Todd scored 0 in play. 5 unit tests + an end-to-end `score()` regression test. **Also done:** flat multiplicative jokers (`MultTimes`, `MultTimes1Dot`) and hand-conditional ×mult jokers (5 new `MPip::MultTimesOn{Pair,Trips,4OfAKind,Straight,Flush}` variants + the 5 Rare jokers The Duo/Trio/Family/Order/Tribe), all via the phase-4 running-score fold, using the "contains" predicates. **Also done:** probabilistic effects via a seeded RNG path — `BuffoonBoard::score_with_seed(seed)` / `score_with_rng(rng)` roll Lucky cards (1-in-N → +20 mult) and the Misprint joker (`MultPlusRandomTo(n)` → +random(0..n) mult) deterministically per seed; pure `score()` is the zero-proc floor. **Also done since (EPIC-01a):** the state-dependent variants — economy (Bull's `ChipsPerDollar`), discards remaining (Banner's `ChipsPerRemainingDiscard`, `board.rs:621`), joker-slot counts (Joker Stencil's `MultTimesOnEmptyJokerSlots`, `board.rs:879`) — and Glass destruction (`ChanceDestroyed` rolls, `board.rs:2681`). **Still carried-but-unscored (silent zero) at close-out:** `MultPlusOnHandPlays` (Supernova, `joker.rs:829`) and `MultTimesEveryXHands` (Loyalty Card, `joker.rs:540`) — both sit in the reachability guard's exclusion list (`joker.rs:2593`) though `hands_played` state now exists to wire them against
- [~] Retrigger mechanics — **joker retriggers landed** (EPIC-01a Phase 4):
  `played_retriggers` (`board.rs:408`) re-runs the whole per-card contribution
  for Dusk, Hack (`RetriggerPlayedRanks`), Seltzer (counter-limited), and
  Hanging Chad (position-based). Red-seal retriggers deferred with seals.
- [~] Edition/enhancement/seal contributions to scoring — **editions score**
  (EPIC-01d: Foil/Holo/Polychrome fold into played-card and joker scoring;
  Negative is a live slot exemption) and **enhancements score** (Steel/Glass/
  Bonus/Mult/Lucky/Stone via phases 2–3). Seal contributions deferred with
  seals.

## Story 7: Game state & economy

- [x] `BuffoonBoard`: deck, in_hand, played, consumables (cap 2), jokers (cap 5), `PokerHands` (`board.rs:6`)
- [x] Capacity mechanics: `new_with_capacity`/`capacity`/`free` (`buffoon_pile.rs:23,78,207`)
- [x] `Draws { hands_to_play, discards }` (`draws.rs`)
- [x] `ToggleCard` selection via `RefCell<bool>` (`toggle.rs`)
- [x] **Real slot limits** — `joker_slots` (5) and `consumable_slots` (2) on the
  board. `new_with_capacity` above is a `Vec` reallocation hint and bounds
  nothing; the "(Must have room)" cards need a limit that can be read and
  enforced. (EPIC-01a 5c)
- [~] Blind/ante progression, boss blind effects — **blind state and three boss
  effects** landed (`types/blind.rs`: The Needle, The Water, The Manacle — the
  ones whose ability is a pure `Draws` mutation). Ante progression and the rest
  of the ~20-boss roster (score requirements, card debuffs) are not modelled.
  (EPIC-01a Phase 8)
- [x] Shop: buying, selling (consume `resell_value`), rerolls, packs —
  **`sell_joker` landed** (it pays out `resell_value` and recomputes the round's
  draws, which is what makes Luchador observable). **The shop and buying landed
  too** (EPIC-01b Phase 2): `Shop { stock, rerolls_used }` on the board,
  `open_shop_with_rng` draws two card slots at Balatro's 20/4/4 · 70/25/5
  weights from the rarity piles, and `buy_stock` routes a joker to `push_joker`
  or a consumable to `create_consumable`, charging its price (joker `rank.value`,
  consumable $3) against a debt floor that reads held Credit Cards live.
  **Rerolls landed too** (EPIC-01b Phase 3): `reroll_cost` ($5 climbing $1,
  reset per shop, free rerolls from Chaos the Clown) and `reroll_with_rng`,
  which fires `GrowthEvent::ShopRerolled` — the event Flash Card
  (`MultPlusPerReroll(2)`) counts. **Booster packs landed** (EPIC-01b Phase 4):
  `PackKind`/`BoosterPack`, `skip_pack` (fires `PackSkipped` → Red Card
  `MultPlusPerPackSkipped(3)`), and `open_pack_with_rng` (pays $4, returns the
  pack's choices, and rolls Hallucination's `CreateTarotOnPackOpen(1, 2)`
  inline). **EPIC-01b is complete** — cash-out, stock/buying, reroll, and packs.
  **Vouchers landed too** (EPIC-01c, complete): the `Voucher` enum + $10 shop
  slot + `redeem_shop_voucher`, and 20 in-scope vouchers wired across draws,
  slots, economy (incl. the interest-cap unification), and shop weights. The
  shop is now whole bar the deferred edition/ante vouchers. (EPIC-01a Phase 8,
  EPIC-01b Phases 2–4, EPIC-01c)
- [x] **Money/economy state on the board** — `money: isize` plus the round-end /
  discard payout seam. (EPIC-01a Phase 1)
- [x] **Cash-out** — a won round pays Balatro's three lines: blind reward
  ($3/$4/$5 Small/Big/Boss) + $1 per unused hand + interest ($1 per $5 held,
  capped at $5). Gated on `round_is_won()`, so it is opt-in through
  `blind_target` and every untargeted board pays exactly what it always did.
  Its delta is computed from the pre-cash-out balance and applied after the `+$`
  payouts, so its interest and To the Moon's `ExtraInterest` cannot compound off
  each other. (EPIC-01b Phase 1)
- [x] **Round loop: play/discard cycle consuming `Draws`** — `deal_to_hand_size`,
  `play_hand` / `play_hand_with_rng`, `discard_cards`, `hands_remaining`,
  `round_is_over` / `round_is_won`, plus the `discarded` pile the board never
  had and `round_score` / `blind_target`. See the EPIC-01a §Round loop note for
  the two bugs it surfaced. Ante progression still supplies no target, so
  `blind_target` is set by the caller (0 = run until the hands are spent).

## Story 8: Modding & solver enablement (the stated end-goals)

- [~] Open effect interpretation — **the extension seam is in.** `MPip::Custom(u32)` (stays `Copy`/const/serde) + an `Effect` trait, `ScoringContext`, `ScoreOp`, and an `EffectRegistry` (`src/funky/types/effect.rs`); `BuffoonBoard::score_with_registry` resolves custom effects on **played cards, held cards, and jokers** — every phase they can occur — without editing any core match arm (9 tests + worked examples). The three phase folds are unified (`fold_played_cards`/`fold_held_cards`/`fold_jokers`), and phase 2 now takes a running score so a custom played ×mult composes correctly. Built-in scoring is unified onto `ScoreOp` (each fold applies one op per item via `builtin_*_op`/`custom_op`), so built-in and custom effects share one application path. Design + migration path in [`2026-07-11-effect-registry-design.md`](./2026-07-11-effect-registry-design.md). **Remaining migration items are all closed** except optional serde-stable string ids for mods.
- [x] Full `Score` pipeline a solver can call without panicking — `BuffoonBoard::score()` (deterministic floor) and `score_with_seed(seed)` (rolls probabilistic effects) run all four phases without panicking; the state-dependent `MPip` variants landed with the round/economy state EPIC-01a put on the board. (Two carried-but-unscored variants remain as data gaps, not pipeline gaps — see Story 6 and the corrigendum)
- [x] Deterministic/seedable shuffle for solver reproducibility — `BuffoonPile::{shuffle_with_seed, shuffled_with_seed, shuffle_with_rng, shuffled_with_rng}` mirror the core `basic` API (`StdRng::seed_from_u64`); 3 determinism tests + a doctest. A solver deals reproducibly via `Deck::basic_buffoon_pile().shuffled_with_seed(seed)`
- [ ] Serde on funky types (core decks got serde in 0.6.x; funky types have none)
- [x] End-to-end example: `examples/buffoon.rs` now deals a board, plays a hand, detects the hand type, and demonstrates phase-4 joker scoring (180 chips × 22 mult) end-to-end

## Story 9: API surface & integration hygiene

- [x] Feature gating: `funky` off by default, requires `std` (`Cargo.toml:32`)
- [x] Prelude: `src/preludes/funky.rs` exports decks, types, `MPip::*`, macros
- [x] Export `Draws` and `ToggleCard`/`Toggle` — now `pub mod draws;` and re-exported from `src/preludes/funky.rs`, so `BuffoonBoard::new(draws: Draws, …)` is callable from outside the crate
- [x] Add funky to CI: the test matrix now runs `cargo test --features funky`, and the clippy job runs `cargo clippy --features funky --lib -- -Dclippy::all -Dclippy::pedantic`
- [x] Fix 6 default-level clippy warnings in funky lib code (unwrap on Option, collapsible ifs, `sort_by_key`, let-if-seq) — funky lib is now clean at `-Dclippy::pedantic`
- [x] Remove or use the `phf` dependency — **removed.** It was the wrong tool for the built-in dispatch (compile-time hash vs. a runtime data-carrying enum; a `match` is already a faster, exhaustive jump table). Built-ins were unified onto `ScoreOp` instead; `phf` + the `dep:phf` feature are gone
- [x] Replace journal-style doc comments with API reference docs. The profanity/
  "STORY TIME" is gone (it lived only in the now-deleted `fpips.rs`). The `DIARY`
  notes in `buffoon_card.rs`, `buffoon_pile.rs`, `joker.rs` are **kept by author
  preference** — each now carries an appended **UPDATE** paragraph with the clean
  API reference doc, so the journal and the reference coexist
- [x] CHANGELOG entries for the funky feature (under `[Unreleased] → Added`)

---

## Verification matrix

- [x] `cargo test --features funky` — full battery (**769** lib + 10 integration
  tests + 101 doctests green at close-out, 2026-07-18 tip `e50fdd0`; up from 395
  at the 2026-07-11 hardening pass)
- [x] `cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic` — **the entire crate is clean** (lib, all tests, all examples, benches) and gated in CI (`unwrap`/`expect` in tests allowed via a `cfg(test)` attribute in `src/lib.rs`)
- [x] `cargo build --no-default-features` — green; `--examples` also green (buffoon correctly gated behind `required-features = ["funky"]`)
- [x] `cargo run --example buffoon --features funky` — demonstrates the full four-phase pipeline: base (100×8) + cards (+51 chips) + held Steel (×1.5 → 12 mult) + jokers (180×22) → `score()` 331×34 = **11254**
- [x] `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings` — clean

## Gotchas

- **All four scoring phases are implemented; `BuffoonBoard::score()` never panics** and folds every phase into one running score, so joker order (and additive-vs-×mult) is honored. A handful of `MPip` variants still fall through to `_ => 0`, so a "wired" joker can silently score nothing — at close-out the known two are `MultPlusOnHandPlays` (Supernova) and `MultTimesEveryXHands` (Loyalty Card); `ChipsPerRemainingDiscard` and the hand-conditional ×mult jokers, formerly listed here, have since been wired. **Effects are scored through the *pile* (`BuffoonPile::calculate_plus`), not just the card — a per-card variant handled only in `BuffoonCard` still scores 0 in play until the pile sums it** (this bit Odd Todd). When implementing a variant, add a test at the pile/board level proving it scores.
- **Silent zero-scoring:** unhandled `MPip` variants fall through to `_ => 0`, so a joker can be "wired" yet contribute nothing (e.g. `MultPlusOn5Ranks`). When implementing a variant, add a test proving it scores.
- **`RefCell` in `ToggleCard`** makes it non-`Sync` — fine for a single-threaded solver loop, a constraint for parallel search.
- **funky is std-only by design** — never import funky types into `basic` modules or the no_std discipline breaks.
- **`beggar my neighbor`** commits seen in history are not part of funky at the current tip — don't go looking for them.

---

## Implementation corrigendum

*Closed out 2026-07-18 at `funky` tip `e50fdd0` (in sync with `origin/funky`,
clean tree). All five verification gates re-run and green at close-out: 769 lib
+ 10 integration tests + 101 doctests; clippy `-Dclippy::all -Dclippy::pedantic
--all-targets` clean; `cargo build --no-default-features` clean; `cargo doc`
with `-D warnings` clean; `examples/buffoon.rs` scores 11254.*

### Design-vs-actual deltas

1. **The work shipped as a five-EPIC chain, not one document's checkboxes.**
   This EPIC's Stories stated the *what*; the *how* was peeled off into
   [01a](./EPIC-01a_Joker_Wiring_Backlog.md) (joker subsystems, closed
   2026-07-16), [01b](./EPIC-01b_Shop.md) (shop), [01c](./EPIC-01c_Vouchers.md)
   (vouchers), [01d](./EPIC-01d_Editions.md) (editions, all closed 2026-07-17),
   and [01e](./EPIC-01e_Spectral_Cards.md) (spectrals, closed 2026-07-18). Each
   child carries its own corrigendum; this table reconciles the parent.
2. **"Wire every joker" became "wire or state why not."** The original Story 4
   goal was effects for all Blanks; the landed design is stronger: 104 of 112
   declared jokers score with exact-wiki-value tests, and the 8 that cannot yet
   (`BLANK_WITH_REASON`, `joker.rs:2844`) each carry a test-enforced reason
   naming the missing subsystem — so "not done yet" and "waiting on Tags" are
   distinguishable in source, and a wired joker can never silently join the list.
3. **The mod-extensibility goal landed as `Effect`/`EffectRegistry`, not `phf`.**
   The function-pointer pip (`fpips.rs`) and the `phf` dependency were both
   removed; built-ins unified onto `ScoreOp` and customs ride
   `MPip::Custom(u32)` + `score_with_registry` (`types/effect.rs`).
4. **The solver goal landed as a deterministic floor + seeded ceiling.**
   `score()` never rolls; `score_with_seed`/`score_with_rng` roll Lucky,
   Misprint, Glass destruction, and spectral creation reproducibly. Retriggers
   (`played_retriggers`, `board.rs:408`) re-run the whole per-card contribution,
   so a retriggered Lucky card re-rolls — matching Balatro.
5. **Editions folded into the existing `ScoreOp` scoring rather than a new
   phase** (01d), and Negative became a live slot exemption rather than a slot
   mutation — which is what let Perkeo (`CreateNegativeConsumableCopy`) be the
   last Legendary wired.
6. **Two carried-but-unscored variants survive close-out** (silent zero by the
   Gotcha's definition): `MultPlusOnHandPlays` (Supernova, `joker.rs:829`) and
   `MultTimesEveryXHands` (Loyalty Card, `joker.rs:540`). Both sit in the
   reachability guard's exclusion list; the `hands_played` counter they need
   now exists (`board.rs:120`). They are the natural first items of any
   follow-on sweep.

### Story status summary

| Story | Status |
|---|---|
| 1 — Core card model & vocabulary | **Complete** |
| 2 — Decks | **Complete for scope** — 3 decks; the other 13 Balatro decks **Deferred** (future Decks EPIC) |
| 3 — Consumables | **Complete for scope** — planets ✅, card-enhancing tarots ✅, 14/18 spectrals ✅, 20/32 vouchers ✅; run-level tarots, seal spectrals, edition/ante vouchers **Deferred** |
| 4 — Jokers | **Complete for scope** — 104/112 wired; 8 `Blank` with test-enforced reasons **Deferred** onto their subsystems |
| 5 — Hand detection & levels | **Complete** — incl. the `HandRules` seam (Four Fingers / Shortcut / Smeared) |
| 6 — Scoring engine | **Complete for scope** — all 4 phases, ×mult composition, retriggers, editions, seeded probabilistics; red-seal retriggers + 2 silent variants **Deferred** |
| 7 — Game state & economy | **Complete for scope** — round loop, cash-out, shop, vouchers, 3 boss blinds; ante progression + full boss roster **Deferred** (future Antes/Bosses EPIC) |
| 8 — Modding & solver enablement | **Complete for scope** — registry seam + non-panicking pipeline + seeded shuffle + real example; serde on funky types **Deferred** (future Serde EPIC) |
| 9 — API surface & hygiene | **Complete** — CI-gated at `-Dpedantic --all-targets`, prelude exports, CHANGELOG |

### Inherited debt (the deferral register)

Named here so nothing silently rots; each is a future EPIC, not a loose end:

- **Seals** — the last ❌ subsystem: 4 seal spectrals, red-seal retriggers,
  seal scoring, and Talisman/Deja Vu/Trance/Medium (`spectral.rs:106-153`).
- **Antes & boss blinds** — ante progression (`blind_target` is caller-set),
  ~17 more bosses, per-hand boss triggers (unblocks Matador).
- **Decks** — 13 remaining Balatro decks beyond Basic/Abandoned/Checkered.
- **Tags** — unblocks Diet Cola.
- **Draw step / mutation hooks** — unblocks DNA, Trading Card, Lucky Cat
  (mutating scoring), To Do List / Mail-In Rebate (per-round random targets).
- **Serde on funky types** + optional serde-stable string ids for mod effects.
- **Edition sourcing** — shop edition rolls and the edition/ante/pack-content
  voucher subset (12 vouchers).
- **Spectral booster pack** — EPIC-01b's deferred fourth `PackKind`; per
  EPIC-01e's Dependencies it "now has cards to draw."
- **Supernova & Loyalty Card** — the two silent-zero variants (delta 6).
