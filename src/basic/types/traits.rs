use crate::basic::decks::cards::french::FLUENT_KEY_BASE_NAME_FRENCH;
use crate::basic::types::basic_card::BasicCard;
pub use crate::basic::types::basic_pile::BasicPile;
pub use crate::basic::types::card::Card;
use crate::basic::types::combos::Combos;
pub use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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
    #[must_use]
    fn deck() -> Pile<DeckType> {
        Pile::<DeckType>::from(Self::deckvec())
    }

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

    #[must_use]
    fn deckvec() -> Vec<Card<DeckType>> {
        let v = Card::<DeckType>::base_vec();
        v.iter().map(|card| Card::<DeckType>::from(*card)).collect()
    }

    // The [capitalize crate](https://crates.io/crates/capitalize) does handle this, but it's
    // funnier just doing it this way.
    //
    // When I was coding it there were three ways that it played out.
    //
    // ## 1. My way aka the lazy way
    //
    // Use the [capitalize crate](https://crates.io/crates/capitalize) crate and be done with it.
    //
    // ## 2. The `CoPilot` way
    //
    // ```
    // fn capitalize_first_letter(s: &str) -> String {
    //     let mut c = "foo".chars();
    //     match c.next() {
    //         None => String::new(),
    //         Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    //     }
    // }
    //
    // fn main() {
    //     let s = "hello";
    //     let capitalized = capitalize_first_letter(s);
    //     println!("{}", capitalized); // Output: "Hello"
    // }
    // ```
    // All that and it's a complete waste of time because it references the `Fluent` name
    // and not the `Deck` name. We ended up creating the method at the `DeckedBase` trait.
    // #[must_use]
    // fn deck_name() -> String {
    //     let binding = Self::fluent_deck_key();
    //     let mut letters = binding.chars();
    //     match letters.next() {
    //         None => String::new(),
    //         Some(first) => first.to_uppercase().collect::<String>() + letters.as_str(),
    //     }
    // }

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

        deck == deck.clone().shuffled().sort()
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
    /// This started out passing a reference, but that didn't work when trying to implement it
    /// with Pile, because the reference vanishes at the end of the call.
    fn my_basic_pile(&self) -> BasicPile;

    fn combos(&self, k: usize) -> Combos {
        let mut hs: HashSet<BasicPile> = HashSet::new();

        for combo in self.my_basic_pile().clone().into_iter().combinations(k) {
            let mut pile = BasicPile::from(combo);
            pile.sort();
            hs.insert(pile);
        }

        let mut combos = hs.into_iter().collect::<Vec<_>>();

        combos.sort();
        Combos::from(combos)
    }

    fn combos_with_dups(&self, k: usize) -> Combos {
        let mut combos = Combos::default();

        for mut combo in self.my_basic_pile().clone().into_iter().combinations(k) {
            combo.sort();
            combos.push(BasicPile::from(combo));
        }

        combos.sort();
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
}
