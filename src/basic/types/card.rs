use crate::basic::types::basic_card::BasicCard;
#[cfg(feature = "colored-display")]
use crate::basic::types::pips::Pip;
#[cfg(feature = "i18n")]
use crate::basic::types::pips::PipType;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
#[cfg(feature = "i18n")]
use crate::localization::{FluentName, Named};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "colored-display")]
use colored::{Color, Colorize};
use core::fmt::{Display, Formatter};
use core::hash::Hash;
use core::marker::PhantomData;
use core::str::FromStr;
#[cfg(feature = "i18n")]
use fluent_templates::LanguageIdentifier;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "colored-display")]
use std::collections::HashMap;

/// A `Card` is a struct that's a generic wrapper around a [`BasicCard`] providing it with additional
/// deck specific superpowers, many of which are at the pile level. The ones at the `Card` level
/// are:
///
/// - `color()` - returns the color of the card based on what's configured at the type parameter's implementation of the `DeckedBase` trait.
/// - `fluent_name()` - returns the long name of the card from the `Named` trait's use of fluent templates.
/// - `from_str()` - allows you to create a `Card` for the specific deck with a string representation of the card.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Card<DeckType>
where
    DeckType: DeckedBase,
{
    pub base_card: BasicCard,
    pub deck: PhantomData<DeckType>,
}

impl<DeckType: DeckedBase> Card<DeckType> {
    #[must_use]
    pub fn new(base_card: BasicCard) -> Self {
        Self::from(base_card)
    }

    /// Returns the underlying [`BasicCard`] struct.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::TREY_DIAMONDS);
    ///
    /// assert_eq!(
    ///     Card::<French>::new(FrenchBasicCard::TREY_DIAMONDS).base(),
    ///     FrenchBasicCard::TREY_DIAMONDS
    /// );
    /// assert_eq!(
    ///     Card::<Pinochle>::new(PinochleBasicCard::TEN_SPADES).base(),
    ///     PinochleBasicCard::TEN_SPADES
    /// );
    /// ```
    #[must_use]
    pub fn base(&self) -> BasicCard {
        self.base_card
    }

    /// Returns the color designated for a Card's specific suit in the deck's configuration.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::TEN_DIAMONDS);
    ///
    /// assert_eq!(card.color(), Color::Red);
    /// ```
    ///
    /// This feels heavy and hackie. It's not important enough to worry about.
    #[cfg(feature = "colored-display")]
    #[must_use]
    pub fn color(&self) -> Color {
        let binding = DeckType::colors();
        let color = binding.get(&self.base_card.suit);

        color.map_or(Color::White, |color| *color)
    }

    /// Returns the color designated for a Card's specific suit in the deck's configuration.
    #[cfg(feature = "colored-display")]
    #[must_use]
    pub fn color_index_string(&self) -> String {
        self.color_string(self.base_card.index())
    }

    /// TODO RF: create a `color_index_string()` version with a common implementation.
    ///
    /// DONE!!!
    #[cfg(feature = "colored-display")]
    #[must_use]
    pub fn color_symbol_string(&self) -> String {
        self.color_string(self.base_card.to_string())
    }

    /// Returns a color formatted version of the String based on the settings in the deck's configuration.
    #[cfg(feature = "colored-display")]
    fn color_string(&self, s: String) -> String {
        match self.color() {
            Color::Red => s.red().to_string(),
            Color::Blue => s.blue().to_string(),
            Color::Green => s.green().to_string(),
            Color::Yellow => s.yellow().to_string(),
            Color::Magenta => s.magenta().to_string(),
            Color::Cyan => s.cyan().to_string(),
            Color::BrightBlack => s.bright_black().to_string(),
            Color::BrightRed => s.bright_red().to_string(),
            Color::BrightGreen => s.bright_green().to_string(),
            Color::BrightYellow => s.bright_yellow().to_string(),
            Color::BrightBlue => s.bright_blue().to_string(),
            Color::BrightMagenta => s.bright_magenta().to_string(),
            Color::BrightCyan => s.bright_cyan().to_string(),
            _ => s,
        }
    }

    /// Returns the basic, text representation of a `Card`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<Skat>::new(SkatBasicCard::ZHEN_EICHEL);
    ///
    /// assert_eq!(card.index(), "ZE");
    /// ```
    #[must_use]
    pub fn index(&self) -> String {
        self.base_card.index()
    }

