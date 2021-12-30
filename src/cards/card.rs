use colored::Colorize;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::Named;

pub const BLANK: &str = "blank";

/// A card encoded using the bit pattern described in Cactus Kev's
/// [article](http://www.suffecool.net/poker/evaluator.html).
#[allow(clippy::module_name_repetitions)]
// pub type CactusKevCard = u32;

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
    pub fn new(rank: Rank, suit: Suit) -> Card {
        let weight = Card::determine_weight(&suit, &rank);
        let index = Card::determine_index(&suit, &rank);
        Card {
            weight,
            index,
            suit,
            rank,
        }
    }

    /// Instantiates a new Card with the default weight as defined in the fluent templates.
    #[must_use]
    pub fn from_index_strings(rank: &'static str, suit: &'static str) -> Card {
        Card::new(Rank::new(rank), Suit::new(suit))
    }

    /// Returns a Symbol String for the Card.
    #[must_use]
    pub fn symbol(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        format!("{}{}", rank, suit)
    }

    /// Returns a Symbol String for the Card in the traditional colors for the Suits.
    #[must_use]
    pub fn symbol_colorized(&self, lid: &LanguageIdentifier) -> String {
        let rank = self.rank.index(lid);
        let suit = self.suit.symbol();
        match self.suit.name() {
            "hearts" | "herz" | "diamonds" => format!("{}{}", rank, suit).red().to_string(),
            "laub" => format!("{}{}", rank, suit).green().to_string(),
            "schellen" => format!("{}{}", rank, suit).yellow().to_string(),
            _ => format!("{}{}", rank, suit),
        }
    }

    #[must_use]
    pub fn blank_card() -> Card {
        Card::from_index_strings(BLANK, BLANK)
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

    /// Creates [Cactus Kev's Hand Evaluator](http://suffe.cool/poker/evaluator.html) value.
    /// ```txt
    /// +--------+--------+--------+--------+
    /// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    /// +--------+--------+--------+--------+
    ///
    /// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
    /// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    /// cdhs = suit of card (bit turned on based on suit of card)
    /// b = bit turned on depending on rank of card
    /// ```
    /// This is used for Poker hand evaluation.
    // #[must_use]
    // pub fn to_cactus_kev_card(&self) -> CactusKevCard {
    //     let suit: u32 = self.suit.binary_signature();
    //     let bits = 1 << (16 + self.rank.weight);
    //     let rank_eight = self.rank.weight << 8;
    //
    //     // println!("{} | {} | {} | {}", bits, self.rank.prime, rank_eight, suit);
    //
    //     bits | self.rank.prime | rank_eight | suit
    // }

    // pub fn debug(&self) {
    //     println!("{}:", self.index);
    //     println!(
    //         "     {:032b} {}",
    //         self.to_cactus_kev_card(),
    //         self.to_cactus_kev_card()
    //     );
    //     println!(
    //         "     {:032b} {} < Suit binary_signature",
    //         self.suit.binary_signature(),
    //         self.suit.binary_signature()
    //     );
    // }

    // Private methods
    fn determine_index(suit: &Suit, rank: &Rank) -> String {
        let rank = rank.index_default();
        let suit = suit.index_default();
        format!("{}{}", rank, suit)
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

// impl fmt::Binary for Card {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fmt::Binary::fmt(&self.to_cactus_kev_card(), f)
//     }
// }

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

        // println!("{:?}", card);

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

    #[test]
    fn to_cactus_kev_card() {
        let card = Card::from_index_strings(KING, DIAMONDS);

        assert_eq!(card.to_cactus_kev_card(), 134236965);
    }

    // #[test]
    // fn fmt_binary() {
    //     assert_eq!(
    //         format!(
    //             "King of Diamonds as binary is: {:032b}",
    //             Card::from_index_strings(KING, DIAMONDS)
    //         ),
    //         "King of Diamonds as binary is: 00001000000000000100101100100101"
    //     );
    // }
}
