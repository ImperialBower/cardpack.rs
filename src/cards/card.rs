use colored::Colorize;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::Named;

pub const BLANK: &str = "blank";

/// `Card` is the core struct in the library. A Card is made up of a Rank,
/// a `Suit`, `weight`, which is an integer that controls how a card is sorted
/// in a `Pile` or as a part of a `Vector`, and index, which is a short `String`
/// representation of the card, suitable for serialization in text format.
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Card {
    /// Used by the Pile struct to sort Cards.
    pub weight: u32,
    /// The identity indicator in the corner of a playing card, such as `AS` for ace of spades.
    pub index: String,
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Instantiates a Card with the weight determined by the passed in Rank and Suit.
    #[must_use]
    pub fn new(rank: Rank, suit: Suit) -> Self {
        let weight = Card::determine_weight(&suit, &rank);
        let index = Card::determine_index(&suit, &rank);
        Self {
            weight,
            index,
            suit,
            rank,
        }
    }

    /// Instantiates a new Card with the default weight as defined in the fluent templates.
    ///
    /// Me not knowing about the  FromStr trait at the time. See also
    /// [Creating a Rust function that accepts String or &str](https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html/)
    #[must_use]
    pub fn from_index_strings(rank: &'static str, suit: &'static str) -> Self {
        Self::new(Rank::new(rank), Suit::new(suit))
    }

    /// Returns a Card where the sorting emphasizes its `Rank` weight over its `Suit` weight.
    /// So `K♥ A♥ A♠ K♠` would return `A♠ A♥ K♠ K♥` instead of `A♠ K♠ A♥ K♥`.
    #[must_use]
    pub fn to_rank_weight(&self) -> Self {
        Self {
            weight: Card::determine_rank_weight(&self.suit, &self.rank),
            index: self.index.clone(),
            suit: self.suit,
            rank: self.rank,
        }
    }

    /// Returns a version of the `Card`, with a geometric `Rank` weighted operation
    /// has been performed on it.
    #[must_use]
    pub fn shift_rank_weight_left(&self, i: usize) -> Self {
        let rank_weight = Card::determine_rank_weight(&self.suit, &self.rank);
        Self {
            weight: rank_weight << (i * i),
            index: self.index.clone(),
            suit: self.suit,
            rank: self.rank,
        }
    }

    /// Returns a Symbol String for the Card.
    #[must_use]
    pub fn symbol(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        format!("{rank}{suit}")
    }

    /// Returns a Symbol String for the Card in the traditional colors for the Suits.
    #[must_use]
    pub fn symbol_colorized(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        match self.suit.name() {
            "hearts" | "herz" | "diamonds" => format!("{rank}{suit}").red().to_string(),
            "laub" => format!("{rank}{suit}").green().to_string(),
            "schellen" => format!("{rank}{suit}").yellow().to_string(),
            _ => format!("{rank}{suit}"),
        }
    }

    #[must_use]
    #[deprecated(since = "0.4.15", note = "Use Card::default()")]
    pub fn blank_card() -> Self {
        Self::from_index_strings(BLANK, BLANK)
    }

    /// A unique index of a `Card` relative to other cards in a `Pile` prioritized by `Rank` and
    /// then by `Suit`, such that a 2 of spades is lower than a 3 of clubs. While Card.weight
    /// prioritizes by `Suit` and then by `Rank`.
    #[must_use]
    pub fn count(&self) -> u32 {
        (self.suit.weight - 1) + (self.rank.weight * 4) + 1
    }

    /// A valid Card is one where the Rank and Suit are not blank.
    /// Cards are blank when an invalid Rank or Suit are passed in.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.rank.is_blank() && !self.suit.is_blank()
    }

    // Private methods
    fn determine_index(suit: &Suit, rank: &Rank) -> String {
        let rank = rank.index_default();
        let suit = suit.index_default();
        format!("{rank}{suit}")
    }

    /// Prioritizes sorting by Suit and then by Rank.
    fn determine_rank_weight(suit: &Suit, rank: &Rank) -> u32 {
        ((rank.weight + 2) * 1000) + suit.weight
    }

    /// Prioritizes sorting by Suit and then by Rank.
    fn determine_weight(suit: &Suit, rank: &Rank) -> u32 {
        (suit.weight * 1000) + rank.weight
    }
}

/// Defaults to a blank `Card`.
impl Default for Card {
    fn default() -> Card {
        Card::from_index_strings(BLANK, BLANK)
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
        format!("{rank}{suit}")
    }

    fn long(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.name.long(lid);
        let suit = self.suit.name.long(lid);
        format!("{rank} {suit}")
    }

