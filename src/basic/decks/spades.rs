use crate::basic::decks::cards;
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::decks::french::French;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::traits::{Decked, DeckedBase};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Spades {}
#[allow(clippy::module_name_repetitions)]
pub type SpadesDeck = Pile<Spades>;
#[allow(clippy::module_name_repetitions)]
pub type SpadesCard = Card<Spades>;

impl Spades {
    pub const DECK_SIZE: usize = 52;

    pub const DECK: [BasicCard; Spades::DECK_SIZE] = [
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
    ];
}

impl DeckedBase for Spades {
    fn base_vec() -> Vec<BasicCard> {
        Spades::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        French::colors()
    }

    fn deck_name() -> String {
        "Spades".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

impl Decked<Spades> for Spades {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__spades_tests {
    use super::*;
    use crate::basic::decks::french::French;
    use crate::basic::types::card::Card;
    use crate::traits::Decked;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            Spades::deck().to_string(),
            "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣"
        );
    }

    #[test]
    fn from_str__card() {
        assert!(Card::<Spades>::from_str("2c").is_err());
    }

    #[test]
    fn from_str__pile() {
        let pile = Pile::<Spades>::from_str("2c 3c 4c");

        assert!(pile.is_err());
    }

    #[test]
    fn decked__validate() {
        assert!(Spades::validate());
    }
}
