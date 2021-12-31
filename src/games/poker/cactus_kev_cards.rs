use crate::cards::card_error::CardError;
use crate::games::poker::alt::lookups;
use crate::games::poker::cactus_kev_card::{ckc, HandRank, CKC, SUITS_FILTER};
use crate::Standard52;
use std::convert::TryInto;

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

    #[must_use]
    pub fn eval_5cards(&self) -> HandRank {
        if !self.is_complete_hand() {
            return 0;
        }
        let i = self.or_shift_16();

        if self.is_flush() {
            return lookups::FLUSHES[i] as HandRank;
        }

        0

        // let q: usize = ((c1 | c2 | c3 | c4 | c5) as usize) >> 16;

        // if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        //     return lookups::FLUSHES[q] as HandRank;
        // }
        // let s = lookups::UNIQUE_5[q] as HandRank;
        // if s != 0 {
        //     return s;
        // }
        //
        // let q = ((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff)) as usize;
        // let lookup = findit(q);
        // lookups::VALUES[lookup] as HandRank
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CKC> {
        self.0.get(index)
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
    pub fn into_five_array(&self) -> Result<[CKC; 5], CardError> {
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

    pub fn push(&mut self, ckc: CKC) {
        self.0.push(ckc);
    }

    #[must_use]
    pub fn or_shift_16(&self) -> usize {
        if !self.is_complete_hand() {
            return 0;
        }
        ((self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4]) as usize) >> 16
    }
}

impl Default for CactusKevCards {
    fn default() -> Self {
        CactusKevCards::new(Vec::new())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_cards_tests {
    use super::*;
    use crate::games::poker::alt::original::eval_5cards_kev_array;
    use crate::games::poker::cactus_kev_card::CKC;

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
    fn into_five_array() {
        let ckc = CactusKevCards::from_index("AS KS QS JS TS").unwrap();

        let a = ckc.into_five_array().unwrap();

        assert_eq!(a.len(), 5);
        assert_eq!(ckc.get(0).unwrap(), &a[0]);
        assert_eq!(ckc.get(1).unwrap(), &a[1]);
        assert_eq!(ckc.get(2).unwrap(), &a[2]);
        assert_eq!(ckc.get(3).unwrap(), &a[3]);
        assert_eq!(ckc.get(4).unwrap(), &a[4]);
    }

    #[test]
    fn into_five_ref_array() {
        let ckc = CactusKevCards::from_index("AS KS QS JS TS").unwrap();

        let mut hand: [&CKC; 5] = [&0; 5];
        hand[0] = &ckc.get(0).unwrap();
        hand[1] = &ckc.get(1).unwrap();
        hand[2] = &ckc.get(2).unwrap();
        hand[3] = &ckc.get(3).unwrap();
        hand[4] = &ckc.get(4).unwrap();

        let rank = eval_5cards_kev_array(&hand);

        println!("{}", rank);
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

        assert_eq!(ckc.or_shift_16(), 7936);
    }
}
