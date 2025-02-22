use crate::basic::decks::cards;
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::Pip;
use crate::common::traits::{Decked, DeckedBase};
use crate::prelude::{Card, Deck, Standard52};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

/// [Manila, aka Six Plus aka Short-deck](https://en.wikipedia.org/wiki/Six-plus_hold_%27em)
/// is a version of Texas Hold'em where the card Ranks of 2 through 5
/// are removed from the deck.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Short {}

#[allow(clippy::module_name_repetitions)]
pub type ShortDeck = Deck<Short>;
#[allow(clippy::module_name_repetitions)]
pub type ShortCard = Card<Short>;

impl Short {
    pub const DECK_SIZE: usize = 36;

    pub const DECK: [BasicCard; Short::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::EIGHT_SPADES,
        FrenchBasicCard::SEVEN_SPADES,
        FrenchBasicCard::SIX_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::EIGHT_HEARTS,
        FrenchBasicCard::SEVEN_HEARTS,
        FrenchBasicCard::SIX_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::EIGHT_DIAMONDS,
        FrenchBasicCard::SEVEN_DIAMONDS,
        FrenchBasicCard::SIX_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::EIGHT_CLUBS,
        FrenchBasicCard::SEVEN_CLUBS,
        FrenchBasicCard::SIX_CLUBS,
    ];
}

impl DeckedBase for Short {
    fn base_vec() -> Vec<BasicCard> {
        Short::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Short".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

impl Decked<Short> for Short {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__short_tests {
    use super::*;
    use crate::basic::types::deck::Deck;
    use crate::common::traits::Decked;

    #[test]
    fn deck() {
        let deck = Short::deck();
        assert_eq!(deck.len(), 36);
        assert_eq!(deck.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣");
    }

    #[test]
    fn decked__validate() {
        assert!(Short::validate());
    }
}
