use crate::prelude::{BasicCard, BasicPile, Ranged};
use core::cell::Cell;
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::Hash;

/// A [`BasicPile`] wrapped in a [`Cell`] to allow interior mutability through shared references.
///
/// Most `Pile` operations need `&mut self`, which is fine for owned values but prevents holding
/// the pile behind a shared reference. `BasicPileCell` solves this by wrapping `BasicPile` in
/// `Cell<T>`, letting callers use `&self` methods everywhere.
///
/// ## Why `Cell` and not `RefCell`?
///
/// `RefCell` adds runtime borrow-checking overhead. `Cell` is zero-cost but only provides
/// [`Cell::get`] for `Copy` types — and `BasicPile` (which contains a `Vec`) is not `Copy`.
/// Instead every method uses the **take → mutate → set** pattern:
///
/// ```ignore
/// let mut inner = self.0.take();  // moves the pile out, leaving Default in place
/// // ... operate on inner ...
/// self.0.set(inner);             // move it back
/// ```
///
/// This is safe because `Cell` is `!Sync`, so only one thread can ever hold the `BasicPileCell`,
/// and the window between `take` and `set` is never observable from outside.
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
    pub fn draw(&self, n: usize) -> Option<Self> {
        let mut inner_pile = self.0.take();
        let drawn_cards = inner_pile.draw(n);
        self.0.set(inner_pile);
        drawn_cards.map(Self::new)
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
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
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
    use super::*;
    use crate::basic_cell;
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
        assert_eq!(Standard52::deck_cell().draw(5).unwrap(), pile);
    }

    #[test]
    fn draw_first__empty() {
        let cell = BasicPileCell::default();
        assert!(cell.draw_first().is_none());
    }

    #[test]
    fn draw_first__non_empty() {
        let cell = Pile::<Standard52>::basic_pile_cell();
        let card = cell.draw_first();
        assert!(card.is_some());
        assert_eq!(cell.len(), Standard52::DECK_SIZE - 1);
    }

    #[test]
    fn shuffle__preserves_length() {
        let mut cell = Pile::<Standard52>::basic_pile_cell();
        cell.shuffle();
        assert_eq!(cell.len(), Standard52::DECK_SIZE);
    }

    #[test]
    fn shuffled__same_length() {
        let cell = Pile::<Standard52>::basic_pile_cell();
        let shuffled = cell.shuffled();
        assert_eq!(shuffled.len(), Standard52::DECK_SIZE);
    }

    #[test]
    fn contains__true_and_false() {
        use crate::prelude::FrenchBasicCard;
        let cell = Pile::<Standard52>::basic_pile_cell();
        assert!(cell.contains(&FrenchBasicCard::ACE_SPADES));
        assert!(!cell.contains(&BasicCard::default()));
    }

    #[test]
    fn extend__adds_cards() {
        let mut cell = basic_cell!("AS KS");
        let other = basic_cell!("QS JS");
        cell.extend(&other);
        assert_eq!(cell.len(), 4);
    }

    #[test]
    fn len__correct() {
        let cell = Pile::<Standard52>::basic_pile_cell();
        assert_eq!(cell.len(), Standard52::DECK_SIZE);
        assert_ne!(cell.len(), 1);
    }

    #[test]
    fn pop__some_and_none() {
        let cell = basic_cell!("AS KS");
        let card1 = cell.pop();
        assert!(card1.is_some());
        assert_ne!(card1, Some(BasicCard::default()));
        cell.pop();
        assert!(cell.pop().is_none());
    }

    #[test]
    fn push__increases_len() {
        use crate::prelude::FrenchBasicCard;
        let cell = BasicPileCell::default();
        cell.push(FrenchBasicCard::ACE_SPADES);
        assert_eq!(cell.len(), 1);
    }

    #[test]
    fn sort__reorders() {
        let cell = basic_cell!("2S 8C 4S");
        cell.sort();
        // after sorting, pile is in order
        assert_eq!(cell.to_string(), "4♠ 2♠ 8♣");
    }

    #[test]
    fn clone__equals_original() {
        let cell = Pile::<Standard52>::basic_pile_cell();
        let cloned = cell.clone();
        assert_eq!(cell, cloned);
        assert_ne!(cloned, BasicPileCell::default());
    }

    #[test]
    fn from_vec__round_trips() {
        use crate::prelude::FrenchBasicCard;
        let cards = vec![FrenchBasicCard::ACE_SPADES, FrenchBasicCard::KING_SPADES];
        let cell = BasicPileCell::from(cards);
        assert_eq!(cell.len(), 2);
        assert_ne!(cell, BasicPileCell::default());
    }

    #[test]
    fn my_basic_pile__not_default() {
        let cell = Pile::<Standard52>::basic_pile_cell();
        let pile = cell.my_basic_pile();
        assert_eq!(pile.len(), Standard52::DECK_SIZE);
        assert_ne!(pile, BasicPile::default());
    }

    #[test]
    fn partial_cmp__returns_some() {
        let cell_a = basic_cell!("AS KS");
        let cell_b = basic_cell!("QS JS");
        assert!(cell_a.partial_cmp(&cell_b).is_some());
    }
}
