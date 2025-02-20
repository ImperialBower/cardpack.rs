use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::DeckedBase;
use crate::common::errors::CardError;
use crate::prelude::{Decked, Pile};
use colored::Color;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;
use std::vec::IntoIter;

/// A `Pack` is a collection of `Cards` that are bound by a specific generic `DeckType`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Deck<DeckType: DeckedBase>(Vec<Card<DeckType>>)
where
    DeckType: Default + Ord + Copy + Hash;

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> Deck<DeckType> {
    /// The `CoPilot` recommended
    ///
    /// ```txt
    /// self.0
    ///     .iter()
    ///     .position(|card| card.index() == index.into())
    ///     .map(|index| self.0[index])
    /// ```
    ///
    /// Shit like this is why I consider switching to Zig. The nightmare
    /// of recursive trait requirements for generics is maddening. I'm sorry, but
    /// `impl<DeckType: DeckedBase + Default + Ord + Copy + Hash + DeckType>` just looks
    /// horrible. Let me save and see if it actually works, LOL. If it works, what do I
    /// care.
    ///
    /// Why TF not just use `Card::from_str()?` I guess the big difference is that
    /// the card is actually in the Pile in question. Do I need this?
    ///
    /// ANSWER: Turns out it's used by the `BridgeBoard` example.
    #[must_use]
    pub fn card_by_index<S: Into<String>>(&self, index: S) -> Option<Card<DeckType>> {
        match Card::<DeckType>::from_str(index.into().as_str()) {
            Ok(c) => {
                if self.contains(&c) {
                    Some(c)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn cards(&self) -> &Vec<Card<DeckType>> {
        &self.0
    }

    #[must_use]
    pub fn contains(&self, card: &Card<DeckType>) -> bool {
        self.0.contains(card)
    }

    pub fn demo_cards(&self, verbose: bool) {
        let deck = self.sort();
        let shuffled = deck.shuffled();
        let name = Self::deck_name();

        println!();
        println!("{name} Deck:          {}", deck.to_color_symbol_string());
        println!("{name} Deck Index:    {}", deck.index());
        println!(
            "{name} Deck Shuffled: {}",
            shuffled.to_color_symbol_string()
        );

        if verbose {
            println!();
            println!("Long in English and German:");

            for card in deck {
                let name = card.fluent_name_default();
                println!("  {name} ");
            }
        }
    }

    /// `CoPilot`'s original recommendation:
    ///
    /// ```txt
    /// if n > self.len() {
    ///     return None;
    /// }
    ///
    /// let mut rng = rand::thread_rng();
    /// let cards = self.0.choose_multiple(&mut rng, n);
    ///
    ///         Some(Pile::<DeckType>::from(cards.cloned().collect()))
    /// ```
    ///
    /// What's missing:
    ///
    /// - Doesn't check for an n value of 0.
    /// - Returns the cards randomly, which is cute, but isn't the contract.
    #[must_use]
    pub fn draw(&mut self, n: usize) -> Option<Self> {
        if n > self.len() {
            return None;
        }

        let mut cards = Deck::<DeckType>::default();
        for _ in 0..n {
            cards.push(self.draw_first()?);
        }
        Some(cards)
    }

    /// My original code was:
    ///```txt
    /// match self.len() {
    ///     0 => None,
    ///     _ => Some(self.remove(0)),
    /// }
    ///```
    ///
    /// And here's `CoPilot`'s
    ///```txt
    /// self.0.first().copied()
    ///```
    ///
    /// Notice the difference. Their's doesn't remove the card from the deck.
    pub fn draw_first(&mut self) -> Option<Card<DeckType>> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// Copilot recommends this hacky version My final version is much simpler, since the original
    /// code it's calling does all the heavy lifting.
    ///
    /// ```txt
    /// match self.len() {
    ///    0 => None,
    ///     _ => Some(self.remove(self.len() - 1)),
    /// }
    pub fn draw_last(&mut self) -> Option<Card<DeckType>> {
        self.0.pop()
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    #[must_use]
    pub fn forgiving_from_str(index: &str) -> Self {
        Deck::<DeckType>::from_str(index).unwrap_or_default()
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<&Card<DeckType>> {
        self.0.get(position)
    }

    #[must_use]
    pub fn index(&self) -> String {
        self.0
            .iter()
            .map(Card::index)
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[must_use]
    pub fn into_basic_cards(&self) -> Vec<BasicCard> {
        self.0.iter().map(Card::base).collect()
    }

    #[must_use]
    pub fn into_hashset(&self) -> HashSet<Card<DeckType>> {
        self.0.iter().copied().collect()
    }

    #[must_use]
    pub fn into_pile(&self) -> Pile {
        Pile::from(self)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Which one? Do I need this? Why did I create this? Sigh.
    // I need to clearly work through these nauces.
    // #[must_use]
    // pub fn iter(&self) -> std::vec::IntoIter<Card<RankType, SuitType>> {
    //     <&Self as IntoIterator>::into_iter(self)
    // }
    // or
    pub fn iter(&self) -> std::slice::Iter<Card<DeckType>> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn map_by_suit(&self) -> HashMap<Pip, Deck<DeckType>> {
        let mut map: HashMap<Pip, Deck<DeckType>> = HashMap::new();

        for card in &self.0 {
            let suit = card.base_card.suit;
            let entry = map.entry(suit).or_default();
            entry.push(*card);
        }

        map
    }

    #[must_use]
    pub fn pile_on(piles: &[Self]) -> Self {
        let mut pile = Self::default();

        for p in piles {
            pile.extend(p);
        }

        pile
    }

    /// Returns a Pile by calling the passed in function n times and consolidating the results into
    /// a single Pile.
    ///
    /// NOTE: why?
    pub fn pile_up(n: usize, f: fn() -> Self) -> Self {
        let mut pile = Self::default();

        for _ in 0..n {
            pile.extend(&f());
        }

        pile
    }

    #[must_use]
    pub fn piles_to_string(piles: &[Self]) -> String {
        piles
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }

    #[must_use]
    pub fn position(&self, card: &Card<DeckType>) -> Option<usize> {
        self.0.iter().position(|c| c == card)
    }

    pub fn prepend(&mut self, other: &Deck<DeckType>) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    pub fn push(&mut self, card: Card<DeckType>) {
        self.0.push(card);
    }

    pub fn pop(&mut self) -> Option<Card<DeckType>> {
        self.0.pop()
    }

    /// TODO: Possible RF change to [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)?
    pub fn remove(&mut self, x: usize) -> Card<DeckType> {
        self.0.remove(x)
    }

    pub fn remove_card(&mut self, card: &Card<DeckType>) -> Option<Card<DeckType>> {
        let position = self.position(card)?;
        Some(self.remove(position))
    }

    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut pile = self.clone();
        pile.reverse_in_place();
        pile
    }

    pub fn reverse_in_place(&mut self) {
        self.0.reverse();
    }

    #[must_use]
    pub fn same(&self, cards: &Deck<DeckType>) -> bool {
        let left = self.sort();
        let right = cards.sort();

        left == right
    }

    /// `shuffled` feels so much better. Nice and succinct.
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    #[must_use]
    pub fn sort(&self) -> Self {
        let mut pile = self.clone();
        pile.sort_in_place();
        pile
    }

    #[must_use]
    pub fn sort_by_rank(&self) -> Self {
        let mut pile = self.clone();
        pile.sort_by_rank_in_place();
        pile
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort();
    }

    /// OK, so here's an example of how I am stupid. I want to be able to sort by `Rank` as well as
    /// by the default by `Suit`.
    ///
    /// By default, the sort places the lowest rank
    /// first. The same for suits, which is why we override the default order for a `Card`. Here's
    /// the first stab at it: (You'll have to forgive me for the backbending I need to do in order
    /// to expose the interals of the sort)
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cardpack::prelude::{Deck, Standard52};
    /// let pack = Deck::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| a.base_card.rank.cmp(&b.base_card.rank));
    ///
    /// assert_eq!(Deck::<Standard52>::from(v).to_string(), "2♠ 4♠ 8♠");
    /// ```
    ///
    /// OK, so it's in the wrong direction.
    ///
    /// Q: So, what's the best way to do that?
    ///
    /// ...
    ///
    /// A: Reverse it, of course.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cardpack::prelude::{Deck, Standard52};
    /// let pack = Deck::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| a.base_card.rank.cmp(&b.base_card.rank));
    /// v.reverse();
    ///
    /// assert_eq!(Deck::<Standard52>::from(v).to_string(), "8♠ 4♠ 2♠");
    /// ```
    ///
    /// Boom! That does it. Prolem solved. But wait, once again we ask the question once we have
    /// made the test pass as desired. Could it be better? Could we refactor it?
    ///
    /// Can you find it? As usual, it's so hard to find because it's so simple.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cardpack::prelude::{Deck, Standard52};
    /// let pack = Deck::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| b.base_card.rank.cmp(&a.base_card.rank));
    ///
    /// assert_eq!(Deck::<Standard52>::from(v).to_string(), "8♠ 4♠ 2♠");
    /// ```
    pub fn sort_by_rank_in_place(&mut self) {
        self.0
            .sort_by(|a, b| b.base_card.rank.cmp(&a.base_card.rank));
    }

    pub fn to_color_symbol_string(&self) -> String {
        self.0
            .iter()
            .map(Card::color_symbol_string)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl<DeckType: DeckedBase + Ord + Default + Copy + Hash> DeckedBase for Deck<DeckType> {
    fn base_vec() -> Vec<BasicCard> {
        DeckType::base_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        DeckType::colors()
    }

    fn deck_name() -> String {
        DeckType::deck_name()
    }

    fn fluent_deck_key() -> String {
        DeckType::fluent_deck_key()
    }
}

impl<DeckType: DeckedBase + Ord + Default + Copy + Hash> Decked<DeckType> for Deck<DeckType> {}

impl<DeckType: DeckedBase + Default + Copy + Ord + Hash> Display for Deck<DeckType> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(Card::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{s}")
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<HashSet<Card<DeckType>>>
    for Deck<DeckType>
{
    fn from(cards: HashSet<Card<DeckType>>) -> Self {
        Self(cards.into_iter().collect()).sort()
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<Vec<Card<DeckType>>>
    for Deck<DeckType>
{
    fn from(cards: Vec<Card<DeckType>>) -> Self {
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<Vec<BasicCard>> for Deck<DeckType> {
    fn from(cards: Vec<BasicCard>) -> Self {
        let cards = Deck::<DeckType>::into_cards(&cards);
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> FromStr for Deck<DeckType> {
    type Err = CardError;

    fn from_str(index: &str) -> Result<Self, Self::Err> {
        let (good, bad): (Vec<_>, Vec<_>) = index
            .split_whitespace()
            .map(Card::<DeckType>::from_str)
            .partition(Result::is_ok);

        if !bad.is_empty() {
            return Err(CardError::InvalidCard(index.to_string()));
        }

        Ok(Deck::<DeckType>::from(
            good.into_iter()
                .map(Result::unwrap_or_default)
                .collect::<Vec<_>>(),
        ))
    }
}

impl<Decked> FromIterator<Card<Decked>> for Deck<Decked>
where
    Decked: DeckedBase + Default + Ord + Copy + Hash,
{
    fn from_iter<I: IntoIterator<Item = Card<Decked>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// This feels like a non-sequitor. Into means that it is iterated
/// by value, so why would I want a reference?
impl<Decked> IntoIterator for &Deck<Decked>
where
    Decked: DeckedBase + Default + Ord + Copy + Hash,
{
    type Item = Card<Decked>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> IntoIterator for Deck<DeckType> {
    type Item = Card<DeckType>;
    type IntoIter = IntoIter<Card<DeckType>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__deck_tests {
    use super::*;
    use crate::basic::decks::cards::french::{FrenchBasicCard, FrenchRank, FrenchSuit};
    use crate::basic::decks::french::French;
    use crate::basic::decks::standard52::Standard52;
    use crate::basic::types::traits::DeckedBase;

    #[test]
    fn basic_cards() {
        let pile = Deck::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();

        assert_eq!(
            pile.into_basic_cards(),
            vec![
                FrenchBasicCard::DEUCE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::FOUR_SPADES
            ]
        );
    }

    #[test]
    fn card_by_index() {
        let pile = Deck::<Standard52>::deck();

        assert_eq!(
            pile.card_by_index("2s"),
            Some(Card::<Standard52>::from(FrenchBasicCard::DEUCE_SPADES))
        );
        assert_eq!(pile.card_by_index("BJ"), None);
    }

    #[test]
    fn contains() {
        let pile = Deck::<French>::from_str("2♠ 8♠ 4♠").unwrap();

        assert!(pile.contains(&Card::<French>::from(FrenchBasicCard::DEUCE_SPADES)));
        assert!(!pile.contains(&Card::<French>::from(FrenchBasicCard::ACE_SPADES)));
    }

    #[test]
    fn draw() {
        let mut pile = Deck::<French>::from_str("2♠ 8♠ 4♠").unwrap();

        assert_eq!(pile.draw(3).unwrap().to_string(), "2♠ 8♠ 4♠");
        assert_eq!(pile.draw(3), None);
    }

    #[test]
    pub fn forgiving_from_str() {
        assert_eq!(
            Deck::<French>::forgiving_from_str("2♠ 8s 4♠").to_string(),
            "2♠ 8♠ 4♠"
        );
        assert_eq!(
            Deck::<French>::forgiving_from_str("2♠ XX 4♠").to_string(),
            ""
        );
    }

    #[test]
    pub fn get() {
        let pile = Deck::<Standard52>::deck();

        assert_eq!(pile.get(0).unwrap().to_string(), "A♠");
        assert_eq!(pile.get(51).unwrap().to_string(), "2♣");
        assert!(pile.get(52).is_none());
    }

    #[test]
    fn into_hashset() {
        let five_deck = Deck::<French>::decks(5);

        let hashset: HashSet<Card<French>> = five_deck.into_hashset();
        let deck = Deck::<French>::from(hashset);

        assert_eq!(five_deck.len(), 270);
        assert_eq!(deck, Deck::<French>::deck());
    }

    #[test]
    fn map_by_suit() {
        let pile = Deck::<Standard52>::deck();

        let map = pile.map_by_suit();

        assert_eq!(map.len(), 4);
        assert_eq!(
            map[&FrenchSuit::SPADES].to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠"
        );
        assert_eq!(
            map[&FrenchSuit::HEARTS].to_string(),
            "A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥"
        );
        assert_eq!(
            map[&FrenchSuit::DIAMONDS].to_string(),
            "A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦"
        );
        assert_eq!(
            map[&FrenchSuit::CLUBS].to_string(),
            "A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        )
    }

    #[test]
    fn pile_on() {
        let pile1 = Deck::<French>::from_str("2♠ 8♠ 4♠").unwrap();
        let pile2 = Deck::<French>::from_str("5♠ 6♠ 7♠").unwrap();
        let piles = vec![pile1, pile2];

        let pile = Deck::<French>::pile_on(&piles);

        assert_eq!(pile.to_string(), "2♠ 8♠ 4♠ 5♠ 6♠ 7♠");
    }

    #[test]
    fn pile_up() {
        fn ak() -> Deck<French> {
            Deck::<French>::from_str("A♠ K♠").unwrap()
        }

        let pile = Deck::<French>::pile_up(3, ak);

        assert_eq!(pile.to_string(), "A♠ K♠ A♠ K♠ A♠ K♠");
    }

    #[test]
    fn position() {
        let pile = Deck::<French>::deck();

        let card = Card::<French>::from_str("2♣").unwrap();

        assert_eq!(pile.position(&card).unwrap(), 53);
    }

    #[test]
    fn prepend() {
        let mut pile1 = Deck::<French>::from_str("2♠ 8♠ 4♠").unwrap();
        let pile2 = Deck::<French>::from_str("5♠ 6♠ 7♠").unwrap();

        pile1.prepend(&pile2);

        assert_eq!(pile1.to_string(), "5♠ 6♠ 7♠ 2♠ 8♠ 4♠");
    }

    #[test]
    fn push() {
        let mut pile = Deck::<French>::default();

        pile.push(Card::default());
        pile.push(Card::<French>::from(FrenchBasicCard::DEUCE_CLUBS));

        assert_eq!(pile.len(), 2);
        assert_eq!(pile.to_string(), "__ 2♣");
    }

    #[test]
    fn remove() {
        let mut pile = Deck::<Standard52>::deck();
        let card = pile.remove(1);

        assert_eq!(card.to_string(), "K♠");
        assert_eq!(pile.draw(2).unwrap().to_string(), "A♠ Q♠");
    }

    #[test]
    fn remove_card() {
        let mut pile = Deck::<Standard52>::deck();
        pile.remove_card(&Card::<Standard52>::from_str("K♠").unwrap());

        let actual = pile.draw(2).unwrap();

        assert_eq!(actual, Deck::<Standard52>::from_str("AS QS").unwrap());
    }

    #[test]
    fn to_color_symbol_string() {
        let pile = Deck::<French>::from_str("2c 3c 4c").unwrap();

        // println!("{}", Pile::<French>::deck().to_color_symbol_string());

        assert_eq!(pile.to_color_symbol_string(), "2♣ 3♣ 4♣");
    }

    #[test]
    fn sort() {
        let pile = Deck::<French>::from_str("2♠ 8♣ 4♠").unwrap();
        let mut pile2 = pile.clone();
        let mut pile3 = pile.clone();

        pile2.sort_in_place();
        pile3.sort_by_rank_in_place();

        assert_eq!(pile.sort().to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile2.to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile.sort_by_rank().to_string(), "8♣ 4♠ 2♠");
        assert_eq!(pile3.to_string(), "8♣ 4♠ 2♠");
    }

    #[test]
    fn decked__deck() {
        let french = Deck::<French>::deck();
        let standard52 = Deck::<Standard52>::deck();

        assert_eq!(french.len(), 54);
        assert_eq!(standard52.len(), 52);
        assert_eq!(
            french.to_string(),
            "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
        assert_eq!(
            standard52.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
    }

    #[test]
    fn decked__decks() {
        let hand_and_foot = Deck::<French>::decks(5).sort();

        assert_eq!(hand_and_foot.len(), 270);
        assert_eq!(
            hand_and_foot.to_string(),
            "B🃟 B🃟 B🃟 B🃟 B🃟 L🃟 L🃟 L🃟 L🃟 L🃟 A♠ A♠ A♠ A♠ A♠ K♠ K♠ K♠ K♠ K♠ Q♠ Q♠ Q♠ Q♠ Q♠ J♠ J♠ J♠ J♠ J♠ T♠ T♠ T♠ T♠ T♠ 9♠ 9♠ 9♠ 9♠ 9♠ 8♠ 8♠ 8♠ 8♠ 8♠ 7♠ 7♠ 7♠ 7♠ 7♠ 6♠ 6♠ 6♠ 6♠ 6♠ 5♠ 5♠ 5♠ 5♠ 5♠ 4♠ 4♠ 4♠ 4♠ 4♠ 3♠ 3♠ 3♠ 3♠ 3♠ 2♠ 2♠ 2♠ 2♠ 2♠ A♥ A♥ A♥ A♥ A♥ K♥ K♥ K♥ K♥ K♥ Q♥ Q♥ Q♥ Q♥ Q♥ J♥ J♥ J♥ J♥ J♥ T♥ T♥ T♥ T♥ T♥ 9♥ 9♥ 9♥ 9♥ 9♥ 8♥ 8♥ 8♥ 8♥ 8♥ 7♥ 7♥ 7♥ 7♥ 7♥ 6♥ 6♥ 6♥ 6♥ 6♥ 5♥ 5♥ 5♥ 5♥ 5♥ 4♥ 4♥ 4♥ 4♥ 4♥ 3♥ 3♥ 3♥ 3♥ 3♥ 2♥ 2♥ 2♥ 2♥ 2♥ A♦ A♦ A♦ A♦ A♦ K♦ K♦ K♦ K♦ K♦ Q♦ Q♦ Q♦ Q♦ Q♦ J♦ J♦ J♦ J♦ J♦ T♦ T♦ T♦ T♦ T♦ 9♦ 9♦ 9♦ 9♦ 9♦ 8♦ 8♦ 8♦ 8♦ 8♦ 7♦ 7♦ 7♦ 7♦ 7♦ 6♦ 6♦ 6♦ 6♦ 6♦ 5♦ 5♦ 5♦ 5♦ 5♦ 4♦ 4♦ 4♦ 4♦ 4♦ 3♦ 3♦ 3♦ 3♦ 3♦ 2♦ 2♦ 2♦ 2♦ 2♦ A♣ A♣ A♣ A♣ A♣ K♣ K♣ K♣ K♣ K♣ Q♣ Q♣ Q♣ Q♣ Q♣ J♣ J♣ J♣ J♣ J♣ T♣ T♣ T♣ T♣ T♣ 9♣ 9♣ 9♣ 9♣ 9♣ 8♣ 8♣ 8♣ 8♣ 8♣ 7♣ 7♣ 7♣ 7♣ 7♣ 6♣ 6♣ 6♣ 6♣ 6♣ 5♣ 5♣ 5♣ 5♣ 5♣ 4♣ 4♣ 4♣ 4♣ 4♣ 3♣ 3♣ 3♣ 3♣ 3♣ 2♣ 2♣ 2♣ 2♣ 2♣"
        );
    }

    #[test]
    fn default() {
        let pile = Deck::<French>::default();
        assert_eq!(pile.len(), 0);
    }

    #[test]
    fn display() {}

    #[test]
    fn from_vec() {
        let base_cards = Card::<French>::base_vec();

        let cards = Deck::<French>::into_cards(&base_cards);

        let pile = Deck::<French>::from(cards.clone());

        assert_eq!(*pile.cards(), cards);
    }

    #[test]
    fn to_string__from_str() {
        let deck = Deck::<French>::deck();
        let deck_str = deck.to_string();
        let deck_from_str = Deck::<French>::from_str(&deck_str).unwrap().shuffled();

        assert_eq!(
            deck_str,
            "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
        assert!(deck.same(&deck_from_str));
        assert_eq!(deck, deck_from_str.sort());
    }
}
