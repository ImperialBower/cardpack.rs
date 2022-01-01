use std::collections::HashSet;
use crate::{Card, Pile, Standard52};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PileSet(HashSet<Card>);

impl PileSet {
    #[must_use]
    pub fn new(pile: Pile) -> PileSet {
        let mut pile_set = PileSet::default();
        for card in pile {
            pile_set.insert(card);
        }
        pile_set
    }

    #[must_use]
    pub fn standard52() -> PileSet {
        PileSet::new(Pile::french_deck())
    }

    #[must_use]
    pub fn contains(self, card: &Card) -> bool {
        self.0.contains(card)
    }

    /// Returns true if a `Card` in the `PileSet` based on its `Standard52` index string is present.
    ///
    /// **WARNING:** This method will return false for indexes that aren't a part of the
    /// `Standard52` deck.
    #[must_use]
    pub fn contains_by_index(self, index: &'static str) -> bool {
        let card = Standard52::card_from_index(index);
        if !card.is_valid() {
            return false;
        }
        self.0.contains(&card)
    }

    #[must_use]
    pub fn get(&self, card: &Card) -> Option<&Card> {
        self.0.get(card)
    }

    /// Inserts a `Card` into the `PileSet`. Returns true if it isn't already present.
    pub fn insert(&mut self, card: Card) -> bool {
        self.0.insert(card)
    }

    /// Inserts a `Card` into the `PileSet` based on its `Standard52` index string. Returns
    /// true if it isn't already present.
    ///
    /// **WARNING:** This method will return false for indexes that aren't a part of the
    /// `Standard52` deck.
    pub fn insert_by_index(&mut self, index: &'static str) -> bool {
        let card = Standard52::card_from_index(index);
        if !card.is_valid() {
            return false;
        }
        self.0.insert(card)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Removes a `Card` from the `PileSet`.
    pub fn remove(&mut self, card: &Card) -> bool {
        self.0.remove(card)
    }

    /// Removes a `Card` from the `PileSet` based on its `Standard52` index string.
    ///
    /// **WARNING:** This method will return false for indexes that aren't a part of the
    /// `Standard52` deck.
    pub fn remove_by_index(&mut self, index: &'static str) -> bool {
        let card = Standard52::card_from_index(index);
        if !card.is_valid() {
            return false;
        }
        self.remove(&card)
    }

    /// Removes and returns a `Card` from the `PileSet`.
    pub fn take(&mut self, card: &Card) -> Option<Card> {
        self.0.take(card)
    }

    /// Removes and returns a `Card` from the `PileSet` based on its `Standard52` index string.
    ///
    /// **WARNING:** This method will return false for indexes that aren't a part of the
    /// `Standard52` deck.
    pub fn take_by_index(&mut self, index: &'static str) -> Option<Card> {
        let card = Standard52::card_from_index(index);
        if !card.is_valid() {
            return None;
        }
        self.take(&card)
    }
}