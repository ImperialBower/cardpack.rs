use crate::funky::types::hands::HandType;
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
    ChipsMultPlus(usize, usize),
    ChipsMultPlusOnHand(usize, usize, HandType),
    ChipsOnFlush(usize),
    ChipsOnPair(usize),
    ChipsOn2Pair(usize),
    ChipsOnStraight(usize),
    ChipsOnTrips(usize),
    ChipsPerRemainingDiscard(usize),
    ChipsPlusOn5Ranks(usize, [char; 5]),
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
    pub const BONUS: Self = Self::Chips(30);
    pub const DEVIL: Self = Self::Gold(3);
    pub const MULT_PLUS3_ON_DIAMONDS: Self = Self::MultPlusOnSuit(3, 'D');
    pub const MULT_PLUS3_ON_HEARTS: Self = Self::MultPlusOnSuit(3, 'H');
    pub const MULT_PLUS3_ON_SPADES: Self = Self::MultPlusOnSuit(3, 'S');
    pub const MULT_PLUS3_ON_CLUBS: Self = Self::MultPlusOnSuit(3, 'C');
    pub const STEEL: Self = Self::MultTimes1Dot(15); // 1.5
    pub const TEMPERANCE: Self = Self::JokersValue(50);
    pub const TOWER: Self = Self::Stone(50);
    pub const WORLD: Self = Self::MultTimes(2);
    pub const WHEEL_OF_FORTUNE: Self = Self::Odds1in(4);
    pub const JUDGEMENT: Self = Self::RandomJoker(1);

    #[must_use]
    pub fn new_chips(chips: usize) -> Self {
        Self::Chips(chips)
    }

    #[must_use]
    pub fn is_wild(&self) -> bool {
        matches!(self, Self::Wild(_))
    }
}

