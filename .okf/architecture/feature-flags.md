---
type: Architecture
title: Cargo feature flags
description: The pure-by-default feature matrix — default is an alloc-only no_std kernel; std, i18n, color, yaml, serde, std-io, and funky are opt-in.
tags: [features, cargo, no_std, purity]
timestamp: 2026-07-22T13:10:00Z
---

# Principle

`default = []`. A bare `cardpack` dependency is an alloc-only, `no_std`,
no-I/O kernel ([domain kernel](/architecture/domain-kernel.md)). Everything
else is opt-in.

# Matrix

| Feature | In `full`? | Pulls in | Turns on |
|---|---|---|---|
| `full` | — | everything below except `std-io`/`funky` | `std` + `i18n` + `colored-display` + `yaml` + `serde` |
| `std` | yes | libstd | thread-RNG shuffle, `draw_random`, etc. |
| `alloc` | (implied) | — | base of the kernel; `serde?/alloc` |
| `i18n` | yes | `fluent-templates` (implies `std`) | `FluentName`, `Named`, `Card::fluent_name*`, [localization](/architecture/localization.md) |
| `colored-display` | yes | `colored` (implies `std`) | `Color`, `Card::color*`, `Pile::to_color_*` |
| `yaml` | yes | `serde_norway` (implies `std`, `serde`) | `BasicCard::cards_from_yaml_str` (pure, in-memory), the `Razz` deck |
| `serde` | yes | `serde` (implies `alloc`) | `Serialize`/`Deserialize` derives on `Pip`/`Card`/`Pile` etc. |
| `std-io` | **no** | (implies `std`, `yaml`) | `BasicCard::cards_from_yaml_file` — the crate's one filesystem seam; deliberately excluded from `full` ([decision](/decisions/std-io-outside-full.md)) |
| `funky` | **no** | (implies `std`) | Balatro-style engine ([funky engine](/architecture/funky-engine.md)) |

# Gotchas

* `yaml` implies `serde` — it deserializes into the serde-derived structs.
* Deck-from-YAML **without** the filesystem is available under plain `yaml`:
  `cards_from_yaml_str` + compile-time `include_str!` (how `Razz` works).
* `rand`'s `std_rng` feature is enabled unconditionally, *not* gated on
  `std` — see [the rand decision](/decisions/rand-std-rng-unconditional.md)
  before "cleaning that up."
* Doctests that need optional features are marked `ignore` with a comment so
  `cargo test --no-default-features` stays green.

# Citations

[1] [Cargo.toml `[features]`](../../Cargo.toml)
[2] [README "Cargo features"](../../README.md)
