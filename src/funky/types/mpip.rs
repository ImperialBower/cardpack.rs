use serde::{Deserialize, Serialize};
use crate::prelude::PipType;

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum MPipType {
    #[default]
    Blank,
    Chips(usize),
    Death(usize),
    DoubleMoney(usize),
    Glass(usize, usize),
    Gold(usize),
    Hanged(usize),
    JokersValue(usize),
    Lucky(usize, usize),
    MultPlus(usize),
    MultTimes(usize),
    MultTimes1Dot(usize),
    Planet,
    Stone(usize),
    Strength,
    Tarot,
    Wheel(usize),
    Wild(PipType),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MPip {
    pub pip_type: MPipType,
    pub index: char,
}

impl MPip {
    pub const BLANK: Self = Self {
        pip_type: MPipType::Blank,
        index: '_',
    };
    pub const BONUS: Self = Self {
        pip_type: MPipType::Chips(30),
        index: 'b',
    };
    pub const DEATH: Self = Self {
        pip_type: MPipType::Death(1),
        index: 'd',
    };
    pub const DEVIL: Self = Self {
        pip_type: MPipType::Gold(3),
        index: 'v',
    };
    pub const DOUBLE_MONEY: Self = Self {
        pip_type: MPipType::DoubleMoney(20),
        index: 'd',
    };
    pub const EMPRESS: Self = Self {
        pip_type: MPipType::MultPlus(4),
        index: 'e',
    };
    pub const GLASS: Self = Self {
        pip_type: MPipType::Glass(2, 4),
        index: 'j',
    };
    pub const HANGED: Self = Self {
        pip_type: MPipType::Hanged(2),
        index: 'h',
    };
    pub const HIGH_PRIESTESS: Self = Self {
        pip_type: MPipType::Planet,
        index: 'h',
    };
    pub const LUCKY: Self = Self {
        pip_type: MPipType::Lucky(5, 15),
        index: 'l',
    };
    pub const RANDOM_TAROT: Self = Self {
        pip_type: MPipType::Tarot,
        index: 'o',
    };
    pub const STEEL: Self = Self {
        pip_type: MPipType::MultTimes1Dot(15), // 1.5
        index: 'c',
    };
    pub const STRENGTH: Self = Self {
        pip_type: MPipType::Strength,
        index: 's',
    };
    pub const TEMPERANCE: Self = Self {
        pip_type: MPipType::JokersValue(50),
        index: 't',
    };
    pub const TOWER: Self = Self {
        pip_type: MPipType::Stone(50),
        index: 'n',
    };
    pub const WORLD: Self = Self {
        pip_type: MPipType::MultTimes(2),
        index: 'w',
    };
    pub const WHEEL_OF_FORTUNE: Self = Self {
        pip_type: MPipType::Wheel(4),
        index: 'f',
    };
    pub const WILD: Self = Self {
        pip_type: MPipType::Wild(PipType::Suit),
        index: 'w',
    };

    #[must_use]
    pub fn new_chips(chips: usize) -> Self {
        Self {
            pip_type: MPipType::Chips(chips),
            index: 'c',
        }
    }
}

impl Default for MPip {
    fn default() -> Self {
        Self::BLANK
    }
}

pub mod enhancement {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__mpips_tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(MPipType::default(), MPipType::Blank);
    }
}
