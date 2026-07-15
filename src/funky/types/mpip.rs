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
    /// Shortcut: straights may skip a rank (a one-gap straight, e.g. 5-7-9-J-K).
    GappedStraight,
    /// Pareidolia: every card counts as a face card, feeding the face-reading
    /// jokers (Scary Face, Sock and Buskin). No standalone score of its own.
    AllCardsAreFaces,
    /// Smeared Joker: Hearts≡Diamonds and Spades≡Clubs when sizing a flush.
    SmearedSuits,
    /// Splash: every played card scores. Inert in this engine, which already
    /// scores every played card (it has no scoring-vs-kicker distinction).
    AllPlayedCardsScore,
    /// Oops! All 6s: doubles every listed probability (a 1-in-N roll wins twice
    /// as often); stacks, capped at certainty.
    DoubleOdds,
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
    MultTimesOnPair(usize),
    MultTimesOnTrips(usize),
    MultTimesOn4OfAKind(usize),
    MultTimesOnStraight(usize),
    MultTimesOnFlush(usize),
    /// ×n mult for **each** played card whose rank is in the set — the factor
    /// compounds (`n^count`). Used by Triboulet (×2 per played King or Queen).
    MultTimesPerScoredRank(usize, [char; 2]),
    /// +n mult for **each** joker on the board (counting the source). Used by
    /// Abstract Joker (+3 per joker).
    MultPlusPerJoker(usize),
    /// +n chips for **each** card remaining in the deck. Used by Blue Joker
    /// (+2 per card in deck).
    ///
    /// Note this reads the **undealt remainder**, not the full deck; contrast
    /// [`ChipsPerFullDeckStone`](Self::ChipsPerFullDeckStone).
    ChipsPerDeckCard(usize),
    /// ×mult that gains ×(n/10) for **each** Steel card in the run's *full*
    /// deck. The factor is additive, not compounding: `1 + (n/10) × count`, so
    /// Steel Joker (`n = 2`) is ×1 with no Steel and ×1.4 with two.
    MultTimesPlusPerFullDeckSteel(usize),
    /// +n chips for **each** Stone card in the run's *full* deck. Used by Stone
    /// Joker (+25 per Stone).
    ChipsPerFullDeckStone(usize),
    /// +n mult for **each** card the run's *full* deck is **below** its
    /// starting size — i.e. per card destroyed. Used by Erosion (+4).
    MultPlusPerMissingDeckCard(usize),
    /// ×(n/10) mult for **each** card of the given rank **held in hand** — the
    /// factor compounds. Used by Baron (×1.5 per held King).
    MultTimesPerHeldRank(usize, char),
    /// +n chips for **each** played face card (J/Q/K). Used by Scary Face (+30).
    ChipsPlusPerScoredFace(usize),
    /// +chips and +mult for **each** played card whose rank is in the set. Used
    /// by Walkie Talkie (+10 chips, +4 mult per played 10 or 4).
    ChipsMultPlusPerScoredRanks(usize, usize, [char; 2]),
    /// ×(n/10) mult for **each** Uncommon joker on the board — compounds. Used
    /// by Baseball Card (×1.5 per Uncommon joker).
    MultTimesPerUncommonJoker(usize),
    /// ×n mult if **every** card held in hand has a suit in the set (vacuously
    /// true when the hand is empty). Used by Blackboard (×3 if all held are
    /// Spades or Clubs).
    MultTimesIfHeldAllSuits(usize, [char; 2]),
    /// +n chips for **each** $1 of money the run holds; debt (negative money)
    /// scores nothing. Used by Bull (+2 chips per dollar).
    ChipsPerDollar(usize),
    /// Green Joker: +n mult per hand played, −n per discard; the accumulator is
    /// the net (hands − discards), the read floors it at 0.
    GainMultPerHandLessDiscard(usize),
    /// Ramen: starts at `base`/100 ×mult and loses `per`/100 per card discarded;
    /// the accumulator counts cards discarded. Read floors at ×1.
    LoseMultTimesPerDiscard(usize, usize),
    /// Ice Cream: starts at `base` chips and loses `per` per hand played; the
    /// accumulator counts hands played. Read floors at 0.
    LoseChipsPerHand(usize, usize),
    /// Square Joker: +`rate` chips per hand played with exactly `n` cards; the
    /// accumulator counts qualifying hands.
    GainChipsPerCardCountHand(usize, usize),
    /// Spare Trousers: +`rate` mult per hand played containing a Two Pair.
    GainMultPerTwoPairHand(usize),
    /// Runner: +`rate` chips per hand played containing a Straight.
    GainChipsPerStraightHand(usize),
    /// Hiker: every played card **permanently** gains `n` chips when scored.
    ///
    /// Unlike its `Gain*` neighbours this is not a counter — nothing accumulates
    /// on the joker. The growth lives on the *cards*: each scored card's base
    /// chips are bumped for the rest of the run, so the joker's contribution is
    /// whatever those fattened cards go on to score. Applied by
    /// `BuffoonBoard::on_scored`, not by a scoring arm.
    GainChipsOnScored(usize),
    Planet(usize),
    RandomJoker(usize),
    RandomTarot(usize),
    RetriggerCardsInHand(usize),
    RetriggerPlayedCardsInFinalRound,
    /// Hack: re-score each played card of one of these ranks `n` additional
    /// times. `RetriggerPlayedRanks(1, ['2','3','4','5'])` retriggers every
    /// played 2, 3, 4, or 5 once more.
    RetriggerPlayedRanks(usize, [char; 4]),
    /// Sock and Buskin: re-score each played **face** card (K/Q/J) `n`
    /// additional times.
    RetriggerPlayedFaces(usize),
    /// Hanging Chad: re-score the **first** played card `n` additional times
    /// (`RetriggerFirstPlayed(2)` = scored 3× total).
    RetriggerFirstPlayed(usize),
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
    /// A mod-defined effect, resolved through an
    /// [`EffectRegistry`](crate::funky::types::effect::EffectRegistry) by this
    /// id. Kept a plain `u32` so cards stay `Copy`, `const` and `Serialize`.
    Custom(u32),
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
    // One arm per variant — a long but flat exhaustive match.
    #[allow(clippy::too_many_lines)]
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
            Self::ChipsOnStraight(chips) => write!(f, "ChipsOnStraight({chips})"),
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
            Self::GappedStraight => write!(f, "GappedStraight"),
            Self::AllCardsAreFaces => write!(f, "AllCardsAreFaces"),
            Self::SmearedSuits => write!(f, "SmearedSuits"),
            Self::AllPlayedCardsScore => write!(f, "AllPlayedCardsScore"),
            Self::DoubleOdds => write!(f, "DoubleOdds"),
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
            Self::MultPlusOnHandPlays => write!(f, "MultPlusOnHandPlays"),
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
            Self::MultTimesOnPair(value) => write!(f, "MultTimesOnPair({value})"),
            Self::MultTimesOnTrips(value) => write!(f, "MultTimesOnTrips({value})"),
            Self::MultTimesOn4OfAKind(value) => write!(f, "MultTimesOn4OfAKind({value})"),
            Self::MultTimesOnStraight(value) => write!(f, "MultTimesOnStraight({value})"),
            Self::MultTimesOnFlush(value) => write!(f, "MultTimesOnFlush({value})"),
            Self::MultTimesPerScoredRank(value, ranks) => {
                write!(f, "MultTimesPerScoredRank({value}, {ranks:?})")
            }
            Self::MultPlusPerJoker(value) => write!(f, "MultPlusPerJoker({value})"),
            Self::ChipsPerDeckCard(value) => write!(f, "ChipsPerDeckCard({value})"),
            Self::MultTimesPlusPerFullDeckSteel(value) => {
                write!(f, "MultTimesPlusPerFullDeckSteel({value})")
            }
            Self::ChipsPerFullDeckStone(value) => write!(f, "ChipsPerFullDeckStone({value})"),
            Self::MultPlusPerMissingDeckCard(value) => {
                write!(f, "MultPlusPerMissingDeckCard({value})")
            }
            Self::MultTimesPerHeldRank(value, rank) => {
                write!(f, "MultTimesPerHeldRank({value}, {rank})")
            }
            Self::ChipsPlusPerScoredFace(value) => write!(f, "ChipsPlusPerScoredFace({value})"),
            Self::ChipsMultPlusPerScoredRanks(chips, mult, ranks) => {
                write!(f, "ChipsMultPlusPerScoredRanks({chips}, {mult}, {ranks:?})")
            }
            Self::MultTimesPerUncommonJoker(value) => {
                write!(f, "MultTimesPerUncommonJoker({value})")
            }
            Self::MultTimesIfHeldAllSuits(value, suits) => {
                write!(f, "MultTimesIfHeldAllSuits({value}, {suits:?})")
            }
            Self::ChipsPerDollar(value) => write!(f, "ChipsPerDollar({value})"),
            Self::GainMultPerHandLessDiscard(n) => write!(f, "GainMultPerHandLessDiscard({n})"),
            Self::LoseMultTimesPerDiscard(base, per) => {
                write!(f, "LoseMultTimesPerDiscard({base}, {per})")
            }
            Self::LoseChipsPerHand(base, per) => write!(f, "LoseChipsPerHand({base}, {per})"),
            Self::GainChipsPerCardCountHand(rate, n) => {
                write!(f, "GainChipsPerCardCountHand({rate}, {n})")
            }
            Self::GainMultPerTwoPairHand(n) => write!(f, "GainMultPerTwoPairHand({n})"),
            Self::GainChipsPerStraightHand(n) => write!(f, "GainChipsPerStraightHand({n})"),
            Self::GainChipsOnScored(n) => write!(f, "GainChipsOnScored({n})"),
            Self::Planet(value) => write!(f, "Planet({value})"),
            Self::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            Self::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            Self::RetriggerCardsInHand(value) => write!(f, "RetriggerCardsInHand({value})"),
            Self::RetriggerPlayedCardsInFinalRound => write!(f, "RetriggerPlayedCardsInFinalRound"),
            Self::RetriggerPlayedRanks(value, ranks) => {
                write!(f, "RetriggerPlayedRanks({value}, {ranks:?})")
            }
            Self::RetriggerPlayedFaces(value) => write!(f, "RetriggerPlayedFaces({value})"),
            Self::RetriggerFirstPlayed(value) => write!(f, "RetriggerFirstPlayed({value})"),
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
            Self::Custom(id) => write!(f, "Custom({id})"),
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

    #[test]
    fn display__counter_variants() {
        assert_eq!(
            MPip::GainMultPerHandLessDiscard(1).to_string(),
            "GainMultPerHandLessDiscard(1)"
        );
        assert_eq!(
            MPip::LoseMultTimesPerDiscard(200, 1).to_string(),
            "LoseMultTimesPerDiscard(200, 1)"
        );
        assert_eq!(
            MPip::LoseChipsPerHand(100, 5).to_string(),
            "LoseChipsPerHand(100, 5)"
        );
        assert_eq!(
            MPip::GainChipsPerCardCountHand(4, 4).to_string(),
            "GainChipsPerCardCountHand(4, 4)"
        );
        assert_eq!(
            MPip::GainMultPerTwoPairHand(2).to_string(),
            "GainMultPerTwoPairHand(2)"
        );
        assert_eq!(
            MPip::GainChipsPerStraightHand(15).to_string(),
            "GainChipsPerStraightHand(15)"
        );
        assert_eq!(
            MPip::GainChipsOnScored(4).to_string(),
            "GainChipsOnScored(4)"
        );
    }
}