    /// Returns true if the `Card` is blank.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert!(Card::<French>::default().is_blank());
    /// assert!(!Card::<French>::new(FrenchBasicCard::ACE_SPADES).is_blank());
    /// ```
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.base_card.is_blank()
    }

    /// `CoPilot` was completely useless for this. It was surprisingly easy to figure it out
    /// for myself one I got used to the patterns with this hint from the compiler:
    ///
    /// ```txt
    /// error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
    ///    --> src/basic/types/bcard:151:9
    ///     |
    /// 151 |           DeckedBase::basic_pile().contains(&self.base_card)
    ///     |           ^^^^^^^^^^^^^^^^^^^^^^^^ cannot call associated function of trait
    ///     |
    ///    ::: src/basic/types/traits.rs:17:5
    ///     |
    /// 17  | /     fn basic_pile() -> BasicPile {
    /// 18  | |         BasicPile::from(Self::base_vec())
    /// 19  | |     }
    ///     | |_____- `DeckedBase::basic_pile` defined here
    ///     |
    /// help: use a fully-qualified path to one of the available implementations
    ///     |
    /// 151 |         <Canasta as DeckedBase>::basic_pile().contains(&self.base_card)
    ///     |         +++++++++++           +
    /// 151 |         <Euchre24 as DeckedBase>::basic_pile().contains(&self.base_card)
    ///     |         ++++++++++++           +
    /// 151 |         <Euchre32 as DeckedBase>::basic_pile().contains(&self.base_card)
    ///     |         ++++++++++++           +
    /// 151 |         <French as DeckedBase>::basic_pile().contains(&self.base_card)
    ///     |         ++++++++++           +
    ///       and 10 other candidates
    /// ```
    ///
    /// This is one of the many reasons why I love `Rust`. Even when it doesn't spell it out for you,
    /// it does make your life a lot easier.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        <DeckType as DeckedBase>::basic_pile().contains(&self.base_card)
    }

    /// Returns the default, aka `US_ENGLISH`, version of the  long name of the whole `Card`
    /// from the `Named` trait's use of fluent templates in the rank and suit [`Pip`]s for the `Card`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::NINE_CLUBS);
    ///
    /// assert_eq!("Nine of Clubs", card.fluent_name_default());
    /// ```
    #[cfg(feature = "i18n")]
    #[must_use]
    pub fn fluent_name_default(&self) -> String {
        self.fluent_name(&FluentName::US_ENGLISH)
    }

    /// Returns the long name of the whole `Card` from the `Named` trait's use of fluent templates
    /// in the rank and suit [`Pip`]s for the `Card`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::NINE_CLUBS);
    ///
    /// assert_eq!("Nine of Clubs", card.fluent_name(&FluentName::US_ENGLISH));
    /// assert_eq!("Neun Klee", card.fluent_name(&FluentName::DEUTSCH));
    /// ```
    /// TODO: HACK
    #[cfg(feature = "i18n")]
    #[must_use]
    pub fn fluent_name(&self, lid: &LanguageIdentifier) -> String {
        match self.base_card.suit.pip_type {
            PipType::Special => self.fluent_rank_name(lid),
            PipType::Joker => {
                format!("Joker {}", self.fluent_rank_name(lid))
            }
            _ => {
                format!(
                    "{}{}{}",
                    self.fluent_rank_name(lid),
                    Self::fluent_connector(lid),
                    self.fluent_suit_name(lid)
                )
            }
        }
    }

    /// Returns the connector string for the rank and suit [`Pip`]s in the `Card`'s name.
    ///
    /// TODO RF: Need a more configurable way to do this.
    #[cfg(feature = "i18n")]
    fn fluent_connector(lid: &LanguageIdentifier) -> String {
        match lid {
            &FluentName::DEUTSCH | &FluentName::LATINA | &FluentName::TLHINGAN => " ".to_string(),
            &FluentName::FRANCAIS => " de ".to_string(),
            _ => " of ".to_string(),
        }
    }

    /// Returns the long name of the rank [`Pip`] for the `Card` set in the localization files.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::DEUCE_DIAMONDS);
    ///
    /// assert_eq!("Deuce", card.fluent_rank_name(&FluentName::US_ENGLISH));
    /// assert_eq!("Zwei", card.fluent_rank_name(&FluentName::DEUTSCH));
    /// ```
    ///
    /// TODO: HACK I am feeling like I have begun to outlive my need
    /// for fluent templates. The deck from yaml idea feels like the path.
    #[cfg(feature = "i18n")]
    #[must_use]
    pub fn fluent_rank_name(&self, lid: &LanguageIdentifier) -> String {
        let s: String = match self.base_card.suit.pip_type {
            PipType::Special => {
                format!(
                    "{}-special-{}",
                    DeckType::fluent_name_base(),
                    self.base_card.rank.index.to_lowercase()
                )
            }
            _ => {
                format!(
                    "{}-{}",
                    DeckType::fluent_name_base(),
                    self.base_card.rank.index.to_lowercase()
                )
            }
        };

        FluentName::new("name-rank").fluent_value(s.as_str(), lid)
    }

    /// Returns the long name of the suit [`Pip`] for the `Card` set in the localization files.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::DEUCE_DIAMONDS);
    ///
    /// assert_eq!("Diamonds", card.fluent_suit_name(&FluentName::US_ENGLISH));
    /// assert_eq!("Diamanten", card.fluent_suit_name(&FluentName::DEUTSCH));
    /// ```
    ///
    #[cfg(feature = "i18n")]
    #[must_use]
    pub fn fluent_suit_name(&self, lid: &LanguageIdentifier) -> String {
        let s = format!(
            "{}-{}",
            DeckType::fluent_name_base(),
            self.base_card.suit.index.to_lowercase()
        );
        FluentName::new("name-suit").fluent_value(s.as_str(), lid)
    }
}

impl<DeckType: DeckedBase> DeckedBase for Card<DeckType> {
    /// Pass through call to the `Card's` underlying type parameter.
    fn base_vec() -> Vec<BasicCard> {
        DeckType::base_vec()
    }

    /// Pass through call to the `Card's` underlying type parameter.
    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        DeckType::colors()
    }

    /// Pass through call to the `Card's` underlying type parameter.
    fn deck_name() -> String {
        DeckType::deck_name()
    }

    /// Pass through call to the `Card's` underlying type parameter.
    fn fluent_deck_key() -> String {
        DeckType::fluent_deck_key()
    }
}

impl<DeckType: Default + Copy + Ord + DeckedBase> Display for Card<DeckType> {
    /// Passes through the `Display` result from the underlying [`BasicCard`].
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
    ///
    /// assert_eq!(card.to_string(), card.base_card.to_string());
    /// ```
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}", self.base_card)
    }
}

impl<DeckType: DeckedBase> From<BasicCard> for Card<DeckType> {
    fn from(pips: BasicCard) -> Self {
        Self {
            base_card: pips,
            deck: PhantomData,
        }
    }
}

impl<DeckType: DeckedBase> From<&BasicCard> for Card<DeckType> {
    fn from(pips: &BasicCard) -> Self {
        Self::from(*pips)
    }
}

impl<DeckType: DeckedBase> FromStr for Card<DeckType> {
    type Err = CardError;

    /// Cards can be created from strings in any combination of index and symbol strings in upper
    /// and lowercase letters.
    ///
    /// # Indexes
    ///
    /// A `Card's` index is make of the unique char (`Pip.index`) or symbol (`Pip.symbol`) for the
    /// suit `Pip` and a unique char for the rank `Pip`. The implementation of the trait is
    /// designed to be very forgiving. For example:
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
    ///
    /// let possible = vec!["AS", "as", "aS", "As", "A♠", "a♠"];
    ///
    /// for s in possible {
    ///    assert_eq!(card, Card::<French>::from_str(s).unwrap());
    /// }
    /// ```
    ///
    /// We've changed the contract for index strings in one way: we are adding support for blank
    /// cards, aka `__`. This is so you can represent a collection that includes a blank spot,
    /// such as `J♥ T♥ __`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();

        let mut cards = Self::base_vec();
        // Add a blank card to the list of cards so that it is considered valid.
        cards.push(BasicCard::default());

