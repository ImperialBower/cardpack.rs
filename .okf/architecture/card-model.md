---
type: Architecture
title: Card model — Pip, BasicCard, Card, Pile
description: The four-layer generic card model and the traits (DeckedBase, Decked, Ranged) that turn a type into a deck.
tags: [architecture, types, generics]
timestamp: 2026-07-28T00:00:00Z
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
  (`remove(0)`, O(n)) — a tracked refactor candidate in the
  [technical debt register](/references/technical-debt.md).
* Macros (`card!`, `cards!`, `french_cards!`) parse index strings into typed
  cards.

This concept is the distilled version; the extended *why* — PhantomData
branding, the trait-stack rationale, and a recipe for porting the pattern to
other card libraries — is the
[generic decks explainer](/references/generic-decks-doc.md).

# Citations

[1] [src/lib.rs module docs](../../src/lib.rs)
[2] [DECON-01 Card Model](/references/decon-01-card-model.md)
[3] [DECON-02 Pile Operations](/references/decon-02-pile-operations.md)
[4] [Generic decks explainer](/references/generic-decks-doc.md)
