use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `PipType` is used to handle control flows for special, conditional processing of pips.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum PipType {
    #[default]
    Blank,
    Suit,
    Rank,
    Joker,
    Special,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pip {
    pub pip_type: PipType,
    pub weight: u32,
    pub index: char,
    pub symbol: char,
    pub value: u32,
}

impl Pip {
    /// The universal index for a blank `Pip` in a [`Card`](crate::basic::types::card::Card).
    pub const BLANK_INDEX: char = '_';

    pub const PRIMES: [u32; 60] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    ];

    #[must_use]
    pub fn new(pip_type: PipType, weight: u32, index: char, symbol: char) -> Self {
        Self {
            pip_type,
            weight,
            index,
            symbol,
            ..Default::default()
        }
    }
}

impl Default for Pip {
    fn default() -> Self {
        Self {
            pip_type: PipType::Blank,
            weight: 0,
            index: Pip::BLANK_INDEX,
            symbol: Pip::BLANK_INDEX,
            value: 0,
        }
    }
}

impl Display for Pip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__pips_tests {
    use super::*;
    use crate::basic::types::basic_card::BasicCard;

    #[test]
    fn pip__default() {
        let pip = Pip::default();
        assert_eq!(pip.pip_type, PipType::Blank);
        assert_eq!(pip.weight, 0);
        assert_eq!(pip.index, Pip::BLANK_INDEX);
        assert_eq!(pip.symbol, Pip::BLANK_INDEX);
        assert_eq!(pip.value, 0);
    }
}
