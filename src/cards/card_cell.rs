use std::cell::Cell;

use crate::cards::card::Card;

pub struct CardCell(Cell<Card>);

impl Default for CardCell {
    fn default() -> Self {
        unimplemented!()
    }
}