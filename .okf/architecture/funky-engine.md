---
type: Architecture
title: Funky — the Balatro-style engine
description: std-only feature-gated module modeling Balatro cards, jokers, scoring, shop, and vouchers on top of the core card model; EPIC-01 closed 2026-07-18 with named deferrals.
tags: [funky, balatro, feature-gated, scoring]
timestamp: 2026-10-18T00:00:00Z
---

# What it is

`src/funky/` (behind `funky = ["std", "serde"]`; every `funky/types/*` file
uses and derives serde unconditionally, so the feature pulls in `serde`) models
[Balatro](https://www.playbalatro.com/)-style cards well enough to (a) power a
Balatro **score solver** and (b) enable **custom mod decks**. It layers on the
core [card model](/architecture/card-model.md) rather than forking it.

# Key constructs

* **`BuffoonCard`** (`funky/types/buffoon_card.rs`) — embeds core `Pip`s for
  suit/rank, adds `card_type: BCardType`, `enhancement: MPip`, `resell_value`,
  `debuffed`, `Edition`.
* **`MPip`** (`funky/types/mpip.rs`) — the big idea: a ~69-variant enum making
  card effects **data, not code**; interpreted at scoring time by
  `calculate_plus_*` match arms.
* **`BuffoonPile`** — implements the core `Ranged` trait so combinatorics and
  hand detection delegate to the mature `basic` engine.
* **Scoring** — four-phase chips × mult pipeline (`score()` /
  `score_with_seed()`): base + played cards + held ×mult + jokers left-to-right;
  never panics; seeded shuffle for determinism.
* **Decks** (`funky/decks/`): basic (52/Abandoned 40/Checkered), 112 jokers,
  12 planets, 22 tarot, 18 spectral.
* **Economy**: `Shop` (stock, rerolls, booster packs, voucher slot), 20 wired
  `Voucher`s, `sell_joker`, blind state with 3 boss blinds.

# Planned move (2026-10-18)

This engine is **moving out of cardpack** into its own repo, `buffoonery`, where it becomes
the foundation of a Balatro solver
([decision](/decisions/funky-moves-to-own-repo.md),
[EPIC-06](/references/epic-06-funky-extraction.md)). Cardpack deprecates the
`funky` feature in 0.11.2 and removes it in 0.12.0. Do not add engine features
here.

# Status (EPIC-01, closed 2026-07-18)

All five child EPICs (01a–01e) are closed; both stated goals are structurally
met. **Named deferrals** to future EPICs: Seals (blocks 4 seal spectrals),
most Decks (~3 of 16), Antes/Bosses, Tags, Serde. 8 of 112 jokers stay `Blank`
with a test-enforced reason list. Both lists are public since 2026-10-18:
`Joker::ALL_JOKERS` / `Joker::pile_all()` is the one list of every joker, and
`Joker::BLANK_WITH_REASON` / `Joker::blank_reason()` says why a joker is
`Blank`. Build any joker catalog from these, never from a hand-kept copy.
`ScoreOp` now derives serde too, and the JSON of `ScoreOp`, `HandType` and
`Edition` is pinned by format tests (groundwork for EPIC-05's `ScoreTrace`).

# Try it

```shell
cargo run --example buffoon --features funky      # 4-phase scoring, phase by phase
cargo run --example funky_tour --features funky   # seeded tour: rounds, editions, shop, spectrals
```

# Citations

[1] [docs/EPIC-01_Funky.md](/references/epic-01-funky.md)
[2] [README "Funky"](/references/readme.md)
[3] [docs/2026-07-11-effect-registry-design.md](/references/effect-registry-design.md)
