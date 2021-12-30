use crate::cards::card_error::CardError;
use crate::games::poker::cactus_kev_card::CKC;
use std::convert::TryInto;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CactusKevCards(Vec<CKC>);

impl CactusKevCards {
    #[must_use]
    pub fn new(v: Vec<CKC>) -> CactusKevCards {
        CactusKevCards(v)
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
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_cards_tests {
    use crate::games::poker::bit_cards::BitCards;

    #[test]
    fn into_five_array() {
        let cards = BitCards::from_index("AS KS QS JS TS").unwrap();
        let ckc = cards.to_cactus_kev_cards().into_five_array().unwrap();

        println!("{:?}", ckc);
        // println!("{:?}", eval_5cards_kev_array(&ckc));
    }
}
