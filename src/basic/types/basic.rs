use crate::prelude::{BasicCard, BasicPile, Ranged};
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

//  Ord, PartialOrd
#[derive(Default)]
pub struct BasicPileCell(Cell<BasicPile>);

impl BasicPileCell {
    /// Creates a new `BasicPileCell` containing the given `BasicPile`.
    #[must_use]
    pub fn new(pile: BasicPile) -> Self {
        Self(Cell::new(pile))
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    ///
    /// let drawn = pile.draw(5).unwrap();
    /// assert_eq!(drawn.to_string(), "A♠ K♠ Q♠ J♠ T♠");
    /// ```
    pub fn draw(&self, n: usize) -> Option<BasicPile> {
        let mut inner_pile = self.0.take();
        let drawn_cards = inner_pile.draw(n);
        self.0.set(inner_pile);
        drawn_cards
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    ///
    /// let card = pile.draw_first().unwrap();
    /// assert_eq!(card, FrenchBasicCard::ACE_SPADES);
    /// ```
    pub fn draw_first(&self) -> Option<BasicCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    pub fn shuffle(&mut self) {
        let mut inner_pile = self.0.take();
        inner_pile.shuffle();
        self.0.set(inner_pile);
    }

    /// ```
    ///  use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    /// // println!("{}", pile.shuffled());
    /// ```
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let inner_pile = self.0.take();
        let shuffled_pile = inner_pile.shuffled();
        self.0.set(inner_pile);
        Self::new(shuffled_pile)
    }

    /// Takes the value of the cell, leaving `Default::default()` in its place.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    ///
    /// assert_eq!(Standard52::DECK_SIZE, pile.take().len());
    /// assert_eq!(0, pile.take().len());
    /// ```
    pub fn take(&self) -> BasicPile {
        self.0.take()
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region vector functions

    #[must_use]
    pub fn contains(&self, card: &BasicCard) -> bool {
        let inner_pile = self.0.take();
        let result = inner_pile.contains(card);
        self.0.set(inner_pile);
        result
    }

    pub fn extend(&mut self, other: &Self) {
        let mut inner_pile = self.0.take();
        let other_pile = other.0.take();
        inner_pile.extend(&other_pile);
        self.0.set(inner_pile);
        other.0.set(other_pile);
    }

    /// ```
    ///  use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    ///
    /// assert!(!pile.is_empty());
    /// assert!(BasicPileCell::default().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        let pile = self.0.take();
        let empty = pile.len() == 0;
        self.0.set(pile);
        empty
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let inner_pile = self.0.take();
        let length = inner_pile.len();
        self.0.set(inner_pile);
        length
    }

    pub fn pop(&self) -> Option<BasicCard> {
        let mut inner_pile = self.0.take();
        let card = inner_pile.pop();
        self.0.set(inner_pile);
        card
    }

    pub fn push(&self, card: BasicCard) {
        let mut inner_pile = self.0.take();
        inner_pile.push(card);
        self.0.set(inner_pile);
    }

    /// ```
    ///  use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    ///
    /// let card = pile.remove(0);
    /// assert_eq!(card.to_string(), "A♠");
    /// let card = pile.remove(50);
    /// assert_eq!(card.to_string(), "2♣");
    /// assert_eq!(
    ///     pile.to_string(),
    ///     "K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣"
    /// );
    /// ```
    pub fn remove(&self, position: usize) -> BasicCard {
        let mut inner_pile = self.0.take();
        let card = inner_pile.remove(position);
        self.0.set(inner_pile);
        card
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile_cell();
    /// pile.reverse();
    ///
    /// assert_eq!(
    ///     pile.to_string(),
    ///     "2♣ 3♣ 4♣ 5♣ 6♣ 7♣ 8♣ 9♣ T♣ J♣ Q♣ K♣ A♣ 2♦ 3♦ 4♦ 5♦ 6♦ 7♦ 8♦ 9♦ T♦ J♦ Q♦ K♦ A♦ 2♥ 3♥ 4♥ 5♥ 6♥ 7♥ 8♥ 9♥ T♥ J♥ Q♥ K♥ A♥ 2♠ 3♠ 4♠ 5♠ 6♠ 7♠ 8♠ 9♠ T♠ J♠ Q♠ K♠ A♠"
    /// );
    /// ```
    pub fn reverse(&self) {
        let mut inner_pile = self.0.take();
        inner_pile.reverse();
        self.0.set(inner_pile);
    }

    pub fn sort(&self) {
        let mut inner_pile = self.0.take();
        inner_pile.sort();
        self.0.set(inner_pile);
    }
    // endregion
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

impl From<Vec<BasicCard>> for BasicPileCell {
    fn from(value: Vec<BasicCard>) -> Self {
        Self(Cell::new(BasicPile::from(value)))
    }
}

impl Ranged for BasicPileCell {
    fn my_basic_pile(&self) -> BasicPile {
        let inner_pile = self.0.take();
        let pile_clone = inner_pile.clone();
        self.0.set(inner_pile);
        pile_clone
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
        Some(self.cmp(other))
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
    use crate::basic_cell;
    use super::*;
    use crate::prelude::{DeckedBase, Pile, Standard52};

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
        // println!("{basic}");
        // println!("{shuffled}");

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

    #[test]
    fn basic_cell() {
        let pile = basic_cell!("AS KS QS JS TS");
        assert_eq!(Standard52::deck_cell().draw(5).unwrap().to_string(), "A♠ K♠ Q♠ J♠ T♠");
    }
}