    fn default_weight(&self) -> u32 {
        Card::determine_weight(&self.suit, &self.rank)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::named::{GERMAN, US_ENGLISH};
    use crate::{
        ACE, BLANK_RANK, BLANK_SUIT, CLUBS, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES, TWO,
    };
    use std::cell::Cell;

    // region impl tests

    #[test]
    fn new() {
        let expected = Card {
            weight: 4012,
            index: "AS".to_string(),
            rank: Rank::new(ACE),
            suit: Suit::new(SPADES),
        };

        assert_eq!(expected, Card::from_index_strings(ACE, SPADES));
    }

    #[test]
    fn new_from_structs() {
        let expected = Card {
            weight: 4012,
            index: "AS".to_string(),
            rank: Rank::new(ACE),
            suit: Suit::new(SPADES),
        };

        assert_eq!(expected, Card::new(Rank::new(ACE), Suit::new(SPADES)));
    }

    #[test]
    fn shift_weight_left() {
        let ace = Card::new(Rank::new(ACE), Suit::new(SPADES));
        let ace_hearts = Card::new(Rank::new(ACE), Suit::new(HEARTS));
        let king = Card::new(Rank::new(KING), Suit::new(SPADES));
        let deuce = Card::new(Rank::new(TWO), Suit::new(SPADES));

        let ace_shift_one = ace.shift_rank_weight_left(1);
        let ace_hearts_shift_one = ace_hearts.shift_rank_weight_left(1);
        let ace_hearts_shift_four = ace_hearts.shift_rank_weight_left(4);
        let king_shift_one = king.shift_rank_weight_left(1);
        let deuce_shift_two = deuce.shift_rank_weight_left(2);

        assert_eq!(ace_shift_one.weight, 28008);
        assert_eq!(ace_hearts_shift_one.weight, 28006);
        assert_eq!(ace_hearts_shift_four.weight, 917700608);
        assert_eq!(king_shift_one.weight, 26008);
        assert_eq!(deuce_shift_two.weight, 32064);
        assert!(ace_shift_one.weight > ace_hearts_shift_one.weight);
        assert!(ace_hearts_shift_one.weight > king_shift_one.weight);
        assert!(king_shift_one.weight < deuce_shift_two.weight);
        assert!(king_shift_one.weight > ace.weight);
    }

    #[test]
    fn to_rank_weight() {
        let card = Card::new(Rank::new(ACE), Suit::new(SPADES));
        let rank_weighted_card = card.to_rank_weight();

        assert_eq!(rank_weighted_card.weight, 14004);
    }

    #[test]
    fn to_rank_weight_deuce() {
        let card = Card::new(Rank::new(TWO), Suit::new(SPADES));
        let rank_weighted_card = card.to_rank_weight();

        assert_eq!(rank_weighted_card.weight, 2004);
    }

    #[test]
    fn count() {
        assert_eq!(52, Card::from_index_strings(ACE, SPADES).count());
        assert_eq!(1, Card::from_index_strings(TWO, CLUBS).count());
        assert_eq!(2, Card::from_index_strings(TWO, DIAMONDS).count());
    }

    #[test]
    fn index() {
        let card = Card::from_index_strings(QUEEN, CLUBS);

        assert_eq!(card.index(&GERMAN), "DK".to_string());
    }

    #[test]
    fn symbol() {
        let card = Card::from_index_strings(QUEEN, HEARTS);

        assert_eq!(card.symbol(&GERMAN), "D♥".to_string());
    }

    #[test]
    fn symbol_colorized() {
        let card = Card::from_index_strings(QUEEN, HEARTS);

        assert_eq!(card.symbol_colorized(&GERMAN), "D♥".red().to_string());
    }

    #[test]
    fn is_valid() {
        assert!(Card::from_index_strings(QUEEN, CLUBS).is_valid())
    }

    #[test]
    fn is_valid__false() {
        assert!(!Card::from_index_strings("", "").is_valid());
        assert!(!Card::from_index_strings(QUEEN, BLANK_SUIT).is_valid());
        assert!(!Card::from_index_strings(BLANK_RANK, CLUBS).is_valid());
        assert!(!Card::from_index_strings(BLANK_RANK, BLANK_SUIT).is_valid());
        assert!(!Card::from_index_strings(" ", BLANK_SUIT).is_valid());
    }

    #[test]
    fn default() {
        let card = Card::default();

        assert_eq!(0, card.weight);
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
        let jack = Card::from_index_strings(JACK, SPADES);

        assert_eq!(&"JS".to_string(), jack.name());
    }

    #[test]
    fn named__default_weight() {
        let original = Card::from_index_strings(ACE, SPADES);
        let mut ace = Card::from_index_strings(ACE, SPADES);
        assert_eq!(ace.weight, ace.default_weight());

        let weight = ace.weight;
        ace.weight = 1;

        assert_eq!(weight, ace.default_weight());
        assert_ne!(original, ace);
    }

    #[test]
    fn named__index() {
        let jack = Card::from_index_strings(JACK, SPADES);

        assert_eq!("JS".to_string(), jack.index(&US_ENGLISH));
        assert_eq!("BS".to_string(), jack.index(&GERMAN));
        assert_eq!("JS".to_string(), jack.index_default());
    }

    #[test]
    fn named__long() {
        let ace = Card::from_index_strings(ACE, SPADES);

        assert_eq!("Ace Spades".to_string(), ace.long(&US_ENGLISH));
        assert_eq!("Ass Spaten".to_string(), ace.long(&GERMAN));
        assert_eq!("Ace Spades".to_string(), ace.long_default());
    }

    // endregion

    #[test]
    fn card_cell() {
        let ace_of_spades = Card::from_index_strings(ACE, SPADES);
        let blank = Card::default();
        let cell = Cell::new(ace_of_spades.clone());

        let aces = cell.take();

        assert_eq!(Card::from_index_strings(ACE, SPADES), aces);
        assert_eq!(blank, cell.take());
        assert_eq!(blank, cell.take());

        cell.replace(ace_of_spades);

        let aces = cell.take();

        assert_eq!(Card::from_index_strings(ACE, SPADES), aces);
        assert_eq!(blank, cell.take());
        assert_eq!(blank, cell.take());
    }
}
