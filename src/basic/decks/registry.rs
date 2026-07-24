//! A non-generic registry of every shipped deck.
//!
//! `Pile<DeckType>::deck()` is generic — useful for type-level guarantees,
//! awkward for CLIs, GUIs, and game launchers that just want "list every
//! deck this crate ships." [`DeckKind`] solves that: each variant maps to
//! one of the typed decks, and the methods dispatch to the corresponding
//! [`DeckedBase`] impl.
//!
//! ```
//! use cardpack::prelude::*;
//!
//! // Every deck cardpack ships, in a stable order:
//! for kind in DeckKind::all() {
//!     println!("{} has {} cards", kind.deck_name(), kind.base_vec().len());
//! }
//! ```

use crate::basic::decks::canasta::Canasta;
use crate::basic::decks::dashavatara::Dashavatara;
use crate::basic::decks::euchre24::Euchre24;
use crate::basic::decks::euchre32::Euchre32;
use crate::basic::decks::french::French;
use crate::basic::decks::mughal::Mughal;
use crate::basic::decks::pinochle::Pinochle;
#[cfg(feature = "yaml")]
use crate::basic::decks::razz::Razz;
use crate::basic::decks::short::Short;
use crate::basic::decks::skat::Skat;
use crate::basic::decks::spades::Spades;
use crate::basic::decks::standard52::Standard52;
use crate::basic::decks::tarot::Tarot;
use crate::basic::decks::tiny::Tiny;
use crate::basic::types::basic_card::BasicCard;
#[cfg(feature = "yaml")]
use crate::basic::types::deck_yaml::DeckYaml;
#[cfg(all(feature = "i18n", feature = "colored-display"))]
use crate::basic::types::traits::Decked;
use crate::basic::types::traits::DeckedBase;
#[cfg(feature = "yaml")]
use crate::common::errors::CardError;
#[cfg(feature = "yaml")]
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "yaml")]
use core::error::Error;

/// Every deck that cardpack ships, exposed as a non-generic enum.
///
/// `Razz` is gated behind the `yaml` feature (it loads its cards from a
/// YAML file at runtime); other variants are always available.
///
/// The enum is `#[non_exhaustive]`: new decks can be added in minor
/// releases without breaking downstream code. Match with a wildcard arm,
/// or iterate [`DeckKind::all()`] instead of matching exhaustively.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DeckKind {
    Canasta,
    Dashavatara,
    Euchre24,
    Euchre32,
    French,
    Mughal,
    Pinochle,
    #[cfg(feature = "yaml")]
    Razz,
    Short,
    Skat,
    Spades,
    Standard52,
    Tarot,
    Tiny,
}

