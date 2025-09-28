use crate::basic::types::traits::Ranged;
use crate::prelude::{BasicCard, DeckedBase, Pile};
use rand::prelude::SliceRandom;
use rand::rng;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct BasicPile(Vec<BasicCard>);

impl BasicPile {
    /// Returns a reference to the internal vector of the struct.
    #[must_use]
    pub fn v(&self) -> &Vec<BasicCard> {
        &self.0
    }

    /// Returns n number of [`BasicCards`](BasicCard) from the
    /// beginning of the `BasicPile`. If there are not enough cards in the `BasicPile` to satisfy
    /// the request, `None` is returned.
    ///
    /// `CoPilot`'s suggestion:
    /// ```txt
    /// use card_game_engine::prelude::{BasicPile, Pile};
    /// Where TF is CoPilot getting card_game_engine from???
    /// ```
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = Pinochle::deck();
    /// let hand = pile.draw(5).unwrap();
    ///
    /// assert_eq!(hand.to_string(), "A♠ A♠ T♠ T♠ K♠");
    /// ```
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
    /// avoid it. My common currency is vectors. The idea of treating the end
    /// of the vector as the top of the deck seems like a good one.
    pub fn draw_first(&mut self) -> Option<BasicCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
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

    pub fn iter(&self) -> std::slice::Iter<'_, BasicCard> {
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

    /// This sorts the cards by rank instead of suit.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut hand = BasicPile::from(vec![FrenchBasicCard::KING_SPADES, FrenchBasicCard::ACE_DIAMONDS]);
    ///
    /// // By default `BasicPile` sorts by suit.
    /// hand.sort();
    /// assert_eq!(hand.to_string(), "K♠ A♦");
    ///
    /// hand.sort_by_rank();
    /// assert_eq!(hand.to_string(), "A♦ K♠");
    /// ```
    pub fn sort_by_rank(&mut self) {
        self.0.sort_by(|a, b| b.rank.cmp(&a.rank));
    }

    /// Returns a new `BasicPile` sorted.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(basic!("2♠ A♣ 3♠ 4♠ 5♣").sorted().to_string(), "4♠ 3♠ 2♠ A♣ 5♣");
    /// ```
    #[must_use]
    pub fn sorted(&self) -> Self {
        let mut pile = self.clone();
        pile.sort();
        pile
    }

    /// Returns a new `BasicPile` with the `BasicCards` sorted.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// assert_eq!(basic!("2♠ A♣ 3♠ 4♠ 5♣").sorted_by_rank().to_string(), "A♣ 5♣ 4♠ 3♠ 2♠");
    /// ```
    #[must_use]
    pub fn sorted_by_rank(self) -> Self {
        let mut pile = self;
        pile.sort_by_rank();
        pile
    }
    // endregion
}

