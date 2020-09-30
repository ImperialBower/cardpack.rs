use std::cell::{Cell, RefCell};

use crate::cards::card::Card;

/// The structure of this struct is to deal with the issue that RefCell.take() is only
/// available in unstable. Once that feature has been merged in we can eliminate the Cell
/// and just have it as a newtype struct.
pub struct CardCell(Cell<Card>, RefCell<Card>);

impl CardCell {
    pub fn new(card: Card) -> CardCell {
        CardCell(Cell::new(card.clone()), RefCell::new(card))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_cell_tests {
    use super::*;

    #[test]
    fn new() {

    }
}