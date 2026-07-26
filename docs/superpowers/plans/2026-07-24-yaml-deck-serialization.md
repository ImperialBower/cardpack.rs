# YAML Deck Serialization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every one of cardpack's 14 shipped decks round-trip through YAML — `deck → YAML → deck` — via a self-describing envelope format, a blanket trait so custom decks get it free, and a three-layer integration test suite under `tests/`.

**Architecture:** A new `DeckYaml` envelope struct wraps the existing `Vec<BasicCard>` payload with `version`/`name`/`fluent_deck_key`/`count` metadata. A `YamlDecked` blanket trait over `DeckedBase` gives every deck type `to_yaml`/`deck_from_yaml`/`validate_yaml`; `DeckKind` and `Pile<T>` get their own symmetric pairs for the registry and instance paths. All new API is behind the existing `yaml` feature, and no `serde_norway` type appears in any public signature.

**Tech Stack:** Rust 2024 edition, MSRV 1.85, `serde_norway` 0.9, `thiserror` 2.0, `proptest` 1.x, `cargo nextest`.

Spec: [`docs/EPIC-03_Yaml_Deck_Serialization.md`](../../EPIC-03_Yaml_Deck_Serialization.md)

---

## Global Constraints

These apply to **every** task. Re-read them before each one.

- **Feature gate:** all new library API carries `#[cfg(feature = "yaml")]`. `cargo build --no-default-features` must stay green after every commit.
- **no_std discipline:** the library is `no_std + alloc`. Use `alloc::string::String`, `alloc::vec::Vec`, `core::fmt`, `core::error::Error` — **never** `std::` in `src/` except behind an existing gate. Examples and tests may use `std`.
- **No format crate in public signatures.** `serde_norway::Error` must never appear in a public type. Return `Result<T, Box<dyn Error>>` — this is the existing convention at `src/basic/types/basic_card.rs:109`.
- **No new filesystem I/O in `src/`.** The only fs seam is `cards_from_yaml_file` behind `std-io`. The fixture generator is an `examples/` binary.
- **Clippy is pedantic-clean:** CI runs `cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic`. In particular every public fallible fn needs an `# Errors` doc section, and every public non-mutating fn that returns a value needs `#[must_use]` where clippy asks.
- **Test naming:** this repo uses `snake_case__with_double_underscores` for test names and `#[allow(non_snake_case)]` at module/file level. Follow `src/basic/decks/registry.rs:274-288` and `tests/properties.rs:16-17`.
- **Commit style:** Conventional Commits (`feat:`, `test:`, `docs:`, `refactor:`). Recent history: `git log --oneline -5`.
- **Do NOT add a self dev-dependency** to `Cargo.toml` to make examples flag-free — it breaks `cargo deny check bans`. Use the `cargo ex` alias. See `Cargo.toml:85-92`.

### Verified format facts

Captured from real `serde_norway` output during planning — do **not** re-derive these from the hand-authored `src/basic/decks/yaml/razz.yaml`, which uses a different field order:

- `Pip` serializes in **declaration order**: `weight`, `pip_type`, `index`, `symbol`, `value` (`src/basic/types/pips.rs:79-85`).
- `char` fields emit single-quoted: `index: 'S'`, `symbol: '♠'`.
- `PipType` emits as a bare variant name: `Suit`, `Rank`.
- Block sequences under a mapping key are **not** extra-indented: `cards:\n- suit:`.
- `serde_norway::to_string` output **ends with a newline**.
- `fluent_deck_key()` values are lowercase (`"french"`, `src/basic/decks/cards/french.rs:8`), while `deck_name()` values are title-case (`"French"`, `"Standard 52"`).

### File structure

| File | Responsibility | Task |
|---|---|---|
| `src/common/errors.rs` | Six new gated `CardError` variants | 1 |
| `src/basic/types/deck_yaml.rs` (new) | `DeckYaml` envelope + shape-sniffing reader | 2 |
| `src/basic/types.rs` | Module declaration | 2 |
| `src/basic/types/basic_card.rs` | Reroute `cards_from_yaml_str` | 3 |
| `src/basic/types/traits.rs` | `YamlDecked` trait + blanket impl | 4 |
| `src/basic/decks/registry.rs` | `DeckKind::to_yaml` / `from_yaml` | 5 |
| `src/basic/types/pile.rs` | `Pile<T>::to_yaml` / `from_yaml` | 6 |
| `examples/yaml_decks.rs` (new) | Fixture generator (crate consumer) | 7 |
| `tests/fixtures/yaml/*.yaml` (new) | 14 golden fixtures | 7 |
| `tests/yaml_roundtrip.rs` (new) | Round-trip layer | 8 |
| `tests/yaml_golden.rs` (new) | Byte-stability layer | 9 |
| `tests/yaml_errors.rs` (new) | Negative layer | 10 |
| `src/prelude.rs`, `CHANGELOG.md`, `README.md`, `.okf/**` | Docs + exports | 11 |

---

## Task 1: `CardError` YAML variants

**Files:**
- Modify: `src/common/errors.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `CardError::{YamlCountMismatch, YamlDeckMismatch, YamlUnknownDeck, YamlEmptyDeck, YamlForeignCard, YamlMalformed}`, all `#[cfg(feature = "yaml")]`. Every later task boxes these into `Box<dyn Error>` and every negative test downcasts to them.

`CardError` currently derives `Error, Debug, Eq, PartialEq` (`src/common/errors.rs:4`) and imports only `alloc::string::String`. The new variants must not break either property — so they carry `String`/`usize` payloads only, never a `serde_norway::Error`.

- [ ] **Step 1: Write the failing test**

Append to `src/common/errors.rs` (the file currently has no test module — create one):

```rust
#[cfg(test)]
#[allow(non_snake_case)]
mod common__errors_tests {
    use super::*;
    use alloc::string::ToString;

    #[cfg(feature = "yaml")]
    #[test]
    fn yaml_variants__display() {
        assert_eq!(
            CardError::YamlCountMismatch { declared: 52, actual: 51 }.to_string(),
            "YAML deck count mismatch: header says `52`, found `51` cards"
        );
        assert_eq!(
            CardError::YamlDeckMismatch {
                expected: "Tarot".to_string(),
                found: "French".to_string()
            }
            .to_string(),
            "YAML deck mismatch: document is `French`, expected `Tarot`"
        );
        assert_eq!(
            CardError::YamlUnknownDeck("Bicycle".to_string()).to_string(),
            "Unknown deck in YAML: `Bicycle`"
        );
        assert_eq!(CardError::YamlEmptyDeck.to_string(), "YAML document has no cards");
        assert_eq!(
            CardError::YamlForeignCard {
                deck: "Tarot".to_string(),
                card: "AS".to_string()
            }
            .to_string(),
            "Card `AS` is not part of the `Tarot` deck"
        );
        assert_eq!(
            CardError::YamlMalformed.to_string(),
            "YAML document is neither a deck envelope nor a card sequence"
        );
    }

    /// Pins the two properties the whole error design depends on: a
    /// `serde_norway::Error` could not be embedded without breaking these,
    /// which is why parse failures stay boxed instead.
    #[cfg(feature = "yaml")]
    #[test]
    fn yaml_variants__stay_eq() {
        assert_eq!(CardError::YamlEmptyDeck, CardError::YamlEmptyDeck);
        assert_ne!(CardError::YamlEmptyDeck, CardError::YamlMalformed);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib common__errors_tests`
Expected: FAIL — `no variant named YamlCountMismatch found for enum CardError`.

- [ ] **Step 3: Write minimal implementation**

Add to the `CardError` enum in `src/common/errors.rs`, after `TooManyCards`:

```rust
    #[cfg(feature = "yaml")]
    #[error("YAML deck count mismatch: header says `{declared}`, found `{actual}` cards")]
    YamlCountMismatch { declared: usize, actual: usize },

    #[cfg(feature = "yaml")]
    #[error("YAML deck mismatch: document is `{found}`, expected `{expected}`")]
    YamlDeckMismatch { expected: String, found: String },

    #[cfg(feature = "yaml")]
    #[error("Unknown deck in YAML: `{0}`")]
    YamlUnknownDeck(String),

    #[cfg(feature = "yaml")]
    #[error("YAML document has no cards")]
    YamlEmptyDeck,

    #[cfg(feature = "yaml")]
    #[error("Card `{card}` is not part of the `{deck}` deck")]
    YamlForeignCard { deck: String, card: String },

    #[cfg(feature = "yaml")]
    #[error("YAML document is neither a deck envelope nor a card sequence")]
    YamlMalformed,
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features yaml --lib common__errors_tests`
Expected: PASS — 2 tests.

- [ ] **Step 5: Verify the purity gate still holds**

Run: `cargo build --no-default-features && cargo test --no-default-features --lib`
Expected: both succeed. The new variants compile out entirely without `yaml`.

- [ ] **Step 6: Commit**

```bash
git add src/common/errors.rs
git commit -m "feat: add gated CardError variants for YAML deck failures"
```

---

## Task 2: `DeckYaml` envelope + shape-sniffing reader

**Files:**
- Create: `src/basic/types/deck_yaml.rs`
- Modify: `src/basic/types.rs`

**Interfaces:**
- Consumes: `CardError::{YamlCountMismatch, YamlMalformed}` from Task 1.
- Produces:
  ```rust
  pub struct DeckYaml { pub version: u8, pub name: String, pub fluent_deck_key: String, pub count: usize, pub cards: Vec<BasicCard> }
  impl DeckYaml {
      pub const VERSION: u8 = 1;
      pub fn from_decked<T: DeckedBase>() -> Self;
      pub fn new(name: String, fluent_deck_key: String, cards: Vec<BasicCard>) -> Self;
      pub fn to_yaml(&self) -> Result<String, Box<dyn Error>>;
      pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>>;
  }
  ```
  Tasks 3–7 all call `to_yaml`/`from_yaml`; Tasks 4–6 call `from_decked`/`new`.

- [ ] **Step 1: Write the failing test**

Create `src/basic/types/deck_yaml.rs` with **only** the test module for now:

