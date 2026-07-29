---
type: Reference
title: Generic Decks — the phantom-type deck pattern
description: "The full explainer of the crate's generics architecture: PhantomData branding, the DeckedBase/Decked trait stack, blanket impls, and a recipe for porting the pattern to other card libraries."
tags: [architecture, generics, pattern, portability]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/generic-decks.md
timestamp: 2026-07-28T00:00:00Z
---

# What it covers

The deep explanation of *why* the card model is shaped the way it is: the
four-layer phantom-type pattern (`Pip`/`BasicCard` plain data → `Card<T>`
brand → `DeckedBase` vocabulary contract → `Decked`/blanket-impl derived
behavior), the type-driven vs data-driven split, the `BasicPile` escape
hatch, and the `DeckKind` runtime façade.

Written to be **portable**: it ends with a nine-step recipe for setting the
pattern up in a new library and a catalog of the sharp edges (derive-line
contract, bound drift, E0790 qualified-syntax, blank-card `Default`,
monomorphization limits).

# Authoritative for

* **The rationale behind the generic architecture.** The bundle's
  [card model](/architecture/card-model.md) concept states *what* the layers
  are; this document is the extended *why*, with the design rules for
  reproducing them elsewhere.
* **The porting recipe.** The ordered setup checklist (data layer first,
  vocabulary trait, brand, collection, blanket impls, `validate()`, in-tree
  `Tiny`, registry last) exists only here.

# In-repo path

`docs/generic-decks.md`

This concept is a pointer, not a copy — the linked document is authoritative.
