use crate::basic::decks::cards;
use crate::basic::decks::cards::french::{FrenchBasicCard, FrenchSuit};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::Pip;
use crate::common::traits::{Decked, DeckedBase};
use crate::prelude::{Card, Deck};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Standard52 {}
#[allow(clippy::module_name_repetitions)]
pub type Standard52Deck = Deck<Standard52>;
#[allow(clippy::module_name_repetitions)]
pub type Standard52Card = Card<Standard52>;

impl Standard52 {
    pub const DECK_SIZE: usize = 52;

    pub const DECK: [BasicCard; Standard52::DECK_SIZE] = [
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

impl DeckedBase for Standard52 {
    fn base_vec() -> Vec<BasicCard> {
        Standard52::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(FrenchSuit::HEARTS, Color::Red);
        mappie.insert(FrenchSuit::DIAMONDS, Color::Red);

        mappie
    }

    fn deck_name() -> String {
        "Standard 52".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

impl Decked<Standard52> for Standard52 {}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__card__standard52_tests {
    use super::*;
    use crate::basic::types::deck::Deck;
    use crate::common::traits::Decked;

    #[test]
    fn decked__validate() {
        assert!(Standard52::validate());
    }
}
