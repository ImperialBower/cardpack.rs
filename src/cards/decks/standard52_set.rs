use crate::{Card, Pile, Standard52};
use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Standard52Set(HashSet<Card>);

impl Standard52Set {
    #[must_use]
    pub fn new(pile: Pile) -> Self {
        let mut pile_set = Standard52Set::default();
        for card in pile {
            pile_set.insert(card);
        }
        pile_set
    }

    #[must_use]
    pub fn standard52() -> Self {
        Self::new(Pile::french_deck())
    }

    #[must_use]
    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(card)
    }

    /// Returns true if a `Card` in the `PileSet` based on its `Standard52` index string is present.
    ///
    /// **WARNING:** This method will return false for indexes that aren't a part of the
    /// `Standard52` deck.
    #[must_use]
    pub fn contains_by_index(&self, index: &'static str) -> bool {
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

    #[must_use]
    pub fn to_pile(&self) -> Pile {
        let mut pile: Pile = Pile::from_vector(self.clone().into_iter().collect::<Vec<_>>());
        pile.sort_in_place();
        pile
    }
}

impl FromIterator<Card> for Standard52Set {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut c = Standard52Set::default();
        for i in iter {
            c.insert(i);
        }
        c
    }
}

impl IntoIterator for Standard52Set {
    type Item = Card;
    type IntoIter = std::collections::hash_set::IntoIter<Card>;

    fn into_iter(self) -> std::collections::hash_set::IntoIter<Card> {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod pile_set_tests {
    use super::*;
    use crate::{CLUBS, CUPS, HEARTS, QUEEN, THREE};

    #[test]
    fn new() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut pile = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);
        pile.sort_in_place();

        let actual = Standard52Set::new(pile.clone());

        assert_eq!(actual.to_pile(), pile);
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn standard52() {
        let actual = Standard52Set::standard52();

        assert_eq!(Pile::french_deck(), actual.to_pile());
    }

    #[test]
    fn contains() {
        let pile_set = Standard52Set::standard52();

        assert!(pile_set.contains(&Card::from_index_strings(QUEEN, CLUBS)));
        assert!(pile_set.contains(&Card::from_index_strings(QUEEN, HEARTS)));
        assert!(!pile_set.contains(&Card::from_index_strings(THREE, CUPS)));
    }

    #[test]
    fn contains_by_index() {
        let pile_set = Standard52Set::standard52();

        assert!(pile_set.contains_by_index("QH"));
        assert!(pile_set.contains_by_index("AS"));
        assert!(!pile_set.contains_by_index("3🏆"));
    }

    #[test]
    fn get() {
        let pile_set = Standard52Set::standard52();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);

        let gotten = pile_set.get(&qclubs);

        assert!(gotten.is_some());
        assert_eq!(gotten.unwrap(), &qclubs);
    }

    #[test]
    fn insert() {
        let mut pile_set = Standard52Set::default();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);

        let inserted = pile_set.insert(qclubs.clone());

        assert!(inserted);
        // returns false if `Card` is already inserted.
        assert!(!pile_set.insert(qclubs));
    }

    #[test]
    fn insert_by_index() {
        let mut pile_set = Standard52Set::default();

        assert!(pile_set.insert_by_index("AH"));
        assert!(pile_set.insert_by_index("2C"));
        assert!(!pile_set.insert_by_index("AH"));
    }

    #[test]
    fn is_empty() {
        let pile_set = Standard52Set::default();

        assert!(pile_set.is_empty());
    }

    #[test]
    fn is_empty__not_empty() {
        let mut pile_set = Standard52Set::default();
        pile_set.insert(Card::from_index_strings(QUEEN, CLUBS));

        assert!(!pile_set.is_empty());
    }

    #[test]
    fn len() {
        assert_eq!(Standard52Set::standard52().len(), 52);
    }

    #[test]
    fn remove() {
        let mut pile_set = Standard52Set::standard52();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);

        assert!(pile_set.remove(&qclubs));
        assert!(!pile_set.remove(&qclubs));
    }

    #[test]
    fn remove_by_index() {
        let mut pile_set = Standard52Set::standard52();

        assert!(pile_set.remove_by_index("KH"));
        assert!(pile_set.remove_by_index("3♠"));
        assert!(!pile_set.remove_by_index("3S"));
        assert!(!pile_set.remove_by_index("KH"));
        assert!(!pile_set.remove_by_index("8🏆"));
    }

    #[test]
    fn take() {
        let mut pile_set = Standard52Set::standard52();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);

        let r = pile_set.take(&qclubs);

        assert!(r.is_some());
        assert_eq!(r.unwrap(), qclubs);
        assert!(pile_set.take(&qclubs).is_none());
    }

    #[test]
    fn take_by_index() {
        let mut pile_set = Standard52Set::standard52();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);

        let r = pile_set.take_by_index("QC");

        assert!(r.is_some());
        assert_eq!(r.unwrap(), qclubs);
        assert!(pile_set.take_by_index("QC").is_none());
    }

    #[test]
    fn to_pile() {
        let pile_set = Standard52Set::standard52();
        let pile = Pile::french_deck();

        assert_eq!(pile_set.to_pile(), pile);
    }
}
