---
type: Reference
title: README
description: "The crate's front door: what cardpack is, the cargo feature table, install snippets, and the deck roster."
tags: [readme, features, overview]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/README.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The consumer-facing introduction to the crate — what a `Pile<DeckType>` is,
the full cargo feature table with what each flag pulls in, install snippets
for the common feature stacks, and the roster of shipped decks.

# Authoritative for

* The **feature table as consumers see it** — the prose explanation of `full`
  vs `std-io`, and which flags imply which. The machine-readable truth is
  `Cargo.toml`; see [feature flags](/architecture/feature-flags.md) for the
  distilled version.

# In-repo path

`README.md`

This concept is a pointer, not a copy — the linked document is authoritative.
