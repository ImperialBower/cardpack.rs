use crate::cards::card::CactusKevCard;
use crate::cards::card_error::CardError;
use std::convert::TryInto;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CactusKevCards(Vec<CactusKevCard>);

impl CactusKevCards {
    #[must_use]
    pub fn new(v: Vec<CactusKevCard>) -> CactusKevCards {
        CactusKevCards(v)
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CactusKevCard> {
        self.0.get(index)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_five_array(&self) -> Result<[CactusKevCard; 5], CardError> {
        match self.len() {
            0..=4 => Err(CardError::NotEnoughCards),
            5 => Ok(self.0.clone().try_into().unwrap()),
            _ => Err(CardError::TooManyCards),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_cards_tests {
    use super::*;
    use crate::games::poker::alt::original::eval_5cards_kev_array;
    use crate::games::poker::bit_cards::BitCards;

    #[test]
    fn into_five_array() {
        let cards = BitCards::from_index("AS KS QS JS TS").unwrap();
        let ckc = cards.to_cactus_kev_cards().into_five_array().unwrap();

        println!("{:?}", ckc);
        // println!("{:?}", eval_5cards_kev_array(&ckc));
    }
}