```rust
#[cfg(test)]
#[allow(non_snake_case)]
mod basic__types__deck_yaml_tests {
    use super::*;
    use crate::basic::decks::french::French;
    use alloc::string::ToString;

    fn french_yaml() -> String {
        DeckYaml::from_decked::<French>().to_yaml().unwrap()
    }

    #[test]
    fn from_decked__captures_metadata() {
        let dy = DeckYaml::from_decked::<French>();

        assert_eq!(dy.version, DeckYaml::VERSION);
        assert_eq!(dy.name, "French");
        assert_eq!(dy.fluent_deck_key, "french");
        assert_eq!(dy.count, 54);
        assert_eq!(dy.cards.len(), 54);
    }

    #[test]
    fn envelope__roundtrips() {
        let original = DeckYaml::from_decked::<French>();
        let parsed = DeckYaml::from_yaml(&french_yaml()).unwrap();

        assert_eq!(original, parsed);
    }

    /// The serialized form must be the envelope, not a bare sequence.
    #[test]
    fn to_yaml__emits_envelope_header() {
        let yml = french_yaml();

        assert!(yml.starts_with("version: 1\nname: French\n"), "got:\n{yml}");
        assert!(yml.contains("\ncount: 54\ncards:\n- suit:\n"), "got:\n{yml}");
    }

    /// A legacy bare sequence still parses, and is marked by an empty `name`.
    #[test]
    fn legacy_sequence__parses_with_empty_name() {
        let legacy = serde_norway::to_string(&French::base_vec()).unwrap();
        let parsed = DeckYaml::from_yaml(&legacy).unwrap();

        assert_eq!(parsed.name, "");
        assert_eq!(parsed.fluent_deck_key, "");
        assert_eq!(parsed.count, 54);
        assert_eq!(parsed.cards, French::base_vec());
    }

    /// Flow style is still a sequence — this is why we sniff the parsed
    /// `Value` rather than the raw text.
    #[test]
    fn legacy_flow_sequence__parses() {
        let flow = "[{suit: {weight: 3, pip_type: Suit, index: 'S', symbol: 'S', value: 4}, \
                    rank: {weight: 12, pip_type: Rank, index: 'A', symbol: 'A', value: 14}}]";
        let parsed = DeckYaml::from_yaml(flow).unwrap();

        assert_eq!(parsed.count, 1);
        assert_eq!(parsed.name, "");
    }

    #[test]
    fn count_mismatch__errors() {
        let mut dy = DeckYaml::from_decked::<French>();
        dy.count = 53;
        let yml = dy.to_yaml().unwrap();

        let err = DeckYaml::from_yaml(&yml).unwrap_err();
        let card_err = err.downcast_ref::<CardError>().expect("should be a CardError");

        assert_eq!(
            *card_err,
            CardError::YamlCountMismatch { declared: 53, actual: 54 }
        );
    }

    #[test]
    fn scalar_document__errors_malformed() {
        let err = DeckYaml::from_yaml("just a string").unwrap_err();
        let card_err = err.downcast_ref::<CardError>().expect("should be a CardError");

        assert_eq!(*card_err, CardError::YamlMalformed);
    }

    #[test]
    fn garbage__errors() {
        assert!(DeckYaml::from_yaml("{{{ not yaml").is_err());
    }

    /// The shipped `razz.yaml` is a legacy bare sequence and must keep
    /// parsing — `Razz::base_vec()` depends on it at build time.
    #[test]
    fn shipped_razz_yaml__still_parses() {
        let parsed = DeckYaml::from_yaml(include_str!("../decks/yaml/razz.yaml")).unwrap();

        assert_eq!(parsed.count, 52);
        assert_eq!(parsed.name, "", "razz.yaml is intentionally still legacy-format");
    }

    #[test]
    fn new__computes_count() {
        let dy = DeckYaml::new("X".to_string(), "x".to_string(), French::base_vec());

        assert_eq!(dy.count, 54);
        assert_eq!(dy.version, DeckYaml::VERSION);
    }
}
```

Then add the module declaration to `src/basic/types.rs`, alphabetically between `combos` and `pile`:

```rust
#[cfg(feature = "yaml")]
pub mod deck_yaml;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib basic__types__deck_yaml_tests`
Expected: FAIL to compile — `cannot find type DeckYaml in this scope`.

- [ ] **Step 3: Write minimal implementation**

Prepend to `src/basic/types/deck_yaml.rs` (above the test module):

```rust
//! A deck rendered as a self-describing YAML document.
//!
//! The envelope carries deck *identity* — `name`, `fluent_deck_key` — which a
//! bare card sequence cannot. That is what makes
//! [`DeckKind::from_yaml`](crate::basic::decks::registry::DeckKind::from_yaml)
//! possible without content-matching against every shipped deck.
//!
//! Reading accepts both the envelope and the legacy bare sequence; writing
//! always produces the envelope. See
//! `.okf/decisions/yaml-envelope-format.md` — the legacy reader is load-bearing
//! and must not be removed.

use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use serde::{Deserialize, Serialize};

/// A deck serialized as YAML: metadata header plus the card list.
///
/// ```
/// use cardpack::prelude::*;
///
/// let dy = DeckYaml::from_decked::<French>();
///
/// assert_eq!(dy.name, "French");
/// assert_eq!(dy.count, 54);
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeckYaml {
    /// Format version. `1` for every document this crate writes.
    pub version: u8,
    /// `DeckedBase::deck_name()` — title-case, e.g. `"Standard 52"`. Empty when
    /// the document came from a legacy bare sequence.
    pub name: String,
    /// `DeckedBase::fluent_deck_key()` — lowercase, e.g. `"french"`.
    pub fluent_deck_key: String,
    /// Redundant with `cards.len()` on purpose: a truncation guard.
    pub count: usize,
    pub cards: Vec<BasicCard>,
}

impl DeckYaml {
    /// The only format version this crate writes.
    pub const VERSION: u8 = 1;

    /// Build an envelope from a deck type's canonical card list.
    #[must_use]
    pub fn from_decked<T: DeckedBase>() -> Self {
        Self::new(T::deck_name(), T::fluent_deck_key(), T::base_vec())
    }

    /// Build an envelope from an explicit, ordered card list.
    ///
    /// `count` is derived from `cards`, never passed in — the guard is only
    /// meaningful if writers cannot get it wrong.
    #[must_use]
    pub fn new(name: String, fluent_deck_key: String, cards: Vec<BasicCard>) -> Self {
        Self {
            version: Self::VERSION,
            name,
            fluent_deck_key,
            count: cards.len(),
            cards,
        }
    }

