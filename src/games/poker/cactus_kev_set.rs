use crate::cards::card_error::CardError;
use crate::games::poker::cactus_kev_card::{ckc, CKC};
use crate::games::poker::cactus_kev_cards::CactusKevCards;
use crate::{Pile, Standard52};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct CactusKevSet(HashSet<CKC>);

impl CactusKevSet {
    #[must_use]
    pub fn new(hs: HashSet<CKC>) -> CactusKevSet {
        CactusKevSet(hs)
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    pub fn from_index(i: &'static str) -> Result<CactusKevSet, CardError> {
        let pile = Standard52::pile_from_index(i);

        if pile.is_err() {
            return Err(CardError::InvalidCard);
        }

        let mut cards = CactusKevSet::default();
        for card in pile.unwrap() {
            cards.insert(ckc::from_card(&card));
        }
        Ok(cards)
    }

    #[must_use]
    pub fn get(&self, ckc: &CKC) -> Option<&CKC> {
        self.0.get(ckc)
    }

    #[must_use]
    pub fn get_from_index(&self, index: &'static str) -> Option<&CKC> {
        self.get(&ckc::from_index(index)?)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CKC> {
        self.0.iter()
    }

    // TODO :-P Hack
    #[must_use]
    pub fn to_cactus_kev_cards(&self) -> CactusKevCards {
        let mut cards = CactusKevCards::default();
        for card in self.0.clone() {
            cards.push(card);
        }
        cards.sort_in_place();
        cards
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
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, ckc: CKC) {
        self.0.insert(ckc);
    }

    #[must_use]
    pub fn to_pile(&self) -> Pile {
        let mut pile = Pile::default();

        for card in &self.0 {
            pile.push(ckc::to_card(card));
        }

        pile.sort()
    }
}

impl Default for CactusKevSet {
    fn default() -> Self {
        CactusKevSet::new(HashSet::new())
    }
}

impl fmt::Display for CactusKevSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pile().sort().to_symbol_index())
    }
}

impl FromIterator<CKC> for CactusKevSet {
    fn from_iter<T: IntoIterator<Item = CKC>>(iter: T) -> Self {
        let mut c = CactusKevSet::default();
        for i in iter {
            c.insert(i);
        }
        c
    }
}

impl IntoIterator for CactusKevSet {
    type Item = CKC;
    type IntoIter = std::collections::hash_set::IntoIter<CKC>;

    fn into_iter(self) -> std::collections::hash_set::IntoIter<CKC> {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_set_tests {
    use super::*;
    use crate::games::poker::alt::original::cactus_kevs_original_eval_5cards;

    #[test]
    fn eval_5cards() {
        assert_eq!(
            1,
            CactusKevSet::from_index("AS KS QS JS TS")
                .unwrap()
                .to_cactus_kev_cards()
                .eval_5cards()
        );
        assert_eq!(
            1,
            CactusKevSet::from_index("AH KH QH JH TH")
                .unwrap()
                .to_cactus_kev_cards()
                .eval_5cards()
        );
    }

    #[test]
    fn eval_5cards__pair() {
        let cards = CactusKevSet::from_index("AS AH QS JS TS").unwrap();

        let expected = cactus_kevs_original_eval_5cards(
            cards.get_from_index("AS").unwrap(),
            cards.get_from_index("AH").unwrap(),
            cards.get_from_index("QS").unwrap(),
            cards.get_from_index("JS").unwrap(),
            cards.get_from_index("TS").unwrap(),
        );

        assert_eq!(expected, cards.to_cactus_kev_cards().eval_5cards());
    }

    #[test]
    fn eval_5cards__invalid_index() {
        let hand = CactusKevSet::from_index("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert_eq!(0, hand.to_cactus_kev_cards().eval_5cards());
    }

    #[test]
    fn get_from_index() {
        let cards = CactusKevSet::from_index("AS AH QS JS TS").unwrap();
        let ace_spades = ckc::from_card(&Standard52::card_from_index("AS"));

        assert_eq!(
            cards.get(&ace_spades).unwrap(),
            cards.get_from_index("AS").unwrap()
        );
    }

    #[test]
    fn to_cactus_kev_cards() {
        let set = CactusKevSet::from_index("AS KS QS JS TS").unwrap();

        let a = set.to_cactus_kev_cards().to_five_array().unwrap();

        assert_eq!(a.len(), 5);

        let one = &a[0];
        let card_one = ckc::to_card(one);
        println!("{}", card_one);

        assert_eq!(set.get_from_index("AS").unwrap(), one);
        assert_eq!(set.get_from_index("KS").unwrap(), &a[1]);
        assert_eq!(set.get_from_index("QS").unwrap(), &a[2]);
        assert_eq!(set.get_from_index("JS").unwrap(), &a[3]);
        assert_eq!(set.get_from_index("TS").unwrap(), &a[4]);
    }
}
