use crate::cards::rank::*;
use crate::cards::suit::*;
use crate::{Card, Pack, Pile};

pub struct Standard52 {
    pub pack: Pack,
    pub deck: Pile,
}

impl Standard52 {
    pub fn from_index_string(card_str: &'static str) -> Standard52 {
        let _rawcards: Vec<&str> = card_str.split(' ').collect();

        Standard52::default()
    }

    pub fn card_from_index_string(card_str: &'static str) -> Card {
        // Card::new(&card_str[..0], &card_str[1..2])
        let rank = Rank::from_french_deck_index(Standard52::rank_str_from_index_string(card_str));
        let suit = Suit::from_french_deck_index(Standard52::suit_str_from_index_string(card_str));

        Card::new_from_structs(rank, suit)
    }

    fn rank_str_from_index_string(card_str: &'static str) -> &'static str {
        &card_str[..1]
    }

    fn suit_str_from_index_string(card_str: &'static str) -> &'static str {
        &card_str[1..2]
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
    fn rank_str_from_index_string() {
        assert_eq!("2", Standard52::rank_str_from_index_string("2S"));
    }

    #[test]
    fn suit_str_from_index_string() {
        assert_eq!("S", Standard52::suit_str_from_index_string("2S"));
    }

    // 2S 3D QS KH 3C 3S TC 9H 3H 6H QD 4H 2H 5S 6D 9S AD 5C 7S JS AC 6S 8H 7C JC 7H JD TS AS KS JH 5D 6C 9C QC 8D 4C 5H 4D 8S 2C AH 2D 9D TH KD 7D KC 4S 8C QH TD

    #[rstest]
    #[case("2S", Card::new(TWO, SPADES))]
    #[case("3S", Card::new(THREE, SPADES))]
    fn card_from_index_string(#[case] input: &'static str, #[case] expected: Card) {
        assert_eq!(expected, Standard52::card_from_index_string(input));
    }
}
