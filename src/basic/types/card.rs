use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};
use crate::common::errors::CardError;
use crate::common::traits::DeckedBase;
use crate::localization::{FluentName, Named};
use colored::{Color, Colorize};
use fluent_templates::LanguageIdentifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
)]
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
        Self {
            base_card,
            deck: PhantomData,
        }
    }

    #[must_use]
    pub fn base(&self) -> BasicCard {
        self.base_card
    }

    /// This feels heavy and hackie. It's not important enough to worry about.
    #[must_use]
    pub fn color(&self) -> Color {
        let binding = DeckType::colors();
        let color = binding.get(&self.base_card.suit);

        match color {
            Some(color) => *color,
            None => Color::White,
        }
    }

    #[must_use]
    pub fn color_symbol_string(&self) -> String {
        match self.color() {
            Color::Red => self.base_card.to_string().red().to_string(),
            Color::Blue => self.base_card.to_string().blue().to_string(),
            Color::Green => self.base_card.to_string().green().to_string(),
            Color::Yellow => self.base_card.to_string().yellow().to_string(),
            Color::Magenta => self.base_card.to_string().magenta().to_string(),
            Color::Cyan => self.base_card.to_string().cyan().to_string(),
            Color::BrightBlack => self.base_card.to_string().bright_black().to_string(),
            Color::BrightRed => self.base_card.to_string().bright_red().to_string(),
            Color::BrightGreen => self.base_card.to_string().bright_green().to_string(),
            Color::BrightYellow => self.base_card.to_string().bright_yellow().to_string(),
            Color::BrightBlue => self.base_card.to_string().bright_blue().to_string(),
            Color::BrightMagenta => self.base_card.to_string().bright_magenta().to_string(),
            Color::BrightCyan => self.base_card.to_string().bright_cyan().to_string(),
            _ => self.base_card.to_string(),
        }
    }

    #[must_use]
    pub fn index(&self) -> String {
        self.base_card.index()
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.base_card.is_blank()
    }

    #[must_use]
    pub fn fluent_name_default(&self) -> String {
        self.fluent_name(&FluentName::US_ENGLISH)
    }

    /// TODO: HACK
    #[must_use]
    pub fn fluent_name(&self, lid: &LanguageIdentifier) -> String {
        match self.base_card.suit.pip_type {
            PipType::Special => self.fluent_rank_name(lid).to_string(),
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

    fn fluent_connector(lid: &LanguageIdentifier) -> String {
        match lid {
            &FluentName::DEUTSCH => " ".to_string(),
            _ => " of ".to_string(),
        }
    }

    /// TODO: HACK I am feeling like I have begun to outlive my need
    /// for fluent templates. The deck from yaml idea feels like the path.
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
    fn base_vec() -> Vec<BasicCard> {
        DeckType::base_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        DeckType::colors()
    }

    fn deck_name() -> String {
        DeckType::deck_name()
    }

    fn fluent_deck_key() -> String {
        DeckType::fluent_deck_key()
    }
}

impl<DeckType: Default + Copy + Ord + DeckedBase> Display for Card<DeckType> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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
        Card::<DeckType>::from(*pips)
    }
}

/// We've changed the contract for index strings in one way: we are adding support for blank
/// cards, aka `__`. This is so you can represent a collection that includes a blank spot,
/// such as `J♥ T♥ __`
impl<DeckType: DeckedBase> FromStr for Card<DeckType> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();

        let mut cards = Card::<DeckType>::base_vec();
        cards.push(BasicCard::default());

        cards
            .iter()
            .find(|c| (c.to_string() == s) || (c.index() == s))
            .copied()
            .map(Card::<DeckType>::from)
            .ok_or(CardError::InvalidCard(s.to_string()))
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__card_tests {
    use super::*;
    use crate::basic::decks::cards::french::FrenchBasicCard;
    use crate::basic::decks::french::French;

    #[test]
    fn new() {
        let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
        assert_eq!(FrenchBasicCard::ACE_SPADES, card.base());
        assert_eq!(card.to_string(), "A♠");
    }

    #[test]
    fn is_blank() {
        let card = Card::<French>::default();
        assert!(card.is_blank());
    }

    #[test]
    fn fluent_name() {
        let nine_of_clubs: Card<French> = FrenchBasicCard::NINE_CLUBS.into();

        assert_eq!(
            "Nine of Clubs",
            nine_of_clubs.fluent_name(&FluentName::US_ENGLISH)
        );
        assert_eq!("Neun Klee", nine_of_clubs.fluent_name(&FluentName::DEUTSCH));
    }

    #[test]
    fn fluent_name_default() {
        let eight_of_diamonds: Card<French> = FrenchBasicCard::EIGHT_DIAMONDS.into();

        assert_eq!("Eight of Diamonds", eight_of_diamonds.fluent_name_default());
    }

    #[test]
    fn fluent_rank_name() {
        let card: Card<French> = FrenchBasicCard::NINE_CLUBS.into();
        assert_eq!("Nine", card.fluent_rank_name(&FluentName::US_ENGLISH));
    }

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
}
