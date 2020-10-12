use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use unic_langid::LanguageIdentifier;

use crate::cards::card::Card;
use crate::cards::rank::*;
use crate::cards::suit::*;
use crate::fluent::named::*;
use crate::Named;

/// A Pile is a sortable collection of Cards.
///
/// # Usage:
/// ```
/// let mut pile = cardpack::Pile::default();
/// let ace_of_spades = cardpack::Card::new(cardpack::ACE, cardpack::SPADES);
/// let ace_of_hearts = cardpack::Card::new(cardpack::ACE, cardpack::HEARTS);
/// pile.add(ace_of_spades);
/// pile.add(ace_of_hearts);
/// pile.shuffle();

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Pile(Vec<Card>);

impl Pile {
    pub fn new_from_vector(v: Vec<Card>) -> Pile {
        Pile(v)
    }

    /// Takes a reference to an Array of Piles and consolidates them into a single Pile of Cards.
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
            pile.push(f())
        }
        Pile::pile_on(pile)
    }

    /// Places the Card at the bottom (end) of the Pile.
    pub fn add(&mut self, elem: Card) {
        self.0.push(elem);
    }

    /// Appends a clone of the passed in Pile of Cards to the existing Pile.
    pub fn append(&mut self, other: &Pile) {
        self.0.append(&mut other.0.clone());
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

    pub fn card_by_index(&self, index: &str) -> Option<&Card> {
        self.0.iter().find(|c| c.index_default() == index)
    }

    /// Returns a reference to the Vector containing all the cards.
    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    fn collect_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.index(lid)).collect()
    }

    fn collect_symbol_index(&self, lid: &LanguageIdentifier) -> Vec<String> {
        self.0.iter().map(|s| s.symbol_colorized(lid)).collect()
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
            let anzugname = card.suit.name.long(&GERMAN);
            let suitname = card.suit.name.long(&US_ENGLISH);
            let rangname = card.rank.name.long(&GERMAN);
            let rankname = card.rank.name.long(&US_ENGLISH);
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
        let shuffled = self.shuffle();
        print!("{}", shuffled.to_string());

        println!();
        print!("   Sort Deck:              ");
        print!("{}", shuffled.sort().to_string());

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

    fn fold_in(&mut self, suits: Vec<Suit>, ranks: Vec<Rank>) {
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                self.add(Card::new_from_structs(*rank, *suit));
            }
        }
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

    /// Takes a pile and returns a HashMap with the key as each Suit in the Pile with the values
    /// as a Pile of the cards for that Suit.
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

    pub fn position(&self, karte: &Card) -> Option<usize> {
        self.0.iter().position(|k| k.index == karte.index )
    }

    pub fn pile_by_index(&self, indexes: &[&str]) -> Option<Pile> {
        let mut pile = Pile::default();
        for index in indexes {
            let card = self.card_by_index(index);
            match card {
                Some(c) => pile.add(c.clone()),
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

    /// Returns a String of all of the Rank Index Characters for a Pile.
    pub fn rank_indexes(&self) -> String {
        self.cards()
            .iter()
            .map(|c| c.rank.name.index_default())
            .collect::<String>()
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

    pub fn jokers() -> Pile {
        let big_joker = Card::new(BIG_JOKER, TRUMP);
        let little_joker = Card::new(LITTLE_JOKER, TRUMP);
        Pile::new_from_vector(vec![big_joker, little_joker])
    }

    pub fn canasta_single_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_canasta_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards.prepend(&Pile::jokers());

        cards.remove_card(&Card::new(THREE, HEARTS));
        cards.remove_card(&Card::new(THREE, DIAMONDS));

        cards.prepend(&Pile::canasta_red_threes());
        cards
    }

    fn canasta_red_threes() -> Pile {
        let mut three_hearts = Card::new(THREE, HEARTS);
        let mut three_diamonds = Card::new(THREE, DIAMONDS);
        three_hearts.weight = 100001;
        three_diamonds.weight = 100000;

        Pile::new_from_vector(vec![three_hearts, three_diamonds])

    }

    pub fn euchre_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_euchre_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards.prepend(&Pile::new_from_vector(vec![Card::new(BIG_JOKER, TRUMP)]));
        cards
    }

    pub fn french_deck() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_french_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards
    }

    pub fn french_deck_with_jokers() -> Pile {
        let mut pile = Pile::french_deck();
        pile.prepend(&Pile::jokers());
        pile
    }

    fn pinochle_pile() -> Pile {
        let suits = Suit::generate_french_suits();
        let ranks = Rank::generate_pinochle_ranks();

        let mut cards: Pile = Pile::default();
        cards.fold_in(suits, ranks);
        cards
    }

    pub fn pinochle_deck() -> Pile {
        Pile::pile_up(2, Pile::pinochle_pile).sort()
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
        deck.remove_card(&Card::new(TWO, CLUBS));
        deck.remove_card(&Card::new(TWO, DIAMONDS));
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
            cards.add(Card::new_from_structs(*rank, *major_arcana_suit));
        }

        // Generate Minor Arcana
        for (_, suit) in arcana_suits_enumerator {
            for (_, rank) in minor_arcana_ranks.iter().enumerate() {
                cards.add(Card::new_from_structs(*rank, *suit));
            }
        }

        cards
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

impl FromIterator<Card> for Pile {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut c = Pile::default();
        for i in iter {
            c.add(i);
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

    #[test]
    fn new_all_add_new_from_vector() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let mut expected = Pile::default();
        expected.add(qclubs.clone());
        expected.add(qhearts.clone());

        let actual = Pile::new_from_vector(vec![qclubs, qhearts]);

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
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let big_joker = Card::new(BIG_JOKER, TRUMP);
        let little_joker = Card::new(LITTLE_JOKER, TRUMP);
        let mut to_deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::jokers();
        let expected = Pile::new_from_vector(vec![qclubs, qhearts, big_joker, little_joker]);

        to_deck.append(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn card_by_index() {
        let deck = Pile::spades_deck();
        let expected = Card::new(LITTLE_JOKER, TRUMP);

        let card = deck.card_by_index("JLT").unwrap();

        assert_eq!(&expected, card);
    }

    #[test]
    fn card_by_index_ne() {
        let deck = Pile::spades_deck();
        let fool_index = Card::new(FOOL, MAJOR_ARCANA).index_default();

        // Verifies that the index for a card in the tarot deck isn't in a spades deck.
        assert!(deck.card_by_index(fool_index.as_str()).is_none());
    }

    #[test]
    fn contains() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
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
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let qspades = Card::new(QUEEN, SPADES);
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
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_first().is_none());
        assert_eq!(deck.draw_first().unwrap(), qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_last() {
        let mut zero = Pile::default();
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_last().is_none());
        assert_eq!(deck.draw_last().unwrap(), qhearts);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn first() {
        let zero = Pile::default();
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.first().is_none());
        assert_eq!(deck.first().unwrap(), &qclubs);
    }

    #[test]
    fn get() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let gotten = deck.get(1);

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn get_random() {
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qhearts.clone()]);

        let gotten = deck.get_random();

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn last() {
        let zero = Pile::default();
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.last().is_none());
        assert_eq!(deck.last().unwrap(), &qhearts);
    }

    #[test]
    fn len() {
        let zero = Pile::default();
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(zero.len(), 0);
        assert_eq!(deck.len(), 2);
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
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let expected = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

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
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(0, deck.position(&qclubs).unwrap());
        assert_eq!(1, deck.position(&qhearts).unwrap());
    }

    #[test]
    fn prepend() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let big_joker = Card::new(BIG_JOKER, SPADES);
        let little_joker = Card::new(LITTLE_JOKER, SPADES);
        let mut to_deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);
        let from_deck = Pile::new_from_vector(vec![big_joker.clone(), little_joker.clone()]);
        let expected = Pile::new_from_vector(vec![big_joker, little_joker, qclubs, qhearts]);

        to_deck.prepend(&from_deck);

        assert_eq!(expected, to_deck);
    }

    #[test]
    fn rank_indexes() {
        let mut deck = Pile::french_deck();
        let expected = "AKQJT".to_string();

        let actual = deck.draw(5).unwrap().rank_indexes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn remove() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
        let mut deck = Pile::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove(0);

        assert_eq!(removed, qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn remove_card() {
        let qclubs = Card::new(QUEEN, CLUBS);
        let qhearts = Card::new(QUEEN, HEARTS);
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

        let sig_english = deck.by_index();
        let sig_german = deck.by_index_locale(&GERMAN);

        assert_eq!("JBT JLT AS KS".to_string(), sig_english);
        assert_eq!("JGT JKT AS KS".to_string(), sig_german);
    }

    #[test]
    fn sig_symbol_index() {
        let deck = Pile::spades_deck().draw(4).unwrap();

        let sig = deck.by_symbol_index();

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
    fn to_string() {
        let deck = Pile::french_deck().draw(4);

        let sig = deck.unwrap().to_string();

        assert_eq!("AS KS QS JS".to_string(), sig);
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
    fn french_deck_with_jokers() {
        let deck = Pile::french_deck_with_jokers();

        assert_eq!(54, deck.len());
    }

    #[test]
    fn spades_deck() {
        let deck = Pile::spades_deck();

        assert!(!deck.contains(&Card::new(TWO, CLUBS)));
        assert!(!deck.contains(&Card::new(TWO, DIAMONDS)));
        assert!(deck.contains(&Card::new(TWO, SPADES)));
    }
}
