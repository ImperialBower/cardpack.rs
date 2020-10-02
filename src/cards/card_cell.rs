use std::cell::{Cell, RefCell};
use std::fmt;
use std::ops::Deref;

use crate::cards::card::Card;

/// The structure of this struct is to deal with the issue that RefCell.take() is only
/// available in unstable. Once that feature has been merged in we can eliminate the Cell
/// and just have it as a newtype struct.
///
/// See https://github.com/rust-lang/rust/issues/71395
///
// #[derive(Debug)]
pub struct CardCell {
    cell: Cell<Card>,
    aligned: Cell<bool>,
    card: RefCell<Card>,
}

impl CardCell {
    pub fn new(card: Card) -> CardCell {
        CardCell {
            /// The internal container for the actual Card. When it is dealt it is replaced with
            /// a blank Card.
            cell: Cell::new(card.clone()),
            /// Tracks the state of the CardCell. True if the Card has not been dealt.
            aligned: Cell::new(true),
            /// A reference to what the card is in the Cell. A Card from a CardCell can only be
            /// returned if it matches this card.
            card: RefCell::new(card),
        }
    }

    pub fn deal(&self) -> Card {
        self.aligned.take();
        self.cell.take()
    }

    pub fn look(&self) -> Card {
        self.card.borrow().deref().clone()
    }

    pub fn is_there(&self) -> bool {
        let is_there = self.aligned.take();
        self.aligned.replace(is_there);
        is_there
    }

    /// Allows for the Card to be returned to the CardCell if the Card return matches the one that
    /// was dealt and the CardCell is empty, otherwise it returns None.
    pub fn replace(&self, card: Card) -> Option<bool> {
        if self.is_there() {
            return None;
        }
        let should = self.card.borrow();

        match &card == should.deref() {
            true => {
                self.cell.set(card);
                self.aligned.set(true);
                Some(true)
            }
            false => None,
        }
    }
}

impl fmt::Debug for CardCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CardCell: [aligned: {}, card: {:?}]",
            self.is_there(),
            self.look()
        )
    }
}

impl Eq for CardCell {}

impl PartialEq for CardCell {
    fn eq(&self, other: &Self) -> bool {
        let is_there_matches = self.is_there() == other.is_there();
        let look_matches = self.look() == other.look();

        is_there_matches && look_matches
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_cell_tests {
    use super::*;
    use crate::{ACE, HEARTS, QUEEN, SPADES, TWO};

    #[test]
    fn new() {
        let deuce = Card::new(TWO, SPADES);
        let expected = CardCell {
            cell: Cell::new(deuce.clone()),
            aligned: Cell::new(true),
            card: RefCell::new(deuce.clone()),
        };

        let actual = CardCell::new(deuce);

        assert_eq!(expected, actual);
    }

    #[test]
    fn deal() {
        let deuce = Card::new(TWO, SPADES);
        let cc = CardCell::new(deuce.clone());

        let actual = cc.deal();

        assert_eq!(deuce, actual);
        assert!(!cc.is_there())
    }

    #[test]
    fn debug() {
        let deuce = Card::new(TWO, SPADES);
        let cc1 = CardCell::new(deuce.clone());
        let cc2 = CardCell::new(deuce.clone());

        assert_eq!(cc1, cc2);
        cc1.deal();
        assert_ne!(cc1, cc2)
    }

    #[test]
    fn debug___ne_different_weight_cards() {
        let ace = Card::new(ACE, SPADES);
        let ace_cell = CardCell::new(ace);
        let mut alt_ace = Card::new(ACE, SPADES);
        alt_ace.weight = 1;
        let alt_cell = CardCell::new(alt_ace);

        assert_ne!(ace_cell, alt_cell);
    }

    #[test]
    fn is_there() {
        let cc = CardCell::new(Card::new(TWO, SPADES));

        assert!(cc.is_there())
    }

    #[test]
    fn look() {
        let deuce = Card::new(TWO, SPADES);
        let cc = CardCell::new(deuce.clone());

        assert_eq!(deuce, cc.look());
    }

    #[test]
    fn replace() {
        let cc = CardCell::new(Card::new(TWO, SPADES));
        let deuce = cc.deal();

        let result = cc.replace(deuce);

        assert!(result.unwrap());
        assert!(cc.is_there());
    }

    #[test]
    fn replace__ne_mismatched_card() {
        let cc = CardCell::new(Card::new(TWO, SPADES));
        cc.deal();

        let result = cc.replace(Card::default());

        assert!(result.is_none());
        assert!(!cc.is_there());
    }

    #[test]
    fn replace__ne_already_there() {
        let queen = Card::new(QUEEN, HEARTS);
        let cc = CardCell::new(queen.clone());

        let result = cc.replace(queen);

        assert!(result.is_none());
    }
}
