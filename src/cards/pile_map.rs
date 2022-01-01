use std::collections::BTreeMap;
use crate::{Card, Named, Pile};

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct PileMap(BTreeMap<String, Card>);

impl PileMap {
    #[must_use]
    pub fn from_pile(pile: Pile) -> PileMap {
        let mut pile_map = PileMap::default();

        for card in pile {
            pile_map.insert(card.index_default(), card);
        }

        pile_map
    }

    pub fn insert(&mut self, s: String, card: Card) {
        self.0.insert(s, card);
    }

    pub fn remove(&mut self, s: &str) -> Option<Card> {
        self.0.remove(s)
    }
}
