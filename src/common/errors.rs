use alloc::string::String;
use thiserror::Error;

/// The crate's single error type.
///
/// The enum is `#[non_exhaustive]`: new error cases can be added in minor
/// releases without breaking downstream `match` statements. Follow the same
/// rule [`DeckKind`](crate::basic::decks::registry::DeckKind) does and always
/// include a wildcard arm. Some variants are gated behind the `yaml` feature,
/// so the set of reachable cases depends on the features you enable — another
/// reason not to match exhaustively.
#[derive(Error, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CardError {
    #[error("Fubar should not be possible.")]
    Fubar,

    #[error("Invalid Card: `{0}`")]
    InvalidCard(String),

    #[error("Invalid Card Count: `{0}`")]
    InvalidCardCount(usize),

    #[error(
        "Invalid FluentName: `{0}`. Must be alphanumeric with hyphens, en-dashes, or em-dashes."
    )]
    InvalidFluentName(String),

    #[error("Invalid Index: `{0}`")]
    InvalidIndex(String),

    #[error("Not enough cards: `{0}` missing")]
    NotEnoughCards(usize),

    #[error("Too many cards: `{0}` extra")]
    TooManyCards(usize),

    // The YAML variants below carry only `String`/`usize` payloads. A
    // `#[from] serde_norway::Error` would break both `Eq` and `PartialEq` on
    // this enum *and* leak a format crate into the public API (domain-kernel
    // Invariant 2), which is why YAML *parse* failures stay boxed as
    // `Box<dyn Error>` and only the semantic failures land here.
    // See docs/EPIC-03_Yaml_Deck_Serialization.md.
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
}

// The whole module is yaml-gated because every test in it is: gating the
// tests individually would leave `use super::*` and `ToString` unused under
// `--no-default-features`. Same idiom as `src/basic/decks/cards/french.rs:376`.
#[cfg(all(test, feature = "yaml"))]
#[allow(non_snake_case)]
mod common__errors_tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn yaml_variants__display() {
        assert_eq!(
            CardError::YamlCountMismatch {
                declared: 52,
                actual: 51
            }
            .to_string(),
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
        assert_eq!(
            CardError::YamlEmptyDeck.to_string(),
            "YAML document has no cards"
        );
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
    #[test]
    fn yaml_variants__stay_eq() {
        assert_eq!(CardError::YamlEmptyDeck, CardError::YamlEmptyDeck);
        assert_ne!(CardError::YamlEmptyDeck, CardError::YamlMalformed);
    }
}
