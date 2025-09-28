use crate::prelude::BasicPile;
use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter};

// , Eq, Hash, PartialEq, Ord, PartialOrd
#[derive(Default)]
pub struct BasicPileCell(Cell<BasicPile>);

impl BasicPileCell {
    /// Creates a new `BasicPileCell` containing the given `BasicPile`.
    pub fn new(pile: BasicPile) -> Self {
        Self(Cell::new(pile))
    }

    pub fn take(&self) -> BasicPile {
        self.0.take()
    }
}

impl Clone for BasicPileCell {
    fn clone(&self) -> Self {
        let internal = self.0.take();
        self.0.set(internal.clone());
        Self(Cell::from(internal))
    }
}

impl Debug for BasicPileCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // We need to temporarily take the value to access it for formatting
        // Since BasicPile doesn't implement Copy, we use take() and set() it back
        let inner_pile = self.0.take();
        let debug_str = format!("BasicPileCell({inner_pile:?})");
        self.0.set(inner_pile);
        f.write_str(&debug_str)
    }
}

impl Display for BasicPileCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // We need to temporarily take the value to access it for formatting
        // Since BasicPile doesn't implement Copy, we use take() and set() it back
        let inner_pile = self.0.take();
        let display_str = format!("{inner_pile}");
        self.0.set(inner_pile);
        f.write_str(&display_str)
    }
}

impl Eq for BasicPileCell {}

impl PartialEq for BasicPileCell {
    fn eq(&self, other: &Self) -> bool {
        let a = self.0.take();
        let b = other.0.take();
        let result = a == b;
        self.0.set(a);
        other.0.set(b);
        result
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__basic_tests {
    use super::*;
    use crate::prelude::{DeckedBase, Pile, Standard52};

    #[test]
    fn take() {
        let pile = Pile::<Standard52>::basic_pile_cell();

        assert_eq!(Standard52::DECK_SIZE, pile.take().len());
        assert_eq!(0, pile.take().len());
    }

    #[test]
    fn debug() {
        let cell = Pile::<Standard52>::basic_pile_cell();

        let debug_str = format!("{:?}", cell);
        assert!(debug_str.contains("BasicPileCell"));
    }

    #[test]
    fn display() {
        let pile = Pile::<Standard52>::basic_pile_cell();

        assert_eq!(
            pile.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
    }

    #[test]
    fn eq() {
        let basic = Pile::<Standard52>::basic_pile_cell();
        assert_eq!(
            Pile::<Standard52>::basic_pile_cell(),
            Pile::<Standard52>::basic_pile_cell()
        );

        let shuffled = BasicPileCell::new(Pile::<Standard52>::basic_pile().shuffled());
        assert_ne!(shuffled, basic);
        println!("{basic}");
        println!("{shuffled}");

        let taken = Pile::<Standard52>::basic_pile_cell();
        taken.take();
        assert_ne!(taken, Pile::<Standard52>::basic_pile_cell());
    }
}
