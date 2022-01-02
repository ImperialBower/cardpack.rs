use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use unic_langid::LanguageIdentifier;

use crate::cards::card::Card;
#[allow(clippy::wildcard_imports)]
use crate::cards::rank::*;
use crate::cards::suit::{Suit, CLUBS, DIAMONDS, HEARTS, TRUMP};
use crate::fluent::named::{GERMAN, US_ENGLISH};
use crate::Named;

/// A Pile is a sortable collection of Cards.
///
/// # Usage:
/// ```
/// let mut pile = cardpack::Pile::default();
/// let ace_of_spades = cardpack::Card::from_index_strings(cardpack::ACE, cardpack::SPADES);
/// let ace_of_hearts = cardpack::Card::from_index_strings(cardpack::ACE, cardpack::HEARTS);
/// pile.push(ace_of_spades);
/// pile.push(ace_of_hearts);
/// pile.shuffle();
/// ```

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Pile(Vec<Card>);

impl Pile {
    #[must_use]
    pub fn from_vector(v: Vec<Card>) -> Pile {
        Pile(v)
    }

    /// Takes a reference to an Array of Piles and consolidates them into a single Pile of Cards.
    #[must_use]
    pub fn pile_on(piles: Vec<Pile>) -> Pile {
        piles.into_iter().flatten().collect()
    }

    /// Allows you to pass in an integer and a Pile returning function method and creates a Pile
    /// made up of the Piles generated that many times.
    ///
    /// # Usage:
    /// ```
    /// let pile = cardpack::Pile::pile_up(6, cardpack::Pile::french_deck);
    /// pile.shuffle();
    /// ```
    /// This creates and shuffles a Pile made up of six traditional French decks, which would be
    /// suitable for a casino blackjack table.
    ///
    pub fn pile_up<F>(x: usize, f: F) -> Pile
    where
        F: Fn() -> Pile,
    {
        let mut pile: Vec<Pile> = Vec::new();
        for _ in 0..x {
            pile.push(f());
        }
        Pile::pile_on(pile)
    }

    /// Places the Card at the bottom (end) of the Pile.
    pub fn push(&mut self, elem: Card) {
        self.0.push(elem);
    }

    /// Appends a clone of the passed in Pile of Cards to the existing Pile.
    pub fn append(&mut self, other: &Pile) {
        self.0.append(&mut other.0.clone());
    }

    /// Returns a simple string representation of the Cards in the Pile based upon the
    /// default language local, which is `US_ENGLISH`.
    #[must_use]
    pub fn to_index(&self) -> String {
        self.to_index_locale(&US_ENGLISH)
    }

