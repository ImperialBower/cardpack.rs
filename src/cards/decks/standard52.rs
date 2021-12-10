use crate::cards::rank::*;
use crate::cards::suit::*;
use crate::{Card, Pack, Pile};

pub struct Standard52 {
    pub pack: Pack,
    pub deck: Pile,
}

impl Standard52 {
    pub fn from_index(card_str: &'static str) -> Standard52 {
        let raw_cards: Vec<&str> = card_str.split(' ').collect();

        let mut pile = Pile::default();
        for index in raw_cards {
            pile.add(Standard52::card_from_index(index));
        }

        Standard52 {
            pack: Pack::french_deck(),
            deck: pile,
        }
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
    fn play() {}

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
