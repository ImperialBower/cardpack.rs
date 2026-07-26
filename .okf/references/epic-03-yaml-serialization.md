---
type: Reference
title: EPIC-03 — YAML deck serialization
description: The DeckYaml envelope, the YamlDecked blanket trait, and the three-layer test suite that makes every deck round-trip through YAML.
tags: [epic, yaml, serialization, closed]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-03_Yaml_Deck_Serialization.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

Making every deck round-trip `deck → YAML → deck`: the self-describing
`DeckYaml` envelope (`version`, `name`, `fluent_deck_key`, `count`, `cards`),
the `YamlDecked` blanket trait over `DeckedBase`, symmetric `to_yaml` /
`from_yaml` on both `DeckKind` and `Pile<T>`, and six gated `CardError`
variants — all behind the existing `yaml` feature.

# Authoritative for

* **The envelope's shape and its back-compat rule.** The load-bearing
  consequence is captured separately in
  [YAML envelope format](/decisions/yaml-envelope-format.md): the legacy
  bare-sequence reader must not be deleted, because `Razz` depends on it at
  build time.
* The three-layer test plan — round-trip, byte-level golden fixtures, and
  negative paths — including the `razz_bad.yml` failure mode.

# In-repo path

`docs/EPIC-03_Yaml_Deck_Serialization.md`

This concept is a pointer, not a copy — the linked document is authoritative.
