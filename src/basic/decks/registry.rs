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
use crate::basic::decks::euchre24::Euchre24;
use crate::basic::decks::euchre32::Euchre32;
use crate::basic::decks::french::French;
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
#[cfg(all(feature = "i18n", feature = "colored-display"))]
use crate::basic::types::traits::Decked;
use alloc::string::String;
use alloc::vec::Vec;
use crate::basic::types::traits::DeckedBase;

/// Every deck that cardpack ships, exposed as a non-generic enum.
///
/// `Razz` is gated behind the `yaml` feature (it loads its cards from a
/// YAML file at runtime); other variants are always available.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeckKind {
    Canasta,
    Euchre24,
    Euchre32,
    French,
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
    /// The slice length is 12 with `yaml` (the default) and 11 without.
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
            Self::Euchre24,
            Self::Euchre32,
            Self::French,
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
            Self::Euchre24 => Euchre24::deck_name(),
            Self::Euchre32 => Euchre32::deck_name(),
            Self::French => French::deck_name(),
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
            Self::Euchre24 => Euchre24::base_vec(),
            Self::Euchre32 => Euchre32::base_vec(),
            Self::French => French::base_vec(),
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
    /// All decks share one of three keys: `french`, `skat`, or `tarot`.
    #[must_use]
    pub fn fluent_deck_key(self) -> String {
        match self {
            Self::Canasta => Canasta::fluent_deck_key(),
            Self::Euchre24 => Euchre24::fluent_deck_key(),
            Self::Euchre32 => Euchre32::fluent_deck_key(),
            Self::French => French::fluent_deck_key(),
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
            Self::Euchre24 => Euchre24::demo(verbose),
            Self::Euchre32 => Euchre32::demo(verbose),
            Self::French => French::demo(verbose),
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

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__decks__registry_tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn all__non_empty() {
        assert!(!DeckKind::all().is_empty());
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
    }

    #[test]
    fn deck_name__matches_typed_deck() {
        for kind in DeckKind::all() {
            let name = kind.deck_name();
            assert!(!name.is_empty(), "{kind:?} returned empty deck_name");
        }
    }

    #[test]
    fn fluent_deck_key__is_one_of_three() {
        for kind in DeckKind::all() {
            let key = kind.fluent_deck_key();
            assert!(
                key == "french" || key == "skat" || key == "tarot",
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