        cards
            .iter()
            .find(|c| (c.to_string() == s) || (c.index() == s))
            .copied()
            .map(Self::from)
            .ok_or_else(|| CardError::InvalidCard(s.clone()))
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__card_tests {
    use super::*;
    use crate::basic::decks::cards::french::FrenchBasicCard;
    use crate::basic::decks::french::French;
    use crate::prelude::SkatBasicCard;

    #[test]
    fn new() {
        let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
        assert_eq!(FrenchBasicCard::ACE_SPADES, card.base());
        assert_eq!(card.to_string(), "A♠");
    }

    /// This test exposes a flaw with my underlying logic. Going to create a validator
    /// of some sort.
    #[test]
    fn new__invalid_basic_card() {
        let card = Card::<French>::new(SkatBasicCard::KÖNIG_LAUB);
        assert_eq!(SkatBasicCard::KÖNIG_LAUB, card.base());
    }

    #[test]
    fn is_blank() {
        let card = Card::<French>::default();
        assert!(card.is_blank());
    }

    #[test]
    fn is_valid() {
        assert!(Card::<French>::new(FrenchBasicCard::ACE_SPADES).is_valid());
        assert!(!Card::<French>::new(SkatBasicCard::KÖNIG_LAUB).is_valid());
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent_name() {
        let nine_of_clubs: Card<French> = FrenchBasicCard::NINE_CLUBS.into();

        assert_eq!(
            "Nine of Clubs",
            nine_of_clubs.fluent_name(&FluentName::US_ENGLISH)
        );
        assert_eq!("Neun Klee", nine_of_clubs.fluent_name(&FluentName::DEUTSCH));
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent_name_default() {
        let eight_of_diamonds: Card<French> = FrenchBasicCard::EIGHT_DIAMONDS.into();

        assert_eq!("Eight of Diamonds", eight_of_diamonds.fluent_name_default());
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent_rank_name() {
        let card: Card<French> = FrenchBasicCard::NINE_CLUBS.into();
        assert_eq!("Nine", card.fluent_rank_name(&FluentName::US_ENGLISH));
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent_suit_name() {
        let card: Card<French> = FrenchBasicCard::NINE_CLUBS.into();
        assert_eq!("Clubs", card.fluent_suit_name(&FluentName::US_ENGLISH));
    }

    #[test]
    fn decked_base__vec() {
        let cards = Card::<French>::base_vec();

        let s: String = cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(
            "B🃟, L🃟, A♠, K♠, Q♠, J♠, T♠, 9♠, 8♠, 7♠, 6♠, 5♠, 4♠, 3♠, 2♠, A♥, K♥, Q♥, J♥, T♥, 9♥, 8♥, 7♥, 6♥, 5♥, 4♥, 3♥, 2♥, A♦, K♦, Q♦, J♦, T♦, 9♦, 8♦, 7♦, 6♦, 5♦, 4♦, 3♦, 2♦, A♣, K♣, Q♣, J♣, T♣, 9♣, 8♣, 7♣, 6♣, 5♣, 4♣, 3♣, 2♣",
            s
        );
    }

    #[test]
    fn display() {
        let basecard = FrenchBasicCard::ACE_SPADES;
        let card: Card<French> = basecard.into();

        assert_eq!("A♠", card.to_string());
        assert_eq!("A♠", basecard.to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!("AS", Card::<French>::from_str("as").unwrap().index());
        assert_eq!("__", Card::<French>::from_str("__").unwrap().index());
    }

    #[test]
    fn to_string_from_str() {
        let base_cards = Card::<French>::base_vec();

        for base_card in base_cards {
            let card: Card<French> = Card::<French>::from(base_card);
            let s = card.to_string();
            let index = card.index();

            assert_eq!(card, Card::<French>::from_str(&s).unwrap());
            assert_eq!(card, Card::<French>::from_str(&index).unwrap());
            assert_eq!(card, Card::<French>::from_str(&s.to_lowercase()).unwrap());
            assert_eq!(
                card,
                Card::<French>::from_str(&index.to_lowercase()).unwrap()
            );
        }
    }

    // Helper: force colors on for testing color_string match arms
    #[cfg(feature = "colored-display")]
    fn with_colors_enabled<F: FnOnce()>(f: F) {
        colored::control::set_override(true);
        f();
        colored::control::unset_override();
    }

    // Minimal deck type factories for each Color variant used in color_string tests.
    // Each creates a deck that maps FrenchSuit::HEARTS to the target color.
    #[cfg(feature = "colored-display")]
    macro_rules! color_deck {
        ($name:ident, $color:expr) => {
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            struct $name;
            impl DeckedBase for $name {
                fn base_vec() -> Vec<BasicCard> {
                    vec![FrenchBasicCard::ACE_HEARTS]
                }
                fn colors() -> HashMap<Pip, colored::Color> {
                    use crate::basic::decks::cards::french::FrenchSuit;
                    let mut m = HashMap::new();
                    m.insert(FrenchSuit::HEARTS, $color);
                    m
                }
                fn deck_name() -> String {
                    stringify!($name).to_string()
                }
                fn fluent_deck_key() -> String {
                    "french".to_string()
                }
            }
        };
    }

    #[cfg(feature = "colored-display")]
    color_deck!(RedDeck, colored::Color::Red);
    #[cfg(feature = "colored-display")]
    color_deck!(BlueDeck, colored::Color::Blue);
    #[cfg(feature = "colored-display")]
    color_deck!(GreenDeck, colored::Color::Green);
    #[cfg(feature = "colored-display")]
    color_deck!(YellowDeck, colored::Color::Yellow);
    #[cfg(feature = "colored-display")]
    color_deck!(MagentaDeck, colored::Color::Magenta);
    #[cfg(feature = "colored-display")]
    color_deck!(CyanDeck, colored::Color::Cyan);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightBlackDeck, colored::Color::BrightBlack);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightRedDeck, colored::Color::BrightRed);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightGreenDeck, colored::Color::BrightGreen);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightYellowDeck, colored::Color::BrightYellow);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightBlueDeck, colored::Color::BrightBlue);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightMagentaDeck, colored::Color::BrightMagenta);
    #[cfg(feature = "colored-display")]
    color_deck!(BrightCyanDeck, colored::Color::BrightCyan);

