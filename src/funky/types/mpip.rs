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

    // ── Chips / Mult on rank condition ───────────────────────────────────────
    AddBaseChips(usize),
    ChipsMultPlus(usize, usize),
    ChipsMultPlusOnHand(usize, usize, HandType),
    /// +chips and +mult when a card with one of two specific ranks is scored (Walkie Talkie)
    ChipsMultPlusOn2Ranks(usize, usize, [char; 2]),
    ChipsPlusOn5Ranks(usize, [char; 5]),
    MultPlusChipsOnRank(usize, usize, char),
    MultPlusOn5Ranks(usize, [char; 5]),

    // ── Chips on hand type ───────────────────────────────────────────────────
    ChipsOnFlush(usize),
    ChipsOnPair(usize),
    ChipsOn2Pair(usize),
    ChipsOnStraight(usize),
    ChipsOnTrips(usize),

    // ── Chips / Cash on suit ─────────────────────────────────────────────────
    /// +chips when a card of the given suit is scored (Arrowhead)
    ChipsOnSuit(usize, char),
    /// earn $X when a card of the given suit is scored (Rough Gem)
    CashOnSuit(usize, char),

    // ── Chips misc ───────────────────────────────────────────────────────────
    Chips(usize),
    ChipsPerRemainingDiscard(usize),

    // ── MultPlus on card properties ──────────────────────────────────────────
    MultPlus(usize),
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
    MultPlusDoubleValueDestroyJokerOnRight(usize),
    /// +mult when a face card is scored (Smiley Face)
    MultPlusOnFaceCard(usize),
    /// +mult per instance of a specific rank held in hand (Shoot the Moon)
    MultPlusOnRankInHand(usize, char),
    /// +mult for every $N owned (Bootstraps)
    MultPlusPerDollars(usize, usize),
    /// adds the sell value of all other owned jokers as +mult (Swashbuckler)
    MultPlusFromOtherJokerSellValues,

    // ── MultTimes on hand type ────────────────────────────────────────────────
    MultTimes(usize),
    MultTimesEveryXHands(usize, usize),
    MultTimes1Dot(usize),
    MultTimesOnEmptyJokerSlots(usize),
    /// Xmult if played hand contains a specific hand type (The Duo/Trio/Family/Order/Tribe)
    MultTimesOnHand(usize, HandType),

    // ── MultTimes on suit composition ─────────────────────────────────────────
    /// Xmult if played hand contains all four suits (Flower Pot)
    MultTimesOnAllFourSuits(usize),
    /// Xmult if played hand has a scoring Club and a scoring card of any other suit (Seeing Double)
    MultTimesOnClubPlusAnySuit(usize),
    /// 1-in-N chance for Xmult on cards of the given suit (Bloodstone)
    OddsMultTimesOnSuit(usize, usize, char),

    // ── MultTimes on game state ───────────────────────────────────────────────
    /// Xmult on the final hand of a round (Acrobat)
    MultTimesOnFinalHand(usize),
    /// gain Xmult for each blind skipped this run (Throwback)
    MultTimesPerBlindSkipped(usize),
    /// Xmult on a randomly chosen rank+suit combination that changes each round (The Idol)
    MultTimesOnDynamicRankSuit(usize),
    /// Xmult if deck contains at least N enhanced cards (Driver's License)
    MultTimesOnEnhancedCardCount(usize, usize),
    /// Xmult for each played King or Queen scored (Triboulet)
    MultTimesOnRoyalFaceCards(usize),

    // ── Gain variants (stateful accumulation) ────────────────────────────────
    /// gain +chips each time a card of the current suit is discarded (Castle)
    GainChipsOnDiscardedSuit(usize),
    /// gain Xmult each time any card is sold; resets on Boss Blind defeat (Campfire)
    GainMultTimesOnCardSold(usize),
    /// gain Xmult each time a Glass card is destroyed (Glass Joker)
    GainMultTimesOnGlassDestroyed(usize),
    /// gain +chips each time a card of the given rank is scored (Wee Joker)
    GainChipsOnRankScored(usize, char),
    /// gain Xmult for each card of the given rank discarded this round (Hit the Road)
    GainMultTimesOnRankDiscarded(usize, char),
    /// gain Xmult each time a face card is destroyed (Canio)
    GainMultTimesOnFaceCardDestroyed(usize),
    /// gain Xmult every N cards discarded across the run (Yorick)
    GainMultTimesEveryXDiscards(usize, usize),

    // ── Retrigger variants ───────────────────────────────────────────────────
    RetriggerCardsInHand(usize),
    RetriggerPlayedCardsInFinalRound,
    /// retrigger all played cards for the next N hands (Seltzer)
    RetriggerAllPlayedCardsForNHands(usize),
    /// retrigger all played face cards (Sock and Buskin)
    RetriggerFaceCards,
    /// retrigger the first scoring card N additional times (Hanging Chad)
    RetriggerFirstScoringCard(usize),

    // ── Cash on condition ────────────────────────────────────────────────────
    Credit(usize),
    DoubleMoney(usize),
    Gold(usize),
    SellValueIncrement(usize),
    /// earn $X when a Gold card is scored (Golden Ticket)
    CashOnGoldCard(usize),
    /// earn $X if the played hand triggers the Boss Blind ability (Matador)
    CashOnBossBlindTrigger(usize),
    /// earn $X at end of round per unique Planet card used this run (Satellite)
    CashPerUniquePlanetUsed(usize),

    // ── Odds-based effects ───────────────────────────────────────────────────
    Odds1in(usize),
    Odds1inCashOn3Ranks(usize, usize, [char; 3]),
    Odds1inUpgradeHand(usize),

    // ── Card creation / transformation ───────────────────────────────────────
    AddCardTypeWhenBlindSelected(BCardType),
    CreateCardOnRankPlay(usize, char, BCardType),
    RandomJoker(usize),
    RandomTarot(usize),

    // ── Card enhancement effects ─────────────────────────────────────────────
    ChanceDestroyed(usize, usize),
    Death(usize),
    Glass(usize, usize),
    Hanged(usize),
    JokersValue(usize),
    Lucky(usize, usize),
    Stone(usize),
    Strength,
    Wild(PipType),

    // ── Suit conversion (tarot suit effects) ─────────────────────────────────
    Diamonds(usize),
    Clubs(usize),
    Hearts(usize),
    Spades(usize),
    Planet(usize),

    // ── Game-state markers (non-scoring rule modifiers) ───────────────────────
    FourFlushAndStraight,
    FreeReroll(usize),
    /// prevent death if chips scored are at least N% of required chips; self-destructs (Mr. Bones)
    PreventDeathAtPercent(usize),
    /// add a random playing card with a random seal to hand at round start (Certificate)
    AddRandomSealedCardOnRoundStart,
    /// Hearts+Diamonds count as the same suit; Spades+Clubs count as the same suit (Smeared Joker)
    SmearedSuits,
    /// Joker/Tarot/Planet/Spectral cards may appear multiple times in the shop (Showman)
    AllowCardTypeDuplicates,
    /// copies the scoring ability of the joker immediately to the right (Blueprint)
    CopyJokerToRight,
    /// doubles all listed probabilities (Oops! All 6s)
    DoubleProbabilities,
    /// after N rounds, sell this card to duplicate a random joker (Invisible Joker)
    DuplicateRandomJokerAfterRounds(usize),
    /// copies the scoring ability of the leftmost joker (Brainstorm)
    CopyLeftmostJoker,
    /// all Planet cards and Celestial Packs in the shop are free (Astronomer)
    FreePlanetCards,
    /// upgrade the level of the first discarded poker hand each round (Burnt Joker)
    UpgradeHandOnDiscard,
    /// disables the effect of every Boss Blind (Chicot)
    DisableBossBlind,
    /// creates a Negative copy of a random consumable at end of shop (Perkeo)
    CreateNegativeCopyOfConsumable,
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
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, "Blank"),
            Self::AddBaseChips(chips) => write!(f, "AddBaseChips({chips})"),
            Self::AddCardTypeWhenBlindSelected(card_type) => {
                write!(f, "AddCardTypeWhenBlindSelected({card_type:?})")
            }
            Self::ChanceDestroyed(chips, value) => write!(f, "ChanceDestroyed({chips}, {value})"),
            Self::Chips(chips) => write!(f, "Chips({chips})"),
            Self::ChipsMultPlus(chips, value) => write!(f, "ChipsMultPlus({chips}, {value})"),
            Self::ChipsMultPlusOnHand(chips, mult, hand_type) => {
                write!(f, "ChipsMultPlusOnHand({chips}, {mult}, {hand_type:?})")
            }
            Self::ChipsMultPlusOn2Ranks(chips, mult, ranks) => {
                write!(f, "ChipsMultPlusOn2Ranks({chips}, {mult}, {ranks:?})")
            }
            Self::ChipsOnFlush(chips) => write!(f, "ChipsOnFlush({chips})"),
            Self::ChipsOnPair(chips) => write!(f, "ChipsOnPair({chips})"),
            Self::ChipsOn2Pair(chips) => write!(f, "ChipsOn2Pair({chips})"),
            Self::ChipsOnStraight(chips) => write!(f, "ChipsOnStraight({chips})"),
            Self::ChipsOnTrips(chips) => write!(f, "ChipsOnTrips({chips})"),
            Self::ChipsOnSuit(chips, suit) => write!(f, "ChipsOnSuit({chips}, {suit})"),
            Self::CashOnSuit(cash, suit) => write!(f, "CashOnSuit({cash}, {suit})"),
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
            Self::MultPlusOnFaceCard(value) => write!(f, "MultPlusOnFaceCard({value})"),
            Self::MultPlusOnRankInHand(value, rank) => {
                write!(f, "MultPlusOnRankInHand({value}, {rank})")
            }
            Self::MultPlusPerDollars(mult, dollars) => {
                write!(f, "MultPlusPerDollars({mult}, {dollars})")
            }
            Self::MultPlusFromOtherJokerSellValues => {
                write!(f, "MultPlusFromOtherJokerSellValues")
            }
            Self::MultTimes(value) => write!(f, "MultTimes({value})"),
            Self::MultTimesEveryXHands(value, hands) => {
                write!(f, "MultTimesEveryXHands({value}, {hands})")
            }
            Self::MultTimesOnEmptyJokerSlots(value) => {
                write!(f, "MultTimesOnEmptyJokerSlots({value})")
            }
            Self::MultTimes1Dot(value) => write!(f, "MultTimes1Dot({value})"),
            Self::MultTimesOnHand(value, hand_type) => {
                write!(f, "MultTimesOnHand({value}, {hand_type:?})")
            }
            Self::MultTimesOnAllFourSuits(value) => write!(f, "MultTimesOnAllFourSuits({value})"),
            Self::MultTimesOnClubPlusAnySuit(value) => {
                write!(f, "MultTimesOnClubPlusAnySuit({value})")
            }
            Self::OddsMultTimesOnSuit(odds, value, suit) => {
                write!(f, "OddsMultTimesOnSuit({odds}, {value}, {suit})")
            }
            Self::MultTimesOnFinalHand(value) => write!(f, "MultTimesOnFinalHand({value})"),
            Self::MultTimesPerBlindSkipped(value) => {
                write!(f, "MultTimesPerBlindSkipped({value})")
            }
            Self::MultTimesOnDynamicRankSuit(value) => {
                write!(f, "MultTimesOnDynamicRankSuit({value})")
            }
            Self::MultTimesOnEnhancedCardCount(value, threshold) => {
                write!(f, "MultTimesOnEnhancedCardCount({value}, {threshold})")
            }
            Self::MultTimesOnRoyalFaceCards(value) => {
                write!(f, "MultTimesOnRoyalFaceCards({value})")
            }
            Self::GainChipsOnDiscardedSuit(value) => {
                write!(f, "GainChipsOnDiscardedSuit({value})")
            }
            Self::GainMultTimesOnCardSold(value) => write!(f, "GainMultTimesOnCardSold({value})"),
            Self::GainMultTimesOnGlassDestroyed(value) => {
                write!(f, "GainMultTimesOnGlassDestroyed({value})")
            }
            Self::GainChipsOnRankScored(value, rank) => {
                write!(f, "GainChipsOnRankScored({value}, {rank})")
            }
            Self::GainMultTimesOnRankDiscarded(value, rank) => {
                write!(f, "GainMultTimesOnRankDiscarded({value}, {rank})")
            }
            Self::GainMultTimesOnFaceCardDestroyed(value) => {
                write!(f, "GainMultTimesOnFaceCardDestroyed({value})")
            }
            Self::GainMultTimesEveryXDiscards(value, every) => {
                write!(f, "GainMultTimesEveryXDiscards({value}, {every})")
            }
            Self::Planet(value) => write!(f, "Planet({value})"),
            Self::RandomJoker(value) => write!(f, "RandomJoker({value})"),
            Self::RandomTarot(value) => write!(f, "RandomTarot({value})"),
            Self::RetriggerCardsInHand(value) => write!(f, "RetriggerCardsInHand({value})"),
            Self::RetriggerPlayedCardsInFinalRound => write!(f, "RetriggerPlayedCardsInFinalRound"),
            Self::RetriggerAllPlayedCardsForNHands(value) => {
                write!(f, "RetriggerAllPlayedCardsForNHands({value})")
            }
            Self::RetriggerFaceCards => write!(f, "RetriggerFaceCards"),
            Self::RetriggerFirstScoringCard(value) => {
                write!(f, "RetriggerFirstScoringCard({value})")
            }
            Self::SellValueIncrement(value) => write!(f, "SellValueIncrement({value})"),
            Self::Stone(value) => write!(f, "Stone({value})"),
            Self::Strength => write!(f, "Strength"),
            Self::Odds1in(value) => write!(f, "Odds1in({value})"),
            Self::Odds1inCashOn3Ranks(value, cash, ranks) => {
                write!(f, "Odds1inCashOn3Ranks({value}, {cash}, {ranks:?})")
            }
            Self::Odds1inUpgradeHand(value) => write!(f, "Odds1inUpgradeHand({value})"),
            Self::Wild(pip_type) => write!(f, "Wild({pip_type:?})"),
            Self::Diamonds(value) => write!(f, "Diamonds({value})"),
            Self::Clubs(value) => write!(f, "Clubs({value})"),
            Self::Hearts(value) => write!(f, "Hearts({value})"),
            Self::Spades(value) => write!(f, "Spades({value})"),
            Self::CashOnGoldCard(value) => write!(f, "CashOnGoldCard({value})"),
            Self::CashOnBossBlindTrigger(value) => write!(f, "CashOnBossBlindTrigger({value})"),
            Self::CashPerUniquePlanetUsed(value) => write!(f, "CashPerUniquePlanetUsed({value})"),
            Self::PreventDeathAtPercent(value) => write!(f, "PreventDeathAtPercent({value})"),
            Self::AddRandomSealedCardOnRoundStart => write!(f, "AddRandomSealedCardOnRoundStart"),
            Self::SmearedSuits => write!(f, "SmearedSuits"),
            Self::AllowCardTypeDuplicates => write!(f, "AllowCardTypeDuplicates"),
            Self::CopyJokerToRight => write!(f, "CopyJokerToRight"),
            Self::DoubleProbabilities => write!(f, "DoubleProbabilities"),
            Self::DuplicateRandomJokerAfterRounds(value) => {
                write!(f, "DuplicateRandomJokerAfterRounds({value})")
            }
            Self::CopyLeftmostJoker => write!(f, "CopyLeftmostJoker"),
            Self::FreePlanetCards => write!(f, "FreePlanetCards"),
            Self::UpgradeHandOnDiscard => write!(f, "UpgradeHandOnDiscard"),
            Self::DisableBossBlind => write!(f, "DisableBossBlind"),
            Self::CreateNegativeCopyOfConsumable => write!(f, "CreateNegativeCopyOfConsumable"),
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
    fn display_new_variants_roundtrip() {
        let cases: &[MPip] = &[
            MPip::ChipsMultPlusOn2Ranks(10, 4, ['T', '4']),
            MPip::ChipsOnSuit(50, 'S'),
            MPip::CashOnSuit(1, 'D'),
            MPip::MultTimesOnHand(2, HandType::Pair),
            MPip::MultTimesOnAllFourSuits(3),
            MPip::MultTimesOnClubPlusAnySuit(2),
            MPip::OddsMultTimesOnSuit(2, 15, 'H'),
            MPip::MultPlusOnFaceCard(5),
            MPip::MultPlusOnRankInHand(13, 'Q'),
            MPip::MultPlusPerDollars(2, 5),
            MPip::MultPlusFromOtherJokerSellValues,
            MPip::MultTimesOnFinalHand(3),
            MPip::MultTimesPerBlindSkipped(25),
            MPip::MultTimesOnDynamicRankSuit(2),
            MPip::MultTimesOnEnhancedCardCount(3, 16),
            MPip::MultTimesOnRoyalFaceCards(2),
            MPip::GainChipsOnDiscardedSuit(3),
            MPip::GainMultTimesOnCardSold(25),
            MPip::GainMultTimesOnGlassDestroyed(75),
            MPip::GainChipsOnRankScored(8, '2'),
            MPip::GainMultTimesOnRankDiscarded(50, 'J'),
            MPip::GainMultTimesOnFaceCardDestroyed(1),
            MPip::GainMultTimesEveryXDiscards(1, 23),
            MPip::RetriggerAllPlayedCardsForNHands(10),
            MPip::RetriggerFaceCards,
            MPip::RetriggerFirstScoringCard(2),
            MPip::CashOnGoldCard(4),
            MPip::CashOnBossBlindTrigger(8),
            MPip::CashPerUniquePlanetUsed(1),
            MPip::PreventDeathAtPercent(25),
            MPip::AddRandomSealedCardOnRoundStart,
            MPip::SmearedSuits,
            MPip::AllowCardTypeDuplicates,
            MPip::CopyJokerToRight,
            MPip::DoubleProbabilities,
            MPip::DuplicateRandomJokerAfterRounds(2),
            MPip::CopyLeftmostJoker,
            MPip::FreePlanetCards,
            MPip::UpgradeHandOnDiscard,
            MPip::DisableBossBlind,
            MPip::CreateNegativeCopyOfConsumable,
        ];
        for mpip in cases {
            // Display must not panic and must produce a non-empty string
            let s = format!("{mpip}");
            assert!(!s.is_empty(), "Display output empty for {mpip:?}");
        }
    }
}
