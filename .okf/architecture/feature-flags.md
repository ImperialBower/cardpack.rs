---
type: Architecture
title: Cargo feature flags
description: The pure-by-default feature matrix — default is an alloc-only no_std kernel; std, i18n, color, yaml, serde, std-io, and funky are opt-in.
tags: [features, cargo, no_std, purity]
timestamp: 2026-08-24T12:00:00Z
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
| `yaml` | yes | `serde_norway` (implies `std`, `serde`) | Full deck ↔ YAML round-tripping, pure and in-memory: `DeckYaml`, the `YamlDecked` blanket trait, `DeckKind::to_yaml`/`from_yaml`, `Pile::to_yaml`/`from_yaml`, `BasicCard::cards_from_yaml_str`, six `CardError::Yaml*` variants, and the `Razz` deck ([envelope decision](/decisions/yaml-envelope-format.md)) |
| `serde` | yes | `serde` (implies `alloc`) | `Serialize`/`Deserialize` derives on `Pip`/`Card`/`Pile` etc. |
| `std-io` | **no** | (implies `std`, `yaml`) | `BasicCard::cards_from_yaml_file` — the crate's one filesystem seam; deliberately excluded from `full` ([decision](/decisions/std-io-outside-full.md)) |
| `funky` | **no** | (implies `std`, `serde`) | Balatro-style engine ([funky engine](/architecture/funky-engine.md)) |
| `seal-test-double` *(planned, EPIC-04)* | **no** | — | `PlaintextSeal` / `PlainToken` (**no security**) and the exported `seal_roundtrip` conformance helper |
| `commit-reveal` *(planned, EPIC-04a)* | **no** | `sha2` (no_std) | `Commitment`, `Contribution`, `ShuffleRound`, `CombinedSeed`, `commit_pile`, `Pile::shuffled_by_round` ([crypto decision](/decisions/crypto-features-outside-full.md)) |
| `seal-aead` *(planned, EPIC-04b)* | **no** | `chacha20poly1305`, `hkdf`, `sha2`, `zeroize` (all no_std) | `HolderKeySeal<D>`, `DealKey`, `CardKey`, `SealedBytes`, `Custody` (a plain `Vec<(SlotId, SealedBytes)>` — the dealer-custody ledger beside a `SlotPile`) |
| `crypto` *(planned, EPIC-04)* | **no** | = `commit-reveal` + `seal-aead` | umbrella over both backends; not in `full` |

# Gotchas

* `yaml` implies `serde` — it deserializes into the serde-derived structs.
* `funky` implies `serde` — unlike the core (which gates serde behind
  `#[cfg(feature = "serde")]`), every `src/funky/types/*` file `use`s and
  derives serde **unconditionally**, so the `funky` feature must pull it in or
  the module won't compile.
* **Examples run flag-free via `cargo ex <x>`**, an alias in
  `.cargo/config.toml` expanding to `run --features full,funky --example`.
  Keeping this in developer tooling is deliberate: a **self dev-dependency**
  (`cardpack = { path = ".", features = ["full", "funky"] }`) achieves the same
  ergonomics but feature-activates those crates on cardpack's own node in
  `cargo metadata`, which breaks `cargo deny check bans` and every host purity
  gate — and no cargo-deny setting can undo it. Read
  [the flag-free-examples decision](/decisions/examples-flag-free-alias.md)
  before changing this.
* Deck-from-YAML **without** the filesystem is available under plain `yaml`:
  `cards_from_yaml_str` + compile-time `include_str!` (how `Razz` works). The
  *write* side (`to_yaml`) is equally pure — it returns a `String`; nothing in
  `yaml` touches `std::fs`. That is what keeps `std-io` a separate seam.
* `yaml` is the only feature that changes `DeckKind::all()`'s **length**: the
  `Razz` variant is gated, so the registry is 13 kinds without it and 14 with.
  Anything sweeping the registry must read `DeckKind::all().len()` rather than
  hardcode a count.
* The `CardError::Yaml*` variants only exist under `yaml`, which is one reason
  `CardError` is `#[non_exhaustive]` — exhaustive downstream matching could
  never have been feature-portable.
* **The seal kernel is *not* feature-gated** (planned, EPIC-04). `SlotId`,
  the non-generic `SlotPile`, `Revealed<D>`, the `Seal<D>` adapter,
  `Ordinal`/`Codebook`, `Permutation` are dependency-free and always on; only
  the crypto *backends* are features, and none of them is in `full`. No kernel
  type is generic over a scheme and none holds ciphertext — see
  [the crypto decision](/decisions/crypto-features-outside-full.md).
* Never enable `chacha20poly1305/rand_core` when `seal-aead` lands — it is
  `rand_core 0.6`, cardpack is on `rand 0.10`. Nonces come from the caller's
  `rand::RngCore`.
* `rand`'s `std_rng` feature is enabled unconditionally, *not* gated on
  `std` — see [the rand decision](/decisions/rand-std-rng-unconditional.md)
  before "cleaning that up."
* Doctests that need optional features are marked `ignore` with a comment so
  `cargo test --no-default-features` stays green. **Prefer the ungated API
  first** — reach for `ignore` only when there is no pure equivalent. Several
  examples were needlessly `std`-only because they called `shuffle()`/
  `shuffled()` (thread RNG, `#[cfg(feature = "std")]`) where
  `shuffle_with_seed`/`shuffled_with_seed` are ungated and deterministic, which
  makes the doctest both portable and reproducible.
* `README.md` is pulled into the crate docs by
  `#![cfg_attr(doc, doc = include_str!("../README.md"))]`, so **its fenced
  `rust` blocks are compiled as doctests too** and are attributed to `src/lib.rs`
  line numbers in failure output. A README example using a gated API breaks
  `cargo test --no-default-features --doc`. Note that gate runs the **doc**
  tests; `--no-default-features --lib` (what most of the Makefile targets use)
  will not catch it.

# Citations

[1] [Cargo.toml `[features]`](../../Cargo.toml)
[2] [README "Cargo features"](/references/readme.md)
