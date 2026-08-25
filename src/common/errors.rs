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

    // Seal-kernel variants (docs/EPIC-04_Sealed_Decks.md). Ungated: the
    // ordinal/permutation/slot types they serve are dependency-free and always
    // on. Payloads are `String`/`usize`/`u16` only, so `CardError` keeps `Eq`
    // and no backend error type ever lands here (EPIC-04 decision 8).
    #[error("Invalid ordinal: `{0}` is out of range for this deck")]
    InvalidOrdinal(u16),

    #[error("Card `{0}` is not in this deck")]
    CardNotInDeck(String),

    #[error("Invalid permutation: {0}")]
    InvalidPermutation(String),

    #[error("Permutation length `{expected}` does not match `{actual}` items")]
    PermutationLength { expected: usize, actual: usize },

    #[error("Cannot cut at `{0}`: out of range")]
    InvalidCut(usize),

    #[error("Malformed canonical bytes: {0}")]
    CanonicalMalformed(String),

    #[error("Duplicate slot `{0}`")]
    DuplicateSlot(u16),

    #[error("Slot `{0}` not found")]
    SlotNotFound(u16),

    #[error("Slot `{0}` is already revealed")]
    SlotAlreadyRevealed(u16),

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

// Seal-kernel variants (EPIC-04) are ungated, so their tests are too.
#[cfg(test)]
#[allow(non_snake_case)]
mod common__errors_seal_tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn seal_variants__display() {
        assert_eq!(
            CardError::InvalidOrdinal(52).to_string(),
            "Invalid ordinal: `52` is out of range for this deck"
        );
        assert_eq!(
            CardError::CardNotInDeck("A♠".to_string()).to_string(),
            "Card `A♠` is not in this deck"
        );
        assert_eq!(
            CardError::InvalidPermutation("duplicate index 3".to_string()).to_string(),
            "Invalid permutation: duplicate index 3"
        );
        assert_eq!(
            CardError::PermutationLength {
                expected: 52,
                actual: 51
            }
            .to_string(),
            "Permutation length `52` does not match `51` items"
        );
        assert_eq!(
            CardError::InvalidCut(99).to_string(),
            "Cannot cut at `99`: out of range"
        );
        assert_eq!(
            CardError::CanonicalMalformed("bad version".to_string()).to_string(),
            "Malformed canonical bytes: bad version"
        );
        assert_eq!(
            CardError::DuplicateSlot(7).to_string(),
            "Duplicate slot `7`"
        );
        assert_eq!(CardError::SlotNotFound(7).to_string(), "Slot `7` not found");
        assert_eq!(
            CardError::SlotAlreadyRevealed(7).to_string(),
            "Slot `7` is already revealed"
        );
    }

    /// The variants carry only `String`/`usize`/`u16`, so `CardError` keeps
    /// `Eq` — the property a backend error type must never break.
    #[test]
    fn seal_variants__stay_eq() {
        assert_eq!(CardError::SlotNotFound(1), CardError::SlotNotFound(1));
        assert_ne!(CardError::SlotNotFound(1), CardError::DuplicateSlot(1));
    }
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
