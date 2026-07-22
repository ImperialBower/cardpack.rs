---
type: Rust Crate
title: cardpack — Generic Deck of Cards
description: Pure-by-default no_std Rust library modeling playing-card decks, with opt-in std, i18n, serde, YAML, and a Balatro-style engine.
resource: https://crates.io/crates/cardpack
tags: [crate, overview, no_std]
timestamp: 2026-07-22T13:10:00Z
---

# What it is

`cardpack` (v0.8.1, Rust 2024 edition, MSRV 1.85, Apache-2.0) is a generic
pack-of-cards library. Its responsibilities:

* Represent specific types of card decks ([deck catalog](/decks/deck-catalog.md)).
* Validate that a collection of cards is valid for a deck type.
* Render/parse textual representations (symbol strings like `A♠`, index
  strings like `AS`).
* Shuffle — including deterministic seeded shuffle that works under `no_std`.
* Localize card names via Project Fluent
  ([localization](/architecture/localization.md)).

A bare dependency is an **alloc-only, `no_std`, no-I/O domain kernel**; every
dependency-bearing capability is opt-in via Cargo features
([feature flags](/architecture/feature-flags.md),
[domain kernel](/architecture/domain-kernel.md)).

# Module map (`src/lib.rs`)

| Module | Gate | Contents |
|---|---|---|
| `basic` | always | Core types ([card model](/architecture/card-model.md)) and all shipped decks |
| `common` | always | Errors (`thiserror`) and utilities |
| `funky` | `funky` feature | Balatro-style engine ([funky engine](/architecture/funky-engine.md)) |
| `localization` | `i18n` feature | Fluent locales |
| `prelude` | always | `use cardpack::prelude::*` — the intended import surface |
| `preludes::funky` | `funky` feature | Prelude for the funky engine |

# Lint posture

`src/lib.rs` warns on `clippy::all/pedantic/nursery/cargo/suspicious` plus
`unwrap_used`/`expect_used` in production paths; `unwrap`/`expect` are allowed
under `cfg(test)`. CI keeps the whole crate clippy-pedantic-clean at
`--all-targets`.

# Citations

[1] [Cargo.toml](../../Cargo.toml)
[2] [src/lib.rs](../../src/lib.rs)
[3] [README.md](../../README.md)