    /// Verifies that color_string applies the expected ANSI color to the output.
    /// Each sub-assertion catches the "delete match arm Color::X" mutation.
    #[cfg(feature = "colored-display")]
    #[test]
    fn color_string__all_variants() {
        use colored::Colorize as _;
        let ace_hearts = FrenchBasicCard::ACE_HEARTS;
        let index = ace_hearts.index();

        with_colors_enabled(|| {
            assert_eq!(
                Card::<RedDeck>::new(ace_hearts).color_index_string(),
                index.red().to_string()
            );
            assert_eq!(
                Card::<BlueDeck>::new(ace_hearts).color_index_string(),
                index.blue().to_string()
            );
            assert_eq!(
                Card::<GreenDeck>::new(ace_hearts).color_index_string(),
                index.green().to_string()
            );
            assert_eq!(
                Card::<YellowDeck>::new(ace_hearts).color_index_string(),
                index.yellow().to_string()
            );
            assert_eq!(
                Card::<MagentaDeck>::new(ace_hearts).color_index_string(),
                index.magenta().to_string()
            );
            assert_eq!(
                Card::<CyanDeck>::new(ace_hearts).color_index_string(),
                index.cyan().to_string()
            );
            assert_eq!(
                Card::<BrightBlackDeck>::new(ace_hearts).color_index_string(),
                index.bright_black().to_string()
            );
            assert_eq!(
                Card::<BrightRedDeck>::new(ace_hearts).color_index_string(),
                index.bright_red().to_string()
            );
            assert_eq!(
                Card::<BrightGreenDeck>::new(ace_hearts).color_index_string(),
                index.bright_green().to_string()
            );
            assert_eq!(
                Card::<BrightYellowDeck>::new(ace_hearts).color_index_string(),
                index.bright_yellow().to_string()
            );
            assert_eq!(
                Card::<BrightBlueDeck>::new(ace_hearts).color_index_string(),
                index.bright_blue().to_string()
            );
            assert_eq!(
                Card::<BrightMagentaDeck>::new(ace_hearts).color_index_string(),
                index.bright_magenta().to_string()
            );
            assert_eq!(
                Card::<BrightCyanDeck>::new(ace_hearts).color_index_string(),
                index.bright_cyan().to_string()
            );
        });
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent_name__joker() {
        let joker_card = Card::<French>::new(FrenchBasicCard::BIG_JOKER);
        let name = joker_card.fluent_name(&FluentName::US_ENGLISH);
        // Joker match arm produces "Joker <rank_name>"
        assert!(
            name.starts_with("Joker"),
            "Expected Joker prefix, got: {name}"
        );
        assert!(!name.is_empty());
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked_base__colors__passthrough() {
        // Catches colors() -> HashMap::new() mutation on Card<DeckType>
        assert!(!Card::<crate::prelude::Standard52>::colors().is_empty());
    }

    #[test]
    fn decked_base__deck_name__passthrough() {
        // Catches deck_name() -> String::new() and -> "xyzzy" mutations
        let name = Card::<crate::prelude::Standard52>::deck_name();
        assert!(!name.is_empty());
        assert_ne!(name, "xyzzy");
        assert_eq!(name, "Standard 52");
    }

    #[test]
    fn decked_base__fluent_deck_key__passthrough() {
        // Catches fluent_deck_key() mutations on Card<DeckType>
        let key = Card::<crate::prelude::Standard52>::fluent_deck_key();
        assert!(!key.is_empty());
        assert_ne!(key, "xyzzy");
    }
}