impl DeckKind {
    /// Returns every shipped deck, in a stable order.
    ///
    /// The slice length is 14 with `yaml` (the default) and 13 without.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert!(!DeckKind::all().is_empty());
    /// assert!(DeckKind::all().contains(&DeckKind::French));
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::Canasta,
            Self::Dashavatara,
            Self::Euchre24,
            Self::Euchre32,
            Self::French,
            Self::Mughal,
            Self::Pinochle,
            #[cfg(feature = "yaml")]
            Self::Razz,
            Self::Short,
            Self::Skat,
            Self::Spades,
            Self::Standard52,
            Self::Tarot,
            Self::Tiny,
        ]
    }

    /// The human-readable name of the deck.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(DeckKind::Standard52.deck_name(), "Standard 52");
    /// assert_eq!(DeckKind::Tarot.deck_name(), "Tarot");
    /// ```
    #[must_use]
    pub fn deck_name(self) -> String {
        match self {
            Self::Canasta => Canasta::deck_name(),
            Self::Dashavatara => Dashavatara::deck_name(),
            Self::Euchre24 => Euchre24::deck_name(),
            Self::Euchre32 => Euchre32::deck_name(),
            Self::French => French::deck_name(),
            Self::Mughal => Mughal::deck_name(),
            Self::Pinochle => Pinochle::deck_name(),
            #[cfg(feature = "yaml")]
            Self::Razz => Razz::deck_name(),
            Self::Short => Short::deck_name(),
            Self::Skat => Skat::deck_name(),
            Self::Spades => Spades::deck_name(),
            Self::Standard52 => Standard52::deck_name(),
            Self::Tarot => Tarot::deck_name(),
            Self::Tiny => Tiny::deck_name(),
        }
    }

    /// The deck's cards as a non-generic [`Vec<BasicCard>`].
    ///
    /// Use this when you want the raw cards without committing to the
    /// generic `Pile<DeckType>` API.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(DeckKind::French.base_vec().len(), 54);
    /// assert_eq!(DeckKind::Standard52.base_vec().len(), 52);
    /// assert_eq!(DeckKind::Tarot.base_vec().len(), 78);
    /// ```
    #[must_use]
    pub fn base_vec(self) -> Vec<BasicCard> {
        match self {
            Self::Canasta => Canasta::base_vec(),
            Self::Dashavatara => Dashavatara::base_vec(),
            Self::Euchre24 => Euchre24::base_vec(),
            Self::Euchre32 => Euchre32::base_vec(),
            Self::French => French::base_vec(),
            Self::Mughal => Mughal::base_vec(),
            Self::Pinochle => Pinochle::base_vec(),
            #[cfg(feature = "yaml")]
            Self::Razz => Razz::base_vec(),
            Self::Short => Short::base_vec(),
            Self::Skat => Skat::base_vec(),
            Self::Spades => Spades::base_vec(),
            Self::Standard52 => Standard52::base_vec(),
            Self::Tarot => Tarot::base_vec(),
            Self::Tiny => Tiny::base_vec(),
        }
    }

    /// The fluent localization key the deck resolves through.
    ///
    /// All decks share one of five keys: `dashavatara`, `french`, `mughal`,
    /// `skat`, or `tarot`.
    #[must_use]
    pub fn fluent_deck_key(self) -> String {
        match self {
            Self::Canasta => Canasta::fluent_deck_key(),
            Self::Dashavatara => Dashavatara::fluent_deck_key(),
            Self::Euchre24 => Euchre24::fluent_deck_key(),
            Self::Euchre32 => Euchre32::fluent_deck_key(),
            Self::French => French::fluent_deck_key(),
            Self::Mughal => Mughal::fluent_deck_key(),
            Self::Pinochle => Pinochle::fluent_deck_key(),
            #[cfg(feature = "yaml")]
            Self::Razz => Razz::fluent_deck_key(),
            Self::Short => Short::fluent_deck_key(),
            Self::Skat => Skat::fluent_deck_key(),
            Self::Spades => Spades::fluent_deck_key(),
            Self::Standard52 => Standard52::fluent_deck_key(),
            Self::Tarot => Tarot::fluent_deck_key(),
            Self::Tiny => Tiny::fluent_deck_key(),
        }
    }

    /// Prints a colored, multi-locale demonstration of the deck.
    ///
    /// Available with `i18n` + `colored-display` (both in `default`).
    #[cfg(all(feature = "i18n", feature = "colored-display"))]
    pub fn demo(self, verbose: bool) {
        match self {
            Self::Canasta => Canasta::demo(verbose),
            Self::Dashavatara => Dashavatara::demo(verbose),
            Self::Euchre24 => Euchre24::demo(verbose),
            Self::Euchre32 => Euchre32::demo(verbose),
            Self::French => French::demo(verbose),
            Self::Mughal => Mughal::demo(verbose),
            Self::Pinochle => Pinochle::demo(verbose),
            #[cfg(feature = "yaml")]
            Self::Razz => Razz::demo(verbose),
            Self::Short => Short::demo(verbose),
            Self::Skat => Skat::demo(verbose),
            Self::Spades => Spades::demo(verbose),
            Self::Standard52 => Standard52::demo(verbose),
            Self::Tarot => Tarot::demo(verbose),
            Self::Tiny => Tiny::demo(verbose),
        }
    }
}

#[cfg(feature = "yaml")]
impl DeckKind {
    /// This deck as an envelope YAML document.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// // Every deck cardpack ships round-trips through YAML:
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
    /// `name` header against every shipped deck.
    ///
    /// A legacy bare sequence has no name and is therefore rejected —
    /// inferring a deck from its cards alone is deliberately out of scope.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let yml = DeckKind::Tarot.to_yaml().unwrap();
    ///
    /// assert_eq!(DeckKind::from_yaml(&yml).unwrap(), DeckKind::Tarot);
    /// ```
    ///
    /// # Errors
    ///
    /// [`CardError::YamlEmptyDeck`] for an empty card list;
    /// [`CardError::YamlUnknownDeck`] for a name matching no shipped deck,
    /// including the empty name a legacy sequence produces.
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let deck_yaml = DeckYaml::from_yaml(yaml_str)?;

