---
type: Reference
title: EPIC-01 — Funky (Balatro-style cards)
description: "The Balatro-style joker/effect engine: scope, the five child EPICs, and the deferrals recorded at close-out."
tags: [epic, funky, balatro, closed]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-01_Funky.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The `BuffoonCard` / `BuffoonPile` type family and the scoring engine built on
it — the solver path (`score()` / `score_with_seed()`, four phases, never
panics, seeded shuffle) and the joker/effect surface around it.

# Status

Closed 2026-07-18 at tip `e50fdd0`, with named deferrals. All five children
are closed: 01a Joker Wiring, 01b Shop, 01c Vouchers, 01d Editions (2026-07-16
– 07-17), and 01e Spectral Cards (2026-07-18, seal spectrals deferred).

# Authoritative for

* The funky scope boundary. The distilled architecture view is
  [funky engine](/architecture/funky-engine.md); the effect-dispatch design is
  [effect registry design](/references/effect-registry-design.md).

# In-repo path

`docs/EPIC-01_Funky.md`

This concept is a pointer, not a copy — the linked document is authoritative.
