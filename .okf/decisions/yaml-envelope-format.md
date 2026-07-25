---
type: Decision
title: The YAML envelope adds a format — it does not replace the legacy sequence reader
description: DeckYaml's reader sniffs document shape and supports both the envelope and the legacy bare card sequence; deleting the legacy path passes every test in the new format and silently breaks Razz at build time.
tags: [decision, yaml, serialization, decks, compatibility]
timestamp: 2026-07-25T00:00:00Z
---

# Decision

`DeckYaml::from_yaml` **sniffs document shape** before deserializing:

* a YAML **mapping** → the v1 envelope (`version`, `name`, `fluent_deck_key`,
  `count`, `cards`)
* a YAML **sequence** → the legacy bare `Vec<BasicCard>` list, parsed with an
  empty `name`
* anything else (a scalar) → `CardError::YamlMalformed`

Both branches are permanently supported. The envelope is a strict **superset**
of the old format, not a migration away from it.
`BasicCard::cards_from_yaml_str` keeps its exact signature and is routed
through this reader, so it transparently gained envelope support.

# Why

**`src/basic/decks/yaml/razz.yaml` is a bare sequence, and `Razz::base_vec()`
parses it at build time via `include_str!`.** Deleting the legacy branch as
"dead code left over from the old format" would compile, would pass every test
written against the envelope, and would break the `Razz` deck — the failure
surfacing as an empty deck plus a `log::error!`, not a compile error, because
`base_vec()` swallows the parse error into `Vec::default()`.

So the two code paths look redundant and are not. A "clean up the duplicate
YAML readers" refactor is exactly the change this note exists to stop.

Secondary reason: the envelope carries deck **identity**. A bare list cannot
say which deck it is, which is why `DeckKind::from_yaml` rejects a legacy
sequence with `YamlUnknownDeck("")` — it has nothing to match against
`all()`. Identity is what makes `validate_yaml`, the golden fixtures, and
round-tripping through the registry possible at all.

# How to apply

* **Never delete the sequence branch** of `DeckYaml::from_yaml`, and never
  rewrite `razz.yaml` into envelope form "for consistency". The `razz.yaml`
  bytes are load-bearing; `tests/yaml_errors.rs` and the `Razz` fixture tests
  are the tripwires.
* Keep `cards_from_yaml_str`'s signature and contract stable — it is the
  crate's oldest YAML entry point and public API.
* New format versions bump the envelope's `version` field; they do not remove
  readers.
* Preserve the **empty-list asymmetry**: `Pile::from_yaml` accepts `cards: []`
  (a fully-drawn pile is legitimate), while `DeckKind::from_yaml` and
  `validate_yaml` reject it with `YamlEmptyDeck`. Harmonizing them
  reintroduces the silent-empty-deck failure the `Razz` comment documents.
* Validate membership, not cardinality, in `Pile::from_yaml` — `French::decks(4)`
  is 216 cards and a dealt hand is five.
* Golden fixtures under `tests/fixtures/yaml/` are compared **byte-for-byte**,
  so `serde_norway` formatting is part of the contract. When a dependency bump
  reformats output, regenerate with `make yaml-fixtures`, review the diff, and
  note it in the CHANGELOG — do not loosen the check to parse-and-compare, as
  that is precisely the drift it is there to catch.

# Citations

[1] [src/basic/types/deck_yaml.rs](../../src/basic/types/deck_yaml.rs)
[2] [src/basic/decks/razz.rs `base_vec()`](../../src/basic/decks/razz.rs)
[3] [src/basic/decks/yaml/razz.yaml](../../src/basic/decks/yaml/razz.yaml)
[4] [EPIC-03 YAML Deck Serialization](../../docs/EPIC-03_Yaml_Deck_Serialization.md)
[5] [Creating a custom deck](/decks/extending-decks.md)
