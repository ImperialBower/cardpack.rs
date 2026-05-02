use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{DeckedBase, Ranged};
use crate::common::errors::CardError;
use crate::prelude::{BasicPile, Decked};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::{IntoIter, Vec};
#[cfg(feature = "colored-display")]
use colored::Color;
use core::fmt::Display;
use core::hash::Hash;
use core::str::FromStr;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
// HashMap is gated on `colored-display` rather than `std` because it is only
// used by the `colors() -> HashMap<Pip, Color>` impl. `colored-display`
// transitively requires `std` (per Cargo.toml `[features]`), so this gate is
// strictly tighter than gating on `std` directly.
#[cfg(feature = "colored-display")]
use std::collections::HashMap;

/// A `Pile` is a [generic data type](https://doc.rust-lang.org/book/ch10-01-syntax.html)
/// collection of [`Cards`](Card) that are bound by a
/// specific deck  type parameter.
///
/// The magic behind all this is enabled by implementing the [`Decked`] and [`DeckedBase`] traits.
/// [`DeckedBase`] defines the [`BasicCards`](BasicCard) that
/// hold the data that is in the [`Cards`](Card) of the `Pile`, and the
/// [`Decked`] trait that ensures that only [`Cards`](Card) that fit
/// the contract defined in the specific deck implementation trait, such as
/// [`French`](crate::basic::decks::french::French) for a traditional pack of cards with jokers, or
/// [`Pinochle`](crate::basic::decks::pinochle::Pinochle). This makes it possible for the users
/// to define a `Pile` of [`Cards`](Card) through simple strings. Here's
/// an example:
///
/// ```
/// use cardpack::prelude::*;
///
/// let hand: Pile<Standard52> = Pile::<Standard52>::from_str("AD KD QD JD TD").unwrap();
///
/// assert_eq!(hand.len(), 5);
/// assert_eq!(hand.to_string(), "A♦ K♦ Q♦ J♦ T♦");
/// ```
///
/// ```
/// use cardpack::prelude::*;
///
/// let mut deck = Standard52::deck();
///
/// assert_eq!(deck.ranks_index(" "), "A K Q J T 9 8 7 6 5 4 3 2");
/// assert_eq!(deck.suit_symbol_index(" "), "♠ ♥ ♦ ♣");
/// assert_eq!(deck.suits_index(" "), "S H D C");
/// assert_eq!(deck.draw(5).unwrap().to_string(), "A♠ K♠ Q♠ J♠ T♠");
/// assert_eq!(deck.len(), 47);
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pile<DeckType: DeckedBase>(Vec<Card<DeckType>>)
where
    DeckType: Default + Ord + Copy + Hash;

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> Pile<DeckType> {
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
        Card::<DeckType>::from_str(index.into().as_str())
            .map_or(None, |c| if self.contains(&c) { Some(c) } else { None })
    }

    /// Returns a reference to the underlying [`Card`] vector for the Pile.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    ///
    /// assert_eq!(pile.cards(), &vec![card!(2S), card!(8S), card!(4S)]);
    /// ```
    #[must_use]
    pub fn cards(&self) -> &Vec<Card<DeckType>> {
        &self.0
    }

    /// Returns true if the passed in [`Card`] is in the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    ///
    /// assert!(pile.contains(&card!(2S)));
    /// assert!(!pile.contains(&card!(AS)));
    /// ```
    #[must_use]
    pub fn contains(&self, card: &Card<DeckType>) -> bool {
        self.0.contains(card)
    }

    /// Construct a `Pile<DeckType>` from a slice of [`BasicCard`]s.
    ///
    /// More ergonomic than `Pile::from(slice.to_vec())`. Note that this
    /// allocates — `Pile` wraps a `Vec` and is not const-constructible.
    /// To define `BasicCard` collections at compile time, use a
    /// `const ARRAY: [BasicCard; N] = [...]` and pass the slice in here.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// const TINY: [BasicCard; 4] = [
    ///     FrenchBasicCard::ACE_SPADES,
    ///     FrenchBasicCard::KING_SPADES,
    ///     FrenchBasicCard::ACE_HEARTS,
    ///     FrenchBasicCard::KING_HEARTS,
    /// ];
    ///
    /// let pile = Pile::<Standard52>::from_slice(&TINY);
    /// assert_eq!(pile.len(), 4);
    /// ```
    #[must_use]
    pub fn from_slice(cards: &[BasicCard]) -> Self {
        Self::from(cards.to_vec())
    }

    /// Prints out a demonstration of the deck. Used in the `cli` example program.
    #[cfg(all(feature = "i18n", feature = "colored-display"))]
    pub fn demo_cards(&self, verbose: bool) {
        use crate::localization::{FluentName, Named};

        let deck = self.sorted();
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
            const SEP: &str = "------------------------";
            println!();
            println!(
                "  {:<24} | {:<24} | {:<24} | {:<24} | Klingon",
                "English", "German", "French", "Latin"
            );
            println!("  {SEP} | {SEP} | {SEP} | {SEP} | {SEP}");

            for card in deck {
                println!(
                    "  {:<24} | {:<24} | {:<24} | {:<24} | {}",
                    card.fluent_name(&FluentName::US_ENGLISH),
                    card.fluent_name(&FluentName::DEUTSCH),
                    card.fluent_name(&FluentName::FRANCAIS),
                    card.fluent_name(&FluentName::LATINA),
                    card.fluent_name(&FluentName::TLHINGAN),
                );
            }
        }
    }

    /// Draws x number of cards from the `Pile`. If the number of cards to draw is greater than the
    /// number available, `None` is returned.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut deck = Standard52::deck();
    ///
    /// // Good recs for asserts from CoPilot the last few times.
    /// assert_eq!(deck.draw(5).unwrap().to_string(), "A♠ K♠ Q♠ J♠ T♠");
    /// assert!(deck.draw(48).is_none());
    /// ```
    ///
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
    /// Some(Pile::<DeckType>::from(cards.cloned().collect()))
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

        let mut cards = Self::default();
        for _ in 0..n {
            cards.push(self.draw_first()?);
        }
        Some(cards)
    }

    /// Draws the first [`Card`]  if there. Returns `None` if the `Pile` is empty.
    ///
    ///```
    /// use cardpack::prelude::*;
    ///
    /// let mut deck = Standard52::deck();
    ///
    /// for x in 0..deck.len() {
    ///    let card = deck.draw_first();
    ///    match x {
    ///        52 => assert!(card.is_none()),
    ///        _ => assert!(card.is_some()),
    ///    }
    /// }
    /// ```
    ///
    /// Here's `CoPilot`'s suggestion:
    ///
    ///```txt
    /// self.0.first().copied()
    ///```
    ///
    /// Notice the difference. Theirs doesn't remove the card from the deck. `vec.first()` returns
    /// a reference to the first element in the vector if it's there.
    pub fn draw_first(&mut self) -> Option<Card<DeckType>> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// Returns the [`Card`] on the bottom of the deck. If the deck is empty, `None` is returned.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Pile::<Standard52>::from_str("6D 2C").unwrap();
    ///
    /// assert_eq!(pile.draw_last().unwrap().to_string(), "2♣");
    /// assert_eq!(pile.draw_last().unwrap().to_string(), "6♦");
    /// assert!(pile.draw_last().is_none())
    /// ```
    ///
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

    /// Draws a random [`Card`] from the `Pile`. If the `Pile` is empty, `None` is returned.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = cards!("AH KH QH");
    ///
    /// let random_card = pile.draw_random().unwrap();
    ///
    /// assert!(!pile.contains(&random_card));
    /// ```
    pub fn draw_random(&mut self) -> Option<Card<DeckType>> {
        let mut rng = rng();
        let position = rng.random_range(0..self.len());
        Some(self.remove(position))
    }

    /// Extends the `Pile` with the cards from another `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = cards!("AH KH QH");
    /// pile.extend(&cards!("JH TH"));
    ///
    /// assert_eq!(pile.to_string(), "A♥ K♥ Q♥ J♥ T♥");
    /// ```
    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    /// Returns an empty `String` if the passed in index is invalid. Mainly a hack in order
    /// to make the `cards!` macro smoother.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(Pile::<Standard52>::forgiving_from_str("2♠ 8s 4♠").to_string(), "2♠ 8♠ 4♠");
    /// assert!(Pile::<Standard52>::forgiving_from_str("XX XX XX").to_string().is_empty());
    /// ```
    ///
    /// Let's be real, logging is hot. Sure, I want my code to be easy to use, but I don't want
    /// it to just sweep under the dev/null how things go wrong.
    #[must_use]
    pub fn forgiving_from_str(index: &str) -> Self {
        Self::from_str(index).unwrap_or_else(|_| {
            log::warn!("Pile::forgiving_from_str(): {index} is invalid. Returning empty Pile.");
            Self::default()
        })
    }

    /// Returns a reference to the [`Card`] at the passed in position. If the position is out of
    /// bounds, `None` is returned.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::deck();
    ///
    /// assert_eq!(pile.get(3).unwrap(), &Card::<Standard52>::new(FrenchBasicCard::JACK_SPADES));
    /// assert!(pile.get(99).is_none());
    /// ```
    #[must_use]
    pub fn get(&self, position: usize) -> Option<&Card<DeckType>> {
        self.0.get(position)
    }

    /// Returns the basic, text representation of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<French>::from_str("T♥ Q♠ J♥").unwrap();
    ///
    /// assert_eq!(pile.index(), "TH QS JH");
    /// ```
    #[must_use]
    pub fn index(&self) -> String {
        self.0
            .iter()
            .map(Card::index)
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Returns the internal [`BasicCard`] `Vector` of the `struct`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<French>::from_str("T♥ Q♠ J♥").unwrap();
    /// let expected = vec![
    ///     FrenchBasicCard::TEN_HEARTS,
    ///     FrenchBasicCard::QUEEN_SPADES,
    ///     FrenchBasicCard::JACK_HEARTS
    /// ];
    ///
    /// assert_eq!(pile.into_basic_cards(), expected);
    /// ```
    #[must_use]
    pub fn into_basic_cards(&self) -> Vec<BasicCard> {
        self.0.iter().map(Card::base).collect()
    }

    /// Returns the `Pile` as a [`BasicPile`].
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<French>::from_str("T♥ Q♠ J♥").unwrap();
    /// let expected = BasicPile::from(vec![
    ///     FrenchBasicCard::TEN_HEARTS,
    ///     FrenchBasicCard::QUEEN_SPADES,
    ///     FrenchBasicCard::JACK_HEARTS
    /// ]);
    ///
    /// assert_eq!(pile.into_basic_pile(), expected);
    /// ```
    #[must_use]
    pub fn into_basic_pile(&self) -> BasicPile {
        BasicPile::from(self)
    }

    /// Returns the `Pile` as a `BasicPileCell`.
    #[must_use]
    pub fn basic_pile_cell() -> crate::basic::types::basic::BasicPileCell {
        crate::basic::types::basic::BasicPileCell::new(Self::basic_pile())
    }

    /// Returns the `Pile` as a `BTreeSet`, an ordered collection of each unique [`Card`].
    /// Duplicate cards in the source pile collapse to a single entry.
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use cardpack::prelude::*;
    ///
    /// // The 2♠ appears twice, but the BTreeSet only keeps one entry.
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 2♠ 4♠").unwrap();
    /// let mut set: BTreeSet<Card<Standard52>> = BTreeSet::new();
    ///
    /// set.insert(card!(2S));
    /// set.insert(card!(8S));
    /// set.insert(card!(4S));
    ///
    /// assert_eq!(pile.unique_cards(), set);
    /// assert_eq!(pile.unique_cards().len(), 3);
    /// ```
    #[must_use]
    pub fn unique_cards(&self) -> BTreeSet<Card<DeckType>> {
        self.0.iter().copied().collect()
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    ///
    /// assert!(Pile::<Euchre32>::default().is_empty());
    /// assert!(!cards!("2♠ 8♠ 4♠").is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut iter = pile.iter();
    ///
    /// assert_eq!(iter.next(), Some(&card!(2S)));
    /// assert_eq!(iter.next(), Some(&card!(8S)));
    /// assert_eq!(iter.next(), Some(&card!(4S)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> core::slice::Iter<'_, Card<DeckType>> {
        self.0.iter()
    }

    /// ```
    /// use cardpack::prelude::*;
    /// assert_eq!(French::deck().len(), 54);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// ```
    /// use cardpack::prelude::*;
    /// use crate::cardpack::basic::decks::tiny::Tiny;
    ///
    /// let pile = Pile::<Tiny>::deck();
    /// let mappie = pile.map_by_suit();
    ///
    /// assert_eq!(
    ///     mappie[&FrenchSuit::SPADES].to_string(),
    ///     "A♠ K♠"
    /// );
    /// assert_eq!(
    ///     mappie[&FrenchSuit::HEARTS].to_string(),
    ///     "A♥ K♥"
    /// );
    /// ```
    #[must_use]
    pub fn map_by_suit(&self) -> BTreeMap<Pip, Self> {
        let mut map: BTreeMap<Pip, Self> = BTreeMap::new();

        for card in &self.0 {
            let suit = card.base_card.suit;
            let entry = map.entry(suit).or_default();
            entry.push(*card);
        }

        map
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Short>::pile_on(
    ///     &[
    ///         Pile::<Short>::forgiving_from_str("AS TD QC"),
    ///         Pile::<Short>::forgiving_from_str("8H 7D AC"),
    ///     ]
    /// );
    ///
    /// assert_eq!(pile.to_string(), "A♠ T♦ Q♣ 8♥ 7♦ A♣");
    /// ```
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
    /// Q: why?
    /// A: Because I can and I wanted to see it in action.
    ///
    /// ```
    /// use itertools::assert_equal;
    /// use cardpack::prelude::*;
    ///
    /// /// The flaw in this is that there can be duplicate Cards.
    /// let pile = Pile::<Standard52>::pile_up(3, || Standard52::deck().shuffled().draw(3).unwrap());
    ///
    /// assert_eq!(pile.len(), 9);
    /// ```
    pub fn pile_up(n: usize, f: fn() -> Self) -> Self {
        let mut pile = Self::default();

        for _ in 0..n {
            pile.extend(&f());
        }

        pile
    }

    /// I am not seeing a need for this function.
    ///
    #[deprecated(
        note = "use `piles.iter().map(ToString::to_string).collect::<Vec<_>>().join(\", \")` directly"
    )]
    #[must_use]
    pub fn piles_to_string(piles: &[Self]) -> String {
        piles
            .iter()
            .map(alloc::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Returns the position of the passed in [`Card`] in the `Pile`. If the [`Card`] isn't there,
    /// it returns `None`.
    ///
    /// ```ignore
    /// // ignored under cargo test --no-default-features (Razz needs the `yaml` feature)
    /// use cardpack::prelude::*;
    ///
    /// let deck = Razz::deck();
    /// let two_spades = Card::<Razz>::from_str("2S").unwrap();
    ///
    /// assert_eq!(deck.position(&two_spades), Some(1));
    /// ```
    #[must_use]
    pub fn position(&self, card: &Card<DeckType>) -> Option<usize> {
        self.0.iter().position(|c| c == card)
    }

    /// Prepends the passed in `Pile` to the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Pile::<Standard52>::from_str("J♠ T♠").unwrap();
    /// let other_pile = Pile::<Standard52>::from_str("A♠ K♠ Q♠").unwrap();
    ///
    /// pile.prepend(&other_pile);
    ///
    /// assert_eq!(pile.to_string(), "A♠ K♠ Q♠ J♠ T♠");
    /// ```
    pub fn prepend(&mut self, other: &Self) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    /// A way to deal from the bottom of the deck.
    ///
    /// (I love how `CoPilot` makes up methods, like `pile.pop_bottom()` for the test here.)
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Pile::<Standard52>::from_str("K♠ 7♠ 6♦").unwrap();
    ///
    /// assert_eq!(pile.pop().unwrap().to_string(), "6♦");
    ///
    /// assert_eq!(pile.to_string(), "K♠ 7♠");
    /// ```
    pub fn pop(&mut self) -> Option<Card<DeckType>> {
        self.0.pop()
    }

    /// Pushed the [`Card`] onto the bottom of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile: Pile<Standard52> = cards!("KD QD JD");
    /// pile.push(card!(TD));
    ///
    /// assert_eq!(pile.to_string(), "K♦ Q♦ J♦ T♦");
    /// ```
    pub fn push(&mut self, card: Card<DeckType>) {
        self.0.push(card);
    }

    /// Removed a [`Card`] from the `Pile` at a specific point. Returns a default blank [`Card`] if
    /// the position is out of bounds. This avoids the underlying panic of the `Vec::remove()` method.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = French::deck();
    ///
    /// assert_eq!(pile.remove(0).index(), "BJ");
    /// assert_eq!(pile.remove(14).to_string(), "A♥");
    /// assert_eq!(pile.remove(51).to_string(), "2♣");
    /// assert!(pile.remove(51).is_blank());
    /// ```
    ///
    /// TODO: Possible RF change to [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)?
    pub fn remove(&mut self, x: usize) -> Card<DeckType> {
        if x >= self.len() {
            Card::<DeckType>::default()
        } else {
            self.0.remove(x)
        }
    }

    /// Removes a [`Card`] from the `Pile`. Returns `None` if the [`Card`] isn't there.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Pile::<Standard52>::from_str("7♠ 6♠ 8♠").unwrap().shuffled();
    /// let eight_of_spades = Card::<Standard52>::from_str("8S").unwrap();
    ///
    /// assert_eq!(pile.remove_card(&eight_of_spades).unwrap().to_string(), "8♠");
    /// assert!(pile.remove_card(&eight_of_spades).is_none());
    /// assert_eq!(pile.sorted().to_string(), "7♠ 6♠");
    /// ```
    pub fn remove_card(&mut self, card: &Card<DeckType>) -> Option<Card<DeckType>> {
        let position = self.position(card)?;
        Some(self.remove(position))
    }

    /// Reverses the order of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Euchre24::deck();
    /// pile.reverse();
    ///
    /// assert_eq!(
    ///     pile.to_string(),
    ///     "9♣ T♣ J♣ Q♣ K♣ A♣ 9♦ T♦ J♦ Q♦ K♦ A♦ 9♥ T♥ J♥ Q♥ K♥ A♥ 9♠ T♠ J♠ Q♠ K♠ A♠"
    /// );
    /// ```
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    /// Returns a new Pile with the [`Card`]s reversed.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Euchre24::deck();
    /// let reversed_pile = pile.reversed();
    ///
    /// assert_eq!(
    ///     reversed_pile.to_string(),
    ///     "9♣ T♣ J♣ Q♣ K♣ A♣ 9♦ T♦ J♦ Q♦ K♦ A♦ 9♥ T♥ J♥ Q♥ K♥ A♥ 9♠ T♠ J♠ Q♠ K♠ A♠"
    /// );
    /// ```
    #[must_use]
    pub fn reversed(&self) -> Self {
        let mut pile = self.clone();
        pile.reverse();
        pile
    }

    /// Returns true of the two `Piles` are the same, regardless of the order of the cards.
    ///
    /// ```ignore
    /// // ignored under cargo test --no-default-features (Razz needs the `yaml` feature)
    /// use cardpack::prelude::*;
    ///
    /// let pile1 = Razz::deck();
    /// let pile2 = Razz::deck().shuffled();
    ///
    /// assert_ne!(pile1, pile2);
    /// assert!(pile1.same(&pile2));
    /// ```
    #[must_use]
    pub fn same(&self, cards: &Self) -> bool {
        let left = self.sorted();
        let right = cards.sorted();

        left == right
    }

    /// `shuffled` feels so much better. Nice and succinct.
    ///
    /// For deterministic shuffling, use
    /// [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the `Pile` in place using the process default RNG
    /// (`rand::rng()`). For deterministic shuffling, use
    /// [`shuffle_with_seed`](Self::shuffle_with_seed).
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    /// Shuffles the `Pile` in place deterministically from a `u64` seed.
    ///
    /// Uses [`rand::rngs::StdRng`] internally. Same seed produces the same
    /// permutation **within one `rand` major version**; a `rand` upgrade may
    /// change the result. For cross-version reproducibility, pass a portable
    /// RNG (e.g., `ChaCha8Rng` from `rand_chacha`) to
    /// [`shuffle_with_rng`](Self::shuffle_with_rng).
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let deck = Pile::<Standard52>::deck();
    /// let a = deck.shuffled_with_seed(42);
    /// let b = deck.shuffled_with_seed(42);
    /// assert_eq!(a, b);
    /// ```
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new `Pile` shuffled deterministically from a `u64` seed.
    ///
    /// See [`shuffle_with_seed`](Self::shuffle_with_seed) for the
    /// portability caveat.
    #[must_use]
    pub fn shuffled_with_seed(&self, seed: u64) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_seed(seed);
        pile
    }

    /// Shuffles the `Pile` in place using the caller's RNG.
    ///
    /// Generic over any `R: Rng + ?Sized`. The seed-based methods are sugar
    /// over this primitive — pass your own RNG (e.g., `ChaCha8Rng`) for
    /// algorithm-stable reproducibility across `rand` major-version bumps.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// Returns a new `Pile` shuffled using the caller's RNG.
    #[must_use]
    pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_rng(rng);
        pile
    }

    /// Returns a sorted clone of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let deck = Euchre32::deck();
    /// let shuffled = deck.shuffled();
    ///
    /// assert!(deck.same(&shuffled));
    /// assert_eq!(deck, shuffled.sorted());
    /// ```
    #[must_use]
    pub fn sorted(&self) -> Self {
        let mut pile = self.clone();
        pile.sort();
        pile
    }

    /// Sorts the `Pile` in place.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    ///
    /// ```
    pub fn sort(&mut self) {
        self.0.sort();
    }

    /// Sorts the `Pile` rank first, instead of the default suit.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Short::deck().sorted_by_rank();
    ///
    /// assert_eq!(
    ///     pile.to_string(),
    ///     "A♠ A♥ A♦ A♣ K♠ K♥ K♦ K♣ Q♠ Q♥ Q♦ Q♣ J♠ J♥ J♦ J♣ T♠ T♥ T♦ T♣ 9♠ 9♥ 9♦ 9♣ 8♠ 8♥ 8♦ 8♣ 7♠ 7♥ 7♦ 7♣ 6♠ 6♥ 6♦ 6♣",
    /// );
    /// ```
    #[must_use]
    pub fn sorted_by_rank(&self) -> Self {
        let mut pile = self.clone();
        pile.sort_by_rank();
        pile
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
    /// use cardpack::prelude::{Pile, Standard52};
    /// let pack = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| a.base_card.rank.cmp(&b.base_card.rank));
    ///
    /// assert_eq!(Pile::<Standard52>::from(v).to_string(), "2♠ 4♠ 8♠");
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
    /// use cardpack::prelude::{Pile, Standard52};
    /// let pack = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| a.base_card.rank.cmp(&b.base_card.rank));
    /// v.reverse();
    ///
    /// assert_eq!(Pile::<Standard52>::from(v).to_string(), "8♠ 4♠ 2♠");
    /// ```
    ///
    /// Boom! That does it. Prolem solved. But wait, once again we ask the question once we have
    /// made the test pass as desired. Could it be better? Could we refactor it?
    ///
    /// Can you find it? As usual, it's so hard to find because it's so simple.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cardpack::prelude::{Pile, Standard52};
    /// let pack = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
    /// let mut v = pack.cards().clone();
    /// v.sort_by(|a, b| b.base_card.rank.cmp(&a.base_card.rank));
    ///
    /// assert_eq!(Pile::<Standard52>::from(v).to_string(), "8♠ 4♠ 2♠");
    /// ```
    pub fn sort_by_rank(&mut self) {
        self.0.sort_by_key(|b| core::cmp::Reverse(b.base_card.rank));
    }

    /// Returns a String of the `Pile` with the passed in function applied to each [`Card`].
    ///
    /// ```ignore
    /// // ignored under cargo test --no-default-features (color_index_string needs `colored-display`)
    /// use cardpack::prelude::*;
    ///
    /// let pile = cards!("A♠ K♠ Q♠ J♠ T♠");
    ///
    /// let result = pile.stringify("~", |card| card.color_index_string().to_lowercase());
    ///
    /// assert_eq!(result, "as~ks~qs~js~ts");
    /// ```
    pub fn stringify(&self, s: &str, f: fn(&Card<DeckType>) -> String) -> String {
        self.0.iter().map(f).collect::<Vec<String>>().join(s)
    }

    /// Returns a Color version of the index string of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = cards!("A♠ K♠ Q♠ J♠ T♠");
    ///
    /// assert_eq!(pile.to_color_index_string(), "AS KS QS JS TS");
    /// ```
    #[cfg(feature = "colored-display")]
    pub fn to_color_index_string(&self) -> String {
        self.stringify(" ", Card::color_index_string)
    }

    /// Returns a Color version of the symbol string of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = cards!("AS KS QS JS TS");
    ///
    /// assert_eq!(pile.to_color_symbol_string(), "A♠ K♠ Q♠ J♠ T♠");
    /// ```
    #[cfg(feature = "colored-display")]
    pub fn to_color_symbol_string(&self) -> String {
        self.stringify(" ", Card::color_symbol_string)
    }
}

/// These are all passthroughs to the underlying type parameter. For instance,
/// `Pile::<French>::base_vec()` is routed to `impl DeckedBase for French`.
impl<DeckType: DeckedBase + Ord + Default + Copy + Hash> DeckedBase for Pile<DeckType> {
    /// Pass through call to the `Pile's` underlying type parameter.
    fn base_vec() -> Vec<BasicCard> {
        DeckType::base_vec()
    }

    /// Pass through call to the `Pile's` underlying type parameter.
    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        DeckType::colors()
    }

    /// Pass through call to the `Pile's` underlying type parameter.
    fn deck_name() -> String {
        DeckType::deck_name()
    }

    /// Pass through call to the `Pile's` underlying type parameter.
    fn fluent_deck_key() -> String {
        DeckType::fluent_deck_key()
    }
}

