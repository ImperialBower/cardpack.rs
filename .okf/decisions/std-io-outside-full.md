---
type: Decision
title: std-io is excluded from full
description: The one filesystem API (cards_from_yaml_file) sits behind its own std-io feature, deliberately left out of the full umbrella so both kernel and convenience stack stay I/O-free.
tags: [decision, purity, features, io]
timestamp: 2026-07-22T13:10:00Z
---

# Decision

`BasicCard::cards_from_yaml_file` (reads a deck from a YAML *file* via
`std::fs`) is the crate's **only** filesystem seam. It lives behind the
`std-io` feature (`std-io = ["std", "yaml"]`), which is intentionally **not**
part of the `full` umbrella.

# Why

Keeps both the pure kernel *and* the batteries-included `full` stack I/O-free
(Invariant 1 of the [domain kernel](/architecture/domain-kernel.md)). Users
who want file loading must name the capability explicitly. Deck-from-YAML
without the filesystem already exists under plain `yaml`:
`cards_from_yaml_str` + `include_str!` (the `Razz` pattern —
[extending decks](/decks/extending-decks.md)).

# How to apply

* Don't add `std-io` to `full` "for convenience" — that reintroduces the
  audit's Invariant-1/3 violation.
* New I/O-adjacent APIs belong behind `std-io` (or a new named seam), never
  in the core or in `full`.
* `make test-std-io` exercises the seam separately
  ([build and test](/workflows/build-and-test.md)).

# Citations

[1] [Cargo.toml std-io comment](../../Cargo.toml)
[2] [docs/audit-2026-07-18-domain-kernel.md](../../docs/audit-2026-07-18-domain-kernel.md)
