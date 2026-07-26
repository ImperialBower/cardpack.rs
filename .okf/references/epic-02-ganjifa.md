---
type: Reference
title: EPIC-02 — Ganjifa decks (Mughal + Dashavatara)
description: Adding the two Ganjifa decks with per-suit inverted pip ranking, full localization, and registry integration.
tags: [epic, decks, ganjifa, closed]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-02_Ganjifa.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

Adding **Mughal Ganjifa** (8 suits × 12 = 96 cards) and **Dashavatara
Ganjifa** (10 suits × 12 = 120 cards) as first-class decks, including the
signature per-suit inverted pip ranking, Fluent localization in all five
locales, and `DeckKind` registry integration.

# Why it matters downstream

* It is what made the registry sweep 14 decks wide — the basis of every
  "for each shipped deck" guarantee in
  [EPIC-03](/references/epic-03-yaml-serialization.md).
* It set the `#[non_exhaustive]` semver precedent for `DeckKind`, later
  followed for `CardError`. See [deck catalog](/decks/deck-catalog.md).

# In-repo path

`docs/EPIC-02_Ganjifa.md`

This concept is a pointer, not a copy — the linked document is authoritative.
