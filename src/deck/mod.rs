pub(crate) mod rank;
mod rank_name;
mod rank_short;
pub(crate) mod suit;
mod suit_letter;
mod suit_name;
mod suit_symbol;

use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::deck::rank::Rank;
use crate::deck::suit::Suit;
use crate::fluent::{ToLocaleString, US_ENGLISH};

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
        let wert = Card::determine_value(&a, &r);
        Card {
            value: wert,
            suit: a,
            rank: r,
        }
    }

    pub fn new_from_structs(rank: Rank, suit: Suit) -> Card {
        let value = Card::determine_value(&suit, &rank);
        Card { value, rank, suit }
    }

    fn determine_value(anzug: &Suit, rang: &Rank) -> isize {
        (anzug.value * 100) + rang.value
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rank.to_locale_string(&lid);
        let anzug = self.suit.letter.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

impl ToLocaleString for Card {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rank.to_locale_string(&lid);
        let anzug = self.suit.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
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
            value: 414,
            rank: Rank::new("ace"),
            suit: Suit::new("spades"),
        };

        assert_eq!(expected, Card::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            value: 414,
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
        let karte = Card::new("queen", "clubs");

        assert_eq!(karte.to_locale_string(&GERMAN), "D♣".to_string());
    }

    #[test]
    fn to_txt_string() {
        let karte = Card::new("queen", "clubs");

        assert_eq!(karte.to_txt_string(&GERMAN), "DK".to_string());
    }
}
