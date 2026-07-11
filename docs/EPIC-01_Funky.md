# EPIC-01: Funky — Balatro-Style Cards

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Checked boxes reflect work already landed on `origin/funky` as of 2026-07-05 (tip `cc1595d`, the merge of main's 0.7.0 no_std work into funky). See the companion status document [`EPIC-01_Funky_Progress.md`](EPIC-01_Funky_Progress.md) for the quality evaluation behind these checkmarks.
>
> **2026-07-11 hardening pass** (uncommitted on local `funky`): funky is now gated in CI (`cargo test --features funky`; `cargo clippy --features funky --lib --tests` at `-Dpedantic`); debug `println!`s removed from scoring paths; the two `MPip` Display bugs fixed; the 6 funky-lib clippy warnings cleared; **the whole crate made clippy-pedantic-clean at `--all-targets`** (lib, all tests, all examples, benches) — `unwrap`/`expect` allowed under `cfg(test)` in `src/lib.rs`, plus mechanical fixes in core `basic` test code and API-level fixes in the examples (`bridge.rs` pass-by-value/`# Panics`/`# Errors` docs/`render` rename, `range.rs`/`demo.rs`/`poker_eval.rs` cleanups); `Draws`/`Toggle`/`ToggleCard` exported; data-invariant tests added for `decks/basic.rs` and `decks/joker.rs` (+12 tests → 395 green); and `examples/buffoon.rs` rewritten to demonstrate phase-4 joker scoring end-to-end. CI now gates `cargo clippy --features funky --all-targets`. **Deferred by design:** scoring phases 1–3 (`todo!()`), the ~54 silently-zero `MPip` variants, the mod/effect-registry redesign, and the remaining Balatro subsystems.

**Goal:** Model [Balatro](https://www.playbalatro.com/)-style cards — jokers with scoring effects, planet cards that level up poker hands, tarot/spectral consumables, enhanced decks, and chips × mult scoring — well enough to (a) power a **Balatro solver** and (b) enable **dynamic creation of custom Balatro mods**.

**Architecture:** A `std`-only, feature-gated module (`funky = ["std", "dep:phf"]`, `#[cfg(feature = "funky")] pub mod funky;` at `src/lib.rs:298`) that layers on top of the crate's core `basic` engine rather than forking it. `BuffoonCard` (`src/funky/types/buffoon_card.rs:76`) embeds core `Pip`s for suit/rank and adds Balatro state: `card_type: BCardType`, `enhancement: MPip`, `resell_value`, `debuffed`. Effects are **data, not code**: a 69-variant `MPip` enum (`src/funky/types/mpip.rs`) attached to each card const, interpreted at scoring time by `calculate_plus_*` match arms. `BuffoonPile` implements the core `Ranged` trait (`buffoon_pile.rs:460`) so combinatorics and hand detection delegate to the mature `basic` engine.

**Tech Stack:** Rust 2024 edition, std-only behind the `funky` feature (deliberately outside the 0.7.0 no_std core), `phf` (declared, currently unused), rstest for case tables.

---

## Domain map (Balatro → funky)

| Balatro concept | funky construct | Status |
|---|---|---|
| Playing card | `BuffoonCard` + `BCardType::Basic` | ✅ done |
| Joker | `BCardType::{Common,Uncommon,Rare,Legendary}Joker` + `MPip` effect | 🟡 ~95/150 declared, ~43 with effects, ~18 effect kinds scored (incl. the per-card rank family: Fibonacci/Even Steven/Odd Todd) |
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
| Shop / economy | `resell_value` field, `MPip::SellValueIncrement` | ❌ no shop engine |
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
- [ ] Decide fate of experimental `FIntPip` function-pointer pip (`fpips.rs` — unused, journal-style docs)

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
- [ ] Wire effects for the ~52 jokers currently carrying `MPip::Blank`
- [ ] Implement jokers 96–150 (currently a commented-out catalog, `joker.rs:1393-1458`)
- [ ] Uncommon/Rare/Legendary pile assemblies
- [x] Data-invariant tests for `decks/joker.rs` (COMMON_JOKERS size, all-jokers, all-tagged-common, distinct) — full per-card coverage still open

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
- [x] Phase 4 joker scoring: `BuffoonBoard::scoring_phase4_joker_scoring` + 4 end-to-end tests (`board.rs`)
- [x] Phase 1 pre-scoring: `BuffoonBoard::scoring_phase1_pre_scoring` — base chips/mult from the played hand's type & level (Royal Flush normalizes to Straight Flush, matching Balatro). Also fixed a `FlushFive` table-entry typo in `hands.rs`
- [x] Phase 2 played-hand scoring: `BuffoonBoard::scoring_phase2_dealt_hand_scoring` — each played card's `get_chips()` (base rank + flat `Chips`) plus per-card `calculate_plus` effects (disjoint `MPip` variants, no double-count)
- [x] Phase 3 held-card effects: `BuffoonBoard::scoring_phase3_effects_in_hand` — applies held ×mult (Steel = `MultTimes1Dot(15)` = ×1.5, `MultTimes(n)` = ×n) to the running score. **All four phases now implemented**; `BuffoonBoard::score()` runs the full Balatro-ordered pipeline (base → cards → held ×mult → jokers) and never panics; 17 board tests total (`board.rs`)
- [~] Handle the `MPip` variants that fall through to `_ => 0`. **Done: the per-card rank family** — `MultPlusOn5Ranks` (Fibonacci +8, Even Steven +4) at both card and pile level, and pile-level summing of `ChipsPlusOn5Ranks` (Odd Todd +31), which was a latent silent bug: its card-level test passed but board scoring goes through the pile, which never summed it, so Odd Todd scored 0 in play. 5 unit tests + an end-to-end `score()` regression test. **Still open:** state-dependent variants (economy, discards/hands remaining, joker-slot counts), multiplicative jokers (`MultTimes*`), and probabilistic effects (`Odds1in`, `Lucky`, `ChanceDestroyed`) — most of which aren't pure `(hand, jokers)` functions
- [ ] Retrigger mechanics (red seal, Dusk, Hack…)
- [ ] Edition/enhancement/seal contributions to scoring

## Story 7: Game state & economy

- [x] `BuffoonBoard`: deck, in_hand, played, consumables (cap 2), jokers (cap 5), `PokerHands` (`board.rs:6`)
- [x] Capacity mechanics: `new_with_capacity`/`capacity`/`free` (`buffoon_pile.rs:23,78,207`)
- [x] `Draws { hands_to_play, discards }` (`draws.rs`)
- [x] `ToggleCard` selection via `RefCell<bool>` (`toggle.rs`)
- [ ] Blind/ante progression, boss blind effects
- [ ] Shop: buying, selling (consume `resell_value`), rerolls, packs
- [ ] Money/economy state on the board
- [ ] Round loop: play/discard cycle consuming `Draws`

## Story 8: Modding & solver enablement (the stated end-goals)

- [ ] Open effect interpretation: today all `MPip` handling is hard-coded `match` arms — a custom mod cannot register a new effect without editing funky source. Needs a trait boundary or effect-registry design (the unused `phf` dependency hints at a planned static registry)
- [~] Full `Score` pipeline a solver can call without panicking — `BuffoonBoard::score()` runs all four phases (base → cards → held ×mult → jokers) without panicking; still partial until multiplicative jokers and the ~54 unhandled `MPip` variants land
- [ ] Deterministic/seedable shuffle for solver reproducibility (TODO at `buffoon_pile.rs:355`; core `basic` already has seeded shuffle)
- [ ] Serde on funky types (core decks got serde in 0.6.x; funky types have none)
- [x] End-to-end example: `examples/buffoon.rs` now deals a board, plays a hand, detects the hand type, and demonstrates phase-4 joker scoring (180 chips × 22 mult) end-to-end

## Story 9: API surface & integration hygiene

- [x] Feature gating: `funky` off by default, requires `std` (`Cargo.toml:32`)
- [x] Prelude: `src/preludes/funky.rs` exports decks, types, `MPip::*`, macros
- [x] Export `Draws` and `ToggleCard`/`Toggle` — now `pub mod draws;` and re-exported from `src/preludes/funky.rs`, so `BuffoonBoard::new(draws: Draws, …)` is callable from outside the crate
- [x] Add funky to CI: the test matrix now runs `cargo test --features funky`, and the clippy job runs `cargo clippy --features funky --lib -- -Dclippy::all -Dclippy::pedantic`
- [x] Fix 6 default-level clippy warnings in funky lib code (unwrap on Option, collapsible ifs, `sort_by_key`, let-if-seq) — funky lib is now clean at `-Dclippy::pedantic`
- [ ] Remove or use the `phf` dependency
- [ ] Replace journal-style doc comments ("DIARY"/"STORY TIME", profanity in `fpips.rs`) with API reference docs
- [ ] CHANGELOG entries for the funky feature

---

## Verification matrix

- [x] `cargo test --features funky` — full battery (**395** unit tests green as of 2026-07-11, +12 new deck/joker data tests)
- [x] `cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic` — **the entire crate is clean** (lib, all tests, all examples, benches) and gated in CI (`unwrap`/`expect` in tests allowed via a `cfg(test)` attribute in `src/lib.rs`)
- [x] `cargo build --no-default-features` — green; `--examples` also green (buffoon correctly gated behind `required-features = ["funky"]`)
- [x] `cargo run --example buffoon --features funky` — demonstrates the full four-phase pipeline: base (100×8) + cards (+51 chips) + held Steel (×1.5 → 12 mult) + jokers (180×22) → `score()` 331×34 = **11254**
- [x] `cargo doc --no-deps --all-features` with `RUSTDOCFLAGS=-D warnings` — clean

## Gotchas

- **All four scoring phases are implemented; `BuffoonBoard::score()` never panics.** But jokers (phase 4) are still **additive-only** — multiplicative jokers (`MultTimes*`) are not applied, and many `MPip` variants still fall through to `_ => 0`, so a "wired" joker can silently score nothing (e.g. `MultPlusOnHandPlays`, `ChipsPerRemainingDiscard`). **Effects are scored through the *pile* (`BuffoonPile::calculate_plus`), not just the card — a per-card variant handled only in `BuffoonCard` still scores 0 in play until the pile sums it** (this bit Odd Todd). When implementing a variant, add a test at the pile/board level proving it scores.
- **Silent zero-scoring:** unhandled `MPip` variants fall through to `_ => 0`, so a joker can be "wired" yet contribute nothing (e.g. `MultPlusOn5Ranks`). When implementing a variant, add a test proving it scores.
- **`RefCell` in `ToggleCard`** makes it non-`Sync` — fine for a single-threaded solver loop, a constraint for parallel search.
- **funky is std-only by design** — never import funky types into `basic` modules or the no_std discipline breaks.
- **`beggar my neighbor`** commits seen in history are not part of funky at the current tip — don't go looking for them.
