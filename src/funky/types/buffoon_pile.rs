use crate::funky::types::buffoon_card::BuffoonCard;
use crate::funky::types::hands::HandType;
use crate::prelude::{BasicPile, CardError, FrenchRank, Pip, Ranged};
use crate::preludes::funky::{MPip, Score};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BuffoonPile(Vec<BuffoonCard>);

/// Straight- and flush-detection parameters, so hand classification can honour
/// the jokers that bend those rules. The [`Default`] is vanilla Balatro (a
/// gap-free five-card straight, a five-card flush, each suit its own); a board
/// loosens it from its active jokers (Four Fingers → four-card straights &
/// flushes; Shortcut → one-gap straights; Smeared → Hearts≡Diamonds and
/// Spades≡Clubs for flushes) and threads it through the `*_with` detection
/// methods.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandRules {
    /// Max rank gap allowed between adjacent straight cards (1 = gap-free;
    /// Shortcut = 2).
    pub straight_distance: usize,
    /// Connectors a straight needs (4 = five cards; Four Fingers = 3 for four).
    pub straight_connectors: usize,
    /// Cards of one suit a flush needs (5; Four Fingers = 4).
    pub flush_len: usize,
    /// Smeared Joker: Hearts and Diamonds count as one suit, Spades and Clubs as
    /// one suit, when sizing a flush.
    pub smeared: bool,
}

impl Default for HandRules {
    fn default() -> Self {
        Self {
            straight_distance: 1,
            straight_connectors: 4,
            flush_len: 5,
            smeared: false,
        }
    }
}

/// # Dimensions
///
/// Cards can have effects on the following dimensions of play:
///
/// - Enhances a single card
/// - Enhances a certain number of cards
/// -
impl BuffoonPile {
    #[must_use]
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn basic_pile(&self) -> BasicPile {
        self.iter().map(BuffoonCard::basic_card).collect()
    }

    #[must_use]
    pub fn calculate_plus(&self, enhancer: &BuffoonCard) -> Score {
        Score {
            chips: self.calculate_plus_chips(enhancer),
            mult: self.calculate_plus_mult(enhancer),
        }
    }

    #[must_use]
    pub fn calculate_plus_chips(&self, enhancer: &BuffoonCard) -> usize {
        match enhancer.enhancement {
            MPip::ChipsOnFlush(m) => self.funky_num(m, Self::has_flush),
            MPip::ChipsOnPair(m) => self.funky_num(m, Self::has_pair),
            MPip::ChipsOn2Pair(m) => self.funky_num(m, Self::has_2pair),
            MPip::ChipsOnStraight(m) => self.funky_num(m, Self::has_straight),
            MPip::ChipsOnTrips(m) => self.funky_num(m, Self::has_trips),
            // Per-card, rank-conditional (e.g. Odd Todd): sum each played
            // card's own contribution.
            MPip::ChipsPlusOn5Ranks(_, _) => {
                self.iter().map(|c| c.calculate_plus_chips(enhancer)).sum()
            }
            _ => 0,
        }
    }

