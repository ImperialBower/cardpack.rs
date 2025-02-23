use crate::basic::types::combos::Combos;
use crate::prelude::{BasicCard, Deck, DeckedBase, Pip, PipType};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Pile(Vec<BasicCard>);

impl Pile {
    #[must_use]
    pub fn v(&self) -> &Vec<BasicCard> {
        &self.0
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

    /// This is very much suboptimal, but I don't have an easy way to
    /// avoid it. My common currency is vectors.
    pub fn draw_first(&mut self) -> Option<BasicCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    #[must_use]
    pub fn filter_cards<F>(&self, filter: F) -> Self
    where
        F: Fn(&BasicCard) -> bool,
    {
        self.iter().filter(|&card| filter(card)).copied().collect()
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region Combos

    // TODO: RF?
    #[must_use]
    pub fn combos(&self, k: usize) -> Combos {
        let mut hs: HashSet<Pile> = HashSet::new();

        for mut combo in self.clone().into_iter().combinations(k) {
            combo.sort();
            hs.insert(Pile(combo));
        }

        let mut combos = hs.into_iter().collect::<Vec<_>>();

        combos.sort();
        Combos::from(combos)
    }

    #[must_use]
    pub fn combos_with_dups(&self, k: usize) -> Combos {
        let mut combos = Combos::default();

        for mut combo in self.clone().into_iter().combinations(k) {
            combo.sort();
            combos.push(Pile(combo));
        }

        combos.sort();
        combos
    }

    #[must_use]
    pub fn all_of_rank(&self, rank: Pip) -> bool {
        self.iter().all(|card| card.rank == rank)
    }

    #[must_use]
    pub fn all_of_same_rank(&self) -> bool {
        if let Some(first_card) = self.0.first() {
            self.iter().all(|card| card.rank == first_card.rank)
        } else {
            true
        }
    }

    #[must_use]
    pub fn all_of_same_suit(&self) -> bool {
        if let Some(first_card) = self.0.first() {
            self.iter().all(|card| card.suit == first_card.suit)
        } else {
            true
        }
    }

    /// I love how `CoPilot` can have the earlier version of the function from Pack and
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
    #[must_use]
    pub fn is_connector(&self) -> bool {
        let mut pile = self.clone();
        pile.sort_by_rank();
        pile.0
            .windows(2)
            .all(|w| w[0].rank.weight == w[1].rank.weight + 1)
    }

    #[must_use]
    pub fn of_same_or_greater_rank(&self, rank: Pip) -> bool {
        self.iter().all(|card| card.rank >= rank)
    }

    // endregion

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region Pips

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
    pub fn cards_of_rank_pip_type(&self, pip_type: PipType) -> Self {
        let rank_types_filter = |basic_card: &BasicCard| basic_card.rank.pip_type == pip_type;
        self.filter_cards(rank_types_filter)
    }

    #[must_use]
    pub fn cards_of_suit_pip_type(&self, pip_type: PipType) -> Self {
        let rank_types_filter = |basic_card: &BasicCard| basic_card.suit.pip_type == pip_type;
        self.filter_cards(rank_types_filter)
    }

    #[must_use]
    pub fn cards_with_pip_type(&self, pip_type: PipType) -> Self {
        let rank_types_filter = |basic_card: &BasicCard| {
            basic_card.rank.pip_type == pip_type || basic_card.suit.pip_type == pip_type
        };
        self.filter_cards(rank_types_filter)
    }

    fn extract_pips<F>(&self, f: F) -> Vec<Pip>
    where
        F: Fn(&BasicCard) -> Pip,
    {
        let set: HashSet<Pip> = self.0.iter().map(f).collect();
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
    pub fn ranks(&self) -> Vec<Pip> {
        self.extract_pips(|card| card.rank)
    }

    #[must_use]
    pub fn ranks_index(&self, joiner: &str) -> String {
        self.pip_index(|card| card.rank, joiner)
    }

    /// TODO RF: Wouldn't it be easier to just return a vector, and if it's empty you know
    /// there were none in the `Pile`.
    #[must_use]
    pub fn ranks_by_suit(&self, suit: Pip) -> Option<Vec<Pip>> {
        let ranks: Vec<Pip> = self
            .0
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
    pub fn ranks_index_by_suit(&self, suit: Pip, joiner: &str) -> Option<String> {
        self.ranks_by_suit(suit).map(|ranks| {
            ranks
                .iter()
                .map(|pip| pip.index.to_string())
                .collect::<Vec<String>>()
                .join(joiner)
        })
    }

    #[must_use]
    pub fn suits(&self) -> Vec<Pip> {
        self.extract_pips(|card| card.suit)
    }

    #[must_use]
    pub fn suits_index(&self, joiner: &str) -> String {
        self.pip_index(|card| card.suit, joiner)
    }

    #[must_use]
    pub fn suit_symbol_index(&self, joiner: &str) -> String {
        self.suits()
            .iter()
            .map(|pip| pip.symbol.to_string())
            .collect::<Vec<String>>()
            .join(joiner)
    }

    // endregion Pips

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region vector functions
    #[must_use]
    pub fn contains(&self, card: &BasicCard) -> bool {
        self.0.contains(card)
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<&BasicCard> {
        self.0.get(position)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<BasicCard> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<BasicCard> {
        self.0.pop()
    }

    pub fn push(&mut self, card: BasicCard) {
        self.0.push(card);
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn remove(&mut self, position: usize) -> BasicCard {
        self.0.remove(position)
    }

    pub fn sort(&mut self) {
        self.0.sort();
    }

    pub fn sort_by_rank(&mut self) {
        self.0.sort_by(|a, b| b.rank.cmp(&a.rank));
    }

    #[must_use]
    pub fn sorted_by_rank(&mut self) -> Self {
        let mut pile = self.clone();
        pile.0.sort_by(|a, b| b.rank.cmp(&a.rank));
        pile
    }
    // endregion
}

impl Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl From<Vec<BasicCard>> for Pile {
    fn from(cards: Vec<BasicCard>) -> Self {
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Copy + Default + Ord + Hash> From<&Deck<DeckType>> for Pile {
    fn from(pack: &Deck<DeckType>) -> Self {
        pack.iter().map(|card| card.base_card).collect()
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// region Iterator

impl FromIterator<BasicCard> for Pile {
    fn from_iter<T: IntoIterator<Item = BasicCard>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// A win for `CoPilot`!
///
/// I initially got an error from the `Rust` compiler:
///
/// ```txt
/// warning: `iter` method without an `IntoIterator` impl for `&Pile`
///    --> src/basic/types/pile.rs:163:5
///     |
/// 163 | /     pub fn iter(&self) -> std::slice::Iter<BasicCard> {
/// 164 | |         self.0.iter()
/// 165 | |     }
///     | |_____^
///     |
///     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#iter_without_into_iter
/// note: the lint level is defined here
///    --> src/lib.rs:1:9
///     |
/// 1   | #![warn(clippy::pedantic)]
///     |         ^^^^^^^^^^^^^^^^
///     = note: `#[warn(clippy::iter_without_into_iter)]` implied by `#[warn(clippy::pedantic)]`
/// help: consider implementing `IntoIterator` for `&Pile`
///     |
/// 14  +
/// 15  + impl IntoIterator for &Pile {
/// 16  +     type Item = &basic::types::basic_card::BasicCard;
/// 17  +     type IntoIter = std::slice::Iter<'_, basic::types::basic_card::BasicCard>;
/// 18  +     fn into_iter(self) -> Self::IntoIter {
/// 19  +         self.iter()
/// 20  +     }
/// 21  + }
///     |
/// ```
///
/// Trying this code recommendation created the following error:
///
/// ```txt
/// error: in the trait associated type is declared without lifetime parameters, so using a borrowed type for them requires that lifetime to come from the implemented type
///    --> src/basic/types/pile.rs:281:17
///     |
/// 281 |     type Item = &basic::types::basic_card::BasicCard;
///     |                 ^ this lifetime must come from the implemented type
///
/// error[E0637]: `'_` cannot be used here
///    --> src/basic/types/pile.rs:282:38
///     |
/// 282 |     type IntoIter = std::slice::Iter<'_, basic::types::basic_card::BasicCard>;
///     |                                      ^^ `'_` is a reserved lifetime name
///
/// error[E0433]: failed to resolve: use of undeclared crate or module `basic`
///    --> src/basic/types/pile.rs:281:18
///     |
/// 281 |     type Item = &basic::types::basic_card::BasicCard;
///     |                  ^^^^^ use of undeclared crate or module `basic`
///     |
/// help: consider importing this module
///     |
/// 1   + use crate::basic::types::basic_card;
///     |
/// help: if you import `basic_card`, refer to it directly
///     |
/// 281 -     type Item = &basic::types::basic_card::BasicCard;
/// 281 +     type Item = &basic_card::BasicCard;
///     |
///
/// error[E0433]: failed to resolve: use of undeclared crate or module `basic`
///    --> src/basic/types/pile.rs:282:42
///     |
/// 282 |     type IntoIter = std::slice::Iter<'_, basic::types::basic_card::BasicCard>;
///     |                                          ^^^^^ use of undeclared crate or module `basic`
///     |
/// help: consider importing this module
///     |
/// 1   + use crate::basic::types::basic_card;
///     |
/// help: if you import `basic_card`, refer to it directly
///     |
/// 282 -     type IntoIter = std::slice::Iter<'_, basic::types::basic_card::BasicCard>;
/// 282 +     type IntoIter = std::slice::Iter<'_, basic_card::BasicCard>;
///     |
/// ```
///
/// This does give me hope that the mighty `Rust` compiler isn't infallible. This does, however,
/// provide one with a moment's pause, that one has to be careful, even when getting advice from
/// the final arbiter of truth in `Rustlevania`.
///
/// This led me to some `DuckDuckGoing` where I found an useful
/// [article by wrongbyte](https://dev.to/wrongbyte/implementing-iterator-and-intoiterator-in-rust-3nio).
///
/// After playing around with that I came up with this:
///
/// ```txt
/// impl<'a> IntoIterator for &Pile {
///     type Item = &'a BasicCard;
///     type IntoIter = std::slice::Iter<'a, BasicCard>;
///
///     fn into_iter(self) -> Self::IntoIter {
///         self.0.iter()
///     }
/// }
/// ```
///
/// which got us down to one error:
///
/// ```txt
/// error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
///    --> src/basic/types/pile.rs:326:6
///     |
/// 326 | impl<'a> IntoIterator for &Pile {
///     |      ^^ unconstrained lifetime parameter
/// ```
///
/// Ah yes, the dreaded [error #E0207](https://doc.rust-lang.org/error_codes/E0207.html), my old friend.
/// This is why coding in `Rust` is maybe not the best of ideas for idiot savants like me who
/// had diving into this level of trivia. Maybe I'm a Zig developer in a Rust devs body or
/// something, but I just want this shit to work.
///
/// Up till now, `CoPilot` has been just regurgitating back out the code I already wrote. In a
/// final hail mary I decided to ask it and see what it came up with.
///
/// > How do I fix the "the lifetime parameter `'a` is not constrained by the impl trait, self type,
/// > or predicates" error on impl<'a> IntoIterator for &Pile
///
/// > The error "the lifetime parameter 'a is not constrained by the impl trait, self type,
/// > or predicates" occurs because the lifetime 'a is not used in the trait implementation.
/// > To fix this, you need to specify the lifetime parameter correctly in the `IntoIterator`
/// > implementation.
///
/// And you can see it in action below. I was throwing lifeline's all over the place, with
/// no luck. I'll be honest with you, I am grokking the logic behind their placement about as
/// much as I was getting 11th grade Trig sitting behind Amy G. Mind you, this was the Golden
/// age of Guess Jeans, so I am not entirely at fault here.
///
/// Here is the corrected implementation:
impl<'a> IntoIterator for &'a Pile {
    type Item = &'a BasicCard;
    type IntoIter = std::slice::Iter<'a, BasicCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for Pile {
    type Item = BasicCard;
    type IntoIter = std::vec::IntoIter<BasicCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// endregion Iterator

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__pile_tests {
    use super::*;
    use crate::prelude::{Decked, French, FrenchRank, FrenchSuit, Standard52, Tarot};
    use std::str::FromStr;

    fn from_str(s: &str) -> Pile {
        Pile::from(&Deck::<Standard52>::from_str(s).unwrap())
    }

    #[test]
    fn combos() {
        let pile: Pile = Deck::<Standard52>::pile();
        let combinations = pile.combos(2);

        assert_eq!(combinations.len(), 1326);
    }

    #[test]
    fn combos_with_dups() {
        // let pile: Pile = (&Deck::<Standard52>::decks(2)).into();
        //
        // Much simper:
        let pile: Pile = Deck::<Standard52>::decks(2).into_pile();
        let combinations = pile.combos(2);
        let dups = pile.combos_with_dups(2);

        assert_eq!(combinations.len(), 1378);
        assert_eq!(dups.len(), 5356);
    }

    #[test]
    fn all_of_rank() {
        assert!(from_str("AS AD").all_of_rank(FrenchRank::ACE));
        assert!(from_str("AS AD AS").all_of_rank(FrenchRank::ACE));
        assert!(!from_str("AS AD").all_of_rank(FrenchRank::KING));
        assert!(!from_str("AS AD KS").all_of_rank(FrenchRank::ACE));
    }

    #[test]
    fn all_of_same_rank() {
        assert!(from_str("AS AD").all_of_same_rank());
        assert!(from_str("AS AD AS").all_of_same_rank());
        assert!(!from_str("AS AD KS").all_of_same_rank());
    }

    #[test]
    fn all_of_same_suit() {
        assert!(from_str("AS KS").all_of_same_suit());
        assert!(from_str("AS KS QS").all_of_same_suit());
        assert!(!from_str("AS KH QD").all_of_same_suit());
    }

    // copilot:
    // assert!(from_str("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(from_str("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(from_str("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(!from_str("AS AD").of_same_or_greater_rank(FrenchRank::KING));
    // assert!(!from_str("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
    #[test]
    fn of_same_or_greater_rank() {
        assert!(from_str("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(from_str("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(from_str("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!from_str("AS QD").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!from_str("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    }

    #[test]
    fn sort() {
        let mut pile = from_str("2♠ 8♣ 4♠");
        let mut pile2 = pile.clone();

        pile.sort();
        pile2.sort_by_rank();

        assert_eq!(pile.to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile2.to_string(), "8♣ 4♠ 2♠");
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region Pips

    #[test]
    fn cards_of_rank_pip_type() {
        let pile = French::pile();
        let jokers = pile.cards_of_rank_pip_type(PipType::Joker);

        assert_eq!(jokers.to_string(), "B🃟 L🃟");
    }

    #[test]
    fn cards_of_suit_pip_type() {
        let pile = French::pile();
        let jokers = pile.cards_of_suit_pip_type(PipType::Joker);

        assert_eq!(jokers.to_string(), "B🃟 L🃟");
    }

    #[test]
    fn cards_with_pip_type() {
        assert_eq!(
            Tarot::pile().cards_with_pip_type(PipType::Special).len(),
            22
        );
        assert_eq!(French::pile().cards_with_pip_type(PipType::Joker).len(), 2);
        assert!(French::pile()
            .cards_with_pip_type(PipType::Special)
            .is_empty());
    }

    #[test]
    fn ranks() {
        let pile = Deck::<French>::pile().shuffled();
        let expected = vec![
            FrenchRank::BIG_JOKER,
            FrenchRank::LITTLE_JOKER,
            FrenchRank::ACE,
            FrenchRank::KING,
            FrenchRank::QUEEN,
            FrenchRank::JACK,
            FrenchRank::TEN,
            FrenchRank::NINE,
            FrenchRank::EIGHT,
            FrenchRank::SEVEN,
            FrenchRank::SIX,
            FrenchRank::FIVE,
            FrenchRank::FOUR,
            FrenchRank::TREY,
            FrenchRank::DEUCE,
        ];

        let ranks = pile.ranks();

        assert_eq!(ranks, expected);
    }

    #[test]
    pub fn ranks_index() {
        let pile = Deck::<French>::pile().shuffled();
        let expected = "B~L~A~K~Q~J~T~9~8~7~6~5~4~3~2";

        let ranks_index = pile.ranks_index("~");

        assert_eq!(ranks_index, expected);
        assert_eq!(
            "K~Q~J~9~8~7",
            Deck::<French>::from_str("K♥ 9♣ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .into_pile()
                .ranks_index("~")
        );
    }

    #[test]
    pub fn ranks_by_suit() {
        let pile = Deck::<French>::from_str("A♠ K♠").unwrap().into_pile();

        let expected = vec![FrenchRank::ACE, FrenchRank::KING];

        assert_eq!(pile.ranks_by_suit(FrenchSuit::SPADES).unwrap(), expected);
        assert!(pile.ranks_by_suit(FrenchSuit::HEARTS).is_none());
    }

    #[test]
    pub fn ranks_index_by_suit() {
        let pile = Deck::<French>::from_str("A♠ K♠ A♣ Q♣ K♥")
            .unwrap()
            .into_pile();

        assert_eq!(
            pile.ranks_index_by_suit(FrenchSuit::SPADES, "-").unwrap(),
            "A-K"
        );
        assert_eq!(
            pile.ranks_index_by_suit(FrenchSuit::HEARTS, "-"),
            Some("K".to_string())
        );
        assert_eq!(
            pile.ranks_index_by_suit(FrenchSuit::CLUBS, "-"),
            Some("A-Q".to_string())
        );
        assert_eq!(pile.ranks_index_by_suit(FrenchSuit::DIAMONDS, "-"), None);
    }

    #[test]
    pub fn suits() {
        let pile = French::deck().shuffled().into_pile();
        let expected = vec![
            FrenchSuit::JOKER,
            FrenchSuit::SPADES,
            FrenchSuit::HEARTS,
            FrenchSuit::DIAMONDS,
            FrenchSuit::CLUBS,
        ];

        let suits = pile.suits();

        assert_eq!(suits, expected);
        assert_eq!(
            vec![FrenchSuit::HEARTS, FrenchSuit::CLUBS],
            Deck::<French>::from_str("K♥ 9♣ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .into_pile()
                .suits()
        );
    }

    #[test]
    pub fn suits_index() {
        let pile = French::deck().shuffled().into_pile();
        let expected = "J~S~H~D~C";

        let suits_index = pile.suits_index("~");

        assert_eq!(suits_index, expected);
        assert_eq!(
            "H~C",
            Deck::<French>::from_str("9♣ K♥ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .into_pile()
                .suits_index("~")
        );
    }

    #[test]
    pub fn suit_symbol_index() {
        let pile = French::deck().shuffled().into_pile();
        let expected = "🃟~♠~♥~♦~♣";

        let suits_index = pile.suit_symbol_index("~");

        assert_eq!(suits_index, expected);
        assert_eq!(
            "♥ ♣",
            Deck::<French>::from_str("9♣ K♥ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .into_pile()
                .suit_symbol_index(" ")
        );
    }

    // endregion Pips

    #[test]
    fn display() {
        let pile: Pile = (&Deck::<Standard52>::deck()).into();

        assert_eq!(pile.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣");
    }
}
