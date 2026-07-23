---
type: Playbook
title: Creating a custom deck
description: Consumers author new deck vocabularies with the same machinery as shipped decks — implement DeckedBase on a marker struct, then Decked for free methods.
tags: [decks, extension, playbook]
timestamp: 2026-07-22T13:10:00Z
---

# Steps

1. Define a marker struct (e.g. `struct Tiny {}`) with a `const DECK: [BasicCard; N]`
   — reuse shipped consts like `FrenchBasicCard::ACE_SPADES` or define new
   `Pip`s with your own weights.
2. Implement `DeckedBase`: `base_vec()`, `colors()`, `deck_name()`,
   `fluent_deck_key()` (reuse e.g. `FLUENT_KEY_BASE_NAME_FRENCH` if names map
   onto an existing locale set).
3. `impl Decked<Tiny> for Tiny {}` — unlocks `Tiny::deck()`, `decks(n)`,
   `validate()`. `Ranged` (combos) comes automatically.
4. Optionally add a `macro_rules!` helper for terse card literals.
5. **Verify with `YourDeck::validate()`** — the fundamental correctness test.

The full worked example lives in the `src/lib.rs` doc comment ("Custom Deck
example") and `src/basic/decks/tiny.rs` is the in-tree version.

# Alternative: decks from YAML

With the `yaml` feature, deck data can live in a YAML file deserialized by
`BasicCard::cards_from_yaml_str` — pair with `include_str!` for a pure,
compile-time-embedded deck (this is exactly how
[`Razz`](/decks/deck-catalog.md) works). Sorting behavior (e.g. Ace-low) is
controlled purely by the weights in the data
([card model](/architecture/card-model.md)).

# Citations

[1] [src/lib.rs "Custom Deck example"](../../src/lib.rs)
[2] [DECON-08 Extension and Registry](../../docs/deconstruct/DECON-08_Extension_And_Registry.md)