    /// **DIARY** OK here is where we put our coding to the test. We should be able to take what we
    /// did on the [`BuffoonCard`] side and apply it at the connection level.
    #[must_use]
    pub fn calculate_plus_mult(&self, enhancer: &BuffoonCard) -> usize {
        match enhancer.enhancement {
            // **DIARY** How do we make this simpler?
            MPip::MultPlus(m) => m,
            MPip::MultPlusOnFlush(m) => self.funky_num(m, Self::has_flush),
            MPip::MultPlusOnPair(m) => self.funky_num(m, Self::has_pair),
            MPip::MultPlusOn2Pair(m) => self.funky_num(m, Self::has_2pair),
            MPip::MultPlusOnStraight(m) => self.funky_num(m, Self::has_straight),
            MPip::MultPlusOnTrips(m) => self.funky_num(m, Self::has_trips),
            // Per-card, rank/suit-conditional (e.g. Fibonacci, Even Steven,
            // the suit jokers): sum each played card's own contribution.
            MPip::MultPlusOnSuit(_, _) | MPip::MultPlusOn5Ranks(_, _) => {
                self.iter().map(|c| c.calculate_plus_mult(enhancer)).sum()
            }
            MPip::MultPlusOnUpToXCards(m, x) if self.has_x_or_fewer_cards(x) => m,
            _ => 0,
        }
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the number of connectors based on the distance between ranks. For a collection
    /// of five cards, if 4 is returned, that's a straight. Allows for the scenarios where a
    /// player has the [`Four Fingers Joker`](https://balatrogame.fandom.com/wiki/Four_Fingers)
    /// and only needs a four card straight. It also supports when the player has the
    /// [`Shortcut Joker`](https://balatrogame.fandom.com/wiki/Shortcut).
    ///
    /// ```
    /// use cardpack::preludes::funky::*;
    ///
    /// // Royal Flush
    /// assert_eq!(bcards!("AS KS QS JS TS").connectors(1), 4);
    ///
    ///
    /// assert_eq!(bcards!("AS KS QS JS TS AC 3D 5S").connectors(1), 4);
    #[must_use]
    pub fn connectors(&self, distance: usize) -> usize {
        let mut ranks = self.0.iter().map(|card| card.rank).collect::<Vec<_>>();
        ranks.sort();

        let mut count = 0;
        let mut fopt: Option<Pip> = None;

        for rank in ranks {
            if fopt.is_none() {
                fopt = Some(rank);
            } else if let Some(first) = fopt {
                if (first.distance(&rank) <= distance) && (first.distance(&rank) != 0) {
                    count += 1;
                }
                fopt = Some(rank);
            }
        }
        count
    }

    #[must_use]
    pub fn contains(&self, card: &BuffoonCard) -> bool {
        self.0.contains(card)
    }

    #[must_use]
    pub fn count_largest_same_suit(&self) -> usize {
        self.combos_by_suit().first().map_or(0, BasicPile::len)
    }

    /// [`count_largest_same_suit`](Self::count_largest_same_suit), but under
    /// Smeared it counts the largest *merged* group — Hearts+Diamonds (red) or
    /// Spades+Clubs (black) — so a mixed red (or black) hand can flush.
    #[must_use]
    pub fn count_largest_same_suit_with(&self, rules: HandRules) -> usize {
        if !rules.smeared {
            return self.count_largest_same_suit();
        }
        let red = self
            .iter()
            .filter(|c| matches!(c.suit.index, 'H' | 'D'))
            .count();
        let black = self
            .iter()
            .filter(|c| matches!(c.suit.index, 'S' | 'C'))
            .count();
        red.max(black)
    }

    /// TODO: HACKY
    #[must_use]
    pub fn determine_hand_type(&self) -> HandType {
        self.determine_hand_type_with(HandRules::default())
    }

    /// [`determine_hand_type`](Self::determine_hand_type) under the given
    /// [`HandRules`], so the straight/flush families reflect the board's active
    /// rule-modifier jokers (Four Fingers, Shortcut). Rank-only categories
    /// (pairs, trips, quads, full house) are rule-independent.
    #[must_use]
    pub fn determine_hand_type_with(&self, rules: HandRules) -> HandType {
        if self.has_flush_five_with(rules) {
            HandType::FlushFive
        } else if self.has_5_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.has_flush_house_with(rules) {
            HandType::FlushHouse
        } else if self.has_royal_flush_with(rules) {
            HandType::RoyalFlush
        } else if self.has_straight_flush_with(rules) {
            HandType::StraightFlush
        } else if self.has_4_of_a_kind() {
            HandType::FourOfAKind
        } else if self.has_full_house() {
            HandType::FullHouse
        } else if self.has_flush_with(rules) {
            HandType::Flush
        } else if self.has_straight_with(rules) {
            HandType::Straight
        } else if self.has_trips() {
            HandType::ThreeOfAKind
        } else if self.has_2pair() {
            HandType::TwoPair
        } else if self.has_pair() {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }

    #[must_use]
    pub fn draw(&mut self, n: usize) -> Option<Self> {
        let mut pile = Self::default();
        for _ in 0..n {
            if let Some(card) = self.pop() {
                pile.push(card);
            } else {
                return None;
            }
        }
        Some(pile)
    }

    pub fn draw_first(&mut self) -> Option<BuffoonCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// **DIARY** This is where TDD kinda breaks down for me in Rust. I can't
    /// write a failing test until I've written the signature for the method
    /// to test. Heck, might as well take a stab at writing it altogether. It
    /// does feel good when I do and get it write, given how insecure I am
    /// with the language.
    ///
    /// However, before I can write this easily, I need a `BuffoonPile::from_str() `
    /// method. I am addicted to them in these style of libraries. I want to manifest state
    /// as easily as possible.
    #[must_use]
    pub fn enhance(&self, enhancer: BuffoonCard) -> Self {
        self.iter().map(|c| c.enhance(enhancer)).collect()
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    #[must_use]
    pub fn forgiving_from_str(index: &str) -> Self {
        Self::from_str(index).unwrap_or_else(|_| Self::default())
    }

    #[must_use]
    pub fn free(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn funky_num(&self, num: usize, func: fn(&Self) -> bool) -> usize {
        if func(self) { num } else { 0 }
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<&BuffoonCard> {
        self.0.get(position)
    }

    #[must_use]
    pub fn has_flush(&self) -> bool {
        self.has_flush_with(HandRules::default())
    }

    /// [`has_flush`](Self::has_flush) under the given [`HandRules`] (Four Fingers
    /// lowers the required same-suit count to 4; Smeared merges suit pairs).
    #[must_use]
    pub fn has_flush_with(&self, rules: HandRules) -> bool {
        self.count_largest_same_suit_with(rules) >= rules.flush_len
    }

    #[must_use]
    pub fn has_flush_five(&self) -> bool {
        self.has_flush_five_with(HandRules::default())
    }

    #[must_use]
    pub fn has_flush_five_with(&self, rules: HandRules) -> bool {
        self.has_flush_with(rules) && self.has_x_of_a_kind(5)
    }

    #[must_use]
    pub fn has_flush_house(&self) -> bool {
        self.has_flush_house_with(HandRules::default())
    }

    #[must_use]
    pub fn has_flush_house_with(&self, rules: HandRules) -> bool {
        self.has_flush_with(rules) && self.has_full_house()
    }

    #[must_use]
    pub fn has_full_house(&self) -> bool {
        let combos = self.combos_by_rank();
        match combos.first() {
            None => false,
            Some(combo) => {
                if combo.len() < 3 {
                    return false;
                }
                combos
                    .second()
                    .is_some_and(|first_combo| first_combo.len() >= 2)
            }
        }
    }

    #[must_use]
    pub fn has_x_of_a_kind(&self, x: usize) -> bool {
        self.combos_by_rank()
            .first()
            .is_some_and(|combo| combo.len() >= x)
    }

    #[must_use]
    pub fn has_4_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(4)
    }

    #[must_use]
    pub fn has_5_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(5)
    }

    /// **DIARY** This is where I am hoping that the synergy between the `BasicPile` code can
    /// be leveraged to quickly enable `Jokers` that are triggered based on the state of the pile
    /// of cards.
    ///
    /// OK, if these tests pass right out of the box, I will be very happy.
    ///
    /// **FIVE SECONDS LATER**
    ///
    /// I am very happy.
    ///
    /// The basic logic is simple. If there are fewer ranks in a pile of cards than the total
    /// number of cards, there must be at least one pair.
    #[must_use]
    pub fn has_pair(&self) -> bool {
        self.has_x_of_a_kind(2)
    }

    #[must_use]
    pub fn has_2pair(&self) -> bool {
        self.combos_by_rank()
            .second()
            .is_some_and(|combo| combo.len() >= 2)
    }

    #[must_use]
    pub fn has_x_or_fewer_cards(&self, x: usize) -> bool {
        self.len() <= x
    }

    /// TODO: HACKY
    #[must_use]
    pub fn has_royal_flush(&self) -> bool {
        self.has_royal_flush_with(HandRules::default())
    }

    #[must_use]
    pub fn has_royal_flush_with(&self, rules: HandRules) -> bool {
        self.basic_pile()
            .sorted()
            .first()
            .is_some_and(|card| self.has_straight_flush_with(rules) && card.rank == FrenchRank::ACE)
    }

    /// Whether the pile forms a straight. Vanilla Balatro (gap-free, five cards)
    /// by default; [`has_straight_with`](Self::has_straight_with) honours the
    /// jokers that loosen it (Four Fingers, Shortcut).
    #[must_use]
    pub fn has_straight(&self) -> bool {
        self.has_straight_with(HandRules::default())
    }

    #[must_use]
    pub fn has_straight_with(&self, rules: HandRules) -> bool {
        self.connectors(rules.straight_distance) >= rules.straight_connectors
    }

    #[must_use]
    pub fn has_straight_flush(&self) -> bool {
        self.has_straight_flush_with(HandRules::default())
    }

    #[must_use]
    pub fn has_straight_flush_with(&self, rules: HandRules) -> bool {
        self.has_straight_with(rules) && self.has_flush_with(rules)
    }

    #[must_use]
    pub fn has_trips(&self) -> bool {
        self.has_x_of_a_kind(3)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BuffoonCard> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<BuffoonCard> {
        self.0.pop()
    }

    pub fn push(&mut self, card: BuffoonCard) {
        self.0.push(card);
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn remove(&mut self, position: usize) -> BuffoonCard {
        self.0.remove(position)
    }

    /// Shuffles the pile in place using the process default RNG (`rand::rng()`).
    /// For deterministic shuffling, use [`shuffle_with_seed`](Self::shuffle_with_seed).
    pub fn shuffle(&mut self) {
        self.shuffle_with_rng(&mut rng());
    }

    /// Returns a new shuffled version of the pile.
    ///
    /// For deterministic shuffling, use
    /// [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the pile in place deterministically from a `u64` seed.
    ///
    /// Uses [`rand::rngs::StdRng`] internally. Same seed produces the same
    /// permutation **within one `rand` major version**; a `rand` upgrade may
    /// change the result. For cross-version reproducibility, pass a portable
    /// RNG (e.g. `ChaCha8Rng`) to [`shuffle_with_rng`](Self::shuffle_with_rng).
    /// This is the entry point a solver uses to make deals reproducible.
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new pile shuffled deterministically from a `u64` seed.
    ///
    /// See [`shuffle_with_seed`](Self::shuffle_with_seed) for the portability
    /// caveat.
    ///
    /// ```
    /// use cardpack::preludes::funky::*;
    ///
    /// let deck = Deck::basic_buffoon_pile();
    /// assert_eq!(deck.shuffled_with_seed(42), deck.shuffled_with_seed(42));
    /// ```
    #[must_use]
    pub fn shuffled_with_seed(&self, seed: u64) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_seed(seed);
        pile
    }

    /// Shuffles the pile in place using the caller's RNG.
    ///
    /// Generic over any `R: Rng + ?Sized`. The seed-based methods are sugar
    /// over this primitive — pass your own RNG (e.g. `ChaCha8Rng`) for
    /// algorithm-stable reproducibility across `rand` major-version bumps.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// Returns a new pile shuffled using the caller's RNG.
    #[must_use]
    pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_rng(rng);
        pile
    }

    pub fn sort(&mut self) {
        self.0.sort();
    }

    pub fn sort_by_rank(&mut self) {
        self.0.sort_by_key(|b| core::cmp::Reverse(b.rank));
    }

    #[must_use]
    pub fn sorted(&self) -> Self {
        let mut pile = self.clone();
        pile.sort();
        pile
    }

    #[must_use]
    pub fn sorted_by_rank(self) -> Self {
        let mut pile = self;
        pile.sort_by_rank();
        pile
    }

    /// Returns a reference to the internal vector of the struct.
    #[must_use]
    pub fn v(&self) -> &Vec<BuffoonCard> {
        &self.0
    }
}

impl Display for BuffoonPile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<&[BuffoonCard]> for BuffoonPile {
    fn from(slice: &[BuffoonCard]) -> Self {
        Self(slice.to_vec())
    }
}

impl From<Vec<BuffoonCard>> for BuffoonPile {
    fn from(v: Vec<BuffoonCard>) -> Self {
        Self(v)
    }
}

impl FromStr for BuffoonPile {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split_whitespace()
            .map(BuffoonCard::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(cards))
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// region Iterator
impl FromIterator<BuffoonCard> for BuffoonPile {
    fn from_iter<T: IntoIterator<Item = BuffoonCard>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a BuffoonPile {
    type Item = &'a BuffoonCard;
    type IntoIter = std::slice::Iter<'a, BuffoonCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for BuffoonPile {
    type Item = BuffoonCard;
    type IntoIter = std::vec::IntoIter<BuffoonCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Ranged for BuffoonPile {
    fn my_basic_pile(&self) -> BasicPile {
        self.basic_pile()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__buffoon_pile_tests {
    use super::*;
    use crate::preludes::funky::*;
    use rstest::rstest;

    #[test]
    fn basic_pile() {
        let pile = bcards!("AS KS QS JS TS");

        assert_eq!(pile.basic_pile().to_string(), "A♠ K♠ Q♠ J♠ T♠");
    }

    #[test]
    fn calculate_plus_chips() {
        assert_eq!(
            bcards!("AS AD QS JS TS").calculate_plus_chips(&bcard!(SLY)),
            50
        );
        assert_eq!(
            bcards!("AS AD AH JS TS").calculate_plus_chips(&bcard!(SLY)),
            50
        );
        assert_eq!(
            bcards!("AS KD QH JS TS").calculate_plus_chips(&bcard!(SLY)),
            0
        );

        assert_eq!(
            bcards!("AS AD AH JS TS").calculate_plus_chips(&bcard!(WILY)),
            100
        );
        assert_eq!(
            bcards!("AS AD AH AC TS").calculate_plus_chips(&bcard!(WILY)),
            100
        );
        assert_eq!(
            bcards!("AS KD QH JS TS").calculate_plus_chips(&bcard!(WILY)),
            0
        );
    }

    #[test]
    fn calculate_plus_mult__fibonacci_sums_across_played_cards() {
        // Fibonacci: +8 mult per played A/2/3/5/8. All five match -> 40.
        assert_eq!(
            bcards!("AS 2D 3C 5H 8S").calculate_plus_mult(&FIBONACCI),
            40
        );
        // Only A and 3 match (K/Q/J do not) -> 16.
        assert_eq!(
            bcards!("AS 3D KC QH JS").calculate_plus_mult(&FIBONACCI),
            16
        );
        assert_eq!(bcards!("KS QD JC TH 9S").calculate_plus_mult(&FIBONACCI), 0);
    }

    #[test]
    fn calculate_plus_mult__even_steven_sums_across_played_cards() {
        // Even Steven: +4 mult per even card (T/8/6/4/2). All five even -> 20.
        assert_eq!(
            bcards!("TS 8D 6C 4H 2S").calculate_plus_mult(&EVEN_STEVEN),
            20
        );
        // Only the T is even -> 4.
        assert_eq!(
            bcards!("TS 9D 7C 5H 3S").calculate_plus_mult(&EVEN_STEVEN),
            4
        );
    }

    #[test]
    fn calculate_plus_chips__odd_todd_sums_across_played_cards() {
        // Regression: Odd Todd (+31 chips per odd card) must score through the
        // pile, not just per card. Five odd (A/9/7/5/3) -> 155.
        assert_eq!(
            bcards!("AS 9D 7C 5H 3S").calculate_plus_chips(&ODD_TODD),
            155
        );
        // Even/face cards contribute nothing.
        assert_eq!(bcards!("TS 8D 6C 4H 2S").calculate_plus_chips(&ODD_TODD), 0);
    }

    #[test]
    fn shuffled_with_seed__deterministic() {
        let deck = Deck::basic_buffoon_pile();
        let a = deck.shuffled_with_seed(42);
        let b = deck.shuffled_with_seed(42);
        assert_eq!(a, b, "same seed must produce identical permutation");
    }

    #[test]
    fn shuffled_with_seed__different_seeds_differ() {
        let deck = Deck::basic_buffoon_pile();
        assert_ne!(deck.shuffled_with_seed(1), deck.shuffled_with_seed(2));
    }

    #[test]
    fn shuffled_with_seed__preserves_all_cards() {
        let deck = Deck::basic_buffoon_pile();
        let shuffled = deck.shuffled_with_seed(0x00C0_FFEE);
        assert_eq!(shuffled.len(), deck.len());
        // Same multiset of cards, just reordered.
        assert_eq!(shuffled.sorted(), deck.sorted());
    }

    /// **DIARY** The unit test code that `CoPilot` generates is baffling to me sometimes. Complete
    /// nonsense:
    ///
    /// ```txt
    /// let pile = BuffoonPile::from(vec![
    ///     BuffoonCard::new(1, 2),
    ///     BuffoonCard::new(3, 4),
    /// ]);
    /// let enhancer = BuffoonCard::new(5, 6);
    /// assert_eq!(pile.calculate_plus_mult(enhancer), 44);
    /// ```
    #[test]
    fn calculate_plus_mult() {
        assert_eq!(
            bcards!("AD KD QD JD TD").calculate_plus_mult(&bcard!(GREEDY)),
            15
        );
        assert_eq!(
            bcards!("AS KD QS JD TD").calculate_plus_mult(&bcard!(GREEDY)),
            9
        );
        assert_eq!(
            bcards!("AS KS QS JD TD").calculate_plus_mult(&bcard!(GREEDY)),
            6
        );
        assert_eq!(
            bcards!("AS KS QS JS TS").calculate_plus_mult(&bcard!(GREEDY)),
            0
        );

        assert_eq!(
            bcards!("AH KH QH JH TH").calculate_plus_mult(&bcard!(LUSTY)),
            15
        );
        assert_eq!(
            bcards!("AS KH QS JH TS").calculate_plus_mult(&bcard!(LUSTY)),
            6
        );
        assert_eq!(
            bcards!("AS KS QS JS TS").calculate_plus_mult(&bcard!(LUSTY)),
            0
        );

        assert_eq!(
            bcards!("AS KS QS JS TS").calculate_plus_mult(&bcard!(WRATHFUL)),
            15
        );
        assert_eq!(
            bcards!("AS KD QD JS 2S").calculate_plus_mult(&bcard!(WRATHFUL)),
            9
        );
        assert_eq!(
            bcards!("AD KD QD JD 2D").calculate_plus_mult(&bcard!(WRATHFUL)),
            0
        );

        assert_eq!(
            bcards!("AC KC QC JC TC").calculate_plus_mult(&bcard!(GLUTTONOUS)),
            15
        );
        assert_eq!(
            bcards!("AC KD QC JS 2C").calculate_plus_mult(&bcard!(GLUTTONOUS)),
            9
        );
        assert_eq!(
            bcards!("AD KD QD JD 2D").calculate_plus_mult(&bcard!(GLUTTONOUS)),
            0
        );

        // Hands
        assert_eq!(
            bcards!("AS AD QS JS TS").calculate_plus_mult(&bcard!(JOLLY)),
            8
        );
        assert_eq!(
            bcards!("AS AD AH JS TS").calculate_plus_mult(&bcard!(JOLLY)),
            8
        );
        assert_eq!(
            bcards!("AS KD QH JS TS").calculate_plus_mult(&bcard!(JOLLY)),
            0
        );
        assert_eq!(
            bcards!("AS AD AH JS TS").calculate_plus_mult(&bcard!(ZANY)),
            12
        );
        assert_eq!(
            bcards!("AS AD QH JS TS").calculate_plus_mult(&bcard!(ZANY)),
            0
        );
        assert_eq!(
            bcards!("AS AD AH JS JD").calculate_plus_mult(&bcard!(MAD)),
            10
        );
        assert_eq!(
            bcards!("AD KD QD JD TD").calculate_plus_mult(&bcard!(CRAZY)),
            12
        );
        assert_eq!(
            bcards!("AD KD QD JD 9D").calculate_plus_mult(&bcard!(CRAZY)),
            0
        );
        assert_eq!(
            bcards!("AS JS QS 2S 8S").calculate_plus_mult(&bcard!(DROLL)),
            10
        );
        assert_eq!(
            bcards!("AS JS QS 2S 8H").calculate_plus_mult(&bcard!(DROLL)),
            0
        );
    }

    #[rstest]
    #[case("9S 9D 9C 9H 9D", HandType::FiveOfAKind)]
    #[case("9D 9D 9D 9D 9D", HandType::FlushFive)] //
    #[case("9S 9S 9S 8S 8S", HandType::FlushHouse)]
    #[case("QS AS KS JS TS", HandType::RoyalFlush)]
    #[case("AS KS QS JS TS", HandType::RoyalFlush)]
    #[case("AD KD QD JD TD", HandType::RoyalFlush)]
    #[case("9S KS QS JS TS", HandType::StraightFlush)]
    #[case("9S KS QS JS TS", HandType::StraightFlush)]
    #[case("9S 9D 9C 9H TS", HandType::FourOfAKind)]
    #[case("9S 9S 9S 8S 8D", HandType::FullHouse)]
    #[case("9S 9S 9S 8S 7S", HandType::Flush)]
    #[case("9S KS QS JD TS", HandType::Straight)]
    #[case("9S KS QC JS TS", HandType::Straight)]
    #[case("9S 9S 9S 7S 6C", HandType::ThreeOfAKind)]
    #[case("9S 9S 7D 7S 6C", HandType::TwoPair)]
    #[case("9S 9S 7D 3S 6C", HandType::Pair)]
    fn determine_hand_type(#[case] input: &str, #[case] expected: HandType) {
        assert_eq!(bcards!(input).determine_hand_type(), expected);
    }

    /// *DIARY** I really don't want to write any more tests.
    #[rstest]
    #[case("AD KS QS JS TS", HandType::RoyalFlush)]
    #[case("9S KS QS JS TS", HandType::RoyalFlush)]
    fn determine_hand_type__negative(#[case] input: &str, #[case] expected: HandType) {
        assert_ne!(bcards!(input).determine_hand_type(), expected);
    }

    #[test]
    fn free() {
        let mut pile = BuffoonPile::new_with_capacity(10);
        assert_eq!(pile.free(), 10);
        pile.push(bcard!(AS));
        assert_eq!(pile.free(), 9);
        pile.push(bcard!(KS));
        assert_eq!(pile.free(), 8);
    }

    #[test]
    fn fun_connectors() {
        assert_eq!(bcards!("AS KS QS JS TS").connectors(1), 4);
        assert_eq!(bcards!("AS KS QS JS TS AC 3D 5S").connectors(1), 4);
        assert_eq!(bcards!("AS KS QS JS TS").connectors(2), 4);
        assert_eq!(bcards!("AS KS KD JS TS").connectors(1), 2);
        assert_eq!(bcards!("AS KS KD JS TS").connectors(2), 3);
        assert_eq!(bcards!("JD QS TC 9S KH").connectors(1), 4);
        assert_eq!(bcards!("JD QS TC 8S KH").connectors(2), 4);
        assert_eq!(bcards!("JD QS TC 8S KH").connectors(2), 4);
        assert_eq!(bcards!("JD JC JH 8S 8H").connectors(1), 0);
    }

    #[test]
    fn funky_plus_mult() {
        assert_eq!(
            bcards!("AS KS QS JS TS").funky_num(4, BuffoonPile::has_pair),
            0
        );
        assert_eq!(
            bcards!("AS KS JD JS TS").funky_num(4, BuffoonPile::has_pair),
            4
        );
        assert_eq!(
            bcards!("AS KS AD AC TS").funky_num(4, BuffoonPile::has_trips),
            4
        );
    }

    #[test]
    fn has_flush() {
        assert!(bcards!("AS KS QS JS TS").has_flush());
        assert!(!bcards!("AS AD QS QC TS").has_flush());
    }

    #[test]
    fn has_flush_house() {
        let hand = bcards!("9S 9S 9S 8S 8S");

        assert!(hand.has_flush());
        assert!(hand.has_full_house());
        assert!(!bcards!("AS AD QS QC TS").has_flush_house());
    }

    #[test]
    fn has_full_house() {
        assert!(bcards!("AS AD QS QH QC").has_full_house());
        assert!(bcards!("AS AS QS QS QS").has_full_house());
        assert!(!bcards!("AS AS QS QS JS").has_full_house());
    }

    #[test]
    fn has_pair() {
        assert!(bcards!("AS AD QS JS TS").has_pair());
        assert!(bcards!("AS AD QS QC TS").has_pair());
        assert!(!bcards!("AS KS QS JS TS").has_pair());
    }

    #[test]
    fn has_royal_flush() {
        assert!(bcards!("AS KS QS JS TS").has_royal_flush());
        assert!(!bcards!("9S KS QS JS TS").has_royal_flush());
    }

    #[test]
    fn has_straight_flush() {
        assert!(bcards!("AS KS QS JS TS").has_straight_flush());
        assert!(bcards!("9S KS QS JS TS").has_straight_flush());
        assert!(!bcards!("8S KS QS JS TS").has_straight_flush());
    }

    #[test]
    fn has_2pair() {
        assert!(bcards!("AS AD QS QC TS").has_2pair());
        assert!(!bcards!("AS AD QS JS TS").has_2pair());
        assert!(!bcards!("AS KS QS JS TS").has_2pair());
    }

    #[test]
    fn has_trips() {
        assert!(bcards!("AS AD AH JS TS").has_trips());
        assert!(bcards!("AS AD AH AC TS").has_trips());
        assert!(bcards!("AS AD QS QC QH").has_trips());
        assert!(bcards!("AS AD 9D QS QC JH 9C 8S 9S").has_trips());
        assert!(bcards!("AS AC AH AD 9D QS QC JH 9C 8S 9S").has_trips());
        assert!(!bcards!("AS AD QS QC JH JC").has_trips());
        assert!(!bcards!("AS KS QS JS TS").has_pair());
    }

    #[test]
    fn map_by_rank() {
        assert_eq!(
            "9♥ 9♦ 9♣, Q♠ Q♦, T♠, J♠",
            bcards!("9C 9H 9D QS QD JS TS")
                .combos_by_rank()
                .sort_internal()
                .to_string()
        );
    }

    #[test]
    fn from__buffoon_card_slice() {
        let pile = BuffoonPile::from(&Joker::COMMON_JOKERS[..]);

        assert_eq!(pile, Joker::pile_common());
    }

    #[test]
    fn from_str() {
        let pile = BuffoonPile::from_str("AS KS QS JS TS").unwrap();
        assert_eq!(pile.to_string(), "AS KS QS JS TS");
        assert_eq!(bcards!("AS KS QS JS TS").to_string(), "AS KS QS JS TS");
    }
}

// region garbage

// so bad:
//
// please write unit tests for all the functions in the BuffoonPile struct
// mod tests {
//     use super::*;
//     use crate::funky::types::buffoon_card::BuffoonCard;
//
//     #[test]
//     fn test_v() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_clear() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.clear();
//         assert!(pile.is_empty());
//     }
//
//     #[test]
//     fn test_contains() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert!(pile.contains(&BuffoonCard::new(1)));
//         assert!(!pile.contains(&BuffoonCard::new(3)));
//     }
//
//     #[test]
//     fn test_draw() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         let drawn = pile.draw(2).unwrap();
//         assert_eq!(drawn.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2)]);
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_draw_first() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.draw_first(), Some(BuffoonCard::new(1)));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_extend() {
//         let mut pile1 = BuffoonPile(vec![BuffoonCard::new(1)]);
//         let pile2 = BuffoonPile(vec![BuffoonCard::new(2)]);
//         pile1.extend(&pile2);
//         assert_eq!(pile1.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_get() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.get(1), Some(&BuffoonCard::new(2)));
//         assert_eq!(pile.get(2), None);
//     }
//
//     #[test]
//     fn test_is_empty() {
//         let pile = BuffoonPile(vec![]);
//         assert!(pile.is_empty());
//         let pile = BuffoonPile(vec![BuffoonCard::new(1)]);
//         assert!(!pile.is_empty());
//     }
//
//     #[test]
//     fn test_iter() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         let mut iter = pile.iter();
//         assert_eq!(iter.next(), Some(&BuffoonCard::new(1)));
//         assert_eq!(iter.next(), Some(&BuffoonCard::new(2)));
//         assert_eq!(iter.next(), None);
//     }
//
//     #[test]
//     fn test_len() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.len(), 2);
//     }
//
//     #[test]
//     fn test_pop() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.pop(), Some(BuffoonCard::new(2)));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_push() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1)]);
//         pile.push(BuffoonCard::new(2));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_reverse() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.reverse();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_remove() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.remove(0), BuffoonCard::new(1));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_shuffle() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         pile.shuffle();
//         assert_eq!(pile.len(), 3);
//     }
//
//     #[test]
//     fn test_shuffled() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         let shuffled = pile.shuffled();
//         assert_eq!(shuffled.len(), 3);
//     }
//
//     #[test]
//     fn test_sort() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(3), BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.sort();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//     }
//
//     #[test]
//     fn test_sort_by_rank() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(3), BuffoonCard::new(2)]);
//         pile.sort_by_rank();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_sorted() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(3), BuffoonCard::new(1), BuffoonCard::new(2)]);
//         let sorted = pile.sorted();
//         assert_eq!(sorted.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//     }
//
//     #[test]
//     fn test_sorted_by_rank() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(3), BuffoonCard::new(2)]);
//         let sorted = pile.sorted_by_rank();
//         assert_eq!(sorted.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
// }

// endregion
