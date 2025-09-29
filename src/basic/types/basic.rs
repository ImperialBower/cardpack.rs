use crate::prelude::BasicPile;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

//  Ord, PartialOrd
#[derive(Default)]
pub struct BasicPileCell(Cell<BasicPile>);

impl BasicPileCell {
    /// Creates a new `BasicPileCell` containing the given `BasicPile`.
    pub fn new(pile: BasicPile) -> Self {
        Self(Cell::new(pile))
    }

    pub fn draw(&self, n: usize) -> Option<BasicPile> {
        let mut inner_pile = self.0.take();
        let drawn_cards = inner_pile.draw(n);
        self.0.set(inner_pile);
        drawn_cards
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

impl Hash for BasicPileCell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner_pile = self.0.take();
        inner_pile.hash(state);
        self.0.set(inner_pile);
    }
}

impl PartialOrd<Self> for BasicPileCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.0.take();
        let b = other.0.take();
        let result = a.partial_cmp(&b);
        self.0.set(a);
        other.0.set(b);
        result
    }
}

impl Ord for BasicPileCell {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0.take();
        let b = other.0.take();
        let result = a.cmp(&b);
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
    fn draw() {
        let pile = Pile::<Standard52>::basic_pile_cell();

        let drawn = pile.draw(5).unwrap();
        assert_eq!(drawn.to_string(), "A♠ K♠ Q♠ J♠ T♠");
    }

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

    #[test]
    fn hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let a = Pile::<Standard52>::basic_pile_cell();
        let b = Pile::<Standard52>::basic_pile_cell();
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();

        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);

        assert_eq!(hasher_a.finish(), hasher_b.finish());

        let shuffled = BasicPileCell::new(Pile::<Standard52>::basic_pile().shuffled());
        let mut hasher_shuffled = DefaultHasher::new();
        shuffled.hash(&mut hasher_shuffled);

        assert_ne!(hasher_a.finish(), hasher_shuffled.finish());
    }

}