        // Rejected here but allowed for a `Pile`, where an empty pile is a
        // fully-drawn deck. See src/basic/types/traits.rs `validate_yaml`.
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

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__decks__registry_tests {
    use super::*;
    // Unconditional: the `not(yaml)` Razz-omission test below uses it, and so
    // do the yaml round-trip tests. Gating it to one config leaves the other
    // without `.to_string()`.
    use alloc::string::ToString;
    // Test-only, so it lives here rather than at module scope — at module
    // scope the non-test lib build would see it unused.
    #[cfg(feature = "yaml")]
    use alloc::collections::BTreeSet;

    #[test]
    fn all__non_empty() {
        assert!(!DeckKind::all().is_empty());
    }

    /// `from_yaml` recovers a `DeckKind` by matching on `deck_name()`. If two
    /// decks ever share a name that match becomes ambiguous, silently
    /// resolving to whichever comes first in `all()`. This is the tripwire.
    #[cfg(feature = "yaml")]
    #[test]
    fn deck_name__all_distinct() {
        let unique: BTreeSet<String> = DeckKind::all().iter().map(|k| k.deck_name()).collect();

        assert_eq!(
            unique.len(),
            DeckKind::all().len(),
            "duplicate deck_name() across DeckKind makes from_yaml ambiguous"
        );
    }

    /// The EPIC's headline claim, at the registry level.
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
            let dy = DeckYaml::from_yaml(&kind.to_yaml().unwrap()).unwrap();

            assert_eq!(dy.version, DeckYaml::VERSION);
            assert_eq!(dy.name, kind.deck_name());
            assert_eq!(dy.fluent_deck_key, kind.fluent_deck_key());
            assert_eq!(dy.cards, kind.base_vec());
            assert_eq!(dy.count, kind.base_vec().len());
        }
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn from_yaml__empty_cards__errors() {
        let empty = "version: 1\nname: French\nfluent_deck_key: french\ncount: 0\ncards: []\n";
        let err = DeckKind::from_yaml(empty).unwrap_err();

        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlEmptyDeck
        );
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn from_yaml__unknown_name__errors() {
        let renamed = DeckKind::French
            .to_yaml()
            .unwrap()
            .replace("name: French", "name: Bicycle");
        let err = DeckKind::from_yaml(&renamed).unwrap_err();

        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlUnknownDeck("Bicycle".to_string())
        );
    }

    /// A legacy bare sequence carries no name, so it is unidentifiable by
    /// design. Inferring a deck from its cards is a different feature with
    /// different failure modes, deliberately out of scope.
    #[cfg(feature = "yaml")]
    #[test]
    fn from_yaml__legacy_sequence__errors() {
        let legacy = serde_norway::to_string(&DeckKind::French.base_vec()).unwrap();
        let err = DeckKind::from_yaml(&legacy).unwrap_err();

        assert_eq!(
            *err.downcast_ref::<CardError>().unwrap(),
            CardError::YamlUnknownDeck(String::new())
        );
    }

    #[test]
    fn all__contains_french_and_standard52() {
        let all = DeckKind::all();
        assert!(all.contains(&DeckKind::French));
        assert!(all.contains(&DeckKind::Standard52));
    }

    #[test]
    fn all__no_duplicates() {
        let all = DeckKind::all();
        let mut sorted: Vec<DeckKind> = all.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), all.len());
    }

    #[test]
    fn base_vec__matches_typed_deck() {
        // Sample three decks of different sizes.
        assert_eq!(DeckKind::French.base_vec(), French::base_vec());
        assert_eq!(DeckKind::Standard52.base_vec(), Standard52::base_vec());
        assert_eq!(DeckKind::Tarot.base_vec(), Tarot::base_vec());
        assert_eq!(DeckKind::Tiny.base_vec(), Tiny::base_vec());
        assert_eq!(DeckKind::Mughal.base_vec(), Mughal::base_vec());
        assert_eq!(DeckKind::Dashavatara.base_vec(), Dashavatara::base_vec());
    }

    #[test]
    fn deck_name__matches_typed_deck() {
        for kind in DeckKind::all() {
            let name = kind.deck_name();
            assert!(!name.is_empty(), "{kind:?} returned empty deck_name");
        }
    }

    #[test]
    fn fluent_deck_key__is_one_of_five() {
        for kind in DeckKind::all() {
            let key = kind.fluent_deck_key();
            assert!(
                key == "dashavatara"
                    || key == "french"
                    || key == "mughal"
                    || key == "skat"
                    || key == "tarot",
                "{kind:?} returned unexpected fluent_deck_key {key:?}"
            );
        }
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn all__includes_razz_with_yaml() {
        assert!(DeckKind::all().contains(&DeckKind::Razz));
        assert_eq!(DeckKind::Razz.base_vec(), Razz::base_vec());
    }

    #[cfg(not(feature = "yaml"))]
    #[test]
    fn all__omits_razz_without_yaml() {
        let names: Vec<String> = DeckKind::all().iter().map(|k| k.deck_name()).collect();
        assert!(
            !names.contains(&"Razz".to_string()),
            "Razz should not appear without the yaml feature"
        );
    }
}