    /// Serialize to a YAML envelope document.
    ///
    /// # Errors
    ///
    /// Propagates the serializer's error, boxed so no format type leaks into
    /// the public API.
    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_norway::to_string(self)?)
    }

    /// Parse YAML in **either** the envelope or the legacy bare-sequence form.
    ///
    /// The document is parsed once into a `serde_norway::Value` and dispatched
    /// on its shape — a mapping is an envelope, a sequence is a legacy card
    /// list. Sniffing the parsed value rather than the raw text handles flow
    /// style and leading comments, and yields one precise error instead of
    /// "both parse attempts failed".
    ///
    /// # Errors
    ///
    /// Malformed YAML, a document that is neither a mapping nor a sequence
    /// ([`CardError::YamlMalformed`]), or a `count` that disagrees with
    /// `cards.len()` ([`CardError::YamlCountMismatch`]).
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let value: serde_norway::Value = serde_norway::from_str(yaml_str)?;

        let deck_yaml = if value.is_mapping() {
            let parsed: Self = serde_norway::from_value(value)?;
            parsed
        } else if value.is_sequence() {
            let cards: Vec<BasicCard> = serde_norway::from_value(value)?;
            Self {
                version: Self::VERSION,
                name: String::new(),
                fluent_deck_key: String::new(),
                count: cards.len(),
                cards,
            }
        } else {
            return Err(Box::new(CardError::YamlMalformed));
        };

        if deck_yaml.count != deck_yaml.cards.len() {
            return Err(Box::new(CardError::YamlCountMismatch {
                declared: deck_yaml.count,
                actual: deck_yaml.cards.len(),
            }));
        }

        Ok(deck_yaml)
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features yaml --lib basic__types__deck_yaml_tests`
Expected: PASS — 9 tests.

If `to_yaml__emits_envelope_header` fails, print the actual output and adjust the *assertion* to match real `serde_norway` behavior — never adjust the implementation to chase a guessed format.

- [ ] **Step 5: Run the purity and lint gates**

Run:
```bash
cargo build --no-default-features
cargo clippy --features yaml -- -Dclippy::all -Dclippy::pedantic
cargo fmt --all -- --check
```
Expected: all three clean.

- [ ] **Step 6: Commit**

```bash
git add src/basic/types/deck_yaml.rs src/basic/types.rs
git commit -m "feat: add DeckYaml envelope with back-compatible sequence reader"
```

---

## Task 3: Reroute `cards_from_yaml_str` through `DeckYaml`

**Files:**
- Modify: `src/basic/types/basic_card.rs:103-113`

**Interfaces:**
- Consumes: `DeckYaml::from_yaml` from Task 2.
- Produces: `BasicCard::cards_from_yaml_str` — unchanged signature, now accepting envelopes as a strict superset.

This is a behavior-preserving widening. The existing signature and its callers (`src/basic/decks/razz.rs:36`, `src/basic/decks/cards/french.rs:398`, and the `std-io` file reader at `basic_card.rs:100`) must all keep working untouched.

- [ ] **Step 1: Write the failing test**

Add to the existing `mod basic__types__basic_card_tests` in `src/basic/types/basic_card.rs`:

```rust
    /// The reroute widens `cards_from_yaml_str`: envelopes now parse too,
    /// while every existing bare-sequence caller is unaffected.
    #[cfg(feature = "yaml")]
    #[test]
    fn cards_from_yaml_str__accepts_envelope() {
        use crate::basic::decks::french::French;
        use crate::basic::types::deck_yaml::DeckYaml;

        let envelope = DeckYaml::from_decked::<French>().to_yaml().unwrap();
        let cards = BasicCard::cards_from_yaml_str(&envelope).unwrap();

        assert_eq!(cards, French::base_vec());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn cards_from_yaml_str__still_accepts_bare_sequence() {
        let cards =
            BasicCard::cards_from_yaml_str(include_str!("../decks/yaml/razz.yaml")).unwrap();

        assert_eq!(cards.len(), 52);
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib cards_from_yaml_str__accepts_envelope`
Expected: FAIL — the current impl does `serde_norway::from_str::<Vec<BasicCard>>`, which cannot parse a mapping. Error mentions `invalid type: map`.

- [ ] **Step 3: Write minimal implementation**

Replace the body of `cards_from_yaml_str` in `src/basic/types/basic_card.rs`, and extend its doc comment:

```rust
    /// Takes in a YAML string and returns a vector of `BasicCards`.
    ///
    /// Accepts **both** supported document shapes: the
    /// [`DeckYaml`](crate::basic::types::deck_yaml::DeckYaml) envelope and the
    /// legacy bare sequence of cards (which is what
    /// `src/basic/decks/yaml/razz.yaml` still uses). Envelope metadata is
    /// discarded — use `DeckYaml::from_yaml` when you need the deck's name.
    ///
    /// # Errors
    ///
    /// Throws an error for invalid data, or for an envelope whose `count`
    /// disagrees with its card list.
    #[cfg(feature = "yaml")]
    pub fn cards_from_yaml_str(yaml_str: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        Ok(crate::basic::types::deck_yaml::DeckYaml::from_yaml(yaml_str)?.cards)
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --features yaml --lib basic_card`
Expected: PASS, including the pre-existing `cards_from_yaml_file` test (it runs only under `std-io`).

- [ ] **Step 5: Verify no caller regressed**

Run:
```bash
cargo test --features full --lib
cargo test --features std-io --lib
cargo test --doc --features full
```
Expected: all green. `Razz::deck()` still yields 52 cards; `french.rs`'s `serde__deck` test still passes.

- [ ] **Step 6: Commit**

```bash
git add src/basic/types/basic_card.rs
git commit -m "refactor: route cards_from_yaml_str through DeckYaml, adding envelope support"
```

---

## Task 4: `YamlDecked` blanket trait

**Files:**
- Modify: `src/basic/types/traits.rs`

**Interfaces:**
- Consumes: `DeckYaml` (Task 2), `CardError::{YamlEmptyDeck, YamlDeckMismatch}` (Task 1).
- Produces:
  ```rust
  pub trait YamlDecked: DeckedBase {
      fn to_yaml() -> Result<String, Box<dyn Error>>;
      fn deck_from_yaml(yaml_str: &str) -> Result<Vec<BasicCard>, Box<dyn Error>>;
      fn validate_yaml(yaml_str: &str) -> Result<(), Box<dyn Error>>;
  }
  impl<T: DeckedBase> YamlDecked for T {}
  ```
  Tasks 8–10 call all three. Note these are **associated functions with no `self`** — call them as `French::to_yaml()`.

`validate_yaml` returning `Result<(), _>` rather than `bool` is a deliberate departure from `Decked::validate()` (`traits.rs:135`), which returns `bool`. The negative tests in Task 10 need to assert *which* failure occurred, and a `bool` cannot carry that.

- [ ] **Step 1: Write the failing test**

Add to the existing test module `basic__types__traits_tests` at the bottom of `src/basic/types/traits.rs` (`traits.rs:474`), as a nested module:

```rust
    #[cfg(feature = "yaml")]
    #[allow(non_snake_case)]
    mod yaml_decked_tests {
        use super::*;
        use crate::basic::decks::french::French;
        use crate::basic::decks::tarot::Tarot;
        use crate::common::errors::CardError;

        #[test]
        fn to_yaml__then_deck_from_yaml__roundtrips() {
            let yml = French::to_yaml().unwrap();

            assert_eq!(French::deck_from_yaml(&yml).unwrap(), French::base_vec());
        }

        #[test]
        fn validate_yaml__accepts_own_deck() {
            assert!(French::validate_yaml(&French::to_yaml().unwrap()).is_ok());
        }

        #[test]
        fn validate_yaml__rejects_other_deck() {
            let err = Tarot::validate_yaml(&French::to_yaml().unwrap()).unwrap_err();
            let card_err = err.downcast_ref::<CardError>().unwrap();

            assert_eq!(
                *card_err,
                CardError::YamlDeckMismatch {
                    expected: "Tarot".to_string(),
                    found: "French".to_string(),
                }
            );
        }

        #[test]
        fn validate_yaml__rejects_empty() {
            let empty = "version: 1\nname: French\nfluent_deck_key: french\ncount: 0\ncards: []\n";
            let err = French::validate_yaml(empty).unwrap_err();

            assert_eq!(*err.downcast_ref::<CardError>().unwrap(), CardError::YamlEmptyDeck);
        }

        /// A legacy sequence has no name to check, so identity falls through
        /// to a card-by-card comparison against `base_vec()`.
        #[test]
        fn validate_yaml__accepts_legacy_sequence_of_own_cards() {
            let legacy = serde_norway::to_string(&French::base_vec()).unwrap();

            assert!(French::validate_yaml(&legacy).is_ok());
        }

        #[test]
        fn validate_yaml__rejects_legacy_sequence_of_other_cards() {
            let legacy = serde_norway::to_string(&Tarot::base_vec()).unwrap();

            assert!(French::validate_yaml(&legacy).is_err());
        }

        /// The blanket impl must reach a type that implements only
        /// `DeckedBase` — this is what gives consumer-authored decks YAML
        /// support for free.
        #[test]
        fn blanket_impl__reaches_custom_decks() {
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            struct OneCard {}

            impl DeckedBase for OneCard {
                fn base_vec() -> Vec<BasicCard> {
                    vec![crate::basic::decks::cards::french::FrenchBasicCard::ACE_SPADES]
                }
                #[cfg(feature = "colored-display")]
                fn colors() -> HashMap<Pip, colored::Color> {
                    HashMap::default()
                }
                fn deck_name() -> String {
                    "One Card".to_string()
                }
                fn fluent_deck_key() -> String {
                    FLUENT_KEY_BASE_NAME_FRENCH.to_string()
                }
            }

            let yml = OneCard::to_yaml().unwrap();

            assert!(yml.contains("name: One Card"));
            assert_eq!(OneCard::deck_from_yaml(&yml).unwrap().len(), 1);
            assert!(OneCard::validate_yaml(&yml).is_ok());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib yaml_decked_tests`
Expected: FAIL to compile — `no function or associated item named to_yaml found`.

- [ ] **Step 3: Write minimal implementation**

Add these imports at the top of `src/basic/types/traits.rs`:

```rust
#[cfg(feature = "yaml")]
use crate::basic::types::deck_yaml::DeckYaml;
#[cfg(feature = "yaml")]
use crate::common::errors::CardError;
#[cfg(feature = "yaml")]
use alloc::boxed::Box;
#[cfg(feature = "yaml")]
use core::error::Error;
```

Then add the trait after the `Decked` trait definition:

```rust
/// YAML serialization for every deck.
///
/// Blanket-implemented for all [`DeckedBase`] types, so shipped decks *and*
/// consumer-authored ones get it with no extra work:
///
/// ```
/// use cardpack::prelude::*;
///
/// let yml = French::to_yaml().unwrap();
///
/// assert!(yml.starts_with("version: 1\nname: French"));
/// assert_eq!(French::deck_from_yaml(&yml).unwrap(), French::base_vec());
/// ```
///
/// There is deliberately no override hook — one format for all decks is what
/// makes the golden fixtures and `DeckKind::from_yaml` meaningful.
#[cfg(feature = "yaml")]
pub trait YamlDecked: DeckedBase {
    /// This deck's canonical card list as an envelope YAML document.
    ///
    /// # Errors
    ///
    /// Propagates serialization failure, boxed.
    fn to_yaml() -> Result<String, Box<dyn Error>> {
        DeckYaml::from_decked::<Self>().to_yaml()
    }

    /// Parse a YAML document — envelope or legacy sequence — into cards.
    ///
    /// This does **not** check that the cards belong to this deck; use
    /// [`validate_yaml`](Self::validate_yaml) for that.
    ///
    /// # Errors
    ///
    /// Malformed YAML or a `count` mismatch.
    fn deck_from_yaml(yaml_str: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
        Ok(DeckYaml::from_yaml(yaml_str)?.cards)
    }

    /// Verify a YAML document describes *this* deck, exactly.
    ///
    /// The YAML analogue of [`Decked::validate`]. This is the check that the
    /// deliberately-broken `src/basic/decks/yaml/razz_bad.yml` fails — a file
    /// can be perfectly well-formed and still be the wrong deck.
    ///
    /// # Errors
    ///
    /// [`CardError::YamlEmptyDeck`] for an empty card list,
    /// [`CardError::YamlDeckMismatch`] when the document names a different
    /// deck or its cards differ from `Self::base_vec()`.
    fn validate_yaml(yaml_str: &str) -> Result<(), Box<dyn Error>> {
        let deck_yaml = DeckYaml::from_yaml(yaml_str)?;

        if deck_yaml.cards.is_empty() {
            return Err(Box::new(CardError::YamlEmptyDeck));
        }

        // An empty `name` means a legacy bare sequence: there is no header to
        // check, so identity is decided by the card comparison below.
        if !deck_yaml.name.is_empty() && deck_yaml.name != Self::deck_name() {
            return Err(Box::new(CardError::YamlDeckMismatch {
                expected: Self::deck_name(),
                found: deck_yaml.name,
            }));
        }

        if deck_yaml.cards != Self::base_vec() {
            return Err(Box::new(CardError::YamlDeckMismatch {
                expected: Self::deck_name(),
                found: deck_yaml.name,
            }));
        }

        Ok(())
    }
}

#[cfg(feature = "yaml")]
impl<T: DeckedBase> YamlDecked for T {}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features yaml --lib yaml_decked_tests`
Expected: PASS — 7 tests.

- [ ] **Step 5: Run the gates**

Run:
```bash
cargo build --no-default-features
cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic
cargo fmt --all -- --check
```
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add src/basic/types/traits.rs
git commit -m "feat: add YamlDecked blanket trait so every deck serializes to YAML"
```

---

## Task 5: `DeckKind::to_yaml` / `from_yaml`

**Files:**
- Modify: `src/basic/decks/registry.rs`

**Interfaces:**
- Consumes: `DeckYaml` (Task 2), `CardError::{YamlEmptyDeck, YamlUnknownDeck}` (Task 1).
- Produces: `DeckKind::to_yaml(self) -> Result<String, Box<dyn Error>>` and `DeckKind::from_yaml(&str) -> Result<DeckKind, Box<dyn Error>>`. Tasks 8–10 use both.

`from_yaml` matches on `deck_name()`, so it depends on all 14 names being distinct. That assumption gets its own test rather than being taken on faith.

- [ ] **Step 1: Write the failing test**

Add to the existing test module `basic__decks__registry_tests` in `src/basic/decks/registry.rs` (`registry.rs:214`):

```rust
    /// `from_yaml` matches on `deck_name()`. If two decks ever share a name,
    /// that match becomes ambiguous — this test is the tripwire.
    #[cfg(feature = "yaml")]
    #[test]
    fn deck_name__all_distinct() {
        let mut names: Vec<String> = DeckKind::all().iter().map(|k| k.deck_name()).collect();
        let total = names.len();
        names.sort();
        names.dedup();

        assert_eq!(names.len(), total, "duplicate deck_name() across DeckKind");
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn to_yaml__from_yaml__roundtrips_every_kind() {
        for kind in DeckKind::all() {
            let yml = kind.to_yaml().unwrap();
            let parsed = DeckKind::from_yaml(&yml).unwrap();

            assert_eq!(parsed, *kind, "round-trip failed for {}", kind.deck_name());
        }
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn to_yaml__preserves_cards_and_metadata() {
        for kind in DeckKind::all() {
            let dy = crate::basic::types::deck_yaml::DeckYaml::from_yaml(&kind.to_yaml().unwrap())
                .unwrap();

            assert_eq!(dy.name, kind.deck_name());
            assert_eq!(dy.fluent_deck_key, kind.fluent_deck_key());
            assert_eq!(dy.cards, kind.base_vec());
            assert_eq!(dy.count, kind.base_vec().len());
        }
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn from_yaml__unknown_name__errors() {
        let yml = "version: 1\nname: Bicycle\nfluent_deck_key: french\ncount: 0\ncards: []\n";
        let err = DeckKind::from_yaml(yml).unwrap_err();

        // Empty cards is checked first, so this document trips YamlEmptyDeck.
        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlEmptyDeck
        );

        // With cards present, the unknown name is what fails.
        let mut dy = crate::basic::types::deck_yaml::DeckYaml::from_decked::<French>();
        dy.name = "Bicycle".to_string();
        let err = DeckKind::from_yaml(&dy.to_yaml().unwrap()).unwrap_err();

        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlUnknownDeck("Bicycle".to_string())
        );
    }

    /// A legacy bare sequence carries no name, so it is unidentifiable by
    /// design. Guessing a deck from its cards is a different feature.
    #[cfg(feature = "yaml")]
    #[test]
    fn from_yaml__legacy_sequence__errors() {
        let legacy = serde_norway::to_string(&French::base_vec()).unwrap();
        let err = DeckKind::from_yaml(&legacy).unwrap_err();

        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlUnknownDeck(String::new())
        );
    }
```

**Import trap — read before running.** The test module currently imports `ToString` *only* when `yaml` is off (`registry.rs:216-217`):

```rust
    #[cfg(not(feature = "yaml"))]
    use alloc::string::ToString;
```

Every new test above runs with `yaml` **on** and calls `.to_string()`, so `ToString` would not be in scope. Make that import unconditional — it is now used in both configurations, so it will not warn either way:

```rust
    use alloc::string::ToString;
```

Then add exactly these two, and nothing more:

```rust
    #[cfg(feature = "yaml")]
    use crate::basic::decks::french::French;
    #[cfg(feature = "yaml")]
    use crate::common::errors::CardError;
```

`String` and `Vec` already arrive through `use super::*` (registry.rs imports both at `:36-37`), and the tests reference `DeckYaml` by full path — so do **not** add imports for those three. This module carries only `#[allow(non_snake_case)]`; unlike the other test modules it has no `unused_imports` allow, so a redundant import here is a clippy failure, not a warning.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib basic__decks__registry`
Expected: FAIL to compile — `no function or associated item named to_yaml found for enum DeckKind`.

- [ ] **Step 3: Write minimal implementation**

Add these gated imports near the top of `src/basic/decks/registry.rs`:

```rust
#[cfg(feature = "yaml")]
use crate::basic::types::deck_yaml::DeckYaml;
#[cfg(feature = "yaml")]
use crate::common::errors::CardError;
#[cfg(feature = "yaml")]
use alloc::boxed::Box;
#[cfg(feature = "yaml")]
use core::error::Error;
```

Then add a new `impl` block after the existing `impl DeckKind`:

```rust
#[cfg(feature = "yaml")]
impl DeckKind {
    /// This deck as an envelope YAML document.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// // Every shipped deck round-trips through YAML:
    /// for kind in DeckKind::all() {
    ///     let yml = kind.to_yaml().unwrap();
    ///     assert_eq!(DeckKind::from_yaml(&yml).unwrap(), *kind);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Propagates serialization failure, boxed.
    pub fn to_yaml(self) -> Result<String, Box<dyn Error>> {
        DeckYaml::new(self.deck_name(), self.fluent_deck_key(), self.base_vec()).to_yaml()
    }

    /// Recover the `DeckKind` a YAML document describes, by matching its
    /// `name` against every shipped deck.
    ///
    /// A legacy bare sequence has no name and is therefore rejected —
    /// inferring a deck from its cards is deliberately out of scope.
    ///
    /// # Errors
    ///
    /// [`CardError::YamlEmptyDeck`] for an empty card list;
    /// [`CardError::YamlUnknownDeck`] for a name that matches no shipped deck
    /// (including the empty name of a legacy sequence).
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let deck_yaml = DeckYaml::from_yaml(yaml_str)?;

        if deck_yaml.cards.is_empty() {
            return Err(Box::new(CardError::YamlEmptyDeck));
        }

        Self::all()
            .iter()
            .find(|kind| kind.deck_name() == deck_yaml.name)
            .copied()
            .ok_or_else(|| Box::new(CardError::YamlUnknownDeck(deck_yaml.name)) as Box<dyn Error>)
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features yaml --lib basic__decks__registry`
Expected: PASS — the five new tests plus the pre-existing registry tests.

- [ ] **Step 5: Run the gates**

Run:
```bash
cargo build --no-default-features
cargo test --doc --features full
cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic
```
Expected: clean. The new doctest on `to_yaml` runs under `--features full`.

- [ ] **Step 6: Commit**

```bash
git add src/basic/decks/registry.rs
git commit -m "feat: add DeckKind::to_yaml and from_yaml for the non-generic deck path"
```

---

## Task 6: `Pile<T>::to_yaml` / `from_yaml`

**Files:**
- Modify: `src/basic/types/pile.rs`

**Interfaces:**
- Consumes: `DeckYaml` (Task 2), `CardError::{YamlDeckMismatch, YamlForeignCard}` (Task 1).
- Produces: `Pile<T>::to_yaml(&self) -> Result<String, Box<dyn Error>>` and `Pile<T>::from_yaml(&str) -> Result<Pile<T>, Box<dyn Error>>`. Task 8 uses both.

This is the **instance** path: it serializes `self.into_basic_cards()` (`pile.rs:387`), not `base_vec()`, so order is preserved and a shuffled pile round-trips. It validates card *membership*, never length — `French::decks(4)` is 216 cards and a dealt hand is five, and both must work.

- [ ] **Step 1: Write the failing test**

Add to the existing test module in `src/basic/types/pile.rs` — note it is named `basic__types__deck_tests`, **not** `..._pile_tests` (`pile.rs:1069`). It already imports `French` and `Standard52` and carries `#[allow(non_snake_case, unused_imports)]`:

```rust
    #[cfg(feature = "yaml")]
    #[allow(non_snake_case)]
    mod pile_yaml_tests {
        use super::*;
        use crate::basic::decks::french::French;
        use crate::basic::decks::standard52::Standard52;
        use crate::basic::decks::tarot::Tarot;
        use crate::common::errors::CardError;

        #[test]
        fn to_yaml__from_yaml__roundtrips_canonical_deck() {
            let deck = French::deck();

            assert_eq!(Pile::<French>::from_yaml(&deck.to_yaml().unwrap()).unwrap(), deck);
        }

        /// Order fidelity is the whole point of the instance path. A shuffle
        /// that serialized `base_vec()` would still pass an equality check
        /// against a *sorted* pile — this asserts against the shuffled one.
        #[test]
        fn to_yaml__preserves_shuffled_order() {
            let shuffled = Standard52::deck().shuffled_with_seed(42);
            let parsed = Pile::<Standard52>::from_yaml(&shuffled.to_yaml().unwrap()).unwrap();

            assert_eq!(parsed, shuffled);
            assert_ne!(parsed, Standard52::deck(), "seed 42 should not be canonical order");
        }

        #[test]
        fn roundtrips__partial_pile() {
            let mut deck = Standard52::deck();
            let hand = deck.draw(5).unwrap();

            assert_eq!(Pile::<Standard52>::from_yaml(&hand.to_yaml().unwrap()).unwrap(), hand);
        }

        /// Multi-deck piles are full of duplicates and are 4x `base_vec()`
        /// length — proof that membership, not cardinality, is the invariant.
        #[test]
        fn roundtrips__multideck_pile() {
            let quad = French::decks(4);
            assert_eq!(quad.len(), 216);

            assert_eq!(Pile::<French>::from_yaml(&quad.to_yaml().unwrap()).unwrap(), quad);
        }

        /// An empty pile is a fully-drawn deck — legitimate, unlike an empty
        /// *deck* document, which `validate_yaml` rejects.
        #[test]
        fn roundtrips__empty_pile() {
            let empty = Pile::<French>::default();

            assert_eq!(Pile::<French>::from_yaml(&empty.to_yaml().unwrap()).unwrap(), empty);
        }

        #[test]
        fn from_yaml__wrong_deck_name__errors() {
            let err = Pile::<Tarot>::from_yaml(&French::deck().to_yaml().unwrap()).unwrap_err();

            assert_eq!(
                *err.downcast_ref::<CardError>().unwrap(),
                CardError::YamlDeckMismatch {
                    expected: "Tarot".to_string(),
                    found: "French".to_string(),
                }
            );
        }

        /// Right header, wrong cards: the membership check is what catches it.
        #[test]
        fn from_yaml__foreign_card__errors() {
            let mut dy = crate::basic::types::deck_yaml::DeckYaml::from_decked::<Tarot>();
            dy.cards = French::base_vec();
            dy.count = dy.cards.len();

            let err = Pile::<Tarot>::from_yaml(&dy.to_yaml().unwrap()).unwrap_err();
            let card_err = err.downcast_ref::<CardError>().unwrap();

            assert!(
                matches!(card_err, CardError::YamlForeignCard { deck, .. } if deck == "Tarot"),
                "expected YamlForeignCard, got {card_err:?}"
            );
        }

        /// A legacy sequence has no name, so it is accepted as long as every
        /// card belongs to the deck.
        #[test]
        fn from_yaml__legacy_sequence__accepted() {
            let legacy = serde_norway::to_string(&French::deck().into_basic_cards()).unwrap();

            assert_eq!(Pile::<French>::from_yaml(&legacy).unwrap(), French::deck());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features yaml --lib pile_yaml_tests`
Expected: FAIL to compile — `no function or associated item named to_yaml found for struct Pile`.

- [ ] **Step 3: Write minimal implementation**

Add these gated imports at the top of `src/basic/types/pile.rs`:

```rust
#[cfg(feature = "yaml")]
use crate::basic::types::deck_yaml::DeckYaml;
#[cfg(feature = "yaml")]
use alloc::boxed::Box;
#[cfg(feature = "yaml")]
use core::error::Error;
```

`CardError` is already imported at `pile.rs:5`. Add the two methods inside the existing `impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> Pile<DeckType>` block:

```rust
    /// This pile's **actual** cards, in their **actual** order, as an
    /// envelope YAML document.
    ///
    /// Unlike [`YamlDecked::to_yaml`](crate::basic::types::traits::YamlDecked::to_yaml),
    /// which always writes the deck's canonical list, this writes what the
    /// pile currently holds — so a shuffled or partially-drawn pile
    /// round-trips exactly.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let shuffled = Standard52::deck().shuffled_with_seed(42);
    /// let yml = shuffled.to_yaml().unwrap();
    ///
    /// assert_eq!(Pile::<Standard52>::from_yaml(&yml).unwrap(), shuffled);
    /// ```
    ///
    /// # Errors
    ///
    /// Propagates serialization failure, boxed.
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        DeckYaml::new(
            DeckType::deck_name(),
            DeckType::fluent_deck_key(),
            self.into_basic_cards(),
        )
        .to_yaml()
    }

    /// Rebuild a pile from a YAML document.
    ///
    /// Card **membership** is validated against `DeckType::base_vec()`;
    /// length deliberately is not. A five-card hand and a 216-card
    /// `French::decks(4)` are both legal piles, and an empty pile is a
    /// fully-drawn deck.
    ///
    /// # Errors
    ///
    /// [`CardError::YamlDeckMismatch`] if the envelope names a different deck;
    /// [`CardError::YamlForeignCard`] if any card is absent from
    /// `DeckType::base_vec()`.
    #[cfg(feature = "yaml")]
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let deck_yaml = DeckYaml::from_yaml(yaml_str)?;

        // An empty `name` means a legacy bare sequence: no header to check.
        if !deck_yaml.name.is_empty() && deck_yaml.name != DeckType::deck_name() {
            return Err(Box::new(CardError::YamlDeckMismatch {
                expected: DeckType::deck_name(),
                found: deck_yaml.name,
            }));
        }

        let base = DeckType::base_vec();
        if let Some(foreign) = deck_yaml.cards.iter().find(|card| !base.contains(card)) {
            return Err(Box::new(CardError::YamlForeignCard {
                deck: DeckType::deck_name(),
                card: foreign.index(),
            }));
        }

        Ok(Self::from(deck_yaml.cards))
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features yaml --lib pile_yaml_tests`
Expected: PASS — 8 tests.

- [ ] **Step 5: Run the gates**

Run:
```bash
cargo build --no-default-features
cargo test --doc --features full
cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic
cargo fmt --all -- --check
```
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add src/basic/types/pile.rs
git commit -m "feat: add order-preserving Pile::to_yaml and from_yaml"
```

---

## Task 7: Fixture generator + 14 golden fixtures

**Files:**
- Create: `examples/yaml_decks.rs`
- Create: `tests/fixtures/yaml/*.yaml` (14 files, generated)
- Modify: `Cargo.toml` (add `[[example]]` entry)
- Modify: `Makefile` (add `yaml-fixtures` target + help line)

**Interfaces:**
- Consumes: `DeckKind::to_yaml` (Task 5).
- Produces: 14 files at `tests/fixtures/yaml/<slug>.yaml` where `slug = deck_name().to_lowercase().replace(' ', "_")`. Tasks 9 and 10 read them.

This is a **crate consumer**, not library code — it does its own `std::fs` and carries the kernel-purity clippy allow, exactly like `examples/deconstruct_vectors.rs:16-20`.

- [ ] **Step 1: Write the generator**

Create `examples/yaml_decks.rs`:

```rust
//! Dumps one golden YAML fixture per shipped deck into `tests/fixtures/yaml/`.
//! Public API only — this program is a consumer of the crate.
//!
//! # Features
//!
//! Uses `std` + `yaml`. cardpack is pure by default (`default = []`), so to use
//! these APIs in your own crate enable them explicitly:
//! `cardpack = { version = "0.9", features = ["std", "yaml"] }`
//! (`yaml` implies `serde`). Note this dumper writes files, but it does so with
//! its own `std::fs` — reading decks from a YAML *file* via cardpack would need
//! the separate `std-io` feature.
//!
//! Run it from this repo with `cargo ex yaml_decks` — the alias in
//! `.cargo/config.toml` supplies the features, so no `--features` flag is
//! needed. Or `make yaml-fixtures`.

// This example is a *consumer* of the crate (a golden-fixture dumper), not part
// of the pure kernel, so it deliberately performs filesystem I/O. The
// kernel-purity lints (clippy.toml) exist to keep the *library* pure; allow
// them for this binary only. See docs/audit-2026-07-18-domain-kernel.md.
#![allow(clippy::disallowed_types, clippy::disallowed_methods)]

use cardpack::prelude::*;
use std::fs;
use std::path::Path;

/// `"Standard 52"` -> `"standard_52"`, `"Dashavatara Ganjifa"` -> `"dashavatara_ganjifa"`.
fn slug(deck_name: &str) -> String {
    deck_name.to_lowercase().replace(' ', "_")
}

fn main() {
    let root = Path::new("tests/fixtures/yaml");
    fs::create_dir_all(root).expect("mkdir tests/fixtures/yaml");

    for kind in DeckKind::all() {
        let yaml = kind.to_yaml().expect("serialize deck");
        let path = root.join(format!("{}.yaml", slug(&kind.deck_name())));
        fs::write(&path, &yaml).expect("write fixture");
        println!("wrote {} ({} bytes)", path.display(), yaml.len());
    }

    println!("\n{} fixtures written", DeckKind::all().len());
}
```

- [ ] **Step 2: Register the example**

Add to `Cargo.toml`, in the `[[example]]` block list (keep it near the others):

```toml
[[example]]
name = "yaml_decks"
required-features = ["std", "yaml"]
```

Do **not** add a self dev-dependency — see `Cargo.toml:85-92`.

- [ ] **Step 3: Run the generator**

Run: `cargo ex yaml_decks`
Expected: 14 lines of `wrote tests/fixtures/yaml/<name>.yaml (N bytes)`, then `14 fixtures written`.

Verify the file list:

Run: `ls tests/fixtures/yaml/ | sort`
Expected exactly these 14:
```
canasta.yaml
dashavatara_ganjifa.yaml
euchre_24.yaml
euchre_32.yaml
french.yaml
mughal_ganjifa.yaml
pinochle.yaml
razz.yaml
short.yaml
skat.yaml
spades.yaml
standard_52.yaml
tarot.yaml
tiny.yaml
```

If a filename differs, the slug rule is fine — just record the real names; Task 9's test reads the directory rather than hardcoding them.

- [ ] **Step 4: Verify idempotence**

Run: `cargo ex yaml_decks && git status --short tests/fixtures/yaml/`
Expected: after the second run, only the untracked-file markers from the first run — no content changes. Sanity-check `tests/fixtures/yaml/tiny.yaml` by eye: it should be a 4-card envelope starting `version: 1\nname: Tiny\n`.

- [ ] **Step 5: Add the Makefile target**

Add near the other test targets in `Makefile`:

```make
# Regenerate the golden YAML deck fixtures read by tests/yaml_golden.rs.
# Run this whenever a deck's card data legitimately changes, then review the
# diff — an unexpected diff means deck data drifted.
yaml-fixtures:
	cargo ex yaml_decks
```

And add to the `help` target's echo list, after the `test-wasm` line:

```make
	@echo "  make yaml-fixtures   - Regenerate golden YAML deck fixtures"
```

Run: `make yaml-fixtures`
Expected: same 14-file output.

- [ ] **Step 6: Commit**

```bash
git add examples/yaml_decks.rs Cargo.toml Makefile tests/fixtures/yaml/
git commit -m "feat: add yaml_decks fixture generator and 14 golden deck fixtures"
```

---

## Task 8: `tests/yaml_roundtrip.rs`

**Files:**
- Create: `tests/yaml_roundtrip.rs`

**Interfaces:**
- Consumes: `DeckKind::to_yaml`/`from_yaml` (Task 5), `YamlDecked` (Task 4), `Pile::to_yaml`/`from_yaml` (Task 6).
- Produces: nothing consumed by later tasks.

Integration tests run against the crate's **public API only** — if something here does not compile, the export is missing, which is a genuine finding, not a test bug. Note `Tiny` is not in the prelude (see EPIC design decision 4); Task 11 adds the re-export, so until then import it by full path.

- [ ] **Step 1: Write the test file**

Create `tests/yaml_roundtrip.rs`:

```rust
//! Round-trip guarantees for YAML deck serialization: every shipped deck, and
//! every kind of `Pile`, survives `deck -> YAML -> deck` unchanged.
//!
//! Driven off `DeckKind::all()` rather than a hardcoded list, so a new deck
//! inherits the guarantee automatically.
//!
//! Skipped on `wasm32-unknown-unknown` because proptest's transitive
//! `wait-timeout` crate is unix-only, matching `tests/properties.rs`.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

use cardpack::basic::decks::tiny::Tiny;
use cardpack::prelude::*;
use proptest::prelude::*;

/// The EPIC's headline claim, for all 14 decks.
#[test]
fn every_deck_kind__roundtrips() {
    for kind in DeckKind::all() {
        let yml = kind.to_yaml().expect("serialize");
        let parsed = DeckKind::from_yaml(&yml).expect("deserialize");

        assert_eq!(parsed, *kind, "round-trip failed for {}", kind.deck_name());
    }
}

#[test]
fn every_deck_kind__preserves_cards() {
    for kind in DeckKind::all() {
        let yml = kind.to_yaml().expect("serialize");
        let cards = BasicCard::cards_from_yaml_str(&yml).expect("deserialize");

        assert_eq!(cards, kind.base_vec(), "cards differ for {}", kind.deck_name());
        assert!(!cards.is_empty(), "{} serialized to an empty deck", kind.deck_name());
    }
}

#[test]
fn every_deck_kind__preserves_metadata() {
    for kind in DeckKind::all() {
        let dy = DeckYaml::from_yaml(&kind.to_yaml().expect("serialize")).expect("deserialize");

        assert_eq!(dy.version, DeckYaml::VERSION);
        assert_eq!(dy.name, kind.deck_name());
        assert_eq!(dy.fluent_deck_key, kind.fluent_deck_key());
        assert_eq!(dy.count, kind.base_vec().len());
    }
}

/// The type-level twin of `every_deck_kind__roundtrips` — this is the path a
/// consumer's own deck takes through the blanket `YamlDecked` impl.
#[test]
fn typed_decks__roundtrip() {
    macro_rules! assert_roundtrips {
        ($($deck:ty),+ $(,)?) => {
            $(
                let yml = <$deck>::to_yaml().expect("serialize");
                assert_eq!(
                    <$deck>::deck_from_yaml(&yml).expect("deserialize"),
                    <$deck>::base_vec(),
                    "round-trip failed for {}",
                    <$deck>::deck_name()
                );
                <$deck>::validate_yaml(&yml).expect("validate");
            )+
        };
    }

    assert_roundtrips!(
        Canasta,
        Dashavatara,
        Euchre24,
        Euchre32,
        French,
        Mughal,
        Pinochle,
        Razz,
        Short,
        Skat,
        Spades,
        Standard52,
        Tarot,
        Tiny,
    );
}

#[test]
fn pile__shuffled_roundtrips_in_order() {
    let shuffled = Standard52::deck().shuffled_with_seed(42);
    let parsed = Pile::<Standard52>::from_yaml(&shuffled.to_yaml().expect("serialize"))
        .expect("deserialize");

    assert_eq!(parsed, shuffled);
    assert_ne!(parsed, Standard52::deck(), "seed 42 should not be canonical order");
}

#[test]
fn pile__partial_roundtrips() {
    let mut deck = Standard52::deck();
    let hand = deck.draw(5).expect("draw 5");

    assert_eq!(
        Pile::<Standard52>::from_yaml(&hand.to_yaml().expect("serialize")).expect("deserialize"),
        hand
    );
}

#[test]
fn pile__multideck_roundtrips() {
    let quad = French::decks(4);
    assert_eq!(quad.len(), 216);

    assert_eq!(
        Pile::<French>::from_yaml(&quad.to_yaml().expect("serialize")).expect("deserialize"),
        quad
    );
}

#[test]
fn pile__empty_roundtrips() {
    let empty = Pile::<French>::default();

    assert_eq!(
        Pile::<French>::from_yaml(&empty.to_yaml().expect("serialize")).expect("deserialize"),
        empty
    );
}

proptest! {
    /// Order fidelity across arbitrary permutations. `shuffled_with_seed` makes
    /// any failure reproducible from the printed seed.
    #[test]
    fn pile__roundtrips_for_any_seed(seed: u64) {
        let shuffled = Standard52::deck().shuffled_with_seed(seed);
        let yml = shuffled.to_yaml().expect("serialize");

        prop_assert_eq!(Pile::<Standard52>::from_yaml(&yml).expect("deserialize"), shuffled);
    }

    /// Any partial draw round-trips too, at every length from 0 to a full deck.
    #[test]
    fn pile__partial_roundtrips_for_any_draw(seed: u64, n in 0usize..=52) {
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let drawn = deck.draw(n).expect("draw n <= 52");
        let yml = drawn.to_yaml().expect("serialize");

        prop_assert_eq!(Pile::<Standard52>::from_yaml(&yml).expect("deserialize"), drawn);
    }
}
```

- [ ] **Step 2: Run to verify it fails**

Run: `cargo test --features full --test yaml_roundtrip`
Expected: FAIL to compile — `DeckYaml` and `YamlDecked` are not yet in the prelude (Task 11 adds them).

This is the test doing its job: it proves the exports are missing.

- [ ] **Step 3: Add the prelude exports**

Add to `src/prelude.rs`, keeping the existing alphabetical grouping:

```rust
#[cfg(feature = "yaml")]
pub use crate::basic::types::deck_yaml::DeckYaml;
```

and extend the traits re-export line:

```rust
pub use crate::basic::types::traits::{CKCRevised, Decked, DeckedBase, Ranged};
#[cfg(feature = "yaml")]
pub use crate::basic::types::traits::YamlDecked;
```

- [ ] **Step 4: Run to verify it passes**

Run: `cargo test --features full --test yaml_roundtrip`
Expected: PASS — 8 unit tests + 2 proptest properties (256 cases each by default).

If `typed_decks__roundtrip` fails to resolve a deck type, check the prelude: every deck except `Tiny` is re-exported, and `Tiny` is imported by full path at the top of the file.

- [ ] **Step 5: Commit**

```bash
git add tests/yaml_roundtrip.rs src/prelude.rs
git commit -m "test: add YAML round-trip integration suite for every deck"
```

---

## Task 9: `tests/yaml_golden.rs`

**Files:**
- Create: `tests/yaml_golden.rs`

**Interfaces:**
- Consumes: the 14 fixtures from Task 7, `DeckKind::to_yaml`/`from_yaml` (Task 5), `YamlDecked::validate_yaml` (Task 4).

Byte-level comparison is the point: it catches deck-data drift *and* serializer format drift, neither of which a parse-and-compare would see.

- [ ] **Step 1: Write the test file**

Create `tests/yaml_golden.rs`:

```rust
//! Golden-fixture tests: the serialized form of every shipped deck must match
//! its committed fixture **byte for byte**.
//!
//! This is deliberately stricter than a round-trip check. A round-trip passes
//! happily even if a deck's card data silently changed; a byte comparison
//! against a reviewed fixture does not. It also catches `serde_norway`
//! reformatting across dependency bumps — regenerate with
//! `make yaml-fixtures`, review the diff, and note it in the CHANGELOG.
//!
//! Needs `std::fs` to read fixtures, so it is skipped on wasm.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

// `Tiny` is the one deck not re-exported from the prelude; Task 11 fixes that,
// but the full path works either way, so this file never depends on it.
use cardpack::basic::decks::tiny::Tiny;
use cardpack::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/yaml")
}

/// `"Standard 52"` -> `"standard_52"`. Must match `examples/yaml_decks.rs`.
fn slug(deck_name: &str) -> String {
    deck_name.to_lowercase().replace(' ', "_")
}

fn fixture_path(kind: DeckKind) -> PathBuf {
    fixture_dir().join(format!("{}.yaml", slug(&kind.deck_name())))
}

/// Adding deck 15 without a fixture must fail here rather than silently
/// leaving it unguarded. Reads the registry, never a hardcoded 14.
#[test]
fn fixture_count__matches_registry() {
    let count = fs::read_dir(fixture_dir())
        .expect("tests/fixtures/yaml must exist — run `make yaml-fixtures`")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
        .count();

    assert_eq!(
        count,
        DeckKind::all().len(),
        "fixture count != DeckKind count; run `make yaml-fixtures`"
    );
}

#[test]
fn every_deck_kind__matches_golden_bytes() {
    for kind in DeckKind::all() {
        let path = fixture_path(*kind);
        let golden = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("missing fixture {}: {e} — run `make yaml-fixtures`", path.display()));
        let generated = kind.to_yaml().expect("serialize");

        assert_eq!(
            generated,
            golden,
            "{} drifted from its fixture; run `make yaml-fixtures` and review the diff",
            kind.deck_name()
        );
    }
}

#[test]
fn every_golden__deserializes_to_its_deck() {
    for kind in DeckKind::all() {
        let golden = fs::read_to_string(fixture_path(*kind)).expect("read fixture");

        assert_eq!(DeckKind::from_yaml(&golden).expect("deserialize"), *kind);
        assert_eq!(
            BasicCard::cards_from_yaml_str(&golden).expect("cards"),
            kind.base_vec()
        );
    }
}

/// Fixtures must be not merely parseable but *correct* — the distinction
/// `razz_bad.yml` exists to teach.
#[test]
fn every_golden__passes_validate_yaml() {
    macro_rules! assert_validates {
        ($($deck:ty),+ $(,)?) => {
            $(
                let golden = fs::read_to_string(
                    fixture_dir().join(format!("{}.yaml", slug(&<$deck>::deck_name())))
                ).expect("read fixture");

                <$deck>::validate_yaml(&golden)
                    .unwrap_or_else(|e| panic!("{} fixture failed validate_yaml: {e}", <$deck>::deck_name()));
            )+
        };
    }

    assert_validates!(
        Canasta, Dashavatara, Euchre24, Euchre32, French, Mughal, Pinochle,
        Razz, Short, Skat, Spades, Standard52, Tarot, Tiny,
    );
}

/// Every fixture carries the current format version, so a future bump is a
/// visible, reviewable change rather than a silent one.
#[test]
fn every_golden__declares_current_version() {
    for kind in DeckKind::all() {
        let golden = fs::read_to_string(fixture_path(*kind)).expect("read fixture");
        let dy = DeckYaml::from_yaml(&golden).expect("parse");

        assert_eq!(dy.version, DeckYaml::VERSION, "{}", kind.deck_name());
    }
}
```

- [ ] **Step 2: Run to verify it passes**

Run: `cargo test --features full --test yaml_golden`
Expected: PASS — 5 tests.

If `every_deck_kind__matches_golden_bytes` fails on the very first run, the fixtures and the code disagree. Re-run `make yaml-fixtures`, inspect `git diff tests/fixtures/yaml/`, and only accept the diff if the change is explainable.

- [ ] **Step 3: Prove the test can fail**

This is the Gold Standard check — a guard no test can kill is not a guard.

Run:
```bash
printf '\n# tamper\n' >> tests/fixtures/yaml/tiny.yaml
cargo test --features full --test yaml_golden
```
Expected: FAIL on `every_deck_kind__matches_golden_bytes` naming `Tiny`.

Then restore:
```bash
make yaml-fixtures && git diff --exit-code tests/fixtures/yaml/
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add tests/yaml_golden.rs
git commit -m "test: add byte-level golden fixture tests for every deck"
```

---

## Task 10: `tests/yaml_errors.rs`

**Files:**
- Create: `tests/yaml_errors.rs`

**Interfaces:**
- Consumes: all `CardError` YAML variants (Task 1) and every API from Tasks 2–6.

Every assertion downcasts to a **specific** `CardError` variant. A bare `is_err()` would pass for the wrong reason — e.g. a typo'd fixture path erroring instead of the validation logic.

- [ ] **Step 1: Write the test file**

Create `tests/yaml_errors.rs`:

```rust
//! Negative-path tests for YAML deck serialization.
//!
//! Every assertion pins a *specific* `CardError` variant. `is_err()` alone
//! would pass for the wrong reason, which is exactly the failure mode this
//! file exists to prevent.
//!
//! The headline test is `razz_bad__is_caught`: `src/basic/decks/yaml/razz_bad.yml`
//! is a real artifact kept in the repo because it parses cleanly and yields the
//! *wrong deck* — `Pile::<Razz>::validate()` did not catch it (see the doc
//! comment at `src/basic/decks/razz.rs:19-24`). This promotes that near-miss
//! from an anecdote to an enforced guarantee.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

use cardpack::prelude::*;

/// Downcast a boxed error to a concrete `CardError`, or fail loudly.
fn card_error(err: &(dyn std::error::Error + 'static)) -> &CardError {
    err.downcast_ref::<CardError>()
        .unwrap_or_else(|| panic!("expected a CardError, got: {err}"))
}

fn french_envelope() -> String {
    DeckKind::French.to_yaml().expect("serialize")
}

#[test]
fn malformed_yaml__errors() {
    assert!(DeckYaml::from_yaml("{{{ not yaml at all").is_err());
    assert!(BasicCard::cards_from_yaml_str("{{{ not yaml at all").is_err());
}

#[test]
fn scalar_document__errors_malformed() {
    let err = DeckYaml::from_yaml("just a plain string").unwrap_err();

    assert_eq!(*card_error(err.as_ref()), CardError::YamlMalformed);
}

/// The header claims a card count the body does not deliver. This is the
/// truncation guard: without it, a half-written file would silently yield a
/// short deck.
#[test]
fn truncated_envelope__errors_count_mismatch() {
    // French is 54 cards. Claim 55; the body still holds 54.
    let tampered = french_envelope().replace("count: 54", "count: 55");
    assert!(tampered.contains("count: 55"), "French envelope should declare count: 54");

    let err = DeckYaml::from_yaml(&tampered).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlCountMismatch { declared: 55, actual: 54 }
    );
}

/// The asymmetry: an empty *deck* is fatal, an empty *pile* is a fully-drawn
/// deck and therefore fine.
#[test]
fn empty_deck__errors_for_deck_but_not_pile() {
    let empty = "version: 1\nname: French\nfluent_deck_key: french\ncount: 0\ncards: []\n";

    let err = DeckKind::from_yaml(empty).unwrap_err();
    assert_eq!(*card_error(err.as_ref()), CardError::YamlEmptyDeck);

    let err = French::validate_yaml(empty).unwrap_err();
    assert_eq!(*card_error(err.as_ref()), CardError::YamlEmptyDeck);

    let pile = Pile::<French>::from_yaml(empty).expect("an empty pile is legitimate");
    assert!(pile.is_empty());
}

#[test]
fn unknown_deck_name__errors() {
    let renamed = french_envelope().replace("name: French", "name: Bicycle");
    let err = DeckKind::from_yaml(&renamed).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlUnknownDeck("Bicycle".to_string())
    );
}

/// A legacy bare sequence is unidentifiable as a deck by design, but is still
/// perfectly readable as cards.
#[test]
fn legacy_sequence__errors_in_deck_kind_but_reads_as_cards() {
    let legacy = serde_norway::to_string(&DeckKind::French.base_vec()).expect("serialize");

    let err = DeckKind::from_yaml(&legacy).unwrap_err();
    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlUnknownDeck(String::new())
    );

    assert_eq!(
        BasicCard::cards_from_yaml_str(&legacy).expect("cards still parse"),
        DeckKind::French.base_vec()
    );
}

#[test]
fn wrong_deck_name__errors_in_pile() {
    let err = Pile::<Tarot>::from_yaml(&french_envelope()).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlDeckMismatch {
            expected: "Tarot".to_string(),
            found: "French".to_string(),
        }
    );
}

/// Right header, wrong cards — only the membership check catches this.
#[test]
fn foreign_card__errors_in_pile() {
    let disguised = french_envelope().replace("name: French", "name: Tarot");
    let err = Pile::<Tarot>::from_yaml(&disguised).unwrap_err();

    assert!(
        matches!(card_error(err.as_ref()), CardError::YamlForeignCard { deck, .. } if deck == "Tarot"),
        "expected YamlForeignCard, got {:?}",
        card_error(err.as_ref())
    );
}

#[test]
fn wrong_deck__errors_in_validate_yaml() {
    let err = Tarot::validate_yaml(&french_envelope()).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlDeckMismatch {
            expected: "Tarot".to_string(),
            found: "French".to_string(),
        }
    );
}

/// **The headline test.** `razz_bad.yml` is well-formed YAML that parses
/// without complaint and produces a deck that is not Razz. Parsing must
/// succeed; validation must fail.
#[test]
fn razz_bad__is_caught() {
    let bad = include_str!("../src/basic/decks/yaml/razz_bad.yml");

    let cards = BasicCard::cards_from_yaml_str(bad)
        .expect("razz_bad.yml is well-formed YAML — that is the whole point");
    assert!(!cards.is_empty(), "it parses into real cards, it is just the wrong deck");

    let err = Razz::validate_yaml(bad)
        .expect_err("razz_bad.yml must NOT validate as Razz");

    assert!(
        matches!(card_error(err.as_ref()), CardError::YamlDeckMismatch { .. }),
        "expected YamlDeckMismatch, got {:?}",
        card_error(err.as_ref())
    );
}

/// The good file, for contrast — same code path, opposite verdict.
#[test]
fn razz_good__validates() {
    let good = include_str!("../src/basic/decks/yaml/razz.yaml");

    Razz::validate_yaml(good).expect("the shipped razz.yaml must validate as Razz");
}
```

- [ ] **Step 2: Run to verify it passes**

Run: `cargo test --features full --test yaml_errors`
Expected: PASS — 11 tests.

`razz_bad__is_caught` is the one to watch. If it fails at `expect("razz_bad.yml is well-formed YAML")`, the file does not parse at all — check whether `razz_bad.yml` differs structurally from `razz.yaml` (it may use a different field spelling), and adjust the assertion to describe reality rather than forcing the file to fit.

If `truncated_envelope__errors_count_mismatch` fails its `assert!` on the replace, the French envelope does not literally contain `count: 54` — print `french_envelope()` and match the real header text.

- [ ] **Step 3: Prove the guards are load-bearing**

For each guard, comment it out, confirm a named test goes red, then restore it:

| Guard | File | Test that must fail |
|---|---|---|
| `count != cards.len()` check | `deck_yaml.rs` | `truncated_envelope__errors_count_mismatch` |
| `cards.is_empty()` check | `traits.rs` (`validate_yaml`) | `empty_deck__errors_for_deck_but_not_pile` |
| `name != deck_name()` check | `pile.rs` (`from_yaml`) | `wrong_deck_name__errors_in_pile` |
| membership check | `pile.rs` (`from_yaml`) | `foreign_card__errors_in_pile` |
| `cards != base_vec()` check | `traits.rs` (`validate_yaml`) | `razz_bad__is_caught` |

Run after restoring: `cargo test --features full --test yaml_errors`
Expected: PASS — 11 tests.

- [ ] **Step 4: Commit**

```bash
git add tests/yaml_errors.rs
git commit -m "test: add YAML negative-path suite, pinning the razz_bad failure mode"
```

---

## Task 11: Docs, exports, and knowledge bundle

**Files:**
- Modify: `src/prelude.rs`, `src/lib.rs`, `CHANGELOG.md`, `README.md`
- Modify: `.okf/decks/extending-decks.md`, `.okf/architecture/feature-flags.md`, `.okf/decisions/index.md`, `.okf/log.md`
- Create: `.okf/decisions/yaml-envelope-format.md`

**Interfaces:**
- Consumes: everything.
- Produces: the public export surface and the documentation of record.

- [ ] **Step 1: Add the missing `Tiny` re-export**

Every deck except `Tiny` is re-exported from the prelude (`src/basic/decks.rs:17` declares the module; `src/prelude.rs` never re-exports it). Add it alphabetically, after the `tarot` line:

```rust
pub use crate::basic::decks::tiny::*;
```

Then drop the now-redundant full-path import from the top of **both** `tests/yaml_roundtrip.rs` and `tests/yaml_golden.rs`, along with the comment above it in the latter:

```rust
// remove from both files:
use cardpack::basic::decks::tiny::Tiny;
```

`Tiny` now arrives via `use cardpack::prelude::*;`. Leaving the explicit import in place would trip clippy's unused-import lint.

Run: `cargo test --features full --test yaml_roundtrip --test yaml_golden`
Expected: PASS — no behavior change, the import path is the only difference.

- [ ] **Step 2: Add the crate-level docs**

Add to the module docs in `src/lib.rs`, after the "Custom Deck example" section:

````rust
//! # Decks as YAML
//!
//! With the `yaml` feature, every deck serializes to a self-describing
//! envelope and back:
//!
//! ```
//! use cardpack::prelude::*;
//!
//! // Type-level: the deck definition.
//! let yml = Tarot::to_yaml().unwrap();
//! assert_eq!(Tarot::deck_from_yaml(&yml).unwrap(), Tarot::base_vec());
//!
//! // Non-generic: the registry.
//! for kind in DeckKind::all() {
//!     assert_eq!(DeckKind::from_yaml(&kind.to_yaml().unwrap()).unwrap(), *kind);
//! }
//!
//! // Instance-level: an ordered pile, shuffle preserved.
//! let shuffled = Standard52::deck().shuffled_with_seed(42);
//! let yml = shuffled.to_yaml().unwrap();
//! assert_eq!(Pile::<Standard52>::from_yaml(&yml).unwrap(), shuffled);
//! ```
//!
//! `YamlDecked` is blanket-implemented for every `DeckedBase` type, so custom
//! decks get all of this for free. Reading also accepts the legacy bare card
//! sequence used by `src/basic/decks/yaml/razz.yaml`.
````

Run: `cargo test --doc --features full`
Expected: PASS.

- [ ] **Step 3: Check the semver impact**

Adding variants to a non-`#[non_exhaustive]` public enum is a breaking change. EPIC-02 hit this with `DeckKind` and resolved it by marking the enum `#[non_exhaustive]` and bumping to 0.9.0.

Run: `cargo semver-checks check-release` (install with `cargo install cargo-semver-checks` if missing)

If it reports `enum_variant_added` for `CardError`, add `#[non_exhaustive]` above the enum in `src/common/errors.rs:4-5`, re-run, and record the decision in the CHANGELOG. If `cargo-semver-checks` is unavailable, note that explicitly in the commit message rather than skipping silently.

- [ ] **Step 4: Update CHANGELOG and README**

Add to `CHANGELOG.md` under `## [Unreleased]` → `### Added`:

```markdown
- **YAML deck serialization.** Every shipped deck now round-trips through YAML.
  New (all behind the `yaml` feature): `DeckYaml`, a versioned, self-describing
  deck envelope; `YamlDecked`, blanket-implemented for every `DeckedBase` type,
  giving `to_yaml`/`deck_from_yaml`/`validate_yaml`; `DeckKind::to_yaml` and
  `DeckKind::from_yaml`; and order-preserving `Pile::to_yaml`/`Pile::from_yaml`.
  `BasicCard::cards_from_yaml_str` now accepts the envelope in addition to the
  legacy bare card sequence — a strict superset, so existing callers and
  `src/basic/decks/yaml/razz.yaml` are unaffected. Six new `CardError` variants
  cover the semantic failures. See `docs/EPIC-03_Yaml_Deck_Serialization.md`.
- `Tiny` is now re-exported from the prelude, like every other deck.
```

Add YAML serialization to the feature list in `README.md` where the `yaml` feature is described.

- [ ] **Step 5: Write the OKF decision doc**

Create `.okf/decisions/yaml-envelope-format.md`:

```markdown
---
type: Decision
title: The YAML envelope keeps a legacy bare-sequence reader
description: DeckYaml::from_yaml accepts both the envelope and the old bare card list — the legacy path is load-bearing because razz.yaml still uses it at build time.
tags: [yaml, serialization, decisions, back-compat]
timestamp: 2026-07-24T12:00:00Z
---

# The decision

`DeckYaml::from_yaml` (`src/basic/types/deck_yaml.rs`) sniffs the parsed
`serde_norway::Value` and accepts **two** document shapes:

* a **mapping** — the versioned envelope (`version`, `name`, `fluent_deck_key`,
  `count`, `cards`), which is what the crate always *writes*;
* a **sequence** — the legacy bare list of `BasicCard`, which is what the crate
  used to be able to read, and all it could read before EPIC-03.

# Why the legacy path must not be deleted

`src/basic/decks/yaml/razz.yaml` was deliberately **not** migrated to the
envelope. `Razz::base_vec()` (`src/basic/decks/razz.rs:36`) embeds it with
`include_str!` and parses it on every build, so the legacy reader is exercised
in production, not just in tests.

Deleting the sequence branch "to simplify the two code paths" compiles fine and
passes any test that only uses the new format — and then `Razz::base_vec()`
returns an empty deck via its `unwrap_or_else` fallback. That is the exact
failure this crate has already been bitten by once
([domain kernel](/architecture/domain-kernel.md), Finding 1a).

If the legacy format is ever genuinely retired, migrate `razz.yaml` **in the
same change**, and keep `razz_bad.yml` parseable — `tests/yaml_errors.rs`
depends on it parsing cleanly while failing validation.

# Citations

[1] [src/basic/types/deck_yaml.rs](../../src/basic/types/deck_yaml.rs)
[2] [src/basic/decks/razz.rs](../../src/basic/decks/razz.rs)
[3] [docs/EPIC-03_Yaml_Deck_Serialization.md](../../docs/EPIC-03_Yaml_Deck_Serialization.md)
```

Add a line to `.okf/decisions/index.md`:

```markdown
* [The YAML envelope keeps a legacy bare-sequence reader](yaml-envelope-format.md) - DeckYaml reads both the envelope and the old bare card list; the legacy branch is load-bearing because razz.yaml still uses it at build time.
```

- [ ] **Step 6: Update the affected OKF concepts**

In `.okf/decks/extending-decks.md`, expand the "Alternative: decks from YAML" section to describe the envelope and `YamlDecked`, and bump its `timestamp` to `2026-07-24T12:00:00Z`.

In `.okf/architecture/feature-flags.md`, note what the `yaml` feature now buys (serialization, not just deserialization), and bump its `timestamp`.

Append to `.okf/log.md`:

```markdown
## 2026-07-24 — EPIC-03 YAML deck serialization

Added `DeckYaml` envelope + `YamlDecked` blanket trait; every deck now
round-trips through YAML. Updated `decks/extending-decks.md` and
`architecture/feature-flags.md`; added
`decisions/yaml-envelope-format.md` (the legacy sequence reader is
load-bearing — `razz.yaml` depends on it).
```

- [ ] **Step 7: Validate the bundle**

Run: `/okf:validate .okf --strict`
Expected: clean. Every non-reserved `.md` needs YAML frontmatter with a non-empty `type`.

- [ ] **Step 8: Run the full gate battery**

Run:
```bash
cargo fmt --all -- --check
cargo build --no-default-features
cargo build --no-default-features --features serde
cargo test --no-default-features --lib
cargo test --features full
cargo test --doc --features full
cargo test --features std-io --lib
cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic
cargo deny check bans
cargo build --target wasm32-unknown-unknown --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
make yaml-fixtures && git diff --exit-code tests/fixtures/yaml/
cargo package --list | grep -c "tests/fixtures" || echo "fixtures correctly excluded from package"
```

Expected: every command succeeds; the last prints `fixtures correctly excluded from package`.

- [ ] **Step 9: Update the EPIC Status table**

In `docs/EPIC-03_Yaml_Deck_Serialization.md`, flip each `## Status` row from `Planned` to `**Complete**`, and check the Story task boxes that actually landed. Per the EPIC's honesty rules, only flip a row the cited code proves.

- [ ] **Step 10: Commit**

```bash
git add src/prelude.rs src/lib.rs src/common/errors.rs CHANGELOG.md README.md \
        .okf/ docs/EPIC-03_Yaml_Deck_Serialization.md \
        tests/yaml_roundtrip.rs tests/yaml_golden.rs
git commit -m "docs: document YAML deck serialization and update the OKF bundle"
```

---

## Verification

The whole feature is done when all of this passes from a clean tree:

```bash
make ayce          # fmt, build_test, clippy, msrv, no-std, docs
make test          # test-unit + test-doc + test-std-io
make yaml-fixtures && git diff --exit-code tests/fixtures/yaml/
cargo deny check bans
cargo build --target wasm32-unknown-unknown --all-features
```

Exit criteria, from the EPIC:

1. `DeckKind::from_yaml(&kind.to_yaml()?)? == kind` for all 14 variants, and `T::deck_from_yaml(&T::to_yaml()?)? == T::base_vec()` for all 14 types.
2. `tests/fixtures/yaml/` holds exactly `DeckKind::all().len()` files, each byte-identical to freshly generated output.
3. `razz_bad.yml` parses without error and is rejected by `Razz::validate_yaml`.
4. Every negative test matches a specific `CardError` variant via downcast.
5. `cargo build --no-default-features` and the wasm target stay green.
6. `cargo deny check bans` stays green.
7. `src/basic/decks/yaml/razz.yaml` is byte-unchanged and `Razz::deck()` still yields 52 cards.
8. `.okf/` updated and `/okf:validate .okf --strict` clean.
