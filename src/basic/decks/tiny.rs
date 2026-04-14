use crate::prelude::{
    BasicCard, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard, Pip, Standard52,
};
use colored::Color;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tiny {}

impl Tiny {
    pub const DECK_SIZE: usize = 4;

    pub const DECK: [BasicCard; Self::DECK_SIZE] = [
        FrenchBasicCard::ACE_SPADES,
        FrenchBasicCard::KING_SPADES,
        FrenchBasicCard::ACE_HEARTS,
        FrenchBasicCard::KING_HEARTS,
    ];
}

impl DeckedBase for Tiny {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        Standard52::colors()
    }

    fn deck_name() -> String {
        "Tiny".to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }
}

// Let's you call Decked methods directly on the Tiny type:
impl Decked<Self> for Tiny {}

#[allow(unused_macros)]
macro_rules! tiny {
    (AS) => {
        Card::<Tiny>::new(FrenchBasicCard::ACE_SPADES)
    };
    (KS) => {
        Card::<Tiny>::new(FrenchBasicCard::KING_SPADES)
    };
    (AH) => {
        Card::<Tiny>::new(FrenchBasicCard::ACE_HEARTS)
    };
    (KH) => {
        Card::<Tiny>::new(FrenchBasicCard::KING_HEARTS)
    };
    (__) => {
        Card::<Tiny>::default()
    };
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__tiny__tests {
    use super::*;
    use crate::prelude::*;

    // This is
    #[test]
    fn test() {
        let mut deck = Tiny::deck();

        assert_eq!(deck.to_string(), "A♠ K♠ A♥ K♥");

        // Every deck comes with the Ranged trait automatically:
        assert_eq!(
            deck.combos(2).to_string(),
            "A♠ K♠, A♠ A♥, A♠ K♥, K♠ K♥, A♥ K♠, A♥ K♥"
        );

        // Deal from the top of the deck:
        assert_eq!(deck.draw_first().unwrap().to_string(), "A♠");

        // Deal from the bottom of the deck:
        assert_eq!(deck.draw_last().unwrap().to_string(), "K♥");

        // Should be two cards remaining:
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.index(), "KS AH");

        // Draw a remaining card:
        assert_eq!(deck.draw_first().unwrap(), tiny!(KS));

        // Draw the last card:
        assert_eq!(deck.draw_last().unwrap(), tiny!(AH));

        // And now the deck is empty:
        assert!(deck.draw_first().is_none());
        assert!(deck.draw_last().is_none());
    }

    #[test]
    fn validate() {
        assert!(Tiny::validate());
    }

    #[test]
    fn decked__colors() {
        assert!(!Tiny::colors().is_empty());
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Tiny::deck_name(), "Tiny");
    }

    #[test]
    fn decked__fluent_deck_key() {
        assert_eq!(
            Tiny::fluent_deck_key(),
            FLUENT_KEY_BASE_NAME_FRENCH.to_string()
        );
    }
}
