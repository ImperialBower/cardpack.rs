---
type: Architecture
title: Funky — the Balatro-style engine
description: std-only feature-gated module modeling Balatro cards, jokers, scoring, shop, and vouchers on top of the core card model; EPIC-01 closed 2026-07-18 with named deferrals.
tags: [funky, balatro, feature-gated, scoring]
timestamp: 2026-07-23T00:00:00Z
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

# Status (EPIC-01, closed 2026-07-18)

All five child EPICs (01a–01e) are closed; both stated goals are structurally
met. **Named deferrals** to future EPICs: Seals (blocks 4 seal spectrals),
most Decks (~3 of 16), Antes/Bosses, Tags, Serde. 8 of 112 jokers stay `Blank`
with a test-enforced reason list (`BLANK_WITH_REASON`).

# Try it

```shell
cargo run --example buffoon --features funky      # 4-phase scoring, phase by phase
cargo run --example funky_tour --features funky   # seeded tour: rounds, editions, shop, spectrals
```

# Citations

[1] [docs/EPIC-01_Funky.md](../../docs/EPIC-01_Funky.md)
[2] [README "Funky"](../../README.md)
[3] [docs/2026-07-11-effect-registry-design.md](../../docs/2026-07-11-effect-registry-design.md)
