use crate::cards::card_error::CardError;
use crate::{Card, Pile, Suit};
use std::collections::HashMap;
// use std::collections::HashMap;

pub struct CardsBySuit {
    pub suit: Suit,
    pub cards: Vec<Card>,
}

/// Possible five card combinations from a `Pile` of seven cards. The u8 value is the binary
/// representation.
#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Spread57 {
    XXXXXOO = 124,
    XXXXOXO = 122,
    XXXXOOX = 121,
    XXXOXOX = 117,
    XXXOOXX = 115,
    XXOXXXO = 110,
    XXOXXOX = 109,
    XXOXOXX = 107,
    XXOOXXX = 103,
    XOXXXOX = 93,
    XOXXOXX = 91,
    XOXOXXX = 87,
    XOOXXXX = 79,
    OXXXXXO = 62,
    OXXXXOX = 61,
    OXXXOXX = 59,
    OXXOXXX = 55,
    OXOXXXX = 47,
    OOXXXXX = 31,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum CardSieve {
    XOOOOOOOO = 256,
    OXOOOOOOO = 128,
    OOXOOOOOO = 64,
    OOOXOOOOO = 32,
    OOOOXOOOO = 16,
    OOOOOXOOO = 8,
    OOOOOOXOO = 4,
    OOOOOOOXO = 2,
    OOOOOOOOX = 1,
}

/// <https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry//>
/// <https://www.reddit.com/r/rust/comments/9xho3i/i_have_a_hashmap_that_pairs_strings_with_vectors//>
#[must_use]
pub fn sort_by_suit(_pile: &Pile) -> HashMap<Suit, Pile> {
    let sorted: HashMap<Suit, Pile> = HashMap::new();

    sorted
}

pub fn pile_by_spread_key(spread: u8, pile: Pile) -> Result<Pile, CardError> {
    Ok(Pile::default())
}

#[cfg(test)]
#[allow(non_snake_case)]
mod eval_tests {
    use super::*;
    use crate::Standard52;
    use rstest::rstest;

    #[test]
    fn spread57() {
        assert_eq!(Spread57::XOOXXXX as u8, 79);
    }

    #[rstest]
    #[case("2S 3S 9S TS QS JH Ac")]
    fn to_a_flush(#[case] input: &'static str) {
        let pile = Standard52::pile_from_index(input).unwrap();

        let _sorted = sort_by_suit(&pile);

        let mut and: u64 = 0xF000;
        for card in pile.cards() {
            let bin = card.binary_signature();
            println!("    {:032b}", and);
            println!("  + {:032b}", bin);
            println!("    ================================");
            and = and & bin;
            println!("    {:032b}", and);
            println!();
        }
        assert_eq!(and, 0);
    }
}
