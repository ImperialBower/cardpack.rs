use crate::basic::decks::cards;
use crate::basic::decks::cards::french::FrenchBasicCard;
use crate::basic::decks::standard52::Standard52;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use crate::prelude::{Card, Pile};
use colored::Color;
use std::collections::HashMap;
use std::hash::Hash;

/// This deck represents the most common 24 card form of
/// [Euchre](https://en.wikipedia.org/wiki/Euchre) with
/// `A K Q J T 9` ranks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Euchre24 {}

#[allow(clippy::module_name_repetitions)]
pub type Euchre24Deck = Pile<Euchre24>;
#[allow(clippy::module_name_repetitions)]
pub type Euchre24Card = Card<Euchre24>;

impl Euchre24 {
    pub const DECK_SIZE: usize = 24;

    pub const DECK: [BasicCard; Self::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::QUEEN_SPADES,
        FrenchBasicCard::JACK_SPADES,
        FrenchBasicCard::TEN_SPADES,
        FrenchBasicCard::NINE_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
        FrenchBasicCard::QUEEN_HEARTS,
        FrenchBasicCard::JACK_HEARTS,
        FrenchBasicCard::TEN_HEARTS,
        FrenchBasicCard::NINE_HEARTS,
        FrenchBasicCard::ACE_DIAMONDS,
        FrenchBasicCard::KING_DIAMONDS,
        FrenchBasicCard::QUEEN_DIAMONDS,
        FrenchBasicCard::JACK_DIAMONDS,
        FrenchBasicCard::TEN_DIAMONDS,
        FrenchBasicCard::NINE_DIAMONDS,
        FrenchBasicCard::ACE_CLUBS,
        FrenchBasicCard::KING_CLUBS,
        FrenchBasicCard::QUEEN_CLUBS,
        FrenchBasicCard::JACK_CLUBS,
        FrenchBasicCard::TEN_CLUBS,
        FrenchBasicCard::NINE_CLUBS,
    ];
}

impl DeckedBase for Euchre24 {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Euchre 24".to_string()
    }

    fn fluent_deck_key() -> String {
        cards::french::FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

impl Decked<Self> for Euchre24 {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__euchre24_tests {
    use super::*;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::Decked;

    #[test]
    fn decked__deck() {
        assert_eq!(
            Euchre24::deck().to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ A♥ K♥ Q♥ J♥ T♥ 9♥ A♦ K♦ Q♦ J♦ T♦ 9♦ A♣ K♣ Q♣ J♣ T♣ 9♣"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Euchre24::validate());
    }
}
