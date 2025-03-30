use crate::prelude::PipType;
use crate::preludes::funky::BCardType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum CardLocation {
    #[default]
    Playing,
    InHand,
    InPile,
    Discarded,
    Deleted,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub enum MPip {
    #[default]
    Blank,
    AddBaseChips(usize),
    AddCardTypeWhenBlindSelected(BCardType),
    ChanceDestroyed(usize, usize),
    Chips(usize),
    ChipsAndMultPlus(usize, usize),
    ChipsOnFlush(usize),
    ChipsOnPair(usize),
    ChipsOn2Pair(usize),
    ChipsOnStraight(usize),
    ChipsOnTrips(usize),
    ChipsPerRemainingDiscard(usize),
    CreateCardOnRankPlay(usize, char, BCardType),
    Credit(usize),
    Death(usize),
    DoubleMoney(usize),
    FourFlushAndStraight,
    FreeReroll(usize),
    Glass(usize, usize),
    Gold(usize),
    Hanged(usize),
    JokersValue(usize),
    Lucky(usize, usize),
    MultPlus(usize),
    MultPlusChipsOnRank(usize, usize, char),
    MultPlusDoubleValueDestroyJokerOnRight(usize),
    MultPlusOn5Ranks(usize, [char; 5]),
    MultPlusOnConsecutiveHandsNo3Ranks(usize, usize, [char; 3]),
    MultPlusOnFlush(usize),
    MultPlusOnHandPlays,
    MultPlusOnPair(usize),
    MultPlusOn2Pair(usize),
    MultPlusOnStraight(usize),
    MultPlusOnTrips(usize),
    MultPlusOnSuit(usize, char),
    MultPlusOnUpToXCards(usize, usize),
    MultPlusOnZeroDiscards(usize),
    MultPlusXOnLowestRankInHand(usize),
    MultPlusRandomTo(usize),
    MultPlusZeroDiscards(usize),
    MultTimes(usize),
    MultTimesEveryXHands(usize, usize),
    MultTimesOnEmptyJokerSlots(usize),
    MultTimes1Dot(usize),
    Planet(usize),
    RandomJoker(usize),
    RandomTarot(usize),
    RetriggerCardsInHand(usize),
    RetriggerPlayedCardsInFinalRound,
    SellValueIncrement(usize),
    Stone(usize),
    Strength,
    Odds1in(usize),
    Odds1inCashOn3Ranks(usize, usize, [char; 3]),
    Odds1inUpgradeHand(usize),
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
            MPip::AddCardTypeWhenBlindSelected(card_type) => {
                write!(f, "AddStoneCardWhenBlindSelected({card_type:?})")
            }
            MPip::ChanceDestroyed(chips, value) => {
                write!(f, "ChanceDestroyed({chips}, {value})")
            }
            MPip::Chips(chips) => write!(f, "Chips({chips})"),
            MPip::ChipsAndMultPlus(chips, value) => {
                write!(f, "ChipsAndMultPlus({chips}, {value})")
            }
            MPip::ChipsOnFlush(chips) => write!(f, "ChipsOnFlush({chips})"),
            MPip::ChipsOnPair(chips) => write!(f, "ChipsOnPair({chips})"),
            MPip::ChipsOn2Pair(chips) => write!(f, "ChipsOn2Pair({chips})"),
            MPip::ChipsOnStraight(chips) => write!(f, "ChipsOn2Straight({chips})"),
            MPip::ChipsOnTrips(chips) => write!(f, "ChipsOnTrips({chips})"),
            MPip::ChipsPerRemainingDiscard(chips) => write!(f, "ChipsPerRemainingDiscard({chips})"),
            MPip::CreateCardOnRankPlay(odds, rank_char, card_type) => {
                write!(
                    f,
                    "CreateCardOnRankPlay({odds}, {rank_char}, {card_type:?})"
                )
            }
            MPip::Credit(value) => write!(f, "Credit({value})"),
            MPip::Death(value) => write!(f, "Death({value})"),
            MPip::DoubleMoney(value) => write!(f, "DoubleMoney({value})"),
            MPip::FourFlushAndStraight => write!(f, "FourFlushAndStraight"),
            MPip::FreeReroll(value) => write!(f, "FreeReroll({value})"),
            MPip::Glass(a, b) => write!(f, "Glass({a}, {b})"),
            MPip::Gold(value) => write!(f, "Gold({value})"),
            MPip::Hanged(value) => write!(f, "Hanged({value})"),
            MPip::JokersValue(value) => write!(f, "JokersValue({value})"),
            MPip::Lucky(a, b) => write!(f, "Lucky({a}, {b})"),
            MPip::MultPlus(value) => write!(f, "MultPlus({value})"),
            MPip::MultPlusChipsOnRank(mult, chips, rank_char) => {
                write!(f, "MultPlusChipsOnRank({mult}, {chips}, {rank_char})")
            }
            MPip::MultPlusDoubleValueDestroyJokerOnRight(value) => {
                write!(f, "MultPlusDoubleValueDestroyJokerOnRight({value})")
            }
            MPip::MultPlusOn5Ranks(value, ranks) => {
                write!(f, "MultPlusOn5Ranks({value}, {ranks:?})")
            }
            MPip::MultPlusOnConsecutiveHandsNo3Ranks(value, increment, ranks) => {
                write!(
                    f,
                    "MultPlusOnConsecutiveHandsNo3Ranks({value}, {increment}, {ranks:?})"
                )
            }
            MPip::MultPlusOnFlush(value) => write!(f, "MultPlusOnFlush({value})"),
            MPip::MultPlusOnHandPlays => write!(f, "MultPlusOnHandPlays)"),
            MPip::MultPlusOnPair(value) => write!(f, "MultPlusOnPair({value})"),
            MPip::MultPlusOn2Pair(value) => write!(f, "MultPlusOn2Pair({value})"),
            MPip::MultPlusOnStraight(value) => write!(f, "MultPlusOnStraight({value})"),
            MPip::MultPlusOnTrips(value) => write!(f, "MultPlusOnTrips({value})"),
            MPip::MultPlusOnSuit(value, c) => write!(f, "MultPlusOnSuit({value}, {c})"),
            MPip::MultPlusOnUpToXCards(value, cards) => {
                write!(f, "MultPlusOnUpToXCards({value}, {cards})")
            }
            MPip::MultPlusOnZeroDiscards(value) => write!(f, "MultPlusOnZeroDiscards({value})"),
            MPip::MultPlusXOnLowestRankInHand(value) => {
                write!(f, "MultPlusXOnLowestRankInHand({value})")
            }
            MPip::MultPlusRandomTo(value) => write!(f, "MultPlusRandomTo({value})"),
            MPip::MultPlusZeroDiscards(value) => write!(f, "MultPlusZeroDiscards({value})"),
            MPip::MultTimes(value) => write!(f, "MultTimes({value})"),
            MPip::MultTimesEveryXHands(value, hands) => {
                write!(f, "MultTimesEveryXHands({value}, {hands})")
            }
            MPip::MultTimesOnEmptyJokerSlots(value) => {
                write!(f, "MultTimesOnEmptyJokerSlots({value})")
            }
            MPip::MultTimes1Dot(value) => write!(f, "MultTimes1Dot({value})"),
            MPip::Planet(value) => write!(f, "Planet({value})"),
            MPip::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            MPip::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            MPip::RetriggerCardsInHand(value) => write!(f, "RetriggerCardsInHand({value})"),
            MPip::RetriggerPlayedCardsInFinalRound => write!(f, "RetriggerPlayedCardsInFinalRound"),
            MPip::SellValueIncrement(value) => write!(f, "SellValueIncrement({value})"),
            MPip::Stone(value) => write!(f, "Stone({value})"),
            MPip::Strength => write!(f, "Strength"),
            MPip::Odds1in(value) => write!(f, "Wheel({value})"),
            MPip::Odds1inCashOn3Ranks(value, cash, ranks) => {
                write!(f, "Odds1inCashOn3Ranks({value}, {cash}, {ranks:?})")
            }
            MPip::Odds1inUpgradeHand(value) => write!(f, "Odds1inUpgradeHand({value})"),
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
