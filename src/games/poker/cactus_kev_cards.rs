use crate::cards::card_error::CardError;
use crate::cards::decks::standard52;
use crate::games::poker::alt::lookups;
use crate::games::poker::cactus_kev_card::{ckc, CKC, SUITS_FILTER};
use crate::{Pile, Standard52};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt;

pub const POSSIBLE_COMBINATIONS: usize = 7937;

pub type HandRankValue = u16;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandRank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
    Invalid,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CactusKevCards(Vec<CKC>);

impl CactusKevCards {
    #[must_use]
    pub fn new(v: Vec<CKC>) -> CactusKevCards {
        CactusKevCards(v)
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    pub fn from_index(i: &'static str) -> Result<CactusKevCards, CardError> {
        let pile = Standard52::pile_from_index(i);

        if pile.is_err() {
            return Err(CardError::InvalidCard);
        }

        let mut cards = CactusKevCards::default();
        for card in pile.unwrap() {
            cards.push(ckc::from_card(&card));
        }
        Ok(cards)
    }

    /// # Panics
    ///
    /// Only if `Standard52` is very foobared.
    #[must_use]
    pub fn deal5(standard52: &mut standard52::Standard52) -> CactusKevCards {
        let pile = standard52.draw(5).unwrap();
        let mut cards = CactusKevCards::default();
        for card in pile {
            cards.push(ckc::from_card(&card));
        }
        cards
    }

    #[must_use]
    pub fn eval_5cards(&self) -> HandRankValue {
        if !self.is_complete_hand() {
            return 0;
        }
        let i = self.or_rank_bits();

        if self.is_flush() {
            return lookups::FLUSHES[i] as HandRankValue;
        }

        let s = CactusKevCards::unique5(i);
        if s != 0 {
            return s;
        }

        // It's not a flush and the cards aren't unique (straight or high card).
        self.last_pass()
    }

    fn last_pass(&self) -> HandRankValue {
        let i = CactusKevCards::find_it(self.multiply_primes());
        lookups::VALUES[i] as HandRankValue
    }

    /// Based on [this](https://github.com/vsupalov/pokereval-rs/blob/d244030715560dbae38c68dbcd09244d5285b518/src/original.rs#L6)
    /// which is in turn based on [find fast method](http://suffe.cool/poker/code/pokerlib.c) from Cactus Kev's original C code.
    ///
    /// TODO: Refactor to [Rust-PHF](https://github.com/rust-phf/rust-phf)
    #[must_use]
    pub fn find_it(key: usize) -> usize {
        let mut low = 0;
        let mut high = 4887;
        let mut mid;

        while low <= high {
            mid = (high + low) >> 1; // divide by two

            let product = lookups::PRODUCTS[mid] as usize;
            match key.cmp(&product) {
                Ordering::Less => high = mid - 1,
                Ordering::Greater => low = mid + 1,
                Ordering::Equal => return mid,
            }
        }
        0
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CKC> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CKC> {
        self.0.iter()
    }

    #[must_use]
    pub fn unique5(index: usize) -> HandRankValue {
        if index > POSSIBLE_COMBINATIONS {
            0
        } else {
            lookups::UNIQUE_5[index] as HandRankValue
        }
    }

    /// # Errors
    ///
    /// Will return `CardError::NotEnoughCards` if there are less than five cards.
    /// Will return `CardError::TooManyCards` if there are more than five cards.
    ///
    /// # Panics
    ///
    /// Shouldn't be able to panic. (fingers crossed)
    ///
    pub fn to_five_array(&self) -> Result<[CKC; 5], CardError> {
        match self.len() {
            0..=4 => Err(CardError::NotEnoughCards),
            5 => Ok(self.0.clone().try_into().unwrap()),
            _ => Err(CardError::TooManyCards),
        }
    }

    #[must_use]
    pub fn is_complete_hand(&self) -> bool {
        self.len() == 5
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn is_flush(&self) -> bool {
        if !self.is_complete_hand() {
            return false;
        }
        (self.0[0] & self.0[1] & self.0[2] & self.0[3] & self.0[4] & SUITS_FILTER) != 0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn multiply_primes(&self) -> usize {
        let mut r: usize = 1;
        for p in self.primes() {
            let q = p as usize;
            r *= q;
        }
        r
    }

    /// Returns a value that is made up of performing an or operation on all of the
    /// rank bit flags of the `CactusKevCard`.
    #[must_use]
    pub fn or_rank_bits(&self) -> usize {
        if !self.is_complete_hand() {
            return 0;
        }
        ((self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4]) as usize) >> 16
    }

    /// Returns a vector of all the prime bits of the CKC.
    #[must_use]
    pub fn primes(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for c in self.iter() {
            v.push(c & 0xff);
        }
        v
    }

    pub fn push(&mut self, ckc: CKC) {
        self.0.push(ckc);
    }

    #[must_use]
    pub fn sort(&self) -> CactusKevCards {
        let mut cards = self.clone();
        cards.sort_in_place();
        cards
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

    #[must_use]
    pub fn to_pile(&self) -> Pile {
        let mut pile = Pile::default();

        for card in &self.0 {
            pile.push(ckc::to_card(card));
        }

        pile
    }

    /// Takes in a calculated `HandRankValue` and returns the `HandRank`.
    ///
    /// 7462 possible combination of hands:
    ///
    ///   10 straight-flushes
    ///  156 four of a kinds
    ///  156 full houses
    /// 1277 flushes
    ///   10 straights
    ///  858 three of a kinds
    ///  858 two pairs
    /// 2860 pairs
    /// 1277 high cards
    #[must_use]
    pub fn get_hand_rank(hrv: &HandRankValue) -> HandRank {
        match *hrv {
            1..=10 => HandRank::StraightFlush,
            11..=166 => HandRank::FourOfAKind,
            167..=322 => HandRank::FullHouse,
            323..=1599 => HandRank::Flush,
            1600..=1609 => HandRank::Straight,
            1610..=2467 => HandRank::ThreeOfAKind,
            2468..=3325 => HandRank::TwoPair,
            3326..=6185 => HandRank::Pair,
            6186..=7462 => HandRank::HighCard,
            _ => HandRank::Invalid,
        }
    }
}

impl Default for CactusKevCards {
    fn default() -> Self {
        CactusKevCards::new(Vec::new())
    }
}

impl fmt::Display for CactusKevCards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pile().to_symbol_index())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_cards_tests {
    use super::*;
    use crate::games::poker::alt::original::cactus_kevs_original_eval_5cards;
    use rstest::rstest;

    #[test]
    fn deal5() {
        let standard52 = &mut standard52::Standard52::new_shuffled();
        let hand1 = CactusKevCards::deal5(standard52);
        let hand2 = CactusKevCards::deal5(standard52);

        println!(
            "{} {} {} {}",
            hand1,
            hand1.eval_5cards(),
            hand2,
            hand2.eval_5cards()
        );
    }

    #[test]
    fn eval_5cards() {
        assert_eq!(
            1,
            CactusKevCards::from_index("AS KS QS JS TS")
                .unwrap()
                .eval_5cards()
        );
        assert_eq!(
            1,
            CactusKevCards::from_index("AH KH QH JH TH")
                .unwrap()
                .eval_5cards()
        );
    }

    #[test]
    fn eval_5cards__pair() {
        let cards = CactusKevCards::from_index("AS AH QS JS TS").unwrap();

        let expected = cactus_kevs_original_eval_5cards(
            cards.get(0).unwrap(),
            cards.get(1).unwrap(),
            cards.get(2).unwrap(),
            cards.get(3).unwrap(),
            cards.get(4).unwrap(),
        );

        assert_eq!(expected, cards.eval_5cards());
    }

    #[test]
    fn eval_5cards__invalid_index() {
        let hand = CactusKevCards::from_index("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(0, hand.eval_5cards());
    }

    #[test]
    fn into_five_array() {
        let ckc = CactusKevCards::from_index("AS KS QS JS TS").unwrap();

        let a = ckc.to_five_array().unwrap();

        assert_eq!(a.len(), 5);
        assert_eq!(ckc.get(0).unwrap(), &a[0]);
        assert_eq!(ckc.get(1).unwrap(), &a[1]);
        assert_eq!(ckc.get(2).unwrap(), &a[2]);
        assert_eq!(ckc.get(3).unwrap(), &a[3]);
        assert_eq!(ckc.get(4).unwrap(), &a[4]);
    }

    #[test]
    fn is_flush() {
        assert!(CactusKevCards::from_index("AS KS QS JS TS")
            .unwrap()
            .is_flush());
        assert!(!CactusKevCards::from_index("AS KS QS JS TC")
            .unwrap()
            .is_flush());
    }

    #[test]
    fn or_shift_16() {
        let ckc = CactusKevCards::from_index("AS KS QS JS TS").unwrap();

        assert_eq!(ckc.or_rank_bits(), 7936);
    }

    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRank::StraightFlush)]
    #[case("A♣ 2♣ 3♣ 4♣ 5♣", 10, HandRank::StraightFlush)]
    #[case("A♠ A♥ A♦ A♣ K♠", 11, HandRank::FourOfAKind)]
    #[case("2♠ 2♥ 2♦ 2♣ 3♠", 166, HandRank::FourOfAKind)]
    #[case("A♠ A♥ A♦ K♠ K♦", 167, HandRank::FullHouse)]
    #[case("2♠ 2♥ 2♦ 3♠ 3♦", 322, HandRank::FullHouse)]
    #[case("A♠ K♠ Q♠ J♠ 9♠", 323, HandRank::Flush)]
    #[case("2♣ 3♣ 4♣ 5♣ 7♣", 1599, HandRank::Flush)]
    #[case("A♣ K♠ Q♠ J♠ T♠", 1600, HandRank::Straight)]
    #[case("A♥ 2♣ 3♣ 4♣ 5♣", 1609, HandRank::Straight)]
    #[case("A♠ A♥ A♦ K♠ Q♣", 1610, HandRank::ThreeOfAKind)]
    #[case("2♠ 2♥ 2♦ 3♠ 4♣", 2467, HandRank::ThreeOfAKind)]
    #[case("A♠ A♥ K♦ K♠ Q♣", 2468, HandRank::TwoPair)]
    #[case("3♠ 3♥ 2♦ 2♠ 4♣", 3325, HandRank::TwoPair)]
    #[case("A♠ A♥ K♠ Q♠ J♠", 3326, HandRank::Pair)]
    #[case("2♠ 2♥ 3♠ 4♠ 5♠", 6185, HandRank::Pair)]
    #[case("A♠ K♠ Q♠ J♠ 9♣", 6186, HandRank::HighCard)]
    #[case("2♣ 3♣ 4♣ 5♥ 7♣", 7462, HandRank::HighCard)]
    fn get_hand_rank(
        #[case] index: &'static str,
        #[case] expected_hand_rank_value: HandRankValue,
        #[case] hand_rank: HandRank,
    ) {
        let hand = CactusKevCards::from_index(index).unwrap();

        let actual_hand_rank_value = hand.eval_5cards();

        assert_eq!(expected_hand_rank_value, actual_hand_rank_value);
        assert_eq!(
            hand_rank,
            CactusKevCards::get_hand_rank(&expected_hand_rank_value)
        );
    }

    #[test]
    fn get_hand_rank__invalid_index() {
        let hand = CactusKevCards::from_index("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(
            HandRank::Invalid,
            CactusKevCards::get_hand_rank(&hand.eval_5cards())
        );
    }
}
