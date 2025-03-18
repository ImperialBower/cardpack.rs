use crate::prelude::PipType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum MPipType {
    #[default]
    Blank,
    AddBaseChips(usize),
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
    Planet(usize),
    RandomJoker(usize),
    RandomTarot(usize),
    Stone(usize),
    Strength,
    Wheel(usize),
    Wild(PipType),
    Diamonds(usize),
    Clubs(usize),
    Hearts(usize),
    Spades(usize),
}

impl Display for MPipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MPipType::Blank => write!(f, "Blank"),
            MPipType::AddBaseChips(chips) => write!(f, "AddBaseChips({chips}) "),
            MPipType::Chips(chips) => write!(f, "Chips({chips})"),
            MPipType::Death(value) => write!(f, "Death({value})"),
            MPipType::DoubleMoney(value) => write!(f, "DoubleMoney({value})"),
            MPipType::Glass(a, b) => write!(f, "Glass({a}, {b})"),
            MPipType::Gold(value) => write!(f, "Gold({value})"),
            MPipType::Hanged(value) => write!(f, "Hanged({value})"),
            MPipType::JokersValue(value) => write!(f, "JokersValue({value})"),
            MPipType::Lucky(a, b) => write!(f, "Lucky({a}, {b})"),
            MPipType::MultPlus(value) => write!(f, "MultPlus({value})"),
            MPipType::MultTimes(value) => write!(f, "MultTimes({value})"),
            MPipType::MultTimes1Dot(value) => write!(f, "MultTimes1Dot({value})"),
            MPipType::Planet(value) => write!(f, "Planet({value})"),
            MPipType::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            MPipType::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            MPipType::Stone(value) => write!(f, "Stone({value})"),
            MPipType::Strength => write!(f, "Strength"),
            MPipType::Wheel(value) => write!(f, "Wheel({value})"),
            MPipType::Wild(pip_type) => write!(f, "Wild({pip_type:?})"),
            MPipType::Diamonds(value) => write!(f, "Diamonds({value})"),
            MPipType::Clubs(value) => write!(f, "Clubs({value})"),
            MPipType::Hearts(value) => write!(f, "Hearts({value})"),
            MPipType::Spades(value) => write!(f, "Spades({value})"),
        }
    }
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
    pub const MOD_DOUBLE_MONEY: Self = Self {
        pip_type: MPipType::DoubleMoney(20),
        index: 'd',
    };
    pub const MOD_MULT_PLUS4: Self = Self {
        pip_type: MPipType::MultPlus(4),
        index: 'e',
    };
    pub const GLASS: Self = Self {
        pip_type: MPipType::Glass(2, 4),
        index: 'g',
    };
    pub const HANGED: Self = Self {
        pip_type: MPipType::Hanged(2),
        index: 'x',
    };
    pub const PLANET: Self = Self {
        pip_type: MPipType::Planet(2),
        index: 'h',
    };
    pub const LUCKY: Self = Self {
        pip_type: MPipType::Lucky(5, 15),
        index: 'l',
    };
    pub const RANDOM_JOKER: Self = Self {
        pip_type: MPipType::RandomJoker(1),
        index: 'j',
    };
    pub const RANDOM_TAROT: Self = Self {
        pip_type: MPipType::RandomTarot(2),
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
    pub const WILD_SUIT: Self = Self {
        pip_type: MPipType::Wild(PipType::Suit),
        index: 'w',
    };
    pub const DIAMONDS: Self = Self {
        pip_type: MPipType::Diamonds(3),
        index: 'd',
    };
    pub const CLUBS: Self = Self {
        pip_type: MPipType::Clubs(3),
        index: 'c',
    };
    pub const HEARTS: Self = Self {
        pip_type: MPipType::Hearts(3),
        index: 'h',
    };
    pub const JUDGEMENT: Self = Self {
        pip_type: MPipType::RandomJoker(2),
        index: 'j',
    };
    pub const SPADES: Self = Self {
        pip_type: MPipType::Spades(3),
        index: 's',
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

impl Display for MPip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pip_type)
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
