use crate::funky::types::buffoon_card::BuffoonCard;
use crate::prelude::CardError;
use rand::prelude::SliceRandom;
use rand::rng;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct BuffoonPile(Vec<BuffoonCard>);

impl BuffoonPile {
    /// Returns a reference to the internal vector of the struct.
    #[must_use]
    pub fn v(&self) -> &Vec<BuffoonCard> {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[must_use]
    pub fn contains(&self, card: &BuffoonCard) -> bool {
        self.0.contains(card)
    }

    #[must_use]
    pub fn draw(&mut self, n: usize) -> Option<Self> {
        let mut pile = Self::default();
        for _ in 0..n {
            if let Some(card) = self.pop() {
                pile.push(card);
            } else {
                return None;
            }
        }
        Some(pile)
    }

    pub fn draw_first(&mut self) -> Option<BuffoonCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// **DIARY** This is where TDD kinda breaks down for me in Rust. I can't
    /// write a failing test until I've written the signature for the method
    /// to test. Heck, might as well take a stab at writing it altogether. It
    /// does feel good when I do and get it write, given how insecure I am
    /// with the language.
    ///
    /// However, before I can write this easily, I need a `BuffoonPile::from_str() `
    /// method.
    #[must_use]
    pub fn enhance(&self, enhancer: BuffoonCard) -> Self {
        self.iter().map(|c| c.enhance(enhancer)).collect()
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<&BuffoonCard> {
        self.0.get(position)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<BuffoonCard> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<BuffoonCard> {
        self.0.pop()
    }

    pub fn push(&mut self, card: BuffoonCard) {
        self.0.push(card);
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn remove(&mut self, position: usize) -> BuffoonCard {
        self.0.remove(position)
    }

    /// Suffles the `BasicPile` in place.
    ///
    /// TODO: I would like to be able to pass in a seed to the shuffle function.
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    /// Returns a new shuffled version of the `BasicPile`.
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    pub fn sort(&mut self) {
        self.0.sort();
    }

    pub fn sort_by_rank(&mut self) {
        self.0.sort_by(|a, b| b.rank.cmp(&a.rank));
    }

    #[must_use]
    pub fn sorted(&self) -> Self {
        let mut pile = self.clone();
        pile.sort();
        pile
    }

    #[must_use]
    pub fn sorted_by_rank(self) -> Self {
        let mut pile = self.clone();
        pile.sort_by_rank();
        pile
    }
}

impl Display for BuffoonPile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<Vec<BuffoonCard>> for BuffoonPile {
    fn from(v: Vec<BuffoonCard>) -> Self {
        Self(v)
    }
}

impl FromStr for BuffoonPile {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split_whitespace()
            .map(BuffoonCard::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(cards))
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// region Iterator
impl FromIterator<BuffoonCard> for BuffoonPile {
    fn from_iter<T: IntoIterator<Item = BuffoonCard>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a BuffoonPile {
    type Item = &'a BuffoonCard;
    type IntoIter = std::slice::Iter<'a, BuffoonCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for BuffoonPile {
    type Item = BuffoonCard;
    type IntoIter = std::vec::IntoIter<BuffoonCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__types__buffoon_card_tests {
    use super::*;

    #[test]
    fn from_str() {
        let pile = BuffoonPile::from_str("AS KS QS JS TS").unwrap();
        assert_eq!(pile.to_string(), "AS KS QS JS TS");
    }
}

// so bad:
//
// please write unit tests for all the functions in the BuffoonPile struct
// mod tests {
//     use super::*;
//     use crate::funky::types::buffoon_card::BuffoonCard;
//
//     #[test]
//     fn test_v() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_clear() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.clear();
//         assert!(pile.is_empty());
//     }
//
//     #[test]
//     fn test_contains() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert!(pile.contains(&BuffoonCard::new(1)));
//         assert!(!pile.contains(&BuffoonCard::new(3)));
//     }
//
//     #[test]
//     fn test_draw() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         let drawn = pile.draw(2).unwrap();
//         assert_eq!(drawn.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2)]);
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_draw_first() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.draw_first(), Some(BuffoonCard::new(1)));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_extend() {
//         let mut pile1 = BuffoonPile(vec![BuffoonCard::new(1)]);
//         let pile2 = BuffoonPile(vec![BuffoonCard::new(2)]);
//         pile1.extend(&pile2);
//         assert_eq!(pile1.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_get() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.get(1), Some(&BuffoonCard::new(2)));
//         assert_eq!(pile.get(2), None);
//     }
//
//     #[test]
//     fn test_is_empty() {
//         let pile = BuffoonPile(vec![]);
//         assert!(pile.is_empty());
//         let pile = BuffoonPile(vec![BuffoonCard::new(1)]);
//         assert!(!pile.is_empty());
//     }
//
//     #[test]
//     fn test_iter() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         let mut iter = pile.iter();
//         assert_eq!(iter.next(), Some(&BuffoonCard::new(1)));
//         assert_eq!(iter.next(), Some(&BuffoonCard::new(2)));
//         assert_eq!(iter.next(), None);
//     }
//
//     #[test]
//     fn test_len() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.len(), 2);
//     }
//
//     #[test]
//     fn test_pop() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.pop(), Some(BuffoonCard::new(2)));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_push() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1)]);
//         pile.push(BuffoonCard::new(2));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_reverse() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.reverse();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_remove() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2)]);
//         assert_eq!(pile.remove(0), BuffoonCard::new(1));
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(2)]);
//     }
//
//     #[test]
//     fn test_shuffle() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         pile.shuffle();
//         assert_eq!(pile.len(), 3);
//     }
//
//     #[test]
//     fn test_shuffled() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//         let shuffled = pile.shuffled();
//         assert_eq!(shuffled.len(), 3);
//     }
//
//     #[test]
//     fn test_sort() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(3), BuffoonCard::new(1), BuffoonCard::new(2)]);
//         pile.sort();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//     }
//
//     #[test]
//     fn test_sort_by_rank() {
//         let mut pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(3), BuffoonCard::new(2)]);
//         pile.sort_by_rank();
//         assert_eq!(pile.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
//
//     #[test]
//     fn test_sorted() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(3), BuffoonCard::new(1), BuffoonCard::new(2)]);
//         let sorted = pile.sorted();
//         assert_eq!(sorted.v(), &vec![BuffoonCard::new(1), BuffoonCard::new(2), BuffoonCard::new(3)]);
//     }
//
//     #[test]
//     fn test_sorted_by_rank() {
//         let pile = BuffoonPile(vec![BuffoonCard::new(1), BuffoonCard::new(3), BuffoonCard::new(2)]);
//         let sorted = pile.sorted_by_rank();
//         assert_eq!(sorted.v(), &vec![BuffoonCard::new(3), BuffoonCard::new(2), BuffoonCard::new(1)]);
//     }
// }
