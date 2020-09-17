use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::fluent::US_ENGLISH;

/// `Card` is the core struct in the library. A Card is made up of a Rank,
/// a Suit and weight, which is an integer that controls how a card is sorted
/// in a Pile or as a part of a Vector.
///
///
///
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    pub weight: isize,
    pub index: String,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Instantiates a new Card with the default weight as defined in the fluent
    /// templates.
    pub fn new<S: std::clone::Clone>(rank: S, suit: S) -> Card
    where
        S: Into<String>,
    {
        let a = Suit::new(suit);
        let r = Rank::new(rank);
        let weight = Card::determine_weight(&a, &r);
        let index = Card::determine_index(&a, &r);
        Card {
            weight,
            index,
            suit: a,
            rank: r,
        }
    }

    /// Instantiates a Card with the weight determined by the passed in Rank and
    /// Suit.
    pub fn new_from_structs(rank: Rank, suit: Suit) -> Card {
        let weight = Card::determine_weight(&suit, &rank);
        let index = Card::determine_index(&suit, &rank);
        Card {
            weight,
            index,
            rank,
            suit,
        }
    }

    fn determine_index(suit: &Suit, rank: &Rank) -> String {
        let rank = rank.get_index(&US_ENGLISH);
        let suit = suit.get_short(&US_ENGLISH);
        format!("{}{}", rank, suit)
    }

    /// Prioritizes sorting by Suit and then by Rank.
    fn determine_weight(suit: &Suit, rank: &Rank) -> isize {
        (suit.weight * 1000) + rank.weight
    }

    pub fn to_symbol_string(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.get_index(&lid);
        let suit = self.suit.get_symbol();
        format!("{}{}", rank, suit)
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.get_index(&lid);
        let suit = self.suit.get_short(&lid);
        format!("{}{}", rank, suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::GERMAN;

    #[test]
    fn new() {
        let expected = Card {
            weight: 4014,
            index: "AS".to_string(),
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(expected, Card::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            weight: 4014,
            index: "AS".to_string(),
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(
            expected,
            Card::new_from_structs(Rank::new("ace"), Suit::new("spades"))
        );
    }

    #[test]
    fn to_txt_string() {
        let card = Card::new("queen", "clubs");

        assert_eq!(card.to_txt_string(&GERMAN), "DK".to_string());
    }

    #[test]
    fn to_symbol_string() {
        let card = Card::new("queen", "hearts");

        assert_eq!(card.to_symbol_string(&GERMAN), "D♥".to_string());
    }
}