/// ```
/// use cardpack::prelude::*;
///
/// let hand = BasicPile::from(
///     vec![
///         FrenchBasicCard::NINE_CLUBS,
///         FrenchBasicCard::EIGHT_DIAMONDS,
///         FrenchBasicCard::SEVEN_CLUBS,
///     ]
/// );
///
/// assert_eq!(hand.to_string(), "9♣ 8♦ 7♣");
/// ```
impl Display for BasicPile {
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

impl From<Vec<BasicCard>> for BasicPile {
    fn from(cards: Vec<BasicCard>) -> Self {
        Self(cards)
    }
}

impl<DeckType: DeckedBase + Copy + Default + Ord + Hash> From<&Pile<DeckType>> for BasicPile {
    fn from(pack: &Pile<DeckType>) -> Self {
        pack.iter().map(|card| card.base_card).collect()
    }
}

impl Ranged for BasicPile {
    fn my_basic_pile(&self) -> BasicPile {
        self.clone()
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// region Iterator

impl FromIterator<BasicCard> for BasicPile {
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
impl<'a> IntoIterator for &'a BasicPile {
    type Item = &'a BasicCard;
    type IntoIter = std::slice::Iter<'a, BasicCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for BasicPile {
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
    use crate::basic;
    use crate::prelude::{Decked, French, FrenchRank, FrenchSuit, PipType, Standard52, Tarot};
    use std::str::FromStr;

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region Ranged

    #[test]
    fn ranged() {
        let pile: BasicPile = Pile::<Standard52>::basic_pile();
        let combinations = pile.combos(2);
        let dups = pile.combos(2);

        assert_eq!(combinations.len(), 1326);
        assert_eq!(combinations, dups);
    }

    #[test]
    fn combos() {
        let pile: BasicPile = Standard52::basic_pile();
        let combinations = pile.combos(2);

        assert_eq!(combinations.len(), 1326);
    }

    #[test]
    fn combos_with_dups() {
        // let pile: Pile = (&Pile::<Standard52>::decks(2)).into();
        //
        // Much simper:
        let pile = Standard52::decks(2);
        let combinations = pile.combos(2);
        let dups = pile.combos_with_dups(2);

        assert_eq!(combinations.len(), 1456);
        assert_eq!(dups.len(), 5356);
    }

    #[test]
    fn all_of_rank() {
        assert!(basic!("AS AD").all_of_rank(FrenchRank::ACE));
        assert!(basic!("AS AD AS").all_of_rank(FrenchRank::ACE));
        assert!(!basic!("AS AD").all_of_rank(FrenchRank::KING));
        assert!(!basic!("AS AD KS").all_of_rank(FrenchRank::ACE));
    }

    #[test]
    fn all_of_same_rank() {
        assert!(basic!("AS AD").all_of_same_rank());
        assert!(basic!("AS AD AS").all_of_same_rank());
        assert!(!basic!("AS AD KS").all_of_same_rank());
    }

    #[test]
    fn all_of_same_suit() {
        assert!(basic!("AS KS").all_of_same_suit());
        assert!(basic!("AS KS QS").all_of_same_suit());
        assert!(!basic!("AS KH QD").all_of_same_suit());
    }

    // copilot:
    // assert!(basic!("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(basic!("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(basic!("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    // assert!(!basic!("AS AD").of_same_or_greater_rank(FrenchRank::KING));
    // assert!(!basic!("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
    #[test]
    fn of_same_or_greater_rank() {
        assert!(basic!("AS AD").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(basic!("AS AD AS").of_same_or_greater_rank(FrenchRank::ACE));
        assert!(basic!("AS AD KS").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!basic!("AS QD").of_same_or_greater_rank(FrenchRank::KING));
        assert!(!basic!("AS AD KS").of_same_or_greater_rank(FrenchRank::ACE));
    }

    #[test]
    fn map_by_rank() {
        assert_eq!(
            "9♠ 9♦ 9♣, Q♠ Q♦, T♠, J♠",
            basic!("QD 9C QS 9S 9D TS JS")
                .combos_by_rank()
                .sort_internal()
                .to_string()
        );
    }

    // endregion Ranged

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region Pips

    #[test]
    fn cards_of_rank_pip_type() {
        let pile = French::basic_pile();
        let jokers = pile.cards_of_rank_pip_type(PipType::Joker);

        assert_eq!(jokers.to_string(), "B🃟 L🃟");
    }

    #[test]
    fn cards_of_suit_pip_type() {
        let pile = French::basic_pile();
        let jokers = pile.cards_of_suit_pip_type(PipType::Joker);

        assert_eq!(jokers.to_string(), "B🃟 L🃟");
    }

    #[test]
    fn cards_with_pip_type() {
        assert_eq!(
            Tarot::basic_pile()
                .cards_with_pip_type(PipType::Special)
                .len(),
            22
        );
        assert_eq!(
            French::basic_pile()
                .cards_with_pip_type(PipType::Joker)
                .len(),
            2
        );
        assert!(
            French::basic_pile()
                .cards_with_pip_type(PipType::Special)
                .is_empty()
        );
    }

    #[test]
    fn ranks() {
        let pile = Pile::<French>::basic_pile().shuffled();
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
        let pile = Pile::<French>::basic_pile().shuffled();
        let expected = "B~L~A~K~Q~J~T~9~8~7~6~5~4~3~2";

        let ranks_index = pile.ranks_index("~");

        assert_eq!(ranks_index, expected);
        assert_eq!(
            "K~Q~J~9~8~7",
            Pile::<French>::from_str("K♥ 9♣ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .ranks_index("~")
        );
    }

    #[test]
    pub fn ranks_by_suit() {
        let pile = Pile::<French>::from_str("A♠ K♠").unwrap();

        let expected = vec![FrenchRank::ACE, FrenchRank::KING];

        assert_eq!(pile.ranks_by_suit(FrenchSuit::SPADES).unwrap(), expected);
        assert!(pile.ranks_by_suit(FrenchSuit::HEARTS).is_none());
    }

    #[test]
    pub fn ranks_index_by_suit() {
        let pile = Pile::<French>::from_str("A♠ K♠ A♣ Q♣ K♥").unwrap();

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
        let pile = French::deck().shuffled();
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
            Pile::<French>::from_str("K♥ 9♣ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .suits()
        );
    }

    #[test]
    pub fn suits_index() {
        let pile = French::deck().shuffled();
        let expected = "J~S~H~D~C";

        let suits_index = pile.suits_index("~");

        assert_eq!(suits_index, expected);
        assert_eq!(
            "H~C",
            Pile::<French>::from_str("9♣ K♥ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .suits_index("~")
        );
    }

    #[test]
    pub fn suit_symbol_index() {
        let pile = French::deck().shuffled();
        let expected = "🃟~♠~♥~♦~♣";

        let suits_index = pile.suit_symbol_index("~");

        assert_eq!(suits_index, expected);
        assert_eq!(
            "♥ ♣",
            Pile::<French>::from_str("9♣ K♥ Q♥ J♥ 8♣ 7♣")
                .unwrap()
                .suit_symbol_index(" ")
        );
    }

    // endregion Pips

    #[test]
    fn sort() {
        let mut pile = basic!("2♠ 8♣ 4♠");
        let mut pile2 = pile.clone();

        pile.sort();
        pile2.sort_by_rank();

        assert_eq!(pile.to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile2.to_string(), "8♣ 4♠ 2♠");
    }

    #[test]
    fn sorted() {
        let pile = basic!("2♠ 8♣ 4♠").sorted();

        assert_eq!(pile.to_string(), "4♠ 2♠ 8♣");
    }

    #[test]
    fn display() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();

        assert_eq!(
            pile.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
    }
}
