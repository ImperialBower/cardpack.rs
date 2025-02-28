use crate::basic::decks::cards::french::FLUENT_KEY_BASE_NAME_FRENCH;
use crate::basic::types::basic_card::BasicCard;
pub use crate::basic::types::basic_pile::BasicPile;
pub use crate::basic::types::card::Card;
use crate::basic::types::combos::Combos;
pub use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::prelude::PipType;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

pub trait DeckedBase {
    /// And just like that we have a `Pile`.
    #[must_use]
    fn basic_pile() -> BasicPile {
        BasicPile::from(Self::base_vec())
    }

    fn base_vec() -> Vec<BasicCard>;

    fn colors() -> HashMap<Pip, colored::Color>;

    fn deck_name() -> String;

    /// Use this to override the fluent keys iside the ftl files
    #[must_use]
    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_FRENCH.to_string()
    }

    fn fluent_deck_key() -> String;
}

pub trait Decked<DeckType>: DeckedBase
where
    DeckType: Copy + Default + Ord + DeckedBase + Hash,
{
    /// The method to generate a deck of cards based on specific type parameters.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// // Each deck can be evoked in two ways:
    /// let deck_from_type_parameter: Pile<Pinochle> = Pinochle::deck();
    /// let deck_from_generic_pile = Pile::<Pinochle>::deck();
    ///
    /// assert_eq!(deck_from_type_parameter, deck_from_generic_pile);
    /// assert_eq!(
    ///     deck_from_type_parameter.to_string(),
    ///     "A♠ A♠ T♠ T♠ K♠ K♠ Q♠ Q♠ J♠ J♠ 9♠ 9♠ A♥ A♥ T♥ T♥ K♥ K♥ Q♥ Q♥ J♥ J♥ 9♥ 9♥ A♦ A♦ T♦ T♦ K♦ K♦ Q♦ Q♦ J♦ J♦ 9♦ 9♦ A♣ A♣ T♣ T♣ K♣ K♣ Q♣ Q♣ J♣ J♣ 9♣ 9♣"
    /// );
    /// ```
    #[must_use]
    fn deck() -> Pile<DeckType> {
        Pile::<DeckType>::from(Self::deckvec())
    }

    /// Creates x numbers of a specific `Deck` in a single [`Pile`].
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let twodecks = Pile::<Standard52>::decks(2);
    ///
    /// assert_eq!(twodecks.len(), 104);
    ///
    /// // Converting it into a `HashSet` will verify that there are only 52 unique cards.
    /// assert_eq!(twodecks.into_hashset().len(), 52);
    /// ```
    ///
    /// This way of doing it from `CoPilot` is an interesting alternative to
    /// my old `pile_up` method:
    ///
    /// ```txt
    /// fn decks(n: usize) -> Pile<RankType, SuitType> {
    ///     Pile::<RankType, SuitType>::pile_up(n, <Self as Decked<RankType, SuitType>>::deck)
    /// }
    /// ```
    #[must_use]
    fn decks(count: usize) -> Pile<DeckType> {
        Pile::<DeckType>::from(Self::deckvec().repeat(count))
    }

    /// Takes the [`BasicCard`] vector for the type parameter and converts it into a vector
    /// of [`Card`]s for the type parameter. This is where the work is done channeling between
    /// the plain and generic enabled versions of the structs.
    #[must_use]
    fn deckvec() -> Vec<Card<DeckType>> {
        let v = Card::<DeckType>::base_vec();
        v.iter().map(|card| Card::<DeckType>::from(*card)).collect()
    }

    /// Used in the `examples/cli.rs` application for showing off the various decks.
    fn demo(verbose: bool) {
        Self::deck().demo_cards(verbose);
    }

    /// Warning on original version of the code.
    /// TODO: I need to understand slices better.
    ///
    /// ```txt
    /// warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    ///   --> src/rev6/traits.rs:22:31
    ///    |
    /// 22 |     fn into_cards(base_cards: &Vec<BaseCard>) -> Vec<Card<DeckType>> {
    ///    |                               ^^^^^^^^^^^^^^ help: change this to: `&[BaseCard]`
    /// ```
    #[must_use]
    fn into_cards(base_cards: &[BasicCard]) -> Vec<Card<DeckType>> {
        base_cards
            .iter()
            .map(|card| Card::<DeckType>::from(*card))
            .collect()
    }

    /// This is the urtest for a deck. If you can do this, you are golden.
    #[must_use]
    fn validate() -> bool {
        let deck = Self::deck();
        let deckfromstr = Pile::<DeckType>::from_str(&deck.to_string()).unwrap();

        deck == deck.clone().shuffled().sort() && deck == deckfromstr
    }
}

