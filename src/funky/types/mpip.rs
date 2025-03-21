use crate::prelude::PipType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum MPip {
    #[default]
    Blank,
    AddBaseChips(usize),
    Chips(usize),
    ChipsAndMultPlus(usize, usize),
    Death(usize),
    DoubleMoney(usize),
    Glass(usize, usize),
    Gold(usize),
    Hanged(usize),
    JokersValue(usize),
    Lucky(usize, usize),
    MultPlus(usize),
    MultPlusOnPair(usize),
    MultPlusOnTrips(usize),
    MultPlusOnSuit(usize, char),
    MultTimes(usize),
    MultTimes1Dot(usize),
    Planet(usize),
    RandomJoker(usize),
    RandomTarot(usize),
    Stone(usize),
    Strength,
    Odds1in(usize),
    Wild(PipType),
    Diamonds(usize),
    Clubs(usize),
    Hearts(usize),
    Spades(usize),
}

impl MPip {
    pub const BONUS: Self = MPip::Chips(30);
    pub const DEVIL: Self = MPip::Gold(3);
    pub const MULT_PLUS3_ON_DIAMONDS: Self = MPip::MultPlusOnSuit(3, 'D');
    pub const MULT_PLUS3_ON_HEARTS: Self = MPip::MultPlusOnSuit(3, 'H');
    pub const MULT_PLUS3_ON_SPADES: Self = MPip::MultPlusOnSuit(3, 'S');
    pub const MULT_PLUS3_ON_CLUBS: Self = MPip::MultPlusOnSuit(3, 'C');
    pub const MULT_PLUS8_ON_PAIR: Self = MPip::MultPlusOnPair(8);
    pub const MULT_PLUS12_ON_TRIPS: Self = MPip::MultPlusOnTrips(12);
    pub const STEEL: Self = MPip::MultTimes1Dot(15); // 1.5
    pub const TEMPERANCE: Self = MPip::JokersValue(50);
    pub const TOWER: Self = MPip::Stone(50);
    pub const WORLD: Self = MPip::MultTimes(2);
    pub const WHEEL_OF_FORTUNE: Self = MPip::Odds1in(4);
    pub const JUDGEMENT: Self = MPip::RandomJoker(1);

    #[must_use]
    pub fn new_chips(chips: usize) -> Self {
        MPip::Chips(chips)
    }

    #[must_use]
    pub fn is_wild(&self) -> bool {
        matches!(self, MPip::Wild(_))
    }
}

impl Display for MPip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MPip::Blank => write!(f, "Blank"),
            MPip::AddBaseChips(chips) => write!(f, "AddBaseChips({chips}) "),
            MPip::Chips(chips) => write!(f, "Chips({chips})"),
            MPip::ChipsAndMultPlus(chips, value) => {
                write!(f, "ChipsAndMultPlus({chips}, {value})")
            }
            MPip::Death(value) => write!(f, "Death({value})"),
            MPip::DoubleMoney(value) => write!(f, "DoubleMoney({value})"),
            MPip::Glass(a, b) => write!(f, "Glass({a}, {b})"),
            MPip::Gold(value) => write!(f, "Gold({value})"),
            MPip::Hanged(value) => write!(f, "Hanged({value})"),
            MPip::JokersValue(value) => write!(f, "JokersValue({value})"),
            MPip::Lucky(a, b) => write!(f, "Lucky({a}, {b})"),
            MPip::MultPlus(value) => write!(f, "MultPlus({value})"),
            MPip::MultPlusOnPair(value) => write!(f, "MultPlusOnPair({value})"),
            MPip::MultPlusOnTrips(value) => write!(f, "MultPlusOnTrips({value})"),
            MPip::MultPlusOnSuit(value, c) => write!(f, "MultPlusOnSuit({value}, {c})"),
            MPip::MultTimes(value) => write!(f, "MultTimes({value})"),
            MPip::MultTimes1Dot(value) => write!(f, "MultTimes1Dot({value})"),
            MPip::Planet(value) => write!(f, "Planet({value})"),
            MPip::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            MPip::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            MPip::Stone(value) => write!(f, "Stone({value})"),
            MPip::Strength => write!(f, "Strength"),
            MPip::Odds1in(value) => write!(f, "Wheel({value})"),
            MPip::Wild(pip_type) => write!(f, "Wild({pip_type:?})"),
            MPip::Diamonds(value) => write!(f, "Diamonds({value})"),
            MPip::Clubs(value) => write!(f, "Clubs({value})"),
            MPip::Hearts(value) => write!(f, "Hearts({value})"),
            MPip::Spades(value) => write!(f, "Spades({value})"),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod funky__types__mpips_tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(MPip::default(), MPip::Blank);
    }
}
