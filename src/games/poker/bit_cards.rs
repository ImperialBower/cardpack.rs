use crate::games::poker::bit_card::BitCard;
use std::fmt::{Display, Formatter};
use wyz::FmtForward;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BitCards(Vec<BitCard>);

impl BitCards {
    #[must_use]
    pub fn new_from_vector(v: Vec<BitCard>) -> BitCards {
        BitCards(v)
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
        BitCards::new_from_vector(Vec::new())
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

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_cards_tests {
    use super::*;

    #[test]
    fn is_empty() {
        assert!(BitCards::default().is_empty());
    }

    #[test]
    fn len() {
        let mut cards = BitCards::default();
        assert_eq!(0, cards.len());

        cards.push(BitCard::new_from_index("AS").unwrap());
        assert_eq!(1, cards.len());
    }

    #[test]
    fn push() {
        let mut cards = BitCards::default();
        cards.push(BitCard::new_from_index("AS").unwrap());
        cards.push(BitCard::new_from_index("KS").unwrap());
        let expected = "[00010000 00000000 00011100 00101001, xxxAKQJT 98765432 CDHSrrrr xxpppppp, 00001000 00000000 00011011 00100101, xxxAKQJT 98765432 CDHSrrrr xxpppppp]";

        // println!("{:#}", cards);
        assert_eq!(format!("{}", cards), expected);
    }
}