/// This is a trait of convenience to organize what needs to be done in order to create a revised
/// [Cactus Kev](https://suffe.cool/poker/evaluator.html) number. I like having functional blocks
/// like this organized in a functional way. Traits feel like a good way to do it.
pub trait CKCRevised {
    #[must_use]
    fn get_ckc_number(&self) -> u32;

    #[must_use]
    fn ckc_rank_number(&self) -> u32;

    #[must_use]
    fn ckc_suit_number(&self) -> u32;

    #[must_use]
    fn ckc_rank_bits(&self) -> u32;

    #[must_use]
    fn ckc_get_prime(&self) -> u32;

    #[must_use]
    fn ckc_rank_shift8(&self) -> u32;
}

/// This trait is a kind of proof of concept for how easy it would be to lay a foundation for creating
/// a [GTO (Game Theory Optimal)](https://www.888poker.com/magazine/strategy/beginners-guide-gto-poker)
/// style poker solver. A foundation to that is around what they call poker ranges. Where, instead
/// of trying to figure out exactly what hand an opponent has, you base your moves on what you
/// believe is the range of hands that they player could have given their particular style.
pub trait Ranged {
    /// This is the starting point for any entity enabling the `Ranged` trait. Since none of the
    /// logic behind any of these methods requires the boundary provided by the generic [`Pile`] struct,
    /// since it is based entirely on the raw data in the collection of [`BasicCard`]s in a
    /// [`BasicPile`]. [`Pile`] is very useful for getting us to where we want to be, re a specific
    /// collection of cards, but once there we can rid ourselves of its confinements and focus on
    /// data.
    ///
    /// This started out passing a reference, but that didn't work when trying to implement it
    /// with Pile, because the reference vanishes at the end of the call.
    fn my_basic_pile(&self) -> BasicPile;

    fn combos(&self, k: usize) -> Combos {
        let mut hs: HashSet<BasicPile> = HashSet::new();

        for combo in self.my_basic_pile().clone().into_iter().combinations(k) {
            let pile = BasicPile::from(combo).sorted_by_rank();
            hs.insert(pile);
        }

        let mut combos = hs.into_iter().collect::<Vec<_>>();

        combos.sort();
        Combos::from(combos)
    }

    fn combos_with_dups(&self, k: usize) -> Combos {
        let mut combos = Combos::default();

        for combo in self.my_basic_pile().clone().into_iter().combinations(k) {
            let pile = BasicPile::from(combo).sorted_by_rank();
            combos.push(pile);
        }

        combos.sort();
        combos.reverse();
        combos
    }

    fn all_of_rank(&self, rank: Pip) -> bool {
        self.my_basic_pile().iter().all(|card| card.rank == rank)
    }

    fn all_of_same_rank(&self) -> bool {
        if let Some(first_card) = self.my_basic_pile().v().first() {
            self.my_basic_pile()
                .iter()
                .all(|card| card.rank == first_card.rank)
        } else {
            true
        }
    }

    fn all_of_same_suit(&self) -> bool {
        if let Some(first_card) = self.my_basic_pile().v().first() {
            self.my_basic_pile()
                .iter()
                .all(|card| card.suit == first_card.suit)
        } else {
            true
        }
    }

    /// I love how `CoPilot` can have the earlier version of the function from Pile and
    /// completely ignore it and instead provide stuff with absolutely no rational:
    ///
    /// ```txt
    /// pub fn is_connector(&self) -> bool {
    ///     if self.len() < 3 {
    ///         return false;
    ///     }
    ///
    ///     let mut cards = self.0.clone();
    ///     cards.sort();
    ///
    ///     for i in 1..cards.len() {
    ///         if cards[i].rank as i32 - cards[i - 1].rank as i32 != 1 {
    ///             return false;
    ///         }
    ///     }
    ///
    ///     true
    /// }
    /// ```
    ///
    /// I mean, why 3 for length? I don't even want to try to figure out why this code is.
    ///
    /// OK, the `core::slice::windows()` function is officially cool.
    fn is_connector(&self) -> bool {
        let mut pile = self.my_basic_pile().clone();
        pile.sort_by_rank();
        pile.v()
            .windows(2)
            .all(|w| w[0].rank.weight == w[1].rank.weight + 1)
    }

    fn of_same_or_greater_rank(&self, rank: Pip) -> bool {
        self.my_basic_pile().iter().all(|card| card.rank >= rank)
    }

    //\\//\\//\\//\\
    // Pips
    #[must_use]
    fn filter_cards<F>(&self, filter: F) -> BasicPile
    where
        F: Fn(&BasicCard) -> bool,
    {
        self.my_basic_pile()
            .iter()
            .filter(|&card| filter(card))
            .copied()
            .collect()
    }

