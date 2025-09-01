use crate::basic::decks::cards;
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::decks::cards::pinochle::PinochleBasicCard;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use crate::prelude::{Card, Pile, Standard52};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pinochle {}

#[allow(clippy::module_name_repetitions)]
pub type PinochleDeck = Pile<Pinochle>;
#[allow(clippy::module_name_repetitions)]
pub type PinochleCard = Card<Pinochle>;

impl Pinochle {
    pub const DECK_SIZE: usize = 48;

    pub const DECK: [BasicCard; Self::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::ACE_SPADES,
        PinochleBasicCard::TEN_SPADES,
        PinochleBasicCard::TEN_SPADES,
        PinochleBasicCard::KING_SPADES,
        PinochleBasicCard::KING_SPADES,
        PinochleBasicCard::QUEEN_SPADES,
        PinochleBasicCard::QUEEN_SPADES,
        PinochleBasicCard::JACK_SPADES,
        PinochleBasicCard::JACK_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::ACE_HEARTS,
        PinochleBasicCard::TEN_HEARTS,
        PinochleBasicCard::TEN_HEARTS,
        PinochleBasicCard::KING_HEARTS,
        PinochleBasicCard::KING_HEARTS,
        PinochleBasicCard::QUEEN_HEARTS,
        PinochleBasicCard::QUEEN_HEARTS,
        PinochleBasicCard::JACK_HEARTS,
        PinochleBasicCard::JACK_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::ACE_DIAMONDS,
        PinochleBasicCard::TEN_DIAMONDS,
        PinochleBasicCard::TEN_DIAMONDS,
        PinochleBasicCard::KING_DIAMONDS,
        PinochleBasicCard::KING_DIAMONDS,
        PinochleBasicCard::QUEEN_DIAMONDS,
        PinochleBasicCard::QUEEN_DIAMONDS,
        PinochleBasicCard::JACK_DIAMONDS,
        PinochleBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::ACE_CLUBS,
        PinochleBasicCard::TEN_CLUBS,
        PinochleBasicCard::TEN_CLUBS,
        PinochleBasicCard::KING_CLUBS,
        PinochleBasicCard::KING_CLUBS,
        PinochleBasicCard::QUEEN_CLUBS,
        PinochleBasicCard::QUEEN_CLUBS,
        PinochleBasicCard::JACK_CLUBS,
        PinochleBasicCard::JACK_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
    ];
}

impl DeckedBase for Pinochle {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Pinochle".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::pinochle::FLUENT_KEY_BASE_NAME_PINOCHLE.to_string()
    }
}

impl Decked<Self> for Pinochle {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__pinochle_tests {
    use super::*;
    use crate::basic::decks::french::French;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::Decked;

    #[test]
    fn decked__validate() {
        assert!(Pinochle::validate());
    }
}
