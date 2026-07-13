# Balatro deck engine — feature spec

**Branch:** `funky` · **Status:** in progress · **Author seed:** snapshot from
post-merge state on 2026-05-03 against `main` v0.7.0.

## Goals

The `funky` module exists to provide a complete, data-driven model of
[Balatro](https://www.playbalatro.com)'s decks, cards, and scoring mechanics
that can serve two consumers:

1. **A Balatro solver.** Given a board state (deck, hand, jokers, consumables,
   poker-hand levels, ante), enumerate or search for the highest-value play.
   This requires deterministic, side-effect-free scoring.
2. **An engine for custom Balatro mods written in Lua.** A Lua-side modder
   should be able to declare new jokers, vouchers, planets, etc. as data and
   have the Rust engine evaluate them — no Rust recompilation per mod. This
   is feasible because the funky module already encodes effects declaratively
   in the `MPip` enum (~50 effect variants) rather than as closures.

Both consumers share the same need: a complete card-data corpus and a
correct, decoupled scoring evaluator.

## Architecture already in place

| Layer | File(s) | What it provides |
|---|---|---|
| Card data type | `src/funky/types/buffoon_card.rs` | `BuffoonCard { suit, rank, card_type, enhancement, resell_value, debuffed }`; `BCardType` enum tagging Basic/Stone/CommonJoker/UncommonJoker/RareJoker/LegendaryJoker/Planet/Spectral/Tarot/Voucher |
| Effect language | `src/funky/types/mpip.rs` | `MPip` sum type with ~50 variants encoding chip/mult adds, retriggers, conditional effects, probabilities, suit/rank predicates, etc. This is the "bytecode" Lua mods will target. |
| Pile container | `src/funky/types/buffoon_pile.rs` | `BuffoonPile` with hand-evaluation (`determine_hand_type`) and `calculate_plus*` chip math |
| Hand types | `src/funky/types/hands.rs` | `HandType` enum (incl. secret hands), `PokerHand { level, chips, mult, times_played }`, `PokerHands` keyed by `HandType` |
| Score | `src/funky/types/score.rs` | `Score { chips, mult }` with arithmetic |
| Game state | `src/funky/types/board.rs` | `BuffoonBoard { draws, deck, in_hand, played, consumables, jokers, poker_hands }` |
| Other | `draws.rs`, `fpips.rs`, `toggle.rs` | Round/run accounting helpers |
| Macros | `src/funky/macros.rs` | `bcard!`, `bcards!` for terse construction |

## Player deck inventory

Balatro ships 15 starting decks. Most differ only by rule modifiers (extra
hand, extra discard, starting voucher) — these are **engine presets**, not
distinct card compositions. Only three decks change which physical cards are
present.

| Balatro deck | Composition difference | Status | Notes |
|---|---|---|---|
| Red Deck | 52 standard, +1 discard | Composition: ✓ (`Deck::DECK`) — Modifier: ✗ | Modifier system not yet built |
| Blue Deck | 52 standard, +1 hand | Composition: ✓ — Modifier: ✗ | |
| Yellow Deck | 52 standard, +$10 starting | Composition: ✓ — Modifier: ✗ | |
| Green Deck | 52 standard, $2/$1 instead of interest | Composition: ✓ — Modifier: ✗ | |
| Black Deck | 52 standard, +1 joker slot, −1 hand | Composition: ✓ — Modifier: ✗ | |
| Magic Deck | 52 standard, starts with Crystal Ball + 2 Fool tarots | Composition: ✓ — Modifier: ✗ | Voucher data not present |
| Nebula Deck | 52 standard, starts with Telescope, no Celestial packs in shop | Composition: ✓ — Modifier: ✗ | |
| Ghost Deck | 52 standard, Spectral cards may appear | Composition: ✓ — Engine: ✗ | Requires Spectral implementation |
| **Abandoned Deck** | 40 cards, no face cards (no J/Q/K) | **Composition: ✓** (`Deck::ABANDONED_DECK`) — Modifier: ✗ | |
| **Checkered Deck** | 52 cards, only ♠ and ♥ (no ♣ ♦) | **Composition: ✓** (`Deck::CHECKERED_DECK`) — Modifier: ✗ | |
| Zodiac Deck | 52 standard, starts with 3 vouchers | Composition: ✓ — Modifier: ✗ | Voucher data not present |
| Painted Deck | 52 standard, +2 hand size, −1 joker slot | Composition: ✓ — Modifier: ✗ | |
| Anaglyph Deck | 52 standard, Double Tag after each Boss Blind | Composition: ✓ — Modifier: ✗ | Tag system not present |
| Plasma Deck | 52 standard, base score balanced (chips=mult), X2 blind size | Composition: ✓ — Modifier: ✗ | |
| **Erratic Deck** | 52 cards, every rank/suit randomized | Composition: ✗ | Needs a generator that draws uniformly from all 52 (rank, suit) pairs with replacement |

**Composition coverage: 14 / 15 decks** (Erratic missing).
**Modifier system coverage: 0 / 15** — none of the deck-specific rule overrides
are implemented yet. Every modifier maps cleanly to existing state on
`BuffoonBoard` (extra `Draws`, extra joker/consumable slot capacity, starting
inventory in `BuffoonBoard::consumables` / `BuffoonBoard::jokers`), so this
is data + a small applicator, not new architecture.

## Card-type inventory

| Card type | In-game count | Implemented | Notes |
|---|---:|---:|---|
| Standard playing cards | 52 | **52 ✓** | `src/funky/decks/basic.rs::card::*` |
| Tarot (Major Arcana) | 22 | **22 ✓** | `src/funky/decks/tarot.rs` — all named, all in `MajorArcana::DECK` |
| Planet | 12 (8 base + 1 Mercury + 3 secret-hand) | **12 const-defined, 8 in `Planet::DECK`** | Mercury (Pair-leveler) is defined but missing from `DECK`; `PLANET_X`/`CERES`/`ERIS` are correctly held out for the secret-hand discovery flow. Verify Mercury inclusion is intentional. |
| Joker | 150 | **95 implemented, 22 categorized as Common, 50 stubbed as `// nnn Name` comments at end of joker.rs** | See [Joker breakdown](#joker-breakdown) below |
| Spectral | 18 | **0** | `BCardType::Spectral` exists; no card module |
| Voucher | 32 | **0** | `BCardType::Voucher` exists; no card module |
| Enhancement (Bonus / Mult / Wild / Glass / Steel / Stone / Gold / Lucky) | 8 | **All 8 representable in `MPip`** | E.g. `MPip::BONUS = Chips(30)`, `MPip::Glass(usize, usize)`, `MPip::Wild(PipType)`, `MPip::Stone(usize)`, `MPip::Gold(usize)`, `MPip::Lucky(usize, usize)` |
| Edition (Foil / Holographic / Polychrome / Negative) | 4 | **0** | No `Edition` field on `BuffoonCard`. The card struct has `enhancement: MPip` but no separate edition slot — needs a design call: extend `MPip` with edition variants, or add a parallel `edition: Edition` field. The latter is closer to Balatro's mental model (a card can simultaneously be e.g. a Glass + Polychrome). |
| Seal (Gold / Red / Blue / Purple) | 4 | **0** | No `Seal` field on `BuffoonCard`. Same shape decision as editions. |
| Tag (skip-blind tags) | 24 | **0** | No tag module |
| Booster Pack | 5 kinds × rarities | **0** | Not modeled; orthogonal to deck contents but needed for shop/round flow |
| Blind (boss blinds) | 28 | **0** | Not modeled; needed for full round simulation |

### Joker breakdown

- **95 jokers** declared as `pub const` in `joker.rs::card`.
- **22** of those are wired into `Joker::COMMON_JOKERS` (the only rarity array
  built so far). Status arrays for Uncommon (52 in-game), Rare (20 in-game),
  and Legendary (5 in-game) do not yet exist.
- **50 jokers** (numbered 101–150 in joker.rs) remain as commented-out
  in-game-text stubs awaiting `MPip` translation.
- **5 jokers** in the 1–100 range are also missing from the implemented set
  (95 vs. 100 expected). Worth a sweep to find which IDs are skipped.

## Engine / scoring inventory

| Capability | Status | Where |
|---|---|---|
| Hand classification (HighCard … FlushFive) | ✓ | `BuffoonPile::determine_hand_type`, `PokerHands::new` |
| Hand-level chips/mult base values | ✓ | `PokerHands::new` (matches in-game level-1 numbers) |
| Hand leveling via Planet cards | partial | `Planet::add_planets` exists; `PokerHand::level` field exists; integration into score not wired |
| Per-card chip math | ✓ | `BuffoonPile::calculate_plus_chips` |
| `MPip` effect interpretation | partial | Some `MPip` variants are evaluated in `buffoon_pile.rs`; many remain unhandled |
| **Phase 1 — Pre-scoring** | **`todo!()`** | `BuffoonBoard::scoring_phase1_pre_scoring` |
| **Phase 2 — Dealt-hand scoring** | **`todo!()`** | `BuffoonBoard::scoring_phase2_dealt_hand_scoring` |
| **Phase 3 — In-hand effects** | **`todo!()`** | `BuffoonBoard::scoring_phase3_effects_in_hand` |
| Phase 4 — Joker effects | not stubbed | Required by solver |
| Phase 5 — End-of-round economy | not stubbed | Required for multi-round simulation |
| Round/run state machine | partial | `Draws`, `BuffoonBoard` exist; transitions not wired |
| RNG model (Balatro is seedable) | inherited from cardpack | Cardpack now has `shuffle_with_seed`/`shuffled_with_rng` (added on `main` v0.7.0). Balatro's actual seed→event mapping is bespoke and not modeled |

## Critical gaps for each goal

### For the solver

The solver needs a pure function `score(board, played_cards) -> Score`. Today
that's blocked on:

1. **All five scoring phases** must be implemented in `board.rs`. The Reddit
   reference cited in `board.rs` is a good blueprint.
2. **Edition + Seal data on `BuffoonCard`** — without these, "polychrome
   joker × glass card with red seal retriggers" cannot be evaluated, and a
   solver that ignores them is unfit for late-game decisions.
3. **Joker effect dispatcher** — a single match on `MPip` per scoring phase
   that mutates the running `Score`. This consolidates the half-implemented
   logic currently spread across `buffoon_pile.rs`.
4. **Hand-level integration** — `PokerHand::level` must feed into the chips/
   mult lookup in phase 2.
5. **Joker corpus completion** — 50 stubbed + 5 missing = 55 cards before the
   solver can claim full coverage. Each is a one-line `MPip` translation.

### For the Lua mod engine

A modder should be able to write Lua like:

```lua
register_joker {
  name = "Foobar",
  rarity = "uncommon",
  cost = 5,
  effect = mpip.mult_plus_on_flush(8),
}
```

To get there:

1. **Expose `MPip` constructors to Lua.** `mlua` (or `rlua`) is the standard
   bridge; each `MPip` variant becomes a Lua function returning the tagged
   value as opaque userdata. No engine logic changes — Rust still owns
   evaluation.
2. **A registry layer** — currently jokers are `pub const` arrays in Rust.
   Mods need a runtime-mutable `Vec<BuffoonCard>` registry per `BCardType`,
   merged with the built-in arrays.
3. **Effect coverage parity with built-ins.** Anything a built-in joker can
   do, a Lua-defined one must be able to express purely via `MPip` — else
   modders will hit a wall and demand Rust changes per mod.
4. **Stable `MPip` discriminants** for save-game / mod compatibility. Right
   now `MPip` is a closed enum, so adding a variant is a breaking change to
   any persisted state. A `serde`-tagged form is already derived but not
   guarded against reordering.
5. **Sandboxing.** Lua scripts shipped as mods must not be able to call
   filesystem / network APIs. `mlua` supports stdlib subsetting; pick a
   policy before exposing any `register_*` API.

## Suggested phased path

The funky branch is currently somewhere between Phase 1 and Phase 2 below.

| Phase | Outcome | Required work |
|---|---|---|
| 1. Card corpus | Every Balatro card representable as data | Finish 50 stubbed jokers; add Spectral (18); add Voucher (32); add Edition + Seal fields on `BuffoonCard`; add Erratic deck generator; verify Mercury planet inclusion |
| 2. Scoring core | Pure `score(board, played) -> Score` works for vanilla Balatro | Implement scoring phases 1–5; build joker dispatcher; integrate hand levels; add edition/seal effects to dispatcher |
| 3. Round simulation | A full run is replayable from seed | Round state machine; shop generation; blind effects; tags; booster packs |
| 4. Solver loop | Search returns optimal play for a given board | Heuristic / search algorithm consuming Phase 2's pure scorer; not blocked by Phase 3 if board state is provided externally |
| 5. Lua bridge | Mods load at runtime | `mlua` integration; registry layer; sandbox policy; example mod; mod-loading documentation |

Phases 1, 2, and 3 unblock the solver. Phase 5 is independent and can begin
in parallel with Phase 1 (Lua-side scaffolding doesn't depend on completing
the corpus, only on the `MPip` API being stable enough to expose).

## Open questions

1. **Edition vs. enhancement representation.** Add an `edition: Edition`
   field on `BuffoonCard`, or extend `MPip` to carry edition variants? The
   former matches Balatro's data model (a single card can be both glass
   *and* polychrome). The latter avoids growing the struct. Recommend the
   former.
2. **Are the 15 player decks really decks-of-cards or run-modifiers?**
   Twelve of them are pure modifiers and likely belong in a `RunPreset`
   struct, not under `src/funky/decks/`. Only Abandoned, Checkered, and
   Erratic warrant being arrays of `BuffoonCard`.
3. **`HashMap` in `PokerHands`.** `hands.rs` uses `std::collections::HashMap`,
   tying funky to `std`. Already gated by the `funky` Cargo feature
   (introduced in the recent merge), but if no_std support for funky is ever
   wanted, `BTreeMap` is the drop-in.
4. **Solver determinism vs. Balatro's RNG.** Balatro probabilities (e.g.
   `1 in 4` lucky-card cash) are deterministic given the seed and event
   ordering. A solver searching over hypothetical futures has to either
   model the RNG state or treat probabilities as expected values. Decide
   early — it shapes the `score()` signature.