    /// Returns a static str of the Pack's index. Mainly used for testing deserialization.
    ///
    /// Idea from: <https://stackoverflow.com/a/52367953/>
    #[must_use]
    pub fn to_index_str(&self) -> &'static str {
        Box::leak(self.to_index().into_boxed_str())
    }

    #[must_use]
    pub fn to_index_locale(&self, lid: &LanguageIdentifier) -> String {
        Pile::sig_generate_from_strings(&self.collect_index(lid))
    }

    #[must_use]
    pub fn to_symbol_index(&self) -> String {
        self.to_symbol_index_locale(&US_ENGLISH)
    }

    #[must_use]
    pub fn to_symbol_index_locale(&self, lid: &LanguageIdentifier) -> String {
        Pile::sig_generate_from_strings(&self.collect_symbol_index(lid))
    }

    #[must_use]
    pub fn card_by_index(&self, index: &str) -> Option<&Card> {
        self.0.iter().find(|c| c.index_default() == index)
    }

    /// Returns a reference to the Vector containing all the cards.
    #[must_use]
    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    /// Returns a Vector of Cards matching the passed in Suit.
    #[must_use]
    pub fn cards_by_suit(&self, suit: Suit) -> Vec<Card> {
        self.sort()
            .0
            .into_iter()
            .filter(|c| c.suit == suit)
            .collect()
    }

    fn collect_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.index(lid)).collect()
    }

    fn collect_symbol_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.symbol(lid)).collect()
    }

    /// Tests if a card is in the Pile.
    #[must_use]
    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(card)
    }

    /// Tests if every element is inside the Pile.
    #[must_use]
    pub fn contains_all(&self, pile: &Pile) -> bool {
        pile.cards().iter().all(|c| self.contains(c))
    }

    /// This function is designed to demonstrate the capabilities of the library.
    #[allow(clippy::similar_names)]
    pub fn demo(&self) {
        println!("   Long in English and German:");
        for card in self.values() {
            let anzugname = card.suit.name.long(&GERMAN);
            let suitname = card.suit.name.long(&US_ENGLISH);
            let rangname = card.rank.name.long(&GERMAN);
            let rankname = card.rank.name.long(&US_ENGLISH);
            println!("      {} of {} ", rankname, suitname);
            println!("      {} von {} ", rangname, anzugname);
        }
        self.demo_short();
    }

    pub fn demo_short(&self) {
        let languages = &[US_ENGLISH, GERMAN];

        for lang in languages {
            println!();
            print!("   Short Symbols in {:<5}: ", format!("{}", lang));
            print!("{}", self.to_symbol_index_locale(lang));
        }

        for lang in languages {
            println!();
            print!("   Short Letters in {:<5}: ", format!("{}", lang));
            print!("{}", self.to_index_locale(lang));
        }

        println!();
        print!("   Shuffle Deck:           ");
        let shuffled = self.shuffle();
        print!("{}", shuffled);

        println!();
        print!("   Sort Deck:              ");
        print!("{}", shuffled.sort());

        println!();
    }

    pub fn draw(&mut self, x: usize) -> Option<Pile> {
        if x > self.len() || x < 1 {
            None
        } else {
            let mut cards = Pile::default();
            for _ in 0..x {
                cards.push(self.draw_first()?);
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

    #[must_use]
    pub fn first(&self) -> Option<&Card> {
        self.0.first()
    }

    fn fold_in(&mut self, suits: &[Suit], ranks: &[Rank]) {
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                self.push(Card::new(*rank, *suit));
            }
        }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Card> {
        self.0.get(index)
    }

    #[must_use]
    pub fn get_random(&self) -> Option<&Card> {
        self.0.choose(&mut rand::thread_rng())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn last(&self) -> Option<&Card> {
        self.0.last()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn map_by_rank_count(&self) -> HashMap<Rank, usize> {
        let mut mappie: HashMap<Rank, usize> = HashMap::new();
        for card in self.cards() {
            *mappie.entry(card.rank).or_insert(0) += 1;
        }
        mappie
    }

    /// Takes a `Pile` and returns a `HashMap` with the key as each `Suit` in the `Pile` with the values
    /// as a `Pile` of the cards for that Suit.
    #[must_use]
    pub fn map_by_suit(&self) -> HashMap<Suit, Pile> {
        let mut mappie: HashMap<Suit, Pile> = HashMap::new();
        for suit in self.suits() {
            let pile = self
                .0
                .clone()
                .into_iter()
                .filter(|c| c.suit == suit)
                .collect();
            mappie.insert(suit, pile);
        }
        mappie
    }

    #[must_use]
    pub fn position(&self, karte: &Card) -> Option<usize> {
        self.0.iter().position(|k| k.index == karte.index)
    }

    #[must_use]
    pub fn pile_by_index(&self, indexes: &[&str]) -> Option<Pile> {
        let mut pile = Pile::default();
        for index in indexes {
            let card = self.card_by_index(index);
            match card {
                Some(c) => pile.push(c.clone()),
                _ => return None,
            }
        }
        Some(pile)
    }

    // Takes a reference to the prepended entity, clones it, appends the original to the passed in
    // entity, and replaces the original with the new one.
    pub fn prepend(&mut self, other: &Pile) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    #[must_use]
    pub fn ranks(&self) -> Vec<Rank> {
        let hashset: HashSet<Rank> = self.0.iter().map(|c| c.rank).collect();
        let mut ranks: Vec<Rank> = Vec::from_iter(hashset);
        ranks.sort();
        ranks.reverse();
        ranks
    }

    /// Returns a String of all of the Rank Index Characters for a Pile.
    #[must_use]
    pub fn rank_indexes(&self) -> String {
        self.ranks()
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>()
    }

    /// Returns a String of all of the Rank Index Characters for a Pile.
    ///
    /// TODO: There has to be an easier way to do this :-P
    #[must_use]
    pub fn rank_indexes_with_separator(&self, separator: &'static str) -> String {
        self.ranks()
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>()
            .chunks(1)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(separator)
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        let position = self.position(card);
        match position {
            None => None,
            _ => Some(self.0.remove(position?)),
        }
    }

    #[must_use]
    pub fn short_index_for_suit(&self, suit: Suit) -> String {
        let cards = Pile::from_vector(self.cards_by_suit(suit));

        suit.symbol().as_str().to_owned() + " " + &cards.rank_indexes_with_separator(" ")
    }

    #[must_use]
    pub fn short_suit_indexes(&self) -> Vec<String> {
        self.sort()
            .suits()
            .iter()
            .map(|suit| self.short_index_for_suit(*suit))
            .collect::<Vec<String>>()
    }

    /// Returns a String where each line is the short suit index for the Pile.
    /// This format is common to display hands in Bridge.
    #[must_use]
    pub fn short_suit_indexes_to_string(&self) -> String {
        self.short_suit_indexes().join("\n")
    }

    #[must_use]
    pub fn shuffle(&self) -> Pile {
        let mut shuffled = self.clone();
        shuffled.shuffle_in_place();
        shuffled
    }

    pub fn shuffle_in_place(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    #[must_use]
    pub fn sort(&self) -> Pile {
        let mut pile = self.clone();
        pile.sort_in_place();
        pile
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    /// Returns a sorted collection of the unique Suits in a Pile.
    #[must_use]
    pub fn suits(&self) -> Vec<Suit> {
        let hashset: HashSet<Suit> = self.0.iter().map(|c| c.suit).collect();
        let mut suits: Vec<Suit> = Vec::from_iter(hashset);
        suits.sort();
        suits.reverse();
        suits
    }

    pub fn values(&self) -> impl Iterator<Item = &Card> {
        self.0.iter()
    }

    #[must_use]
    pub fn jokers() -> Pile {
        let big_joker = Card::from_index_strings(BIG_JOKER, TRUMP);
        let little_joker = Card::from_index_strings(LITTLE_JOKER, TRUMP);
        Pile::from_vector(vec![big_joker, little_joker])
    }

    #[must_use]
    pub fn canasta_base_single_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_canasta_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards.prepend(&Pile::jokers());
        cards
    }

    #[must_use]
    pub fn canasta_single_deck() -> Pile {
        let mut cards: Pile = Pile::canasta_base_single_deck();

        cards.remove_card(&Card::from_index_strings(THREE, HEARTS));
        cards.remove_card(&Card::from_index_strings(THREE, DIAMONDS));

        cards.prepend(&Pile::canasta_red_threes());
        cards
    }

    fn canasta_red_threes() -> Pile {
        let mut three_hearts = Card::from_index_strings(THREE, HEARTS);
        let mut three_diamonds = Card::from_index_strings(THREE, DIAMONDS);
        three_hearts.weight = 100_001;
        three_diamonds.weight = 100_000;

        Pile::from_vector(vec![three_hearts, three_diamonds])
    }

    #[must_use]
    pub fn euchre_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_euchre_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards.prepend(&Pile::from_vector(vec![Card::from_index_strings(
            BIG_JOKER, TRUMP,
        )]));
        cards
    }

    #[must_use]
    pub fn french_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_french_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards
    }

    #[must_use]
    pub fn french_deck_with_jokers() -> Pile {
        let mut pile = Pile::french_deck();
        pile.prepend(&Pile::jokers());
        pile
    }

    fn pinochle_pile() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_pinochle_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards
    }

    #[must_use]
    pub fn pinochle_deck() -> Pile {
        Pile::pile_up(2, Pile::pinochle_pile).sort()
    }

    #[must_use]
    pub fn short_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_short_deck_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards
    }

    #[must_use]
    pub fn skat_deck() -> Pile {
        let suits = Suit::generate_skat_suits();
        let ranks = Rank::generate_skat_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(&suits, &ranks);
        cards
    }

    #[must_use]
    pub fn spades_deck() -> Pile {
        let mut deck = Pile::french_deck();
        deck.remove_card(&Card::from_index_strings(TWO, CLUBS));
        deck.remove_card(&Card::from_index_strings(TWO, DIAMONDS));
        let jokers = Pile::jokers();

        deck.prepend(&jokers);
        deck
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn tarot_deck() -> Pile {
        let arcana_suits = Suit::generate_arcana_suits();
        let mut arcana_suits_enumerator = arcana_suits.iter().enumerate();
        let major_arcana_ranks = Rank::generate_major_arcana_ranks();
        let minor_arcana_ranks = Rank::generate_minor_arcana_ranks();

        let mut cards: Pile = Pile::default();

        let (_, major_arcana_suit) = arcana_suits_enumerator.next().unwrap();

        // Generate Major Arcana
        for (_, rank) in major_arcana_ranks.iter().enumerate() {
            cards.push(Card::new(*rank, *major_arcana_suit));
        }

        // Generate Minor Arcana
        for (_, suit) in arcana_suits_enumerator {
            for (_, rank) in minor_arcana_ranks.iter().enumerate() {
                cards.push(Card::new(*rank, *suit));
            }
        }

        cards
    }

    #[must_use]
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
        Pile::from_vector(Vec::new())
    }
}

