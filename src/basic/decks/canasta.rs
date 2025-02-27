use crate::basic::decks::cards::canasta::{CanastaBasicCard, FLUENT_KEY_BASE_NAME_CANASTA};
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::decks::french::French;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use colored::Color;
use std::collections::HashMap;

/// [Canasta](https://en.wikipedia.org/wiki/Canasta) deck
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Canasta {}
#[allow(clippy::module_name_repetitions)]
pub type CanastaDeck = Pile<Canasta>;
#[allow(clippy::module_name_repetitions)]
pub type CanastaCard = Card<Canasta>;

impl Canasta {
    pub const DECK_SIZE: usize = 108;

    pub const DECK: [BasicCard; Canasta::DECK_SIZE] = [
        CanastaBasicCard::TREY_HEARTS,
        CanastaBasicCard::TREY_HEARTS,
        CanastaBasicCard::TREY_DIAMONDS,
        CanastaBasicCard::TREY_DIAMONDS,
        CanastaBasicCard::BIG_JOKER,
        CanastaBasicCard::BIG_JOKER,
        CanastaBasicCard::LITTLE_JOKER,
        CanastaBasicCard::LITTLE_JOKER,
        CanastaBasicCard::DEUCE_SPADES,
        CanastaBasicCard::DEUCE_SPADES,
        CanastaBasicCard::DEUCE_HEARTS,
        CanastaBasicCard::DEUCE_HEARTS,
        CanastaBasicCard::DEUCE_DIAMONDS,
        CanastaBasicCard::DEUCE_DIAMONDS,
        CanastaBasicCard::DEUCE_CLUBS,
        CanastaBasicCard::DEUCE_CLUBS,
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::EIGHT_SPADES,
        FrenchBasicCard::EIGHT_SPADES,
        FrenchBasicCard::SEVEN_SPADES,
        FrenchBasicCard::SEVEN_SPADES,
        FrenchBasicCard::SIX_SPADES,
        FrenchBasicCard::SIX_SPADES,
        FrenchBasicCard::FIVE_SPADES,
        FrenchBasicCard::FIVE_SPADES,
        FrenchBasicCard::FOUR_SPADES,
        FrenchBasicCard::FOUR_SPADES,
        FrenchBasicCard::TREY_SPADES,
        FrenchBasicCard::TREY_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::EIGHT_HEARTS,
        FrenchBasicCard::EIGHT_HEARTS,
        FrenchBasicCard::SEVEN_HEARTS,
        FrenchBasicCard::SEVEN_HEARTS,
        FrenchBasicCard::SIX_HEARTS,
        FrenchBasicCard::SIX_HEARTS,
        FrenchBasicCard::FIVE_HEARTS,
        FrenchBasicCard::FIVE_HEARTS,
        FrenchBasicCard::FOUR_HEARTS,
        FrenchBasicCard::FOUR_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::EIGHT_DIAMONDS,
        FrenchBasicCard::EIGHT_DIAMONDS,
        FrenchBasicCard::SEVEN_DIAMONDS,
        FrenchBasicCard::SEVEN_DIAMONDS,
        FrenchBasicCard::SIX_DIAMONDS,
        FrenchBasicCard::SIX_DIAMONDS,
        FrenchBasicCard::FIVE_DIAMONDS,
        FrenchBasicCard::FIVE_DIAMONDS,
        FrenchBasicCard::FOUR_DIAMONDS,
        FrenchBasicCard::FOUR_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::EIGHT_CLUBS,
        FrenchBasicCard::EIGHT_CLUBS,
        FrenchBasicCard::SEVEN_CLUBS,
        FrenchBasicCard::SEVEN_CLUBS,
        FrenchBasicCard::SIX_CLUBS,
        FrenchBasicCard::SIX_CLUBS,
        FrenchBasicCard::FIVE_CLUBS,
        FrenchBasicCard::FIVE_CLUBS,
        FrenchBasicCard::FOUR_CLUBS,
        FrenchBasicCard::FOUR_CLUBS,
        FrenchBasicCard::TREY_CLUBS,
        FrenchBasicCard::TREY_CLUBS,
    ];
}

impl DeckedBase for Canasta {
    fn base_vec() -> Vec<BasicCard> {
        Canasta::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        French::colors()
    }

    fn deck_name() -> String {
        "Canasta".to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_CANASTA.to_string()
    }
}

impl Decked<Canasta> for Canasta {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__canasta_tests {
    use super::*;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::Decked;

    #[test]
    fn decked__deck() {
        assert_eq!(
            Canasta::deck().to_string(),
            "3♥ 3♥ 3♦ 3♦ B🃟 B🃟 L🃟 L🃟 2♠ 2♠ 2♥ 2♥ 2♦ 2♦ 2♣ 2♣ A♠ A♠ K♠ K♠ Q♠ Q♠ J♠ J♠ T♠ T♠ 9♠ 9♠ 8♠ 8♠ 7♠ 7♠ 6♠ 6♠ 5♠ 5♠ 4♠ 4♠ 3♠ 3♠ A♥ A♥ K♥ K♥ Q♥ Q♥ J♥ J♥ T♥ T♥ 9♥ 9♥ 8♥ 8♥ 7♥ 7♥ 6♥ 6♥ 5♥ 5♥ 4♥ 4♥ A♦ A♦ K♦ K♦ Q♦ Q♦ J♦ J♦ T♦ T♦ 9♦ 9♦ 8♦ 8♦ 7♦ 7♦ 6♦ 6♦ 5♦ 5♦ 4♦ 4♦ A♣ A♣ K♣ K♣ Q♣ Q♣ J♣ J♣ T♣ T♣ 9♣ 9♣ 8♣ 8♣ 7♣ 7♣ 6♣ 6♣ 5♣ 5♣ 4♣ 4♣ 3♣ 3♣"
        );
    }

    #[test]
    pub fn ranks_index() {
        let pile = Canasta::deck().shuffled();
        let expected = "3~B~L~2~A~K~Q~J~T~9~8~7~6~5~4~3";

        let ranks_index = pile.into_basic_pile().ranks_index("~");

        assert_eq!(ranks_index, expected);
    }

    /// TODO: WTF??!!
    /// TODO: WTF do I mean by WTF??? Don't do this.
    #[test]
    pub fn suits_index() {
        let pile = Canasta::deck().shuffled().into_basic_pile();
        let expected = "H~D~J~S~H~D~C~S~H~D~C";

        let ranks_index = pile.suits_index("~");

        assert_eq!(ranks_index, expected);
    }

    #[test]
    fn decked__validate() {
        assert!(Canasta::validate());
    }
}
