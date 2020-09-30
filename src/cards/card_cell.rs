use std::cell::{Cell, RefCell};

use crate::cards::card::Card;

/// The structure of this struct is to deal with the issue that RefCell.take() is only
/// available in unstable. Once that feature has been merged in we can eliminate the Cell
/// and just have it as a newtype struct.
///
// #[derive(Debug)]
pub struct CardCell(Cell<Card>, RefCell<Card>);

impl CardCell {
    pub fn new(card: Card) -> CardCell {
        CardCell(Cell::new(card.clone()), RefCell::new(card))
    }
}

// impl PartialEq for CardCell {
//     fn eq(&self, other: &Self) -> bool {
//         let taken = self.0.take();
//         self.0.replace(taken.clone());
//
//         self.path == other.path
//     }
// }

#[cfg(test)]
#[allow(non_snake_case)]
mod card_cell_tests {
    use super::*;
    use crate::{SPADES, TWO};

    #[test]
    fn new() {
        let deuce = Card::new(TWO, SPADES);
        let expected = CardCell(Cell::new(deuce.clone()), RefCell::new(deuce.clone()));

        let actual = CardCell::new(deuce);

        // assert_eq!(expected, actual)
    }
}
