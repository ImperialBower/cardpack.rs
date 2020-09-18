/*  CardPack - A generic pack of cards library written in Rust.
Copyright (C) <2020>  Christoph Baker

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::card::Card;
use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use crate::fluent::{GERMAN, US_ENGLISH};

/// A Pile is a sortable collection of Cards.
///
/// # Usage:
/// ```
/// let mut pile = cardpack::Pile::default();
/// let ace_of_spades = cardpack::Card::new("ace", "spades");
/// let ace_of_hearts = cardpack::Card::new("ace", "hearts");
/// pile.add(ace_of_spades);
/// pile.add(ace_of_hearts);
/// pile.shuffle();

#[derive(Clone, Debug, PartialEq)]
pub struct Pile(Vec<Card>);

impl Pile {
    pub fn new_from_vector(v: Vec<Card>) -> Pile {
        Pile(v)
    }

    pub fn add(&mut self, elem: Card) {
        self.0.push(elem);
    }

    /// Appends a clone of the passed in Pile of Cards to the existing Pile.
    pub fn append(&mut self, other: &Pile) {
        self.0.append(&mut other.0.clone());
    }

    pub fn card_by_index(&self, index: &str) -> Option<&Card> {
        self.0.iter().find(|c| c.index == index)
    }

    /// Returns a reference to the Vector containing all the cards.
    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    /// Tests if a card is in the Pile.
    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(card)
    }

    /// Tests if every element is inside the Pile.
    pub fn contains_all(&self, pile: &Pile) -> bool {
        pile.cards().iter().all(|c| self.contains(c))
    }

    /// This function is designed to demonstrate the capabilities of the library.
    pub fn demo(&self) {
        println!("   Long in English and German:");
        for card in self.values() {
            let anzugname = card.suit.get_long(&GERMAN);
            let suitname = card.suit.get_long(&US_ENGLISH);
            let rangname = card.rank.get_long(&GERMAN);
            let rankname = card.rank.get_long(&US_ENGLISH);
            println!("      {} of {} ", rankname, suitname);
            println!("      {} von {} ", rangname, anzugname);
        }
        self.demo_short()
    }

    pub fn demo_short(&self) {
        let langs = &[US_ENGLISH, GERMAN];

        for lang in langs {
            println!();
            print!("   Short Symbols in {:<5}: ", format!("{}", lang));
            print!("{}", self.by_symbol_index_locale(lang));
        }

        for lang in langs {
            println!();
            print!("   Short Letters in {:<5}: ", format!("{}", lang));
            print!("{}", self.by_index_locale(lang));
        }

        println!();
        print!("   Shuffle Deck:           ");
        let mut shuffled = self.shuffle();
        print!("{}", shuffled.to_string());

        println!();
        print!("   Sort Deck:              ");
        shuffled.sort();
        print!("{}", shuffled.to_string());

        println!();
    }

    pub fn draw(&mut self, x: usize) -> Option<Pile> {
        if x > self.len() {
            None
        } else {
            let mut cards = Pile::default();
            for _ in 0..x {
                cards.add(self.draw_first().unwrap());
            }
            Some(cards)
        }
    }

    pub fn draw_first(&mut self) -> Option<Card> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    pub fn draw_last(&mut self) -> Option<Card> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(self.len() - 1)),
        }
    }

    pub fn first(&self) -> Option<&Card> {
        self.0.first()
    }

    pub fn get(&self, index: usize) -> Option<&Card> {
        self.0.get(index)
    }

    pub fn get_random(&self) -> Option<&Card> {
        self.0.choose(&mut rand::thread_rng())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn last(&self) -> Option<&Card> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn position(&self, karte: &Card) -> Option<usize> {
        self.0.iter().position(|k| k == karte)
    }

    // Takes a reference to the prepended entity, clones it, appends the original to the passed in
    // entity, and replaces the original with the new one.
    pub fn prepend(&mut self, other: &Pile) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        let position = self.position(card);
        match position {
            None => None,
            _ => Some(self.0.remove(position.unwrap())),
        }
    }

    pub fn shuffle(&self) -> Pile {
        let mut shuffled = self.clone();
        shuffled.shuffle_in_place();
        shuffled
    }

    pub fn shuffle_in_place(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn sort(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    pub fn values(&self) -> impl Iterator<Item = &Card> {
        self.0.iter()
    }

    pub fn jokers() -> Pile {
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        Pile::new_from_vector(vec![big_joker, little_joker])
    }

    fn fold_in(&mut self, suits: Vec<Suit>, ranks: Vec<Rank>) {
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                self.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }
    }

    pub fn french_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_french_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards
    }

    pub fn pinochle_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_pinochle_ranks();

        let mut cards: Pile = Pile::default();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                cards.add(Card::new_from_structs(rank.clone(), suit.clone()));
                cards.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }
        cards
    }

    pub fn skat_deck() -> Pile {
        let suits = Suit::generate_skat_suits();
        let ranks = Rank::generate_skat_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards
    }

    pub fn spades_deck() -> Pile {
        let mut deck = Pile::french_deck();
        deck.remove_card(&Card::new("two", "clubs"));
        deck.remove_card(&Card::new("two", "diamonds"));
        let jokers = Pile::jokers();

        deck.prepend(&jokers);
        deck
    }

    pub fn tarot_deck() -> Pile {
        let arcana_suits = Suit::generate_arcana_suits();
        let mut arcana_suits_enumerator = arcana_suits.iter().enumerate();
        let major_arcana_ranks = Rank::generate_major_arcana_ranks();
        let minor_arcana_ranks = Rank::generate_minor_arcana_ranks();

        let mut cards: Pile = Pile::default();

        let (_, major_arcana_suit) = arcana_suits_enumerator.next().unwrap();

        // Generate Major Arcana
        for (_, rank) in major_arcana_ranks.iter().enumerate() {
            cards.add(Card::new_from_structs(
                rank.clone(),
                major_arcana_suit.clone(),
            ));
        }

        // Generate Minor Arcana
        for (_, suit) in arcana_suits_enumerator {
            for (_, rank) in minor_arcana_ranks.iter().enumerate() {
                cards.add(Card::new_from_structs(rank.clone(), suit.clone()));
            }
        }

        cards
    }

    fn collect_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.to_txt_string(lid)).collect()
    }

    fn collect_symbol_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.to_symbol_string(lid)).collect()
    }

    pub fn by_index(&self) -> String {
        self.by_index_locale(&US_ENGLISH)
    }

    pub fn by_index_locale(&self, lid: &LanguageIdentifier) -> String {
        Pile::sig_generate_from_strings(&self.collect_index(lid))
    }

    pub fn by_symbol_index(&self) -> String {
        self.by_symbol_index_locale(&US_ENGLISH)
    }

    pub fn by_symbol_index_locale(&self, lid: &LanguageIdentifier) -> String {
        Pile::sig_generate_from_strings(&self.collect_symbol_index(lid))
    }

    pub fn sig_generate_from_strings(strings: &[String]) -> String {
        strings
            .iter()
            .map(|s| format!("{} ", s))
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

impl Default for Pile {
    fn default() -> Self {
        Pile::new_from_vector(Vec::new())
    }
}

/// Sets the to_string() function for a Pile to return the default index signature for the Pile.
impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self.by_index();
        write!(f, "{}", sig)
    }
}

impl IntoIterator for Pile {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_deck_tests {
    use super::*;

    #[test]
    fn new_all_add_new_from_vector() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut expected = Pile::default();
        expected.add(qclubs.clone());
        expected.add(qhearts.clone());

        let actual = Pile::new_from_vector(vec![qclubs, qhearts]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn append() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        let mut to_deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::jokers();
        let expected = Pile::new_from_vector(vec![qclubs, qhearts, big_joker, little_joker]);

        to_deck.append(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn card_by_index() {
        let deck = Pile::spades_deck();
        let expected = Card::new("little-joker", "spades");

        let card = deck.card_by_index("JLS").unwrap();

        assert_eq!(&expected, card);
    }

    #[test]
    fn card_by_index_ne() {
        let deck = Pile::spades_deck();
        let fool_index = Card::new("fool", "major-arcana").index;

        // Verifies that the index for a card in the tarot deck isn't in a spades deck.
        assert!(deck.card_by_index(fool_index.as_str()).is_none());
    }

    #[test]
    fn contains() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(deck.contains(&qclubs));
        assert!(deck.contains(&qhearts));
    }

    #[test]
    fn contains_all() {
        let deck = Pile::spades_deck();
        let hand = Pile::spades_deck().shuffle().draw(4).unwrap();

        assert!(deck.contains_all(&hand));
    }

    #[test]
    fn contains_all_ne() {
        let deck = Pile::spades_deck();
        let hand = Pile::skat_deck().shuffle().draw(4).unwrap();

        assert!(!deck.contains_all(&hand));
    }

    #[test]
    fn draw() {
        let mut zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let qspades = Card::new("queen", "spades");
        let mut deck =
            Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone(), qspades.clone()]);

        assert!(zero.draw(2).is_none());
        assert_eq!(
            deck.draw(2).unwrap(),
            Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()])
        );
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_first() {
        let mut zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_first().is_none());
        assert_eq!(deck.draw_first().unwrap(), qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_last() {
        let mut zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_last().is_none());
        assert_eq!(deck.draw_last().unwrap(), qhearts);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn first() {
        let zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.first().is_none());
        assert_eq!(deck.first().unwrap(), &qclubs);
    }

    #[test]
    fn get() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let gotten = deck.get(1);

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn get_random() {
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qhearts.clone()]);

        let gotten = deck.get_random();

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn last() {
        let zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.last().is_none());
        assert_eq!(deck.last().unwrap(), &qhearts);
    }

    #[test]
    fn len() {
        let zero = Pile::default();
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(zero.len(), 0);
        assert_eq!(deck.len(), 2);
    }

    #[test]
    fn position() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(0, deck.position(&qclubs).unwrap());
        assert_eq!(1, deck.position(&qhearts).unwrap());
    }

    #[test]
    fn prepend() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let big_joker = Card::new("big-joker", "spades");
        let little_joker = Card::new("little-joker", "spades");
        let mut to_deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::new_from_vector(vec![big_joker.clone(), little_joker.clone()]);
        let expected = Pile::new_from_vector(vec![big_joker, little_joker, qclubs, qhearts]);

        to_deck.prepend(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn remove() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove(0);

        assert_eq!(removed, qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn remove_card() {
        let qclubs = Card::new("queen", "clubs");
        let qhearts = Card::new("queen", "hearts");
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove_card(&qclubs);

        assert_eq!(removed.unwrap(), qclubs);
        assert!(deck.contains(&qhearts));
        assert!(!deck.contains(&qclubs));
    }

    // Signature methods

    #[test]
    fn sig_generate_from_strings() {
        let deck = Pile::french_deck().draw(4).unwrap();

        let sig_english = Pile::sig_generate_from_strings(&deck.collect_index(&US_ENGLISH));
        let sig_german = Pile::sig_generate_from_strings(&deck.collect_index(&GERMAN));

        assert_eq!("AS KS QS JS".to_string(), sig_english);
        assert_eq!("AS KS DS BS".to_string(), sig_german);
    }

    #[test]
    fn shuffle() {
        let pile = Pile::french_deck();

        let mut shuffled = pile.shuffle();

        assert_ne!(pile, shuffled);
        shuffled.sort();
        assert_eq!(pile, shuffled);
    }

    #[test]
    fn shuffle_in_place() {
        let actual = Pile::pinochle_deck();
        let mut shuffled = Pile::pinochle_deck();
        assert_eq!(actual, shuffled);

        shuffled.shuffle_in_place();
        assert_ne!(actual, shuffled);

        shuffled.sort();
        assert_eq!(actual, shuffled);
    }

    #[test]
    fn sig_index() {
        let deck = Pile::spades_deck().draw(4).unwrap();

        let sig_english = deck.by_index();
        let sig_german = deck.by_index_locale(&GERMAN);

        assert_eq!("JBS JLS AS KS".to_string(), sig_english);
        assert_eq!("JGS JKS AS KS".to_string(), sig_german);
    }

    #[test]
    fn sig_symbol_index() {
        let deck = Pile::spades_deck().draw(4).unwrap();

        let sig = deck.by_symbol_index();

        assert_eq!("JB♠ JL♠ A♠ K♠".to_string(), sig);
    }

    #[test]
    fn to_string() {
        let deck = Pile::french_deck().draw(4);

        let sig = deck.unwrap().to_string();

        assert_eq!("AS KS QS JS".to_string(), sig);
    }

    #[test]
    fn sort() {
        let decks = vec![
            Pile::french_deck(),
            Pile::skat_deck(),
            Pile::spades_deck(),
            Pile::tarot_deck(),
        ];

        for deck in decks {
            let mut shuffled = deck.shuffle();
            assert_ne!(deck, shuffled);
            shuffled.sort();

            assert_eq!(deck, shuffled);
        }
    }

    #[test]
    fn spades_deck() {
        let deck = Pile::spades_deck();

        assert!(!deck.contains(&Card::new("two", "clubs")));
        assert!(!deck.contains(&Card::new("two", "diamonds")));
        assert!(deck.contains(&Card::new("two", "spades")));
    }
}
