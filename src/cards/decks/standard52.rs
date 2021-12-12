use std::fmt;

use crate::cards::rank::*;
use crate::cards::suit::*;
use crate::{Card, Pack, Pile};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Standard52 {
    pub pack: Pack,
    pub deck: Pile,
}

impl Standard52 {
    pub fn new(pile: Pile) -> Standard52 {
        Standard52 {
            pack: Pack::french_deck(),
            deck: pile,
        }
    }

    pub fn new_shuffled() -> Standard52 {
        Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck().shuffle(),
        }
    }

    pub fn from_index(card_str: &'static str) -> Standard52 {
        let mut pile = Pile::default();
        for index in card_str.split_whitespace() {
            pile.add(Standard52::card_from_index(index));
        }

        Standard52 {
            pack: Pack::french_deck(),
            deck: pile,
        }
    }

    pub fn to_index(&self) -> String {
        self.deck.to_index()
    }

    pub fn to_index_str(&self) -> &'static str {
        self.deck.to_index_str()
    }

    pub fn to_symbol_index(&self) -> String {
        self.deck.to_symbol_index()
    }

    pub fn is_complete(&self) -> bool {
        let pile = self.deck.sort();
        &pile == self.pack.cards()
    }

    pub fn card_from_index(card_str: &'static str) -> Card {
        let rank = Rank::from_french_deck_index(Standard52::rank_str_from_index(card_str));
        let suit = Suit::from_french_deck_index(Standard52::suit_char_from_index(card_str));

        if rank.is_blank() || suit.is_blank() {
            Card::blank_card()
        } else {
            Card::new_from_structs(rank, suit)
        }
    }

    fn rank_str_from_index(card_str: &'static str) -> &'static str {
        if card_str.len() < 2 {
            return BLANK_RANK;
        }
        &card_str[..1]
    }

    fn suit_char_from_index(card_str: &'static str) -> char {
        if card_str.len() < 2 {
            return '_';
        }
        card_str.char_indices().nth(1).unwrap().1
    }
}

impl Default for Standard52 {
    fn default() -> Standard52 {
        Standard52 {
            pack: Pack::french_deck(),
            deck: Pile::french_deck(),
        }
    }
}

impl fmt::Display for Standard52 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self.to_index();
        write!(f, "{}", sig)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod standard52_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn from_index() {
        let index_string = "2S 3D QS KH 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD";
        let mut standard52 = Standard52::from_index(index_string);

        assert_eq!(
            Card::new(TWO, SPADES),
            standard52.deck.draw_first().unwrap()
        );
        assert_eq!(
            Card::new(THREE, DIAMONDS),
            standard52.deck.draw_first().unwrap()
        );
    }

    #[test]
    fn from_index_shuffled() {
        let starter = Standard52::new_shuffled();
        let standard52 = Standard52::from_index(starter.to_index_str());

        assert!(standard52.is_complete());
        assert_eq!(starter, standard52);
    }

    #[test]
    fn is_complete() {
        assert!(Standard52::default().is_complete());
        assert!(Standard52::new_shuffled().is_complete());
        assert!(Standard52::new(Pile::french_deck().draw(52).unwrap()).is_complete());
    }

    #[test]
    fn is_complete__false() {
        assert!(!Standard52::new(Pile::french_deck().draw(4).unwrap()).is_complete());
        assert!(!Standard52::new(Pile::french_deck_with_jokers()).is_complete());
    }

    #[test]
    fn rank_str_from_index() {
        assert_eq!("2", Standard52::rank_str_from_index("2S"));
    }

    #[test]
    fn suit_char_from_index() {
        assert_eq!('S', Standard52::suit_char_from_index("2S"));
        assert_eq!('♠', Standard52::suit_char_from_index("2♠"));
    }

    #[rstest]
    #[case("2S", Card::new(TWO, SPADES))]
    #[case("2♠", Card::new(TWO, SPADES))]
    #[case("3S", Card::new(THREE, SPADES))]
    #[case("3♠", Card::new(THREE, SPADES))]
    #[case("4♠", Card::new(FOUR, SPADES))]
    #[case("4S", Card::new(FOUR, SPADES))]
    #[case("5♠", Card::new(FIVE, SPADES))]
    #[case("5S", Card::new(FIVE, SPADES))]
    fn card_from_index(#[case] input: &'static str, #[case] expected: Card) {
        assert_eq!(expected, Standard52::card_from_index(input));
    }

    #[rstest]
    #[case("XX")]
    #[case("2X")]
    #[case("2s")]
    #[case("XS")]
    #[case("  ")]
    #[case(" ")]
    #[case("")]
    fn card_from_index__invalid_index(#[case] input: &'static str) {
        assert_eq!(Card::blank_card(), Standard52::card_from_index(input));
    }
}
