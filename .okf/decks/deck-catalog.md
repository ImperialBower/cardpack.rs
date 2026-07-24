---
type: Catalog
title: Shipped deck catalog
description: The 14 deck kinds the crate ships, their sizes, and their quirks; DeckKind is the runtime registry over all of them.
tags: [decks, catalog, registry]
timestamp: 2026-07-23T12:00:00Z
---

# The 14 deck kinds

All in `src/basic/decks/`, each a marker struct implementing `DeckedBase`
([card model](/architecture/card-model.md)):

| Deck | Cards | Notes |
|---|---|---|
| `French` | 54 | Foundation deck: 13 ranks × 4 suits + Big/Little jokers; most others derive from its `FrenchBasicCard` consts |
| `Standard52` | 52 | French minus jokers; home of the `card!`/`cards!` macros |
| `Short` | 36 | Ranks 6 through Ace (six-plus hold'em) |
| `Spades` | 52 | French with jokers, minus 2♣ and 2♦ |
| `Euchre24` | 24 | 24-card Euchre variant |
| `Euchre32` | 32 | 32-card Euchre variant |
| `Pinochle` | 48 | Two copies of 9–Ace; **Ten outranks King** |
| `Canasta` | 108 | 2 modern decks, red 3s promoted to jokers |
| `Skat` | 32 | German suits/ranks (own `.ftl` locale files) |
| `Tarot` | 78 | 22 Major + 56 Minor Arcana, trump precedence |
| `Mughal` | 96 | Mughal Ganjifa: 8 suits × 12; weak suits (Red Coins, Harps, Bills, Cloth) use **inverted pip ranking** (A > 2 > … > 10) via a second weight-inverted rank ladder |
| `Dashavatara` | 120 | Dashavatara Ganjifa: 10 avatar suits × 12; Matsya–Vamana weak (inverted pips), Parashurama–Kalki strong; shares the `cards::ganjifa` vocabulary with Mughal |
| `Razz` | 52 | **Ace-low** (inverted rank weights); built from YAML via `include_str!`; requires the `yaml` feature |
| `Tiny` | 4 | A♠ K♠ A♥ K♥ — minimal teaching/testing deck |

# The registry

`DeckKind` (`src/basic/decks/registry.rs`) is a runtime enum over all 14 —
`DeckKind::all()`, `.deck_name()`, `.base_vec()`, `.fluent_deck_key()` — for
"give me every deck this crate ships" use-cases the generic
`Pile<DeckType>` API can't express. The enum is `#[non_exhaustive]` (since
0.9.0), so new decks land in minor releases; match with a wildcard arm or
iterate `DeckKind::all()`.

# No dedicated multi-deck types

Hand-and-Foot-style games use `French::decks(n)` (4 players → 216 cards)
instead of a bespoke deck type.

# Citations

[1] [src/basic/decks/registry.rs](../../src/basic/decks/registry.rs)
[2] [DECON-05 French Deck Family](../../docs/deconstruct/DECON-05_French_Deck_Family.md)
[3] [DECON-06 Tarot and Skat](../../docs/deconstruct/DECON-06_Tarot_And_Skat.md)
