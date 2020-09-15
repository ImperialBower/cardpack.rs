use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::fluent::{ToLocaleString, US_ENGLISH};

/// `Card` is the core struct in the library.
///
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    pub value: isize,
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
        let value = Card::determine_value(&a, &r);
        Card {
            value,
            suit: a,
            rank: r,
        }
    }

    pub fn new_from_structs(rank: Rank, suit: Suit) -> Card {
        let value = Card::determine_value(&suit, &rank);
        Card { value, rank, suit }
    }

    fn determine_value(suit: &Suit, rang: &Rank) -> isize {
        (suit.value * 1000) + rang.value
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.to_locale_string(&lid);
        let suit = self.suit.letter.to_locale_string(&lid);
        format!("{}{}", rank, suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

impl ToLocaleString for Card {
    fn get_fluent_key(&self) -> String {
        unimplemented!()
    }

    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.to_locale_string(&lid);
        let suit = self.suit.to_locale_string(&lid);
        format!("{}{}", rank, suit)
    }

    // TODO make this work
    fn get_raw_name(&self) -> &str {
        "TODO"
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn new() {
        let expected = Card {
            value: 4014,
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(expected, Card::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            value: 4014,
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(
            expected,
            Card::new_from_structs(Rank::new("ace"), Suit::new("spades"))
        );
    }

    #[test]
    fn to_string_by_locale() {
        let card = Card::new("queen", "clubs");

        assert_eq!(card.to_locale_string(&GERMAN), "D♣".to_string());
    }

    #[test]
    fn to_txt_string() {
        let karte = Card::new("queen", "clubs");

        assert_eq!(karte.to_txt_string(&GERMAN), "DK".to_string());
    }
}
