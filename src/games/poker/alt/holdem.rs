//! holdem-rs contains more or less common types and functions used by other poker related libraries.

use std::cmp::Ord;

/// All hand rank classes that a 5-card hand can be worth in Texas Hold'em.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HandRankClass {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

/// A card encoded using the bit pattern described in Cactus Kev's
/// [article](http://www.suffecool.net/poker/evaluator.html).
pub type CactusKevCard = u32;

/// A value representing the strength of a hand. The higher, the better.
/// The numbers go from 0 to 7461 inclusive.
pub type HandRank = u16; //TODO: struct HandRank(u16); //TODO: pub?
pub const HAND_RANK_COUNT: u16 = 7462;

/// Translates a hand rank to a rank class.
/// assumes there are `HAND_RANK_COUNT` distinct hand ranks, where the
/// largest are the most valuable. Numbers based on: <http://www.suffecool.net/poker/evaluator.html/>
pub fn hand_rank_to_class(val: &HandRank) -> HandRankClass {
    match *val {
        0..=1276 => HandRankClass::HighCard,
        1277..=4136 => HandRankClass::OnePair,
        4137..=4994 => HandRankClass::TwoPair,
        4995..=5852 => HandRankClass::ThreeOfAKind,
        5853..=5862 => HandRankClass::Straight,
        5863..=7139 => HandRankClass::Flush,
        7140..=7295 => HandRankClass::FullHouse,
        7296..=7451 => HandRankClass::FourOfAKind,
        7452..=7461 => HandRankClass::StraightFlush,
        _ => panic!("Unexpected hand rank value! '{}'", *val),
    }
}