/// Sets the `to_string()` function for a `Pile` to return the default index signature for the `Pile`.
impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self.to_index();
        write!(f, "{}", sig)
    }
}

impl FromIterator<Card> for Pile {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut c = Pile::default();
        for i in iter {
            c.push(i);
        }
        c
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
    use crate::cards::suit::{MAJOR_ARCANA, SPADES};

    #[test]
    fn new_from_vector() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut expected = Pile::default();
        expected.push(qclubs.clone());
        expected.push(qhearts.clone());

        let actual = Pile::from_vector(vec![qclubs, qhearts]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn pile_on() {
        let mut deck = Pile::french_deck();
        let half = deck.draw(26).unwrap();

        let actual = Pile::pile_on(vec![half, deck]);

        assert_eq!(Pile::french_deck(), actual);
    }

    #[test]
    fn append() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let big_joker = Card::from_index_strings(BIG_JOKER, TRUMP);
        let little_joker = Card::from_index_strings(LITTLE_JOKER, TRUMP);
        let mut to_deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::jokers();
        let expected = Pile::from_vector(vec![qclubs, qhearts, big_joker, little_joker]);

        to_deck.append(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn card_by_index() {
        let deck = Pile::spades_deck();
        let expected = Card::from_index_strings(LITTLE_JOKER, TRUMP);

        let card = deck.card_by_index("JLT").unwrap();

        assert_eq!(&expected, card);
    }

    #[test]
    fn card_by_index__ne() {
        let deck = Pile::spades_deck();
        let fool_index = Card::from_index_strings(FOOL, MAJOR_ARCANA).index_default();

        // Verifies that the index for a card in the tarot deck isn't in a spades deck.
        assert!(deck.card_by_index(fool_index.as_str()).is_none());
    }

    #[test]
    fn cards() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let expected = vec![qclubs.clone(), qhearts.clone()];
        let pile = Pile::from_vector(expected.clone());

        let v = pile.cards();

        assert_eq!(&expected, v);
    }

    #[test]
    fn cards_by_suit() {
        let qh = Card::from_index_strings(QUEEN, HEARTS);
        let jh = Card::from_index_strings(JACK, HEARTS);
        let qc = Card::from_index_strings(QUEEN, CLUBS);
        let jc = Card::from_index_strings(JACK, CLUBS);
        let expected = vec![qc.clone(), jc.clone()];

        let pile = Pile::from_vector(vec![jh.clone(), jc.clone(), qh.clone(), qc.clone()]);

        let v = pile.cards_by_suit(qc.suit);

        assert_eq!(expected, v);
    }

    #[test]
    fn short_index_by_suit() {
        let qh = Card::from_index_strings(QUEEN, HEARTS);
        let jh = Card::from_index_strings(JACK, HEARTS);
        let qc = Card::from_index_strings(QUEEN, CLUBS);
        let jc = Card::from_index_strings(JACK, CLUBS);
        let pile = Pile::from_vector(vec![jh.clone(), jc.clone(), qh.clone(), qc.clone()]);

        let expected = String::from("♥ Q J");
        let actual = pile.short_index_for_suit(qh.suit);
        assert_eq!(expected, actual);
        let expected = String::from("♣ Q J");
        let actual = pile.short_index_for_suit(qc.suit);
        assert_eq!(expected, actual);
    }

    #[test]
    fn contains() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

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
    fn contains_all__ne() {
        let deck = Pile::spades_deck();
        let hand = Pile::skat_deck().shuffle().draw(4).unwrap();

        assert!(!deck.contains_all(&hand));
    }

    #[test]
    fn draw() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let qspades = Card::from_index_strings(QUEEN, SPADES);
        let mut deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone(), qspades.clone()]);

        assert_eq!(
            deck.draw(2).unwrap(),
            Pile::from_vector(vec![qclubs.clone(), qhearts.clone()])
        );
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw__empty_deck() {
        let mut zero = Pile::default();

        assert!(zero.draw(1).is_none());
        assert!(zero.draw(2).is_none());
        assert!(zero.draw(0).is_none());
    }

    #[test]
    fn draw__index_too_high() {
        let mut deck = Pile::french_deck();

        assert!(deck.draw(53).is_none());
        assert!(deck.draw(100).is_none());
    }

    #[test]
    fn draw_first() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(deck.draw_first().unwrap(), qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_first__empty_deck() {
        assert!(Pile::default().draw_first().is_none());
    }

    #[test]
    fn draw_last() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(deck.draw_last().unwrap(), qhearts);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_last__empty_deck() {
        assert!(Pile::default().draw_last().is_none());
    }

    #[test]
    fn first() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(deck.first().unwrap(), &qclubs);
    }

    #[test]
    fn first__empty_deck() {
        assert!(Pile::default().first().is_none());
    }

    #[test]
    fn get() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let gotten = deck.get(1);

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn get_random() {
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qhearts.clone()]);

        let gotten = deck.get_random();

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn last() {
        let zero = Pile::default();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.last().is_none());
        assert_eq!(deck.last().unwrap(), &qhearts);
    }

    #[test]
    fn len() {
        let zero = Pile::default();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(zero.len(), 0);
        assert_eq!(deck.len(), 2);
    }

    #[test]
    fn map_by_rank_count() {
        let pile = Pile::french_deck()
            .pile_by_index(&["QS", "9S", "QC", "QH", "QD"])
            .unwrap();
        let mappie = pile.map_by_rank_count();

        assert_eq!(*mappie.get(&Rank::new(QUEEN)).unwrap(), 4);
        assert_eq!(*mappie.get(&Rank::new(NINE)).unwrap(), 1);
        assert!(mappie.get(&Rank::new(EIGHT)).is_none());
    }

    #[test]
    fn map_by_suit() {
        let pile = Pile::french_deck()
            .pile_by_index(&["QS", "9S", "QC", "QH", "QD"])
            .unwrap();
        let qs = pile.get(0).unwrap();
        let qc = pile.get(2).unwrap();
        let spades = Suit::new(SPADES);
        let clubs = Suit::new(CLUBS);

        let mappie = pile.map_by_suit();

        assert_eq!(
            qs.index_default(),
            mappie
                .get(&spades)
                .unwrap()
                .first()
                .unwrap()
                .index_default()
        );
        assert_eq!(
            qc.index_default(),
            mappie.get(&clubs).unwrap().first().unwrap().index_default()
        );
    }

    #[test]
    fn pile_by_index() {
        let deck = Pile::french_deck();
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let expected = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let actual = deck.pile_by_index(&["QC", "QH"]);

        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn pile_by_index_ne() {
        let deck = Pile::french_deck();

        // Try with indexes from other types of decks.
        let actual = deck.pile_by_index(&["UA", "2MA"]);

        assert_eq!(None, actual)
    }

    #[test]
    fn position() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(0, deck.position(&qclubs).unwrap());
        assert_eq!(1, deck.position(&qhearts).unwrap());
    }

    #[test]
    fn prepend() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let big_joker = Card::from_index_strings(BIG_JOKER, SPADES);
        let little_joker = Card::from_index_strings(LITTLE_JOKER, SPADES);
        let mut to_deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::from_vector(vec![big_joker.clone(), little_joker.clone()]);
        let expected = Pile::from_vector(vec![big_joker, little_joker, qclubs, qhearts]);

        to_deck.prepend(&from_deck);

        assert_eq!(expected, to_deck);
    }

    // todo
    #[test]
    fn ranks() {
        let qc = Card::from_index_strings(QUEEN, CLUBS);
        let qh = Card::from_index_strings(QUEEN, HEARTS);
        let jh = Card::from_index_strings(JACK, HEARTS);
        let expected: Vec<Rank> = vec![qc.clone().rank, jh.clone().rank];
        let deck = Pile::from_vector(vec![jh.clone(), qc.clone(), qh.clone()]);

        assert_eq!(expected, deck.ranks());
    }

    #[test]
    fn rank_indexes() {
        let mut deck = Pile::french_deck();
        let expected = "AKQJT".to_string();

        let actual = deck.draw(5).unwrap().rank_indexes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn rank_indexes__shuffled() {
        let qc = Card::from_index_strings(QUEEN, CLUBS);
        let qh = Card::from_index_strings(QUEEN, HEARTS);
        let jh = Card::from_index_strings(JACK, HEARTS);
        let expected = "QJ".to_string();
        let deck = Pile::from_vector(vec![jh.clone(), qc.clone(), qh.clone()]);

        let actual = deck.rank_indexes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn rank_indexes_with_separator() {
        let mut deck = Pile::french_deck();
        let expected = "A K Q J T".to_string();

        let actual = deck.draw(5).unwrap().rank_indexes_with_separator(" ");

        assert_eq!(expected, actual);
    }

    #[test]
    fn remove() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove(0);

        assert_eq!(removed, qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn remove_card() {
        let qclubs = Card::from_index_strings(QUEEN, CLUBS);
        let qhearts = Card::from_index_strings(QUEEN, HEARTS);
        let mut deck = Pile::from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove_card(&qclubs);

        assert_eq!(removed.unwrap(), qclubs);
        assert!(deck.contains(&qhearts));
        assert!(!deck.contains(&qclubs));
    }

    #[test]
    fn short_suit_indexes() {
        let french_deck = Pile::french_deck();
        let expected = vec![
            "♠ A K Q J T 9 8 7 6 5 4 3 2",
            "♥ A K Q J T 9 8 7 6 5 4 3 2",
            "♦ A K Q J T 9 8 7 6 5 4 3 2",
            "♣ A K Q J T 9 8 7 6 5 4 3 2",
        ];

        let actual = french_deck.short_suit_indexes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn short_suit_indexes_to_string() {
        let expected = "♠ A K Q J T 9 8 7 6 5 4 3 2\n♥ A K Q J T 9 8 7 6 5 4 3 2\n♦ A K Q J T 9 8 7 6 5 4 3 2\n♣ A K Q J T 9 8 7 6 5 4 3 2";

        assert_eq!(expected, Pile::french_deck().short_suit_indexes_to_string());
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
        shuffled.sort_in_place();
        assert_eq!(pile, shuffled);
    }

    #[test]
    fn shuffle_in_place() {
        let actual = Pile::pinochle_deck();
        let mut shuffled = Pile::pinochle_deck();
        assert_eq!(actual, shuffled);

        shuffled.shuffle_in_place();
        assert_ne!(actual, shuffled);

        assert_eq!(actual, shuffled.sort());
    }

    #[test]
    fn sig_index() {
        let deck = Pile::spades_deck().draw(4).unwrap();

        let sig_english = deck.to_index();
        let sig_german = deck.to_index_locale(&GERMAN);

        assert_eq!("JBT JLT AS KS".to_string(), sig_english);
        assert_eq!("JGT JKT AS KS".to_string(), sig_german);
    }

    #[test]
    fn sig_symbol_index() {
        let deck = Pile::spades_deck().draw(4).unwrap();

        let sig = deck.to_symbol_index();

        assert_eq!("JB🃟 JL🃟 A♠ K♠".to_string(), sig);
    }

    #[test]
    fn suits() {
        let deck = Pile::french_deck();
        let expected: Vec<Suit> = vec![
            Suit::new(SPADES),
            Suit::new(HEARTS),
            Suit::new(DIAMONDS),
            Suit::new(CLUBS),
        ];

        let suits = deck.suits();

        assert_eq!(expected, suits);
    }

    #[test]
    fn to_index() {
        let expected = "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C".to_string();
        assert_eq!(expected, Pile::french_deck().to_index())
    }

    #[test]
    fn to_index_str() {
        let expected = "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C";
        assert_eq!(expected, Pile::french_deck().to_index_str())
    }

    #[test]
    fn to_symbol_index() {
        let expected = "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣".to_string();
        assert_eq!(expected, Pile::french_deck().to_symbol_index())
    }

    #[test]
    fn to_string() {
        let deck = Pile::french_deck().draw(4);

        assert_eq!("AS KS QS JS".to_string(), deck.unwrap().to_string());
    }

    #[test]
    fn sort() {
        let decks = vec![
            Pile::french_deck(),
            Pile::french_deck_with_jokers(),
            Pile::skat_deck(),
            Pile::spades_deck(),
            Pile::tarot_deck(),
        ];

        for deck in decks {
            let shuffled = deck.shuffle();
            assert_ne!(deck, shuffled);
            assert_eq!(deck, shuffled.sort());
        }
    }

    #[test]
    fn len__french_deck_with_jokers() {
        let deck = Pile::french_deck_with_jokers();

        assert_eq!(54, deck.len());
    }

    #[test]
    fn short_deck() {
        let deck = Pile::short_deck();

        assert!(!deck.contains(&Card::from_index_strings(FIVE, CLUBS)));
        assert!(!deck.contains(&Card::from_index_strings(TWO, CLUBS)));
        assert!(!deck.contains(&Card::from_index_strings(TWO, DIAMONDS)));
        assert!(!deck.contains(&Card::from_index_strings(TWO, SPADES)));
    }

    #[test]
    fn spades_deck() {
        let deck = Pile::spades_deck();

        assert!(!deck.contains(&Card::from_index_strings(TWO, CLUBS)));
        assert!(!deck.contains(&Card::from_index_strings(TWO, DIAMONDS)));
        assert!(deck.contains(&Card::from_index_strings(TWO, SPADES)));
    }
}
