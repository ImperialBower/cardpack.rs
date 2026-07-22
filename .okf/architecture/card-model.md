---
type: Architecture
title: Card model — Pip, BasicCard, Card, Pile
description: The four-layer generic card model and the traits (DeckedBase, Decked, Ranged) that turn a type into a deck.
tags: [architecture, types, generics]
timestamp: 2026-07-22T13:10:00Z
---

# The layered model

From smallest to largest (all in `src/basic/types/`):

| Type | File | Role |
|---|---|---|
| `Pip` | `pips.rs` | Atomic rank-or-suit facet: symbol, index char, and a **weight** used for sorting/precedence |
| `BasicCard` | `basic_card.rs` | Rank `Pip` + suit `Pip`, no generic constraints — the raw data of a card |
| `Card<T>` | `card.rs` | Generic wrapper around `BasicCard` where `T: DeckedBase` binds the card to a deck vocabulary |
| `Pile<T>` | `pile.rs` | Ordered collection of `Card<T>`: construct, draw, sort, shuffle, extract, validate |

Key consequence: **weight lives inside the card**. Sorting is data-driven, so
an Ace-low game needs a separate deck type with inverted rank weights (the
`Razz` deck is the shipped example) rather than a comparator.

# Traits (`src/basic/types/traits.rs`)

* **`DeckedBase`** — what a type must provide to *be* a deck: `base_vec()`
  (the canonical `Vec<BasicCard>`), `colors()`, `deck_name()`,
  `fluent_deck_key()`. Implementing it on a marker struct is the whole
  extension mechanism ([extending decks](/decks/extending-decks.md)).
* **`Decked<T>`** — convenience layer: `T::deck()`, `T::decks(n)` (multi-deck
  games like Hand and Foot), `validate()`.
* **`Ranged`** — combinatorics (`combos(k)` etc.); every deck gets it. The
  funky engine reuses it for poker-hand detection.

# Conventions

* Canonical deck order is high-to-low rank, suit-major (spades → hearts →
  diamonds → clubs for French-family decks).
* Two string forms round-trip: symbol strings (`A♠ K♠ …`) and index strings
  (`AS KS …`). Jokers use `B`/`L` (big/little) with suit symbol `🃟`.
* `draw(n)` is all-or-nothing (`Option`); `draw_first`/`draw_last` deal from
  top/bottom. Note: top-of-deck is the *front* of the vector today
  (`remove(0)`, O(n)) — a tracked refactor candidate in
  [docs/TECHNICAL_DEBT.md](../../docs/TECHNICAL_DEBT.md).
* Macros (`card!`, `cards!`, `french_cards!`) parse index strings into typed
  cards.

# Citations

[1] [src/lib.rs module docs](../../src/lib.rs)
[2] [DECON-01 Card Model](../../docs/deconstruct/DECON-01_Card_Model_And_Ordering.md)
[3] [DECON-02 Pile Operations](../../docs/deconstruct/DECON-02_Pile_Operations.md)
