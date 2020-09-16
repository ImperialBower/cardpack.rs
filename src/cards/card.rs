use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::fluent::US_ENGLISH;

/// `Card` is the core struct in the library.
///
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    pub weight: isize,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new<S: std::clone::Clone>(rang: S, anzug: S) -> Card
    where
        S: Into<String>,
    {
        let a = Suit::new(anzug);
        let r = Rank::new(rang);
        let weight = Card::determine_weight(&a, &r);
        Card {
            weight,
            suit: a,
            rank: r,
        }
    }

    pub fn new_from_structs(rank: Rank, suit: Suit) -> Card {
        let weight = Card::determine_weight(&suit, &rank);
        Card { weight, rank, suit }
    }

    fn determine_weight(suit: &Suit, rang: &Rank) -> isize {
        (suit.weight * 1000) + rang.weight
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.get_index(&lid);
        let suit = self.suit.get_short(&lid);
        format!("{}{}", rank, suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_txt_string(&US_ENGLISH))
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
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(expected, Card::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            weight: 4014,
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
}
