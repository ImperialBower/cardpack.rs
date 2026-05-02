use crate::basic::types::traits::Ranged;
use crate::prelude::{BasicCard, DeckedBase, Pile};
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Display;
use core::hash::Hash;
use rand::prelude::SliceRandom;
#[cfg(feature = "std")]
use rand::rng;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// let mut pile = Pinochle::deck().into_basic_pile();
    /// let hand = pile.draw(5).unwrap();
    ///
    /// assert_eq!(hand.to_string(), "A♠ A♠ T♠ T♠ K♠");
    /// ```
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

    /// This is very much suboptimal, but I don't have an easy way to
    /// avoid it. My common currency is vectors. The idea of treating the end
    /// of the vector as the top of the deck seems like a good one.
    pub fn draw_first(&mut self) -> Option<BasicCard> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// Shuffles the `BasicPile` in place using the process default RNG
    /// (`rand::rng()`). For deterministic shuffling, use
    /// [`shuffle_with_seed`](Self::shuffle_with_seed).
    #[cfg(feature = "std")]
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rng());
    }

    /// Returns a new shuffled version of the `BasicPile`.
    ///
    /// For deterministic shuffling, use
    /// [`shuffled_with_seed`](Self::shuffled_with_seed).
    #[cfg(feature = "std")]
    #[must_use]
    pub fn shuffled(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle();
        pile
    }

    /// Shuffles the `BasicPile` in place deterministically from a `u64` seed.
    ///
    /// Uses [`rand::rngs::StdRng`] internally. Same seed produces the same
    /// permutation **within one `rand` major version**; a `rand` upgrade may
    /// change the result. For long-lived replay logs or cross-version
    /// reproducibility, pass a portable RNG (e.g., `ChaCha8Rng` from
    /// `rand_chacha`) to [`shuffle_with_rng`](Self::shuffle_with_rng) instead.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = Pile::<Standard52>::basic_pile();
    /// let a = pile.shuffled_with_seed(42);
    /// let b = pile.shuffled_with_seed(42);
    /// assert_eq!(a, b);
    /// ```
    pub fn shuffle_with_seed(&mut self, seed: u64) {
        self.shuffle_with_rng(&mut StdRng::seed_from_u64(seed));
    }

    /// Returns a new `BasicPile` shuffled deterministically from a `u64` seed.
    ///
    /// See [`shuffle_with_seed`](Self::shuffle_with_seed) for the
    /// portability caveat.
    #[must_use]
    pub fn shuffled_with_seed(&self, seed: u64) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_seed(seed);
        pile
    }

    /// Shuffles the `BasicPile` in place using the caller's RNG.
    ///
    /// Generic over any `R: Rng + ?Sized`. The seed-based methods are sugar
    /// over this primitive — pass your own RNG (e.g., `ChaCha8Rng`) for
    /// algorithm-stable reproducibility across `rand` major-version bumps.
    pub fn shuffle_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }

    /// Returns a new `BasicPile` shuffled using the caller's RNG.
    #[must_use]
    pub fn shuffled_with_rng<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let mut pile = self.clone();
        pile.shuffle_with_rng(rng);
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

    pub fn iter(&self) -> core::slice::Iter<'_, BasicCard> {
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
        self.0.sort_by_key(|b| core::cmp::Reverse(b.rank));
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
                .map(alloc::string::ToString::to_string)
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
    type IntoIter = core::slice::Iter<'a, BasicCard>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for BasicPile {
    type Item = BasicCard;
    type IntoIter = alloc::vec::IntoIter<BasicCard>;

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
    use alloc::string::ToString;
    use core::str::FromStr;

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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[test]
    fn draw__boundary_conditions() {
        let mut pile = basic!("AS KS QS JS");

        // Drawing more than available returns None (catches > -> == and > -> >= mutations)
        assert!(pile.draw(5).is_none());
        // Drawing exactly the pile length returns all cards
        let drawn = pile.draw(4);
        assert!(drawn.is_some());
        assert_eq!(drawn.unwrap().len(), 4);
        // Drawing from empty pile returns None
        assert!(pile.draw(1).is_none());
    }

    #[test]
    fn draw__zero_returns_empty() {
        let mut pile = basic!("AS KS");
        let drawn = pile.draw(0);
        assert!(drawn.is_some());
        assert_eq!(drawn.unwrap().len(), 0);
        assert_eq!(pile.len(), 2); // pile unchanged
    }

    #[test]
    fn draw_first__empty() {
        let mut pile = BasicPile::default();
        assert!(pile.draw_first().is_none());
    }

    #[test]
    fn extend__adds_cards() {
        let mut pile = basic!("AS KS");
        let other = basic!("QS JS");
        pile.extend(&other);
        assert_eq!(pile.len(), 4);
        assert_eq!(pile.to_string(), "A♠ K♠ Q♠ J♠");
    }

    #[test]
    fn get__returns_correct() {
        let pile = basic!("AS KS QS");
        // get(0) returns the first card
        assert!(pile.get(0).is_some());
        // get out of bounds returns None
        assert!(pile.get(100).is_none());
    }

    #[test]
    fn is_empty__false_when_populated() {
        let pile = basic!("AS KS");
        assert!(!pile.is_empty());
    }

    #[test]
    fn pop__removes_last() {
        let mut pile = basic!("AS KS");
        let popped = pile.pop();
        assert!(popped.is_some());
        assert_eq!(pile.len(), 1);
        // pop from empty returns None
        let mut empty = BasicPile::default();
        assert!(empty.pop().is_none());
    }

    #[test]
    fn shuffled_with_seed__deterministic() {
        let pile = Pile::<French>::basic_pile();
        let a = pile.shuffled_with_seed(42);
        let b = pile.shuffled_with_seed(42);
        assert_eq!(a, b, "same seed must produce identical permutation");
    }

    #[test]
    fn shuffled_with_seed__different_seeds_differ() {
        let pile = Pile::<French>::basic_pile();
        assert_ne!(
            pile.shuffled_with_seed(1),
            pile.shuffled_with_seed(2),
            "different seeds should almost always produce different orderings"
        );
    }

    #[test]
    fn shuffled_with_seed__same_cards() {
        let pile = Pile::<French>::basic_pile();
        let shuffled = pile.shuffled_with_seed(0xC0FFEE);
        assert_eq!(pile.len(), shuffled.len());
        let mut o_vec = pile.v().clone();
        let mut s_vec = shuffled.v().clone();
        o_vec.sort();
        s_vec.sort();
        assert_eq!(o_vec, s_vec, "shuffle must permute, not transform");
    }
}
