//! A deck rendered as a self-describing YAML document.
//!
//! The envelope carries deck *identity* — `name`, `fluent_deck_key` — which a
//! bare card sequence cannot. That is what makes
//! [`DeckKind::from_yaml`](crate::basic::decks::registry::DeckKind) possible
//! without content-matching against every shipped deck.
//!
//! Reading accepts both the envelope and the legacy bare sequence; writing
//! always produces the envelope. See `.okf/decisions/yaml-envelope-format.md` —
//! the legacy reader is load-bearing and must not be removed, because
//! `src/basic/decks/yaml/razz.yaml` still uses that form at build time.

use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use serde::{Deserialize, Serialize};

/// A deck serialized as YAML: a metadata header plus the card list.
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
    /// the document came from a legacy bare sequence, which carries no header.
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
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(DeckYaml::from_decked::<Tarot>().count, 78);
    /// ```
    ///
    /// `T` is `?Sized` because every method it uses is an associated function
    /// returning an owned value — nothing here needs a `T` value. Without the
    /// relaxation, [`YamlDecked`](crate::basic::types::traits::YamlDecked)
    /// could not call this with its own implicitly-`?Sized` `Self`.
    #[must_use]
    pub fn from_decked<T: DeckedBase + ?Sized>() -> Self {
        Self::new(T::deck_name(), T::fluent_deck_key(), T::base_vec())
    }

    /// Build an envelope from an explicit, ordered card list.
    ///
    /// `count` is derived from `cards`, never passed in — the truncation guard
    /// is only meaningful if writers cannot get it wrong.
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
    /// the public API (domain-kernel Invariant 2).
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
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// // The envelope form:
    /// let yml = DeckYaml::from_decked::<French>().to_yaml().unwrap();
    /// assert_eq!(DeckYaml::from_yaml(&yml).unwrap().name, "French");
    ///
    /// // The legacy form still reads, with no name to report:
    /// let legacy = "- suit: {weight: 3, pip_type: Suit, index: 'S', symbol: 'S', value: 4}\n  \
    ///                 rank: {weight: 12, pip_type: Rank, index: 'A', symbol: 'A', value: 14}";
    /// let parsed = DeckYaml::from_yaml(legacy).unwrap();
    /// assert_eq!(parsed.name, "");
    /// assert_eq!(parsed.count, 1);
    /// ```
    ///
    /// # Errors
    ///
    /// Malformed YAML, a document that is neither a mapping nor a sequence
    /// ([`CardError::YamlMalformed`]), or a `count` that disagrees with
    /// `cards.len()` ([`CardError::YamlCountMismatch`]).
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let value: serde_norway::Value = serde_norway::from_str(yaml_str)?;

        let deck_yaml = if value.is_mapping() {
            serde_norway::from_value::<Self>(value)?
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
        assert!(
            yml.contains("\ncount: 54\ncards:\n- suit:\n"),
            "got:\n{yml}"
        );
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
        let card_err = err
            .downcast_ref::<CardError>()
            .expect("should be a CardError");

        assert_eq!(
            *card_err,
            CardError::YamlCountMismatch {
                declared: 53,
                actual: 54
            }
        );
    }

    #[test]
    fn scalar_document__errors_malformed() {
        let err = DeckYaml::from_yaml("just a string").unwrap_err();
        let card_err = err
            .downcast_ref::<CardError>()
            .expect("should be a CardError");

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
        assert_eq!(
            parsed.name, "",
            "razz.yaml is intentionally still legacy-format"
        );
    }

    #[test]
    fn new__computes_count() {
        let dy = DeckYaml::new("X".to_string(), "x".to_string(), French::base_vec());

        assert_eq!(dy.count, 54);
        assert_eq!(dy.version, DeckYaml::VERSION);
    }
}
