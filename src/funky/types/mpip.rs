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
    /// A `numerator`-in-`denominator` chance the card is destroyed at end of
    /// round, and **nothing else**. Rolled by
    /// `BuffoonBoard::on_round_end_with_rng` like its compound siblings.
    ///
    /// Prefer a compound variant like
    /// [`MultPlusChanceDestroyed`](Self::MultPlusChanceDestroyed) for any card
    /// that also scores. Encoding only the destruction half is what hid Gros
    /// Michel's +15 mult: the reachability guard classifies by variant, and this
    /// variant legitimately does not score, so a card wearing it is invisible to
    /// the guard whether or not it *should* have scored.
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
    /// Gros Michel: `+mult` unconditionally, **and** a `numerator`-in-`denominator`
    /// chance the joker is destroyed at end of round —
    /// `MultPlusChanceDestroyed(15, 1, 6)`.
    ///
    /// Compound because one card is one `MPip`, and both halves are the card:
    /// splitting them is what let the mult go missing. The scoring half is a
    /// pure `AddMult`, applied in `builtin_joker_op`. The destruction half
    /// rolls in `BuffoonBoard::on_round_end_with_rng`, routed through
    /// `BuffoonBoard::probability_numerator` so it inherits Oops! All 6s for
    /// free, the way the seeded-RNG path already works.
    MultPlusChanceDestroyed(usize, usize, usize),
    /// Cavendish: ×mult unconditionally, **and** a `numerator`-in-`denominator`
    /// chance the joker is destroyed at end of round —
    /// `MultTimesChanceDestroyed(3, 1, 1000)`. The
    /// [`MultPlusChanceDestroyed`](Self::MultPlusChanceDestroyed) compound
    /// shape, on the ×mult side: the scoring half applies in `joker_x_mult`,
    /// the destruction half rolls in `BuffoonBoard::on_round_end_with_rng`,
    /// routed through `probability_numerator` so Oops! All 6s doubles it.
    MultTimesChanceDestroyed(usize, usize, usize),
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
    /// Earn $n at end of round. Used by Golden Joker ($4). Paid by
    /// `BuffoonBoard::on_round_end`, never by a scoring arm.
    CashOnRoundEnd(usize),
    /// Earn $n per **remaining** discard at end of round, forfeited entirely
    /// if any discard was used this round. Used by Delayed Gratification ($2).
    CashPerDiscardIfNoneUsed(usize),
    /// Earn $n per card of the given rank in the run's **full deck** at end of
    /// round. Used by Cloud 9 ($1 per 9) — destroyed 9s stop paying, added
    /// ones start.
    CashPerFullDeckRank(usize, char),
    /// Earn $n extra interest per $5 held at end of round, capped at the base
    /// interest cap (5 steps); debt earns nothing. Used by To the Moon ($1).
    ExtraInterest(usize),
    /// Earn $cash when at least `min` face cards are discarded at once. Used
    /// by Faceless Joker ($5 at ≥3 faces); Pareidolia widens what counts as a
    /// face, as in Balatro.
    CashOnFacesDiscarded(usize, usize),
    /// +n hand size while held. Used by Juggler (+1). Applied when
    /// `BuffoonBoard::on_blind_selected` recomputes the round's `Draws`.
    HandSizeIncrement(usize),
    /// +n discards per round while held. Used by Drunkard (+1). Applied when
    /// `BuffoonBoard::on_blind_selected` recomputes the round's `Draws`.
    DiscardIncrement(usize),
    /// Burglar: when a Blind is selected, gain +n hands and lose **all**
    /// discards — including any other joker's discard bonus, as in Balatro.
    GainHandsLoseDiscardsWhenBlindSelected(usize),
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
    /// Popcorn: starts at `base` mult and loses `per` per **round ended**; the
    /// accumulator counts rounds. Read floors at 0, and the joker is destroyed
    /// by the round that empties it — Ice Cream's
    /// [`LoseChipsPerHand`](Self::LoseChipsPerHand) shape on the mult side,
    /// decaying per round rather than per hand.
    LoseMultPerRound(usize, usize),
    /// Yorick: gains `rate`/100 ×mult per `per` **cards discarded** — the
    /// accumulator counts cards, not discard *actions*, so one big discard and
    /// several small ones of the same size are worth the same. Base ×1.
    GainMultTimesPerDiscardedCards(usize, usize),
    /// Hologram: gains `rate`/100 ×mult per playing card **added to the run's
    /// deck** (`BuffoonBoard::add_card_to_deck`). Base ×1.
    GainMultTimesPerCardAdded(usize),
    /// Canio: gains `rate`/100 ×mult per **face card destroyed**
    /// (`BuffoonBoard::destroy_deck_card`), classified through
    /// `BuffoonBoard::is_face_card` — so Pareidolia makes every destroyed card
    /// feed it, as it does Faceless Joker. Base ×1.
    GainMultTimesPerFaceDestroyed(usize),
    /// Riff-Raff: create `n` jokers of the given rarity when a Blind is
    /// selected, **while there is room** — `CreateJokersWhenBlindSelected(2,
    /// BCardType::CommonJoker)`.
    ///
    /// Room is checked per joker, not all-or-nothing: with one slot free, a
    /// `n = 2` creator fills it and stops. A full board creates nothing.
    /// Applied by `BuffoonBoard::on_blind_selected_with_rng`, since which
    /// jokers arrive is a random draw from the rarity's pool.
    CreateJokersWhenBlindSelected(usize, BCardType),
    /// Superposition: create a Tarot when the played hand is a **Straight** and
    /// contains an **Ace**, if there is a free consumable slot. Both conditions
    /// are required.
    CreateTarotOnAceStraight,
    /// Vagabond: create a Tarot when a hand is played holding **$n or less**, if
    /// there is a free consumable slot.
    CreateTarotOnLowMoney(usize),
    /// Card Sharp: ×n mult if the played hand type has **already been played
    /// this round**. Reads `BuffoonBoard::hands_by_type_this_round`.
    MultTimesOnRepeatedHandThisRound(usize),
    /// Ancient Joker: ×(n/10) mult for **each** played card of the run's current
    /// "ancient" suit (`BuffoonBoard::ancient_suit`), which re-rolls at the end
    /// of every round. The factor **compounds**, so three matching cards at
    /// ×1.5 is ×3.375.
    ///
    /// The suit lives on the board rather than in this variant because it is run
    /// state: two Ancient Jokers share it.
    MultTimesPerScoredAncientSuit(usize),
    /// Madness: gains `rate`/100 ×mult when a **Small or Big** Blind is selected
    /// — never on a Boss Blind — and destroys a random other joker. Base ×1.
    ///
    /// The two halves are independent: the ×mult is gained whether or not there
    /// was anything to destroy, so a lone Madness still grows. The destruction
    /// is random, so it lives in
    /// `BuffoonBoard::on_blind_selected_with_rng` while the gain is applied by
    /// the pure hook.
    GainMultTimesOnNonBossBlindDestroyingJoker(usize),
    /// Luchador: selling it disables the **current** Boss Blind's ability.
    /// Handled by `BuffoonBoard::sell_joker`; no standalone score.
    DisableBossBlindOnSell,
    /// Chicot: disables the ability of **every** Boss Blind, passively, while it
    /// is on the board. Read live by `BuffoonBoard::boss_ability_active`.
    DisablesAllBossBlinds,
    /// Rocket: earn `base` at end of round, with the payout growing by `increase`
    /// for each **Boss Blind defeated** — `CashOnRoundEndGrowingOnBossDefeat(1, 2)`.
    ///
    /// The accumulator counts bosses defeated. The increment lands *before* the
    /// round it was earned in pays out, so the boss round itself pays the
    /// already-raised amount.
    CashOnRoundEndGrowingOnBossDefeat(usize, usize),
    /// Constellation: gains `rate`/100 ×mult per **Planet card used**. Base ×1.
    ///
    /// A plain counter — it does **not** scale retroactively, so a Constellation
    /// bought after ten Planets starts at ×1. Contrast
    /// [`MultPlusPerTarotUsedThisRun`](Self::MultPlusPerTarotUsedThisRun).
    GainMultTimesPerPlanetUsed(usize),
    /// Fortune Teller: `+n` mult per Tarot card used **this run**.
    ///
    /// Reads `BuffoonBoard::tarots_used`, a run-wide statistic, rather than a
    /// per-joker accumulator — Balatro's Fortune Teller is retroactive, so one
    /// bought after ten Tarots is immediately worth +10.
    MultPlusPerTarotUsedThisRun(usize),
    /// Flash Card: `+n` mult per shop **reroll**.
    ///
    /// A plain per-joker counter grown on `GrowthEvent::ShopRerolled` — the
    /// Green Joker shape, *not* retroactive: a Flash Card bought after five
    /// rerolls starts at +0, and only climbs on rerolls that happen while it is
    /// held.
    MultPlusPerReroll(usize),
    /// Red Card: `+n` mult per booster pack **skipped**.
    ///
    /// The counter twin of [`MultPlusPerReroll`](Self::MultPlusPerReroll), grown
    /// on `GrowthEvent::PackSkipped` and read as `+n × skipped` at scoring time.
    MultPlusPerPackSkipped(usize),
    /// Hallucination: on each booster pack **opened**, a `numerator`-in-`denominator`
    /// chance (base 1-in-2) to create a Tarot, "(Must have room)".
    ///
    /// A probabilistic *creation*, not a counter — it is rolled immediately when
    /// a pack is opened (through the shared `probability_numerator`, so Oops! All
    /// 6s doubles it), the Riff-Raff shape rather than the Green Joker shape.
    /// Scores nothing on its own.
    CreateTarotOnPackOpen(usize, usize),
    /// Vampire: gains `rate`/100 ×mult per **enhanced card played**, and strips
    /// the enhancement off each one it counts. Base ×1.
    ///
    /// Both halves land in `BuffoonBoard::on_scored`, i.e. **before** the hand
    /// scores. That order is the joker: the ×mult applies to the hand it just
    /// ate, and the eaten enhancement does *not* — a Glass card Vampire eats
    /// gives neither its ×2 nor its chance to break.
    GainMultTimesPerEnhancedPlayed(usize),
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
    /// Seltzer: re-score **every** played card `n` additional times, for the
    /// joker's first `hands` hands — `RetriggerAllPlayedForHands(1, 10)` — after
    /// which it is destroyed.
    ///
    /// The counter is hands *completed*, so the tenth hand still retriggers and
    /// `BuffoonBoard::melt_emptied_jokers` takes the joker straight after it —
    /// the Ice Cream clock, spending retriggers instead of chips.
    RetriggerAllPlayedForHands(usize, usize),
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
            Self::MultPlusChanceDestroyed(mult, numerator, denominator) => {
                write!(
                    f,
                    "MultPlusChanceDestroyed({mult}, {numerator}, {denominator})"
                )
            }
            Self::MultTimesChanceDestroyed(mult, numerator, denominator) => {
                write!(
                    f,
                    "MultTimesChanceDestroyed({mult}, {numerator}, {denominator})"
                )
            }
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
            Self::CashOnRoundEnd(value) => write!(f, "CashOnRoundEnd({value})"),
            Self::CashPerDiscardIfNoneUsed(value) => {
                write!(f, "CashPerDiscardIfNoneUsed({value})")
            }
            Self::CashPerFullDeckRank(value, rank) => {
                write!(f, "CashPerFullDeckRank({value}, {rank})")
            }
            Self::ExtraInterest(value) => write!(f, "ExtraInterest({value})"),
            Self::CashOnFacesDiscarded(cash, min_faces) => {
                write!(f, "CashOnFacesDiscarded({cash}, {min_faces})")
            }
            Self::HandSizeIncrement(value) => write!(f, "HandSizeIncrement({value})"),
            Self::DiscardIncrement(value) => write!(f, "DiscardIncrement({value})"),
            Self::GainHandsLoseDiscardsWhenBlindSelected(value) => {
                write!(f, "GainHandsLoseDiscardsWhenBlindSelected({value})")
            }
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
            Self::LoseMultPerRound(base, per) => write!(f, "LoseMultPerRound({base}, {per})"),
            Self::GainMultTimesPerDiscardedCards(rate, per) => {
                write!(f, "GainMultTimesPerDiscardedCards({rate}, {per})")
            }
            Self::GainMultTimesPerCardAdded(rate) => {
                write!(f, "GainMultTimesPerCardAdded({rate})")
            }
            Self::GainMultTimesPerFaceDestroyed(rate) => {
                write!(f, "GainMultTimesPerFaceDestroyed({rate})")
            }
            Self::GainMultTimesPerEnhancedPlayed(rate) => {
                write!(f, "GainMultTimesPerEnhancedPlayed({rate})")
            }
            Self::GainMultTimesPerPlanetUsed(rate) => {
                write!(f, "GainMultTimesPerPlanetUsed({rate})")
            }
            Self::CreateJokersWhenBlindSelected(n, rarity) => {
                write!(f, "CreateJokersWhenBlindSelected({n}, {rarity:?})")
            }
            Self::CreateTarotOnAceStraight => write!(f, "CreateTarotOnAceStraight"),
            Self::CreateTarotOnLowMoney(n) => write!(f, "CreateTarotOnLowMoney({n})"),
            Self::MultTimesOnRepeatedHandThisRound(n) => {
                write!(f, "MultTimesOnRepeatedHandThisRound({n})")
            }
            Self::MultTimesPerScoredAncientSuit(n) => {
                write!(f, "MultTimesPerScoredAncientSuit({n})")
            }
            Self::GainMultTimesOnNonBossBlindDestroyingJoker(rate) => {
                write!(f, "GainMultTimesOnNonBossBlindDestroyingJoker({rate})")
            }
            Self::DisableBossBlindOnSell => write!(f, "DisableBossBlindOnSell"),
            Self::DisablesAllBossBlinds => write!(f, "DisablesAllBossBlinds"),
            Self::CashOnRoundEndGrowingOnBossDefeat(base, increase) => {
                write!(f, "CashOnRoundEndGrowingOnBossDefeat({base}, {increase})")
            }
            Self::MultPlusPerTarotUsedThisRun(n) => {
                write!(f, "MultPlusPerTarotUsedThisRun({n})")
            }
            Self::MultPlusPerReroll(n) => write!(f, "MultPlusPerReroll({n})"),
            Self::MultPlusPerPackSkipped(n) => write!(f, "MultPlusPerPackSkipped({n})"),
            Self::CreateTarotOnPackOpen(num, den) => {
                write!(f, "CreateTarotOnPackOpen({num}, {den})")
            }
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
            Self::RetriggerAllPlayedForHands(value, hands) => {
                write!(f, "RetriggerAllPlayedForHands({value}, {hands})")
            }
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
        assert_eq!(
            MPip::MultPlusChanceDestroyed(15, 1, 6).to_string(),
            "MultPlusChanceDestroyed(15, 1, 6)"
        );
    }

    #[test]
    fn display__payout_variants() {
        assert_eq!(MPip::CashOnRoundEnd(4).to_string(), "CashOnRoundEnd(4)");
        assert_eq!(
            MPip::CashPerDiscardIfNoneUsed(2).to_string(),
            "CashPerDiscardIfNoneUsed(2)"
        );
        assert_eq!(
            MPip::CashPerFullDeckRank(1, '9').to_string(),
            "CashPerFullDeckRank(1, 9)"
        );
        assert_eq!(MPip::ExtraInterest(1).to_string(), "ExtraInterest(1)");
        assert_eq!(
            MPip::CashOnFacesDiscarded(5, 3).to_string(),
            "CashOnFacesDiscarded(5, 3)"
        );
        assert_eq!(
            MPip::MultTimesChanceDestroyed(3, 1, 1000).to_string(),
            "MultTimesChanceDestroyed(3, 1, 1000)"
        );
    }

    #[test]
    fn display__draw_modifier_variants() {
        assert_eq!(
            MPip::HandSizeIncrement(1).to_string(),
            "HandSizeIncrement(1)"
        );
        assert_eq!(MPip::DiscardIncrement(1).to_string(), "DiscardIncrement(1)");
        assert_eq!(
            MPip::GainHandsLoseDiscardsWhenBlindSelected(3).to_string(),
            "GainHandsLoseDiscardsWhenBlindSelected(3)"
        );
    }
}