impl Display for MPip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, "Blank"),
            Self::AddBaseChips(chips) => write!(f, "AddBaseChips({chips}) "),
            Self::AddCardTypeWhenBlindSelected(card_type) => {
                write!(f, "AddStoneCardWhenBlindSelected({card_type:?})")
            }
            Self::ChanceDestroyed(chips, value) => {
                write!(f, "ChanceDestroyed({chips}, {value})")
            }
            Self::Chips(chips) => write!(f, "Chips({chips})"),
            Self::ChipsMultPlus(chips, value) => {
                write!(f, "ChipsMultPlus({chips}, {value})")
            }
            Self::ChipsMultPlusOnHand(chips, mult, hand_type) => {
                write!(f, "ChipsMultPlusOnHand({chips}, {mult}, {hand_type:?})")
            }
            Self::ChipsOnFlush(chips) => write!(f, "ChipsOnFlush({chips})"),
            Self::ChipsOnPair(chips) => write!(f, "ChipsOnPair({chips})"),
            Self::ChipsOn2Pair(chips) => write!(f, "ChipsOn2Pair({chips})"),
            Self::ChipsOnStraight(chips) => write!(f, "ChipsOn2Straight({chips})"),
            Self::ChipsOnTrips(chips) => write!(f, "ChipsOnTrips({chips})"),
            Self::ChipsPerRemainingDiscard(chips) => write!(f, "ChipsPerRemainingDiscard({chips})"),
            Self::ChipsPlusOn5Ranks(chips, ranks) => {
                write!(f, "ChipsPlusOn5Ranks({chips}, {ranks:?})")
            }
            Self::CreateCardOnRankPlay(odds, rank_char, card_type) => {
                write!(
                    f,
                    "CreateCardOnRankPlay({odds}, {rank_char}, {card_type:?})"
                )
            }
            Self::Credit(value) => write!(f, "Credit({value})"),
            Self::Death(value) => write!(f, "Death({value})"),
            Self::DoubleMoney(value) => write!(f, "DoubleMoney({value})"),
            Self::FourFlushAndStraight => write!(f, "FourFlushAndStraight"),
            Self::FreeReroll(value) => write!(f, "FreeReroll({value})"),
            Self::Glass(a, b) => write!(f, "Glass({a}, {b})"),
            Self::Gold(value) => write!(f, "Gold({value})"),
            Self::Hanged(value) => write!(f, "Hanged({value})"),
            Self::JokersValue(value) => write!(f, "JokersValue({value})"),
            Self::Lucky(a, b) => write!(f, "Lucky({a}, {b})"),
            Self::MultPlus(value) => write!(f, "MultPlus({value})"),
            Self::MultPlusChipsOnRank(mult, chips, rank_char) => {
                write!(f, "MultPlusChipsOnRank({mult}, {chips}, {rank_char})")
            }
            Self::MultPlusDoubleValueDestroyJokerOnRight(value) => {
                write!(f, "MultPlusDoubleValueDestroyJokerOnRight({value})")
            }
            Self::MultPlusOn5Ranks(value, ranks) => {
                write!(f, "MultPlusOn5Ranks({value}, {ranks:?})")
            }
            Self::MultPlusOnConsecutiveHandsNo3Ranks(value, increment, ranks) => {
                write!(
                    f,
                    "MultPlusOnConsecutiveHandsNo3Ranks({value}, {increment}, {ranks:?})"
                )
            }
            Self::MultPlusOnFlush(value) => write!(f, "MultPlusOnFlush({value})"),
            Self::MultPlusOnHandPlays => write!(f, "MultPlusOnHandPlays)"),
            Self::MultPlusOnPair(value) => write!(f, "MultPlusOnPair({value})"),
            Self::MultPlusOn2Pair(value) => write!(f, "MultPlusOn2Pair({value})"),
            Self::MultPlusOnStraight(value) => write!(f, "MultPlusOnStraight({value})"),
            Self::MultPlusOnTrips(value) => write!(f, "MultPlusOnTrips({value})"),
            Self::MultPlusOnSuit(value, c) => write!(f, "MultPlusOnSuit({value}, {c})"),
            Self::MultPlusOnUpToXCards(value, cards) => {
                write!(f, "MultPlusOnUpToXCards({value}, {cards})")
            }
            Self::MultPlusOnZeroDiscards(value) => write!(f, "MultPlusOnZeroDiscards({value})"),
            Self::MultPlusXOnLowestRankInHand(value) => {
                write!(f, "MultPlusXOnLowestRankInHand({value})")
            }
            Self::MultPlusRandomTo(value) => write!(f, "MultPlusRandomTo({value})"),
            Self::MultPlusZeroDiscards(value) => write!(f, "MultPlusZeroDiscards({value})"),
            Self::MultTimes(value) => write!(f, "MultTimes({value})"),
            Self::MultTimesEveryXHands(value, hands) => {
                write!(f, "MultTimesEveryXHands({value}, {hands})")
            }
            Self::MultTimesOnEmptyJokerSlots(value) => {
                write!(f, "MultTimesOnEmptyJokerSlots({value})")
            }
            Self::MultTimes1Dot(value) => write!(f, "MultTimes1Dot({value})"),
            Self::Planet(value) => write!(f, "Planet({value})"),
            Self::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            Self::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            Self::RetriggerCardsInHand(value) => write!(f, "RetriggerCardsInHand({value})"),
            Self::RetriggerPlayedCardsInFinalRound => write!(f, "RetriggerPlayedCardsInFinalRound"),
            Self::SellValueIncrement(value) => write!(f, "SellValueIncrement({value})"),
            Self::Stone(value) => write!(f, "Stone({value})"),
            Self::Strength => write!(f, "Strength"),
            Self::Odds1in(value) => write!(f, "Wheel({value})"),
            Self::Odds1inCashOn3Ranks(value, cash, ranks) => {
                write!(f, "Odds1inCashOn3Ranks({value}, {cash}, {ranks:?})")
            }
            Self::Odds1inUpgradeHand(value) => write!(f, "Odds1inUpgradeHand({value})"),
            Self::Wild(pip_type) => write!(f, "Wild({pip_type:?})"),
            Self::Diamonds(value) => write!(f, "Diamonds({value})"),
            Self::Clubs(value) => write!(f, "Clubs({value})"),
            Self::Hearts(value) => write!(f, "Hearts({value})"),
            Self::Spades(value) => write!(f, "Spades({value})"),
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
