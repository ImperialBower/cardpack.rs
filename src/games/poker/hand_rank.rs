use std::cmp::Ordering;
use std::fmt;
use strum_macros::Display;

#[allow(clippy::module_name_repetitions)]
pub type HandRankValue = u16;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandRankName {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
    Invalid,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HandRank {
    pub value: HandRankValue,
    pub name: HandRankName,
}

impl HandRank {
    #[must_use]
    pub fn new(value: HandRankValue) -> HandRank {
        HandRank {
            value,
            name: HandRank::get_hand_rank_name(&value),
        }
    }

    /// Takes in a calculated `HandRankValue` and returns the `HandRank`.
    ///
    /// 7462 possible combination of hands:
    ///
    ///   10 straight-flushes
    ///  156 four of a kinds
    ///  156 full houses
    /// 1277 flushes
    ///   10 straights
    ///  858 three of a kinds
    ///  858 two pairs
    /// 2860 pairs
    /// 1277 high cards
    #[must_use]
    pub fn get_hand_rank_name(hrv: &HandRankValue) -> HandRankName {
        match *hrv {
            1..=10 => HandRankName::StraightFlush,
            11..=166 => HandRankName::FourOfAKind,
            167..=322 => HandRankName::FullHouse,
            323..=1599 => HandRankName::Flush,
            1600..=1609 => HandRankName::Straight,
            1610..=2467 => HandRankName::ThreeOfAKind,
            2468..=3325 => HandRankName::TwoPair,
            3326..=6185 => HandRankName::Pair,
            6186..=7462 => HandRankName::HighCard,
            _ => HandRankName::Invalid,
        }
    }

    #[must_use]
    pub fn is_aligned(&self) -> bool {
        self.name == HandRank::new(self.value).name
    }

    #[must_use]
    pub fn is_invalid(&self) -> bool {
        self.name == HandRankName::Invalid
    }
}

impl Default for HandRank {
    fn default() -> HandRank {
        HandRank::new(0)
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialOrd<Self> for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The lower the `HandRankValue` the higher the value of the `HandRank`, unless it's invalid.
#[allow(clippy::if_same_then_else)]
impl Ord for HandRank {
    fn cmp(&self, other: &HandRank) -> Ordering {
        if self.is_invalid() && other.is_invalid() {
            Ordering::Equal
        } else if self.is_invalid() {
            Ordering::Less
        } else if other.is_invalid() {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Greater
        } else if self.value > other.value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod hand_rank_tests {
    use super::*;

    #[test]
    fn is_aligned() {
        assert!(HandRank::new(0).is_aligned());
        assert!(HandRank::new(1).is_aligned());
    }

    #[test]
    fn is_invalid() {
        assert!(HandRank::new(0).is_invalid());
        assert!(HandRank::new(7463).is_invalid());
        assert!(HandRank::default().is_invalid());
        assert!(!HandRank::new(6186).is_invalid());
    }

    #[test]
    fn display() {
        assert_eq!(
            "HandRank { value: 1, name: StraightFlush }",
            format!("{}", HandRank::new(1))
        );
    }

    #[test]
    fn ord() {
        assert!(HandRank::new(1) > HandRank::new(2));
        assert!(HandRank::new(2000) < HandRank::new(2));
        assert!(HandRank::new(0) < HandRank::new(2));
        assert_eq!(HandRank::new(2), HandRank::new(2));
    }
}
