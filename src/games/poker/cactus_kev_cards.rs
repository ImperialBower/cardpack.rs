use crate::cards::card_error::CardError;
use crate::cards::decks::standard52;
use crate::games::poker::cactus_kev_card::{ckc, CKC, SUITS_FILTER};
use crate::games::poker::cactus_kev_hand::CactusKevHand;
use crate::games::poker::hand_rank::{HandRank, HandRankValue};
use crate::{Pile, Standard52};
use std::convert::TryInto;
use std::fmt;

pub const POSSIBLE_COMBINATIONS: usize = 7937;

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

    /// # Panics
    ///
    /// Shouldn't be able to panic. (fingers crossed)
    ///
    #[must_use]
    pub fn eval_5cards(&self) -> HandRank {
        if !self.is_complete_hand() {
            return HandRank::default();
        }
        CactusKevHand::new(self.to_five_array().unwrap()).eval()
    }

    /// # Errors
    ///
    /// Will return `CardError::NotEnoughCards` if there are less than six cards.
    ///
    /// Will return `CardError::TooManyCards` if there are more than six cards.
    ///
    pub fn eval_6cards(&self) -> Result<CactusKevHand, CardError> {
        let array = self.to_six_array();
        if array.is_err() {
            return Err(array.unwrap_err());
        }

        let mut _tmp: HandRankValue = 0;
        let mut _best: HandRankValue = 0;
        let _dummy_kev_value: CKC = 0;

        Ok(CactusKevHand::default())
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CKC> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CKC> {
        self.0.iter()
    }

    /// # Errors
    ///
    /// Will return `CardError::NotEnoughCards` if there are less than five cards.
    ///
    /// Will return `CardError::TooManyCards` if there are more than five cards.
    pub fn to_cactus_kev_hand(&self) -> Result<CactusKevHand, CardError> {
        match self.to_five_array() {
            Ok(hand) => Ok(CactusKevHand::new(hand)),
            Err(e) => Err(e),
        }
    }

    /// # Errors
    ///
    /// Will return `CardError::NotEnoughCards` if there are less than five cards.
    ///
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

    /// # Errors
    ///
    /// Will return `CardError::NotEnoughCards` if there are less than six cards.
    ///
    /// Will return `CardError::TooManyCards` if there are more than six cards.
    ///
    /// # Panics
    ///
    /// Shouldn't be able to panic. (fingers crossed)
    ///
    pub fn to_six_array(&self) -> Result<[CKC; 6], CardError> {
        match self.len() {
            0..=5 => Err(CardError::NotEnoughCards),
            6 => Ok(self.0.clone().try_into().unwrap()),
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
    use crate::games::poker::hand_rank::{HandRankName, HandRankValue};
    use rstest::rstest;

    #[test]
    fn deal5() {
        let standard52 = &mut standard52::Standard52::new_shuffled();
        let hand1 = CactusKevCards::deal5(standard52);
        let hand2 = CactusKevCards::deal5(standard52);

        println!(
            "{} {:?} {} {:?}",
            hand1,
            hand1.eval_5cards().name,
            hand2,
            hand2.eval_5cards().name
        );
    }

    #[test]
    fn eval_5cards() {
        assert_eq!(
            HandRank::new(1),
            CactusKevCards::from_index("AS KS QS JS TS")
                .unwrap()
                .eval_5cards()
        );
        assert_eq!(
            HandRank::new(1),
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

        assert_eq!(expected, cards.eval_5cards().value);
    }

    #[test]
    fn eval_5cards__invalid_index() {
        let hand = CactusKevCards::from_index("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(0, hand.eval_5cards().value);
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

    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRankName::StraightFlush)]
    #[case("A♣ 2♣ 3♣ 4♣ 5♣", 10, HandRankName::StraightFlush)]
    #[case("A♠ A♥ A♦ A♣ K♠", 11, HandRankName::FourOfAKind)]
    #[case("2♠ 2♥ 2♦ 2♣ 3♠", 166, HandRankName::FourOfAKind)]
    #[case("A♠ A♥ A♦ K♠ K♦", 167, HandRankName::FullHouse)]
    #[case("2♠ 2♥ 2♦ 3♠ 3♦", 322, HandRankName::FullHouse)]
    #[case("A♠ K♠ Q♠ J♠ 9♠", 323, HandRankName::Flush)]
    #[case("2♣ 3♣ 4♣ 5♣ 7♣", 1599, HandRankName::Flush)]
    #[case("A♣ K♠ Q♠ J♠ T♠", 1600, HandRankName::Straight)]
    #[case("A♥ 2♣ 3♣ 4♣ 5♣", 1609, HandRankName::Straight)]
    #[case("A♠ A♥ A♦ K♠ Q♣", 1610, HandRankName::ThreeOfAKind)]
    #[case("2♠ 2♥ 2♦ 3♠ 4♣", 2467, HandRankName::ThreeOfAKind)]
    #[case("A♠ A♥ K♦ K♠ Q♣", 2468, HandRankName::TwoPair)]
    #[case("3♠ 3♥ 2♦ 2♠ 4♣", 3325, HandRankName::TwoPair)]
    #[case("A♠ A♥ K♠ Q♠ J♠", 3326, HandRankName::Pair)]
    #[case("2♠ 2♥ 3♠ 4♠ 5♠", 6185, HandRankName::Pair)]
    #[case("A♠ K♠ Q♠ J♠ 9♣", 6186, HandRankName::HighCard)]
    #[case("2♣ 3♣ 4♣ 5♥ 7♣", 7462, HandRankName::HighCard)]
    fn get_hand_rank(
        #[case] index: &'static str,
        #[case] hand_rank_value: HandRankValue,
        #[case] hand_rank_name: HandRankName,
    ) {
        let hand = CactusKevCards::from_index(index).unwrap();

        let actual_hand_rank = hand.eval_5cards();

        assert_eq!(hand_rank_value, actual_hand_rank.value);
        assert_eq!(hand_rank_name, actual_hand_rank.name);
    }

    #[test]
    fn get_hand_rank__invalid_index() {
        let hand = CactusKevCards::from_index("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(HandRankName::Invalid, hand.eval_5cards().name);
    }
}
