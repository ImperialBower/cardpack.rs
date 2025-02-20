use crate::basic::decks::cards;
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::decks::standard52::Standard52;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::DeckedBase;
use crate::prelude::{Card, Deck};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Euchre32 {}

#[allow(clippy::module_name_repetitions)]
pub type Euchre32Deck = Deck<Euchre32>;
#[allow(clippy::module_name_repetitions)]
pub type Euchre32Card = Card<Euchre32>;

impl Euchre32 {
    pub const DECK_SIZE: usize = 32;

    pub const DECK: [BasicCard; Euchre32::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::EIGHT_SPADES,
        FrenchBasicCard::SEVEN_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::EIGHT_HEARTS,
        FrenchBasicCard::SEVEN_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::EIGHT_DIAMONDS,
        FrenchBasicCard::SEVEN_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
        FrenchBasicCard::EIGHT_CLUBS,
        FrenchBasicCard::SEVEN_CLUBS,
    ];
}

impl DeckedBase for Euchre32 {
    fn base_vec() -> Vec<BasicCard> {
        Euchre32::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Euchre 32".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__euchre32_tests {
    use super::*;
    use crate::basic::decks::french::French;
    use crate::basic::types::deck::Deck;
    use crate::basic::types::traits::Decked;

    #[test]
    fn decked__deck() {
        assert_eq!(
            Deck::<Euchre32>::deck().to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Deck::<Euchre32>::validate());
    }
}
