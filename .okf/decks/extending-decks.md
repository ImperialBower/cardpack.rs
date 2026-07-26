---
type: Playbook
title: Creating a custom deck
description: Consumers author new deck vocabularies with the same machinery as shipped decks — implement DeckedBase on a marker struct, then Decked for free methods.
tags: [decks, extension, playbook]
timestamp: 2026-07-25T00:00:00Z
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

# YAML is free for your deck too

`YamlDecked` is **blanket-implemented for every `DeckedBase`**, so step 2 above
is all a consumer deck needs to get YAML — there is no second trait to write
and deliberately no override hook, because one shared format is what makes the
golden fixtures and `DeckKind::from_yaml` meaningful:

* `YourDeck::to_yaml()` — the canonical card list as an envelope document.
* `YourDeck::deck_from_yaml(&s)` — parse to `Vec<BasicCard>`; does **not**
  check deck identity.
* `YourDeck::validate_yaml(&s)` — the YAML analogue of `validate()`. This is
  the check to run in your test suite.

## The envelope format

Documents are self-describing rather than a bare card list, so a file can be
checked against the deck it claims to be:

```yaml
version: 1
name: Tiny            # matched against DeckedBase::deck_name()
fluent_deck_key: french
count: 4              # must equal cards.len(), or YamlCountMismatch
cards:
- suit: { weight: 3, pip_type: Suit, index: 'S', symbol: '♠', value: 4 }
  rank: { weight: 12, pip_type: Rank, index: 'A', symbol: 'A', value: 11 }
```

The reader sniffs document shape: a mapping is an envelope, a sequence is the
legacy bare list (parsed with an empty `name`, so identity falls to the card
comparison). Both remain supported — see
[the envelope decision](/decisions/yaml-envelope-format.md), which explains why
the legacy path **must not** be deleted.

`Pile<T>::to_yaml`/`from_yaml` serialize an *instance* — order preserved, empty
allowed, multi-deck allowed. `DeckKind::to_yaml`/`from_yaml` cover the
non-generic, runtime-known case, but only for decks in the shipped registry;
a consumer deck uses the `YamlDecked` path.

## Validate, don't assume

`validate_yaml` exists because `src/basic/decks/yaml/razz_bad.yml` is the
cautionary tale: perfectly well-formed YAML that parses without complaint and
is still the wrong deck. Well-formed ≠ correct. `tests/yaml_errors.rs` pins
that case.

# Citations

[1] [src/lib.rs "Custom Deck example" and "Decks as YAML"](../../src/lib.rs)
[2] [DECON-08 Extension and Registry](/references/decon-08-extension-and-registry.md)
[3] [EPIC-03 YAML Deck Serialization](/references/epic-03-yaml-serialization.md)
