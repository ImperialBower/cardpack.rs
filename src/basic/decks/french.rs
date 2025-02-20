use crate::basic::decks::cards;
use crate::basic::decks::cards::french::{FrenchBasicCard, FrenchSuit};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::deck::Deck;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::DeckedBase;
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct French {}
#[allow(clippy::module_name_repetitions)]
pub type FrenchDeck = Deck<French>;
#[allow(clippy::module_name_repetitions)]
pub type FrenchCard = Card<French>;

impl French {
    pub const DECK_SIZE: usize = 54;

    pub const DECK: [BasicCard; French::DECK_SIZE] = [
        FrenchBasicCard::BIG_JOKER,
        FrenchBasicCard::LITTLE_JOKER,
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::EIGHT_SPADES,
        FrenchBasicCard::SEVEN_SPADES,
        FrenchBasicCard::SIX_SPADES,
        FrenchBasicCard::FIVE_SPADES,
        FrenchBasicCard::FOUR_SPADES,
        FrenchBasicCard::TREY_SPADES,
        FrenchBasicCard::DEUCE_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::EIGHT_HEARTS,
        FrenchBasicCard::SEVEN_HEARTS,
        FrenchBasicCard::SIX_HEARTS,
        FrenchBasicCard::FIVE_HEARTS,
        FrenchBasicCard::FOUR_HEARTS,
        FrenchBasicCard::TREY_HEARTS,
        FrenchBasicCard::DEUCE_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::EIGHT_DIAMONDS,
        FrenchBasicCard::SEVEN_DIAMONDS,
        FrenchBasicCard::SIX_DIAMONDS,
        FrenchBasicCard::FIVE_DIAMONDS,
        FrenchBasicCard::FOUR_DIAMONDS,
        FrenchBasicCard::TREY_DIAMONDS,
        FrenchBasicCard::DEUCE_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::EIGHT_CLUBS,
        FrenchBasicCard::SEVEN_CLUBS,
        FrenchBasicCard::SIX_CLUBS,
        FrenchBasicCard::FIVE_CLUBS,
        FrenchBasicCard::FOUR_CLUBS,
        FrenchBasicCard::TREY_CLUBS,
        FrenchBasicCard::DEUCE_CLUBS,
    ];
}

impl DeckedBase for French {
    fn base_vec() -> Vec<BasicCard> {
        French::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(FrenchSuit::JOKER, Color::Blue);
        mappie.insert(FrenchSuit::HEARTS, Color::Red);
        mappie.insert(FrenchSuit::DIAMONDS, Color::Red);

        mappie
    }

    fn deck_name() -> String {
        "French".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__french__tests {
    use super::*;
    use crate::basic::decks::french::French;
    use crate::basic::types::card::Card;
    use crate::basic::types::traits::Decked;
    use std::str::FromStr;

    #[test]
    fn from_str__card() {
        assert_eq!(
            Card::<French>::from_str("2c").unwrap(),
            FrenchBasicCard::DEUCE_CLUBS.into()
        );
    }

    #[test]
    fn from_str__pile() {
        let pile = Deck::<French>::from_str("2c 3c 4c").unwrap();

        assert_eq!(pile.len(), 3);
        assert_eq!(pile.to_string(), "2♣ 3♣ 4♣");
    }

    #[test]
    fn decked__validate() {
        assert!(Deck::<French>::validate());
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Deck::<French>::deck_name(), "French");
    }
}