    /// Returns a [`BasicPile`] filtering on rank [`PipType`].
    ///
    /// TODO: Mark this as an example of the abstraction pushing the limits.
    ///
    /// This example verifies that there are two `Jokers` in the
    /// [`French`](crate::basic::decks::french::French) [`Pile`]
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut deck = French::deck();
    ///
    /// assert_eq!(deck.cards_of_rank_pip_type(PipType::Joker).len(), 2);
    /// ```
    ///
    /// Here's the original code before being refactored into using the `filter_cards` closure,
    ///
    /// ```txt
    /// #[must_use]
    /// pub fn cards_of_rank_pip_type(&self, pip_type: PipType) -> Self {
    ///     self.iter()
    ///         .filter(|card| card.rank.pip_type == pip_type)
    ///         .cloned()
    ///         .collect()
    /// }
    /// ```
    #[must_use]
    fn cards_of_rank_pip_type(&self, pip_type: PipType) -> BasicPile {
        let rank_types_filter = |basic_card: &BasicCard| basic_card.rank.pip_type == pip_type;
        self.filter_cards(rank_types_filter)
    }

    /// Returns a [`BasicPile`] filtering on suit [`PipType`].
    ///
    /// This example verifies how many `Special` (`Major Arcana`) cards are in the
    /// [`Tarot`](crate::basic::decks::tarot::Tarot) deck.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut deck = Tarot::deck();
    ///
    /// assert_eq!(deck.cards_of_suit_pip_type(PipType::Special).len(), 22);
    /// ```
    #[must_use]
    fn cards_of_suit_pip_type(&self, pip_type: PipType) -> BasicPile {
        let rank_types_filter = |basic_card: &BasicCard| basic_card.suit.pip_type == pip_type;
        self.filter_cards(rank_types_filter)
    }

    /// Returns a [`BasicPile`] where either the suit or rank [`Pip`] have the specified [`PipType`].\
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(Tarot::deck().cards_with_pip_type(PipType::Special).len(), 22);
    /// assert_eq!(French::deck().cards_with_pip_type(PipType::Joker).len(), 2);
    /// assert!(French::deck().cards_with_pip_type(PipType::Special).is_empty());
    /// ```
    #[must_use]
    fn cards_with_pip_type(&self, pip_type: PipType) -> BasicPile {
        let rank_types_filter = |basic_card: &BasicCard| {
            basic_card.rank.pip_type == pip_type || basic_card.suit.pip_type == pip_type
        };
        self.filter_cards(rank_types_filter)
    }

    fn extract_pips<F>(&self, f: F) -> Vec<Pip>
    where
        F: Fn(&BasicCard) -> Pip,
    {
        let set: HashSet<Pip> = self.my_basic_pile().iter().map(f).collect();
        let mut vec: Vec<Pip> = set.into_iter().collect::<Vec<_>>();
        vec.sort();
        vec.reverse();
        vec
    }

    fn pip_index<F>(&self, f: F, joiner: &str) -> String
    where
        F: Fn(&BasicCard) -> Pip,
    {
        self.extract_pips(f)
            .iter()
            .map(|pip| pip.index.to_string())
            .collect::<Vec<String>>()
            .join(joiner)
    }

    #[must_use]
    fn ranks(&self) -> Vec<Pip> {
        self.extract_pips(|card| card.rank)
    }

    #[must_use]
    fn ranks_index(&self, joiner: &str) -> String {
        self.pip_index(|card| card.rank, joiner)
    }

    /// TODO RF: Wouldn't it be easier to just return a vector, and if it's empty you know
    /// there were none in the `Pile`.
    #[must_use]
    fn ranks_by_suit(&self, suit: Pip) -> Option<Vec<Pip>> {
        let ranks: Vec<Pip> = self
            .my_basic_pile()
            .iter()
            .filter(|card| card.suit == suit)
            .map(|card| card.rank)
            .collect();

        match ranks.len() {
            0 => None,
            _ => Some(ranks),
        }
    }

    #[must_use]
    fn ranks_index_by_suit(&self, suit: Pip, joiner: &str) -> Option<String> {
        self.ranks_by_suit(suit).map(|ranks| {
            ranks
                .iter()
                .map(|pip| pip.index.to_string())
                .collect::<Vec<String>>()
                .join(joiner)
        })
    }

    #[must_use]
    fn suits(&self) -> Vec<Pip> {
        self.extract_pips(|card| card.suit)
    }

    #[must_use]
    fn suits_index(&self, joiner: &str) -> String {
        self.pip_index(|card| card.suit, joiner)
    }

    #[must_use]
    fn suit_symbol_index(&self, joiner: &str) -> String {
        self.suits()
            .iter()
            .map(|pip| pip.symbol.to_string())
            .collect::<Vec<String>>()
            .join(joiner)
    }
}
