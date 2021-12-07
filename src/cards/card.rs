use colored::*;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::*;
use crate::cards::suit::*;
use crate::Named;

pub const BLANK: &str = "blank";

/// `Card` is the core struct in the library. A Card is made up of a Rank,
/// a Suit and weight, which is an integer that controls how a card is sorted
/// in a Pile or as a part of a Vector.
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Card {
    /// Used by the Pile struct to sort Cards.
    pub weight: isize,
    /// The identity indicator in the corner of a playing card, such as `AS` for ace of spades.
    pub index: String,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Instantiates a new Card with the default weight as defined in the fluent
    /// templates.
    pub fn new(rank: &'static str, suit: &'static str) -> Card {
        let suit = Suit::new(suit);
        let rank = Rank::new(rank);
        let weight = Card::determine_weight(&suit, &rank);
        let index = Card::determine_index(&suit, &rank);
        Card {
            weight,
            index,
            suit,
            rank,
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

    pub fn from_french_deck_index(_index: &'static str) -> Card {
        Card::new("A", "♠")
    }

    /// Returns a Symbol String for the Card.
    pub fn symbol(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        format!("{}{}", rank, suit)
    }

    /// Returns a Symbol String for the Card in the traditional colors for the Suits.
    pub fn symbol_colorized(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        match self.suit.name() {
            "hearts" => format!("{}{}", rank, suit).red().to_string(),
            "diamonds" => format!("{}{}", rank, suit).red().to_string(),
            "laub" => format!("{}{}", rank, suit).green().to_string(),
            "herz" => format!("{}{}", rank, suit).red().to_string(),
            "schellen" => format!("{}{}", rank, suit).yellow().to_string(),
            _ => format!("{}{}", rank, suit),
        }
    }

    // Private methods
    fn determine_index(suit: &Suit, rank: &Rank) -> String {
        let rank = rank.index_default();
        let suit = suit.index_default();
        format!("{}{}", rank, suit)
    }

    /// Prioritizes sorting by Suit and then by Rank.
    fn determine_weight(suit: &Suit, rank: &Rank) -> isize {
        (suit.weight * 1000) + rank.weight
    }
}

impl Default for Card {
    fn default() -> Card {
        Card::new(BLANK, BLANK)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl Named for Card {
    fn name(&self) -> &str {
        self.index.as_str()
    }

    fn index(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.name.index(lid);
        let suit = self.suit.name.index(lid);
        format!("{}{}", rank, suit)
    }

    fn long(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.name.long(lid);
        let suit = self.suit.name.long(lid);
        format!("{} {}", rank, suit)
    }

    fn default_weight(&self) -> isize {
        Card::determine_weight(&self.suit, &self.rank)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::named::{GERMAN, US_ENGLISH};
    use std::cell::Cell;

    // region impl tests

    #[test]
    fn new() {
        let expected = Card {
            weight: 4014,
            index: "AS".to_string(),
            rank: Rank::new(ACE),
            suit: Suit::new(SPADES),
        };

        assert_eq!(expected, Card::new(ACE, SPADES));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            weight: 4014,
            index: "AS".to_string(),
            rank: Rank::new(ACE),
            suit: Suit::new(SPADES),
        };

        assert_eq!(
            expected,
            Card::new_from_structs(Rank::new(ACE), Suit::new(SPADES))
        );
    }

    #[test]
    fn index() {
        let card = Card::new(QUEEN, CLUBS);

        assert_eq!(card.index(&GERMAN), "DK".to_string());
    }

    #[test]
    fn symbol() {
        let card = Card::new(QUEEN, HEARTS);

        assert_eq!(card.symbol(&GERMAN), "D♥".to_string());
    }

    #[test]
    fn symbol_colorized() {
        let card = Card::new(QUEEN, HEARTS);

        assert_eq!(card.symbol_colorized(&GERMAN), "D♥".red().to_string());
    }

    #[test]
    fn default() {
        let card = Card::default();

        assert_eq!(-1001, card.weight);
        assert_eq!("__".to_string(), card.index);
        assert_eq!("__".to_string(), card.index_default());
        assert_eq!("__".to_string(), card.symbol(&US_ENGLISH));
        assert_eq!(&"__".to_string(), card.name());
        assert_eq!("_____ _____".to_string(), card.long_default());
    }

    // endregion

    // region named

    #[test]
    fn named__name() {
        let jack = Card::new(JACK, SPADES);

        assert_eq!(&"JS".to_string(), jack.name());
    }

    #[test]
    fn named__default_weight() {
        let original = Card::new(ACE, SPADES);
        let mut ace = Card::new(ACE, SPADES);
        assert_eq!(ace.weight, ace.default_weight());

        let weight = ace.weight;
        ace.weight = 1;

        assert_eq!(weight, ace.default_weight());
        assert_ne!(original, ace);
    }

    #[test]
    fn named__index() {
        let jack = Card::new(JACK, SPADES);

        assert_eq!("JS".to_string(), jack.index(&US_ENGLISH));
        assert_eq!("BS".to_string(), jack.index(&GERMAN));
        assert_eq!("JS".to_string(), jack.index_default());
    }

    #[test]
    fn named__long() {
        let ace = Card::new(ACE, SPADES);

        assert_eq!("Ace Spades".to_string(), ace.long(&US_ENGLISH));
        assert_eq!("Ass Spaten".to_string(), ace.long(&GERMAN));
        assert_eq!("Ace Spades".to_string(), ace.long_default());
    }

    // endregion

    #[test]
    fn card_cell() {
        let ace_of_spades = Card::new(ACE, SPADES);
        let blank = Card::default();
        let cell = Cell::new(ace_of_spades.clone());

        let aces = cell.take();

        assert_eq!(Card::new(ACE, SPADES), aces);
        assert_eq!(blank, cell.take());
        assert_eq!(blank, cell.take());

        cell.replace(ace_of_spades);

        let aces = cell.take();

        assert_eq!(Card::new(ACE, SPADES), aces);
        assert_eq!(blank, cell.take());
        assert_eq!(blank, cell.take());
    }
}
