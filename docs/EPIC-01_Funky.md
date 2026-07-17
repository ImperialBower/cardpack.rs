# EPIC-01: Funky — Balatro-Style Cards

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
| Joker | `BCardType::{Common,Uncommon,Rare,Legendary}Joker` + `MPip` effect | 🟡 ~105/150 declared (incl. 5 Rare + 5 Legendary), 4 rarity piles assembled, ~34 effect kinds scored (hand-conditional +/×mult, per-scored-rank/face, per-held-rank, board-count, held-suit, seeded probabilistic); the rest need subsystems (economy/counters/retriggers/mutation/blinds) |
| Planet card | `src/funky/decks/planet.rs` (12 cards) + `PokerHands::increment` | ✅ done |
| Tarot card | `src/funky/decks/tarot.rs` (22 Major Arcana) | 🟡 cards + effects declared, mostly unscored |
| Spectral card | `BCardType::Spectral` tag only | ❌ no cards |
| Voucher | `BCardType::Voucher` tag only | ❌ no cards |
| Enhancements (Glass/Steel/Gold/Stone…) | `MPip` variants + `enhance()` | 🟡 partial |
| Editions (foil/holo/polychrome/negative) | — | ❌ absent |
| Seals | — | ❌ absent |
| Decks (Red, Checkered, Abandoned…) | `basic.rs`: Basic 52, Abandoned 40, Checkered | 🟡 3 of ~16 |
| Poker hands + levels | `HandType`/`PokerHand`/`PokerHands` (`hands.rs`) | ✅ done, incl. FiveOfAKind/FlushHouse/FlushFive |
| Chips × mult scoring | `Score` (`score.rs`), 4-phase `BuffoonBoard` scoring + `score()` aggregate | 🟢 all 4 phases done (base + played cards + held ×mult + jokers); jokers still additive-only |
| Blinds / antes / boss blinds | `MPip::AddCardTypeWhenBlindSelected` stub | ❌ absent |
| Shop / economy | `money`, cash-out, `Shop` (stock draw + `buy_stock`), `sell_joker`, `resell_value` | 🟡 cash-out + buying + selling done; rerolls/packs/vouchers open (EPIC-01b) |
| Hand/discard counts | `Draws` (`draws.rs`) | ✅ done (not exported) |
| Card selection | `ToggleCard` (`toggle.rs`, `RefCell<bool>`) | ✅ done (not exported) |

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
- [ ] Score/apply tarot effects in the scoring engine (declared but mostly unhandled)
- [ ] Spectral cards (18 in Balatro) — nothing beyond the `BCardType::Spectral` tag
- [ ] Vouchers (32 in Balatro) — nothing beyond the tag
- [ ] Tests for `decks/planet.rs` (2) and `decks/tarot.rs` (0)

## Story 4: Jokers