impl<DeckType: DeckedBase + Ord + Default + Copy + Hash> Decked<DeckType> for Pile<DeckType> {}

impl<DeckType: DeckedBase + Default + Copy + Ord + Hash> Display for Pile<DeckType> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = self
            .0
            .iter()
            .map(Card::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{s}")
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<Vec<Card<DeckType>>>
    for Pile<DeckType>
{
    fn from(cards: Vec<Card<DeckType>>) -> Self {
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<Vec<BasicCard>> for Pile<DeckType> {
    fn from(cards: Vec<BasicCard>) -> Self {
        let cards = Self::into_cards(&cards);
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<BasicPile> for Pile<DeckType> {
    fn from(pile: BasicPile) -> Self {
        let cards = Self::into_cards(pile.v());
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> FromStr for Pile<DeckType> {
    type Err = CardError;

    fn from_str(index: &str) -> Result<Self, Self::Err> {
        let (good, bad): (Vec<_>, Vec<_>) = index
            .split_whitespace()
            .map(Card::<DeckType>::from_str)
            .partition(Result::is_ok);

        if !bad.is_empty() {
            return Err(CardError::InvalidCard(index.to_string()));
        }

        Ok(Self::from(
            good.into_iter()
                .map(Result::unwrap_or_default)
                .collect::<Vec<_>>(),
        ))
    }
}

impl<Decked> FromIterator<Card<Decked>> for Pile<Decked>
where
    Decked: DeckedBase + Default + Ord + Copy + Hash,
{
    fn from_iter<I: IntoIterator<Item = Card<Decked>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> Ranged for Pile<DeckType> {
    fn my_basic_pile(&self) -> BasicPile {
        self.into_basic_pile()
    }
}

/// This feels like a non-sequitur. Into means that it is iterated
/// by value, so why would I want a reference?
impl<Decked> IntoIterator for &Pile<Decked>
where
    Decked: DeckedBase + Default + Ord + Copy + Hash,
{
    type Item = Card<Decked>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> IntoIterator for Pile<DeckType> {
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
    use crate::cards;
    use crate::prelude::FLUENT_KEY_BASE_NAME_FRENCH;

    #[test]
    fn basic_cards() {
        let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();

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
        let pile = Pile::<Standard52>::deck();

        assert_eq!(
            pile.card_by_index("2s"),
            Some(Card::<Standard52>::from(FrenchBasicCard::DEUCE_SPADES))
        );
        assert_eq!(pile.card_by_index("BJ"), None);
    }

    #[test]
    fn contains() {
        let pile = Pile::<French>::from_str("2♠ 8♠ 4♠").unwrap();

        assert!(pile.contains(&Card::<French>::from(FrenchBasicCard::DEUCE_SPADES)));
        assert!(!pile.contains(&Card::<French>::from(FrenchBasicCard::ACE_SPADES)));
    }

    #[test]
    fn draw() {
        let mut pile = Pile::<French>::from_str("2♠ 8♠ 4♠").unwrap();

        assert_eq!(pile.draw(3).unwrap().to_string(), "2♠ 8♠ 4♠");
        assert_eq!(pile.draw(3), None);
    }

    #[test]
    fn draw_first() {
        let mut deck = Standard52::deck();

        for x in 0..deck.len() {
            let card = deck.draw_first();
            match x {
                52 => assert!(card.is_none()),
                _ => assert!(card.is_some()),
            }
        }
    }

    #[test]
    fn draw_last() {
        let mut deck = Standard52::deck();

        for x in 0..deck.len() {
            let card = deck.draw_first();
            match x {
                52 => assert!(card.is_none()),
                _ => assert!(card.is_some()),
            }
        }
    }

    #[test]
    fn draw_random() {
        let mut deck = Standard52::deck();

        let random_card = deck.draw_random().unwrap();

        assert!(!deck.contains(&random_card));
    }

    #[test]
    pub fn forgiving_from_str() {
        assert_eq!(
            Pile::<French>::forgiving_from_str("2♠ 8s 4♠").to_string(),
            "2♠ 8♠ 4♠"
        );
        assert_eq!(
            Pile::<French>::forgiving_from_str("2♠ XX 4♠").to_string(),
            ""
        );
    }

    #[test]
    pub fn get() {
        let pile = Pile::<Standard52>::deck();

        assert_eq!(pile.get(0).unwrap().to_string(), "A♠");
        assert_eq!(pile.get(51).unwrap().to_string(), "2♣");
        assert!(pile.get(52).is_none());
    }

    #[test]
    fn unique_cards() {
        let five_deck = French::decks(5);

        let btreeset: alloc::collections::BTreeSet<Card<French>> = five_deck.unique_cards();

        assert_eq!(five_deck.len(), 270);
        assert_eq!(btreeset.len(), 54);

        // BTreeSet iterates high-to-low via BasicCard's reversed Ord (see basic_card.rs:24),
        // which matches French::deck()'s deck-order. If the Ord impl is ever flipped, this
        // assertion breaks and the test needs an explicit .sorted() on the deck.
        let deck: Pile<French> = btreeset.into_iter().collect();
        assert_eq!(deck, French::deck());
    }

    #[test]
    fn from_iterator_collects_pile_from_card_iter() {
        use crate::prelude::*;

        let cards = vec![card!(AS), card!(KH), card!(QC)];
        let pile: Pile<Standard52> = cards.into_iter().collect();

        assert_eq!(pile.len(), 3);
        // FromIterator preserves insertion order (no internal sort/dedupe).
        assert_eq!(pile.to_string(), "A♠ K♥ Q♣");
    }

    #[test]
    fn map_by_suit() {
        let pile = Pile::<Standard52>::deck();

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
        let pile1 = Pile::<French>::from_str("2♠ 8♠ 4♠").unwrap();
        let pile2 = Pile::<French>::from_str("5♠ 6♠ 7♠").unwrap();
        let piles = vec![pile1, pile2];

        let pile = Pile::<French>::pile_on(&piles);

        assert_eq!(pile.to_string(), "2♠ 8♠ 4♠ 5♠ 6♠ 7♠");
    }

    #[test]
    fn pile_up() {
        fn ak() -> Pile<French> {
            Pile::<French>::from_str("A♠ K♠").unwrap()
        }

        let pile = Pile::<French>::pile_up(3, ak);

        assert_eq!(pile.to_string(), "A♠ K♠ A♠ K♠ A♠ K♠");
    }

    #[test]
    fn position() {
        let pile = French::deck();

        let card = Card::<French>::from_str("2♣").unwrap();

        assert_eq!(pile.position(&card).unwrap(), 53);
    }

    #[test]
    fn prepend() {
        let mut pile1 = Pile::<French>::from_str("2♠ 8♠ 4♠").unwrap();
        let pile2 = Pile::<French>::from_str("5♠ 6♠ 7♠").unwrap();

        pile1.prepend(&pile2);

        assert_eq!(pile1.to_string(), "5♠ 6♠ 7♠ 2♠ 8♠ 4♠");
    }

    #[test]
    fn pop() {
        let mut pile = Pile::<Standard52>::deck();
        let card = pile.pop();

        assert_eq!(card.unwrap().to_string(), "2♣");
        assert_eq!(pile.len(), 51);
    }

    #[test]
    fn push() {
        let mut pile = Pile::<French>::default();

        pile.push(Card::default());
        pile.push(Card::<French>::from(FrenchBasicCard::DEUCE_CLUBS));

        assert_eq!(pile.len(), 2);
        assert_eq!(pile.to_string(), "__ 2♣");
    }

    #[test]
    fn remove() {
        let mut pile = Pile::<Standard52>::deck();
        let card = pile.remove(1);

        assert_eq!(card.to_string(), "K♠");
        assert_eq!(pile.draw(2).unwrap().to_string(), "A♠ Q♠");
    }

    #[test]
    fn remove_card() {
        let mut pile = Pile::<Standard52>::deck();
        pile.remove_card(&Card::<Standard52>::from_str("K♠").unwrap());

        let actual = pile.draw(2).unwrap();

        assert_eq!(actual, Pile::<Standard52>::from_str("AS QS").unwrap());
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn to_color_symbol_string() {
        let pile = Pile::<French>::from_str("2c 3c 4c").unwrap();

        // println!("{}", Pile::<French>::deck().to_color_symbol_string());

        assert_eq!(pile.to_color_symbol_string(), "2♣ 3♣ 4♣");
    }

    #[test]
    fn sort() {
        let pile = Pile::<French>::from_str("2♠ 8♣ 4♠").unwrap();
        let mut pile2 = pile.clone();
        let mut pile3 = pile.clone();

        pile2.sort();
        pile3.sort_by_rank();

        assert_eq!(pile.sorted().to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile2.to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile.sorted_by_rank().to_string(), "8♣ 4♠ 2♠");
        assert_eq!(pile3.to_string(), "8♣ 4♠ 2♠");
    }

    #[test]
    fn decked__deck() {
        let french = French::deck();
        let standard52 = Pile::<Standard52>::deck();

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
        let hand_and_foot = Pile::<French>::decks(5).sorted();

        assert_eq!(hand_and_foot.len(), 270);
        assert_eq!(
            hand_and_foot.to_string(),
            "B🃟 B🃟 B🃟 B🃟 B🃟 L🃟 L🃟 L🃟 L🃟 L🃟 A♠ A♠ A♠ A♠ A♠ K♠ K♠ K♠ K♠ K♠ Q♠ Q♠ Q♠ Q♠ Q♠ J♠ J♠ J♠ J♠ J♠ T♠ T♠ T♠ T♠ T♠ 9♠ 9♠ 9♠ 9♠ 9♠ 8♠ 8♠ 8♠ 8♠ 8♠ 7♠ 7♠ 7♠ 7♠ 7♠ 6♠ 6♠ 6♠ 6♠ 6♠ 5♠ 5♠ 5♠ 5♠ 5♠ 4♠ 4♠ 4♠ 4♠ 4♠ 3♠ 3♠ 3♠ 3♠ 3♠ 2♠ 2♠ 2♠ 2♠ 2♠ A♥ A♥ A♥ A♥ A♥ K♥ K♥ K♥ K♥ K♥ Q♥ Q♥ Q♥ Q♥ Q♥ J♥ J♥ J♥ J♥ J♥ T♥ T♥ T♥ T♥ T♥ 9♥ 9♥ 9♥ 9♥ 9♥ 8♥ 8♥ 8♥ 8♥ 8♥ 7♥ 7♥ 7♥ 7♥ 7♥ 6♥ 6♥ 6♥ 6♥ 6♥ 5♥ 5♥ 5♥ 5♥ 5♥ 4♥ 4♥ 4♥ 4♥ 4♥ 3♥ 3♥ 3♥ 3♥ 3♥ 2♥ 2♥ 2♥ 2♥ 2♥ A♦ A♦ A♦ A♦ A♦ K♦ K♦ K♦ K♦ K♦ Q♦ Q♦ Q♦ Q♦ Q♦ J♦ J♦ J♦ J♦ J♦ T♦ T♦ T♦ T♦ T♦ 9♦ 9♦ 9♦ 9♦ 9♦ 8♦ 8♦ 8♦ 8♦ 8♦ 7♦ 7♦ 7♦ 7♦ 7♦ 6♦ 6♦ 6♦ 6♦ 6♦ 5♦ 5♦ 5♦ 5♦ 5♦ 4♦ 4♦ 4♦ 4♦ 4♦ 3♦ 3♦ 3♦ 3♦ 3♦ 2♦ 2♦ 2♦ 2♦ 2♦ A♣ A♣ A♣ A♣ A♣ K♣ K♣ K♣ K♣ K♣ Q♣ Q♣ Q♣ Q♣ Q♣ J♣ J♣ J♣ J♣ J♣ T♣ T♣ T♣ T♣ T♣ 9♣ 9♣ 9♣ 9♣ 9♣ 8♣ 8♣ 8♣ 8♣ 8♣ 7♣ 7♣ 7♣ 7♣ 7♣ 6♣ 6♣ 6♣ 6♣ 6♣ 5♣ 5♣ 5♣ 5♣ 5♣ 4♣ 4♣ 4♣ 4♣ 4♣ 3♣ 3♣ 3♣ 3♣ 3♣ 2♣ 2♣ 2♣ 2♣ 2♣"
        );
    }

    #[test]
    fn default() {
        let pile = Pile::<French>::default();
        assert_eq!(pile.len(), 0);
    }

    #[test]
    fn display() {}

    #[test]
    fn from_vec() {
        let base_cards = Card::<French>::base_vec();

        let cards = Pile::<French>::into_cards(&base_cards);

        let pile = Pile::<French>::from(cards.clone());

        assert_eq!(*pile.cards(), cards);
    }

    #[test]
    fn to_string__from_str() {
        let deck = French::deck();
        let deck_str = deck.to_string();
        let deck_from_str = Pile::<French>::from_str(&deck_str).unwrap().shuffled();

        assert_eq!(
            deck_str,
            "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
        assert!(deck.same(&deck_from_str));
        assert_eq!(deck, deck_from_str.sorted());
    }

    /// This is just a copy of the `Tiny` example in the main docs.
    #[test]
    fn tiny_example() {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Tiny {}

        impl Tiny {
            pub const DECK_SIZE: usize = 4;

            pub const DECK: [BasicCard; Tiny::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
            ];
        }

        impl DeckedBase for Tiny {
            fn base_vec() -> Vec<BasicCard> {
                Tiny::DECK.to_vec()
            }

            #[cfg(feature = "colored-display")]
            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Tiny".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        let mut deck = Pile::<Tiny>::deck();
        assert_eq!(deck.to_string(), "A♠ K♠ A♥ K♥");
        assert_eq!(deck.draw_first().unwrap().to_string(), "A♠");
        assert_eq!(deck.draw_last().unwrap().to_string(), "K♥");
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.index(), "KS AH");
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region GTOed

    #[test]
    fn combos() {
        let pile = Pile::<Standard52>::deck();
        let combinations = pile.combos(2);
        let dups = pile.combos(2);

        assert_eq!(combinations.len(), 1326);
        assert_eq!(combinations, dups);
    }

    #[test]
    fn combos_with_dups() {
        let pile = Pile::<Standard52>::decks(2);
        let combinations = pile.combos(2);
        let dups = pile.combos_with_dups(2);

        assert_eq!(combinations.len(), 1456);
        assert_eq!(dups.len(), 5356);
    }

    #[test]
    fn all_of_rank() {
        assert!(cards!("AS AD").all_of_rank(FrenchRank::ACE));
        assert!(cards!("AS AD AS").all_of_rank(FrenchRank::ACE));
        assert!(!cards!("AS AD").all_of_rank(FrenchRank::KING));
        assert!(!cards!("AS AD KS").all_of_rank(FrenchRank::ACE));
    }

    #[test]
    fn all_of_same_rank() {
        assert!(cards!("AS AD").all_of_same_rank());
        assert!(cards!("AS AD AS").all_of_same_rank());
        assert!(!cards!("AS AD KS").all_of_same_rank());
    }

    #[test]
    fn all_of_same_suit() {
        assert!(cards!("AS KS").all_of_same_suit());
        assert!(cards!("AS KS QS").all_of_same_suit());
        assert!(!cards!("AS KH QD").all_of_same_suit());
    }

    // copilot:
    // assert!(cards!("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(cards!("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(cards!("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(!cards!("AS AD").of_same_or_greater_rank(FrenchRank::KING));
    // assert!(!cards!("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
    #[test]
    fn of_same_or_greater_rank() {
        assert!(cards!("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(cards!("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(cards!("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!cards!("AS QD").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!cards!("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    }

    // endregion GTOed

    #[test]
    fn draw_random__is_from_deck() {
        // Catches mutation: return Some(Card::new(Default::default()))
        let deck = Standard52::deck();
        let mut deck_copy = deck.clone();
        let drawn = deck_copy.draw_random().unwrap();
        // The drawn card must have been in the original deck, not a blank default
        assert!(deck.contains(&drawn));
        assert_ne!(drawn.base(), BasicCard::default());
    }

    #[test]
    #[allow(deprecated)]
    fn piles_to_string__not_empty() {
        let pile1 = cards!("AS KS");
        let pile2 = cards!("AH KH");
        let piles = vec![pile1, pile2];
        let result = Pile::<Standard52>::piles_to_string(&piles);
        assert!(!result.is_empty());
        assert_ne!(result, "xyzzy");
    }

    #[test]
    fn same__true_for_same_cards_different_order() {
        let pile1 = cards!("AS KS");
        let pile2 = cards!("KS AS");
        assert!(pile1.same(&pile2));
    }

    #[test]
    fn same__false_for_different_cards() {
        // Catches same() -> true mutation
        let pile1 = cards!("AS KS");
        let pile2 = cards!("AH KH");
        assert!(!pile1.same(&pile2));
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked_base__colors__not_empty() {
        // Catches colors() -> HashMap::new() mutation on Pile<DeckType>
        assert!(!Pile::<Standard52>::colors().is_empty());
    }

    #[test]
    fn decked_base__fluent_deck_key__not_empty() {
        // Catches fluent_deck_key() -> String::new() and -> "xyzzy" mutations on Pile<DeckType>
        // (fluent_deck_key is part of DeckedBase, no feature gating needed)
        let key = Pile::<Standard52>::fluent_deck_key();
        assert!(!key.is_empty());
        assert_ne!(key, "xyzzy");
    }

    #[test]
    fn from_vec_basic_card() {
        // Catches From<Vec<BasicCard>> -> Default::default() mutation
        let cards: Vec<BasicCard> = vec![FrenchBasicCard::ACE_SPADES, FrenchBasicCard::KING_SPADES];
        let pile = Pile::<Standard52>::from(cards);
        assert_eq!(pile.len(), 2);
        assert_ne!(pile, Pile::<Standard52>::default());
    }

    #[test]
    fn from_basic_pile() {
        // Catches From<BasicPile> -> Default::default() mutation
        let basic = Standard52::basic_pile();
        let pile = Pile::<Standard52>::from(basic);
        assert_eq!(pile.len(), Standard52::DECK_SIZE);
        assert_ne!(pile, Pile::<Standard52>::default());
    }

    #[test]
    fn from_iterator() {
        // Catches FromIterator -> Default::default() mutation
        let deck = Standard52::deck();
        let pile: Pile<Standard52> = deck.into_iter().collect();
        assert_eq!(pile.len(), Standard52::DECK_SIZE);
    }

    #[test]
    fn into_iterator_ref() {
        // Catches IntoIterator for &Pile -> Default::default() mutation
        let deck = Standard52::deck();
        let count = deck.into_iter().count();
        assert_eq!(count, Standard52::DECK_SIZE);
    }

    #[test]
    fn from_slice__roundtrips_a_const_array() {
        const TINY: [BasicCard; 4] = [
            FrenchBasicCard::ACE_SPADES,
            FrenchBasicCard::KING_SPADES,
            FrenchBasicCard::ACE_HEARTS,
            FrenchBasicCard::KING_HEARTS,
        ];
        let pile = Pile::<Standard52>::from_slice(&TINY);
        assert_eq!(pile.len(), 4);
        assert!(pile.contains(&Card::<Standard52>::new(FrenchBasicCard::ACE_SPADES)));
    }

    #[cfg(all(feature = "i18n", feature = "colored-display"))]
    #[test]
    fn demo_cards__does_not_panic() {
        let deck = Standard52::deck();
        deck.demo_cards(false);
    }

    #[test]
    fn shuffled_with_seed__deterministic() {
        let deck = Pile::<Standard52>::deck();
        let a = deck.shuffled_with_seed(42);
        let b = deck.shuffled_with_seed(42);
        assert_eq!(a, b, "same seed must produce identical permutation");
    }

    #[test]
    fn shuffled_with_seed__different_seeds_differ() {
        let deck = Pile::<Standard52>::deck();
        assert_ne!(
            deck.shuffled_with_seed(1),
            deck.shuffled_with_seed(2),
            "different seeds should almost always produce different orderings"
        );
    }

    #[test]
    fn shuffled_with_seed__same_cards() {
        let deck = Pile::<Standard52>::deck();
        let shuffled = deck.shuffled_with_seed(0xC0FFEE);
        assert_eq!(deck.len(), shuffled.len());
        let mut o = deck.cards().clone();
        let mut s = shuffled.cards().clone();
        o.sort();
        s.sort();
        assert_eq!(o, s, "shuffle must permute, not transform");
    }
}
