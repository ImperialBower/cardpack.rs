---
type: Reference
title: Getting started — cardpack.rs Knowledge Bundle
description: How this bundle is organized and where to start reading.
tags: [getting-started, meta]
timestamp: 2026-07-22T13:10:00Z
---

# Overview

This bundle captures durable knowledge about **cardpack.rs**, a generic
deck-of-cards library in Rust. Start with the
[crate overview](/architecture/crate-overview.md), then follow links by need:

* Understanding the type system → [card model](/architecture/card-model.md)
* Choosing Cargo features → [feature flags](/architecture/feature-flags.md)
* Why the crate is structured the way it is → [domain kernel](/architecture/domain-kernel.md)
* What decks exist → [deck catalog](/decks/deck-catalog.md)
* Building / testing / releasing → [build and test](/workflows/build-and-test.md)

# Layout

* `architecture/` — how the crate is designed and why.
* `decks/` — the shipped deck vocabulary and how to extend it.
* `workflows/` — build, test, CI, and WebAssembly workflows.
* `decisions/` — load-bearing decisions that are easy to accidentally undo.
* `references/` — map of the in-repo documentation this bundle distills.