- [x] ~95 joker consts with rarity, cost, and (for ~43) a wired `MPip` effect (`decks/joker.rs`, 1,459 lines)
- [x] `COMMON_JOKERS` pile assembly (22 jokers, `joker.rs:8`)
- [~] Wire effects for the jokers carrying `MPip::Blank` — **all the deterministic pure-board-state ones are now wired** (8): Cavendish (`MultTimes(3)`), Abstract Joker (`MultPlusPerJoker(3)`), Blue Joker (`ChipsPerDeckCard(2)` → +104 fresh deck), Baron (`MultTimesPerHeldRank(15,'K')`, compounds), Scary Face (`ChipsPlusPerScoredFace(30)`), Walkie Talkie (`ChipsMultPlusPerScoredRanks(10,4,['T','4'])`), Blackboard (`MultTimesIfHeldAllSuits(3,['S','C'])`, vacuous-true when hand empty), Baseball Card (`MultTimesPerUncommonJoker(15)`, compounds). Each reads only current board state (`jokers`/`deck`/`in_hand`/`played`), exact wiki values, one test each. **The remaining ~43 were driven out by [`EPIC-01a_Joker_Wiring_Backlog.md`](./EPIC-01a_Joker_Wiring_Backlog.md), closed out 2026-07-16:** its eight subsystem phases (economy, round & hand state, per-run counters, retriggers, deck mutation/consumables, detection-rule hooks, full-deck view, boss blinds) wired 29 more jokers, each with an exact-wiki-value test. **14 stay `Blank` with a stated, test-enforced reason** (`BLANK_WITH_REASON`, `decks/joker.rs` — enforced by `all_jokers__every_blank_joker_has_a_stated_reason`), blocked on subsystems outside that EPIC's scope: spectral cards, booster packs, the shop, tags, a draw step, in-fold effects, per-hand boss abilities. The live silent-zero bug it flagged (Banner/Mystic Summit) is fixed — first by the scoring arms (2a/2b), then again by `discards_remaining()` when the round loop caught both reading granted rather than remaining discards
- [~] Implement jokers 96–150 (mostly still a commented-out catalog). **Done: the 5 Rare jokers** (The Duo/Trio/Family/Order/Tribe, #131–135) and **the 5 Legendary jokers** (#146–150) are declared. **Triboulet is scored** (`MultTimesPerScoredRank(2, ['K','Q'])` = ×2 per played King/Queen, compounding); **Canio, Yorick and Chicot are now wired too** (EPIC-01a Phases 3b/8); only Perkeo stays `Blank` (needs the shop and Negative editions)
- [x] Uncommon/Rare/Legendary pile assemblies — **all done**: `UNCOMMON_JOKERS` (12), `RARE_JOKERS` (5), `LEGENDARY_JOKERS` (5), mirroring `COMMON_JOKERS`, with `pile_uncommon()`/`pile_rare()`/`pile_legendary()`
- [x] Data-invariant tests for `decks/joker.rs` — a shared `assert_rarity_pile` helper checks size / all-jokers / correct-rarity / distinct across all four rarity piles, plus a cross-pile no-duplicate check (44 jokers). Per-card cost checks still open

## Story 5: Hand detection & hand levels

- [x] Full detection ladder `HighCard → FlushFive` incl. Balatro-only FiveOfAKind, FlushHouse, FlushFive (`BuffoonPile::determine_hand_type`, `buffoon_pile.rs:133`)
- [x] 4-card straights via `connectors(distance)` — groundwork for Four Fingers / Shortcut jokers
- [x] Base chips/mult table per Balatro, level and times-played tracking (`hands.rs:58-93`)
- [x] Well tested: 37 tests + 20 rstest cases on `buffoon_pile.rs`
- [ ] Resolve the "TODO: HACKY" markers on `determine_hand_type` (`buffoon_pile.rs:131`) and `has_royal_flush` (`:297`)
- [ ] Joker-modified hand detection (e.g. Smeared Joker suit merging)

## Story 6: Scoring engine

- [x] Per-card effect scoring: `BuffoonCard::calculate_plus/_chips/_mult` (`buffoon_card.rs:125-168`)
- [x] Per-pile hand-conditional scoring with `funky_num(n, has_flush)` (`buffoon_pile.rs:32-75`)
- [x] Phase 4 joker scoring: `BuffoonBoard::scoring_phase4_joker_scoring(running: Score)` — folds each joker into the running score **left-to-right**, so additive (`+mult`) and multiplicative (`MultTimes(n)` = ×n, `MultTimes1Dot(n)` = ×n/10) jokers compose in the correct order (order-sensitivity is tested both ways). Previously additive-only
- [x] Phase 1 pre-scoring: `BuffoonBoard::scoring_phase1_pre_scoring` — base chips/mult from the played hand's type & level (Royal Flush normalizes to Straight Flush, matching Balatro). Also fixed a `FlushFive` table-entry typo in `hands.rs`
- [x] Phase 2 played-hand scoring: `BuffoonBoard::scoring_phase2_dealt_hand_scoring` — each played card's `get_chips()` (base rank + flat `Chips`) plus per-card `calculate_plus` effects (disjoint `MPip` variants, no double-count)
- [x] Phase 3 held-card effects: `BuffoonBoard::scoring_phase3_effects_in_hand` — applies held ×mult (Steel = `MultTimes1Dot(15)` = ×1.5, `MultTimes(n)` = ×n) to the running score. **All four phases now implemented**; `BuffoonBoard::score()` folds them into one running score in Balatro order (base → cards → held ×mult → jokers L→R) and never panics; 23 board tests total (`board.rs`)
- [~] Handle the `MPip` variants that fall through to `_ => 0`. **Done: the per-card rank family** — `MultPlusOn5Ranks` (Fibonacci +8, Even Steven +4) at both card and pile level, and pile-level summing of `ChipsPlusOn5Ranks` (Odd Todd +31), which was a latent silent bug: its card-level test passed but board scoring goes through the pile, which never summed it, so Odd Todd scored 0 in play. 5 unit tests + an end-to-end `score()` regression test. **Also done:** flat multiplicative jokers (`MultTimes`, `MultTimes1Dot`) and hand-conditional ×mult jokers (5 new `MPip::MultTimesOn{Pair,Trips,4OfAKind,Straight,Flush}` variants + the 5 Rare jokers The Duo/Trio/Family/Order/Tribe), all via the phase-4 running-score fold, using the "contains" predicates. **Also done:** probabilistic effects via a seeded RNG path — `BuffoonBoard::score_with_seed(seed)` / `score_with_rng(rng)` roll Lucky cards (1-in-N → +20 mult) and the Misprint joker (`MultPlusRandomTo(n)` → +random(0..n) mult) deterministically per seed; pure `score()` is the zero-proc floor. **Still open:** state-dependent variants (economy, discards/hands remaining, joker-slot counts like `MultTimesOnEmptyJokerSlots`/`MultTimesEveryXHands`) and non-scoring probabilistic effects (Glass/`ChanceDestroyed` destruction, `Odds1inUpgradeHand`, `Odds1inCashOn3Ranks` — deck-mutation/economy, not score contributions)
- [ ] Retrigger mechanics (red seal, Dusk, Hack…)
- [ ] Edition/enhancement/seal contributions to scoring

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
- [~] Shop: buying, selling (consume `resell_value`), rerolls, packs —
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
  (`MultPlusPerReroll(2)`) counts. Booster packs are still not modelled.
  (EPIC-01a Phase 8, EPIC-01b Phases 2–3)
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
- [~] Full `Score` pipeline a solver can call without panicking — `BuffoonBoard::score()` (deterministic floor) and `score_with_seed(seed)` (rolls probabilistic effects) run all four phases without panicking; still partial until the state-dependent `MPip` variants land (they need round/economy state on the board)
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
- [~] Replace journal-style doc comments with API reference docs. **Done: the profanity/"STORY TIME" is gone** (it lived only in the now-deleted `fpips.rs`). **Remaining:** `DIARY` notes in `buffoon_card.rs`, `buffoon_pile.rs`, `joker.rs`
- [ ] CHANGELOG entries for the funky feature

---

## Verification matrix

- [x] `cargo test --features funky` — full battery (**395** unit tests green as of 2026-07-11, +12 new deck/joker data tests)
- [x] `cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic` — **the entire crate is clean** (lib, all tests, all examples, benches) and gated in CI (`unwrap`/`expect` in tests allowed via a `cfg(test)` attribute in `src/lib.rs`)
- [x] `cargo build --no-default-features` — green; `--examples` also green (buffoon correctly gated behind `required-features = ["funky"]`)
- [x] `cargo run --example buffoon --features funky` — demonstrates the full four-phase pipeline: base (100×8) + cards (+51 chips) + held Steel (×1.5 → 12 mult) + jokers (180×22) → `score()` 331×34 = **11254**
- [x] `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings` — clean

## Gotchas

- **All four scoring phases are implemented; `BuffoonBoard::score()` never panics** and folds every phase into one running score, so joker order (and additive-vs-×mult) is honored. But many `MPip` variants still fall through to `_ => 0`, so a "wired" joker can silently score nothing (e.g. `MultPlusOnHandPlays`, `ChipsPerRemainingDiscard`, the hand-conditional ×mult jokers). **Effects are scored through the *pile* (`BuffoonPile::calculate_plus`), not just the card — a per-card variant handled only in `BuffoonCard` still scores 0 in play until the pile sums it** (this bit Odd Todd). When implementing a variant, add a test at the pile/board level proving it scores.
- **Silent zero-scoring:** unhandled `MPip` variants fall through to `_ => 0`, so a joker can be "wired" yet contribute nothing (e.g. `MultPlusOn5Ranks`). When implementing a variant, add a test proving it scores.
- **`RefCell` in `ToggleCard`** makes it non-`Sync` — fine for a single-threaded solver loop, a constraint for parallel search.
- **funky is std-only by design** — never import funky types into `basic` modules or the no_std discipline breaks.
- **`beggar my neighbor`** commits seen in history are not part of funky at the current tip — don't go looking for them.
