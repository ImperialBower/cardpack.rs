use crate::cards::card_error::CardError;
use crate::games::poker::bit_card::BitCard;
use crate::Standard52;
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BitCards(Vec<BitCard>);

impl BitCards {
    #[must_use]
    pub fn new(v: Vec<BitCard>) -> BitCards {
        BitCards(v)
    }

    /// # Errors
    ///
    /// Will return `CardError::InvalidCard` for an invalid index.
    #[allow(clippy::missing_panics_doc)]
    pub fn from_index(i: &'static str) -> Result<BitCards, CardError> {
        let pile = Standard52::pile_from_index(i);

        if pile.is_err() {
            return Err(CardError::InvalidCard);
        }

        let mut cards = BitCards::default();
        for card in pile.unwrap() {
            cards.push(BitCard::from_card(&card));
        }
        Ok(cards)
    }

    #[must_use]
    pub fn get(&self, i: usize) -> Option<&BitCard> {
        self.0.get(i)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, bit_card: BitCard) {
        self.0.push(bit_card);
    }
}

impl Default for BitCards {
    fn default() -> Self {
        BitCards::new(Vec::new())
    }
}

impl Display for BitCards {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        for bit_card in self.0.clone() {
            let mut mark_string = String::with_capacity(35);
            mark_string.push_str("xxxAKQJT 98765432 CDHSrrrr xxpppppp");

            out.entry(&(bit_card.display(true)).fmt_display());
            out.entry(&(&mark_string).fmt_display());
        }

        out.finish()
    }
}

impl FromIterator<BitCard> for BitCards {
    fn from_iter<T: IntoIterator<Item = BitCard>>(iter: T) -> Self {
        let mut c = BitCards::default();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl IntoIterator for BitCards {
    type Item = BitCard;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_cards_tests {
    use super::*;

    #[test]
    fn from_index() {
        let cards = BitCards::from_index("AS KS QS JS TS").unwrap();

        assert_eq!(cards.len(), 5);
        let c = cards.get(1).unwrap();
        // let ex = BitCard::from_index("AS").unwrap();
        assert_eq!(c, &BitCard::from_index("KS").unwrap());
    }

    #[test]
    fn is_empty() {
        assert!(BitCards::default().is_empty());
    }

    #[test]
    fn len() {
        let mut cards = BitCards::default();
        assert_eq!(0, cards.len());

        cards.push(BitCard::from_index("AS").unwrap());
        assert_eq!(1, cards.len());
    }

    #[test]
    fn push() {
        let mut cards = BitCards::default();
        cards.push(BitCard::from_index("AS").unwrap());
        cards.push(BitCard::from_index("KS").unwrap());
        let expected = "[00010000 00000000 00011100 00101001, xxxAKQJT 98765432 CDHSrrrr xxpppppp, 00001000 00000000 00011011 00100101, xxxAKQJT 98765432 CDHSrrrr xxpppppp]";

        // println!("{:#}", cards);
        assert_eq!(format!("{}", cards), expected);
    }

    #[test]
    fn scratch() {
        let cards = BitCards::from_index("AS KS QS JS TS").unwrap();

        // cards.into_iter().map()

        for _c in cards {}
    }
}
