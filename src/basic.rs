pub mod types {
    pub mod pips {
        use serde::{Deserialize, Serialize};
        use std::fmt::Display;

        /// `PipType` is used to handle control flows for special, conditional processing of pips.
        ///
        /// Here's a simple hypothetical example:
        /// B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣
        /// ```
        /// use cardpack::prelude::*;
        ///
        /// let hand = french_cards!("A♠ B🃟 Q♠ J♠ T♠");
        ///
        /// let optimal_hand = match hand.cards_of_suit_pip_type(PipType::Joker).len() {
        ///   0 => hand,
        ///   _ => find_optimal_hand(hand),
        /// };
        ///
        /// fn find_optimal_hand(hand: Pile<French>) -> Pile<French> {
        ///     // Logic that returns the best scoring version of the hand with the joker.
        ///     hand
        /// }
        /// ```
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            Serialize,
            Deserialize,
        )]
        pub enum PipType {
            #[default]
            Blank,
            Suit,
            Rank,
            Joker,
            Special,
        }

        /// `Pip` is the smallest unit of a [`BasicCard`](crate::basic::types::card::BasicCard).
        ///
        /// Originally, I had different structs for `Rank` and `Suit`. The, I came to the realization that
        /// I could get the same results with a single struct. Eventually, I could see creating a card type
        /// that has an unlimited type of different pips stored in a vector. That's a TODO for after this
        /// version is done.
        ///
        /// Each Pip is made up of the following fields:
        ///
        /// - `weight`: A `u32` that is used for sorting.
        /// - `pip_type`: Used to classify the type of pip it is.
        /// - `index`: A `char` that is the key identifier for the `Pip`, such as 'A' for Ace.
        /// - `symbol`: A `char` that is the visual representation of the `Pip`, such as '♠' for Spades.
        /// - `value`: A `u32` that is used when a numerical valus is needed that is different than the `weight`.
        ///
        /// Each [`BasicCard`](crate::basic::types::card::BasicCard) struct is made up of two `Pips`, one
        /// representing the suit of the card and another for the rank.
        ///
        /// Here's a basic example of `Pips` in action:
        ///
        /// ```
        /// use cardpack::prelude::*;
        ///
        /// let trey_of_hearts = BasicCard {
        ///    suit: Pip {
        ///         weight: 2,
        ///         pip_type: PipType::Suit,
        ///         index: 'H',
        ///         symbol: '♥',
        ///         value: 3,
        ///     },
        ///     rank: Pip {
        ///         weight: 1,
        ///         pip_type: PipType::Rank,
        ///         index: '3',
        ///         symbol: '3',
        ///         value: 3,
        ///     },
        /// };
        ///
        /// assert_eq!(trey_of_hearts, FrenchBasicCard::TREY_HEARTS);
        /// ```
        #[derive(
            Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
        )]
        pub struct Pip {
            pub weight: u32,
            pub pip_type: PipType,
            pub index: char,
            pub symbol: char,
            pub value: u32,
        }

        impl Pip {
            /// The universal index for a blank `Pip` in a [`Card`](crate::pack::types::card::Card). Blank
            /// is the default value for all cards.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// assert!(Card::<French>::default().is_blank());
            /// ```
            pub const BLANK_INDEX: char = '_';

            /// TODO: HACK
            pub const PRIMES: [u32; 60] = [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281,
            ];

            #[must_use]
            pub fn new(pip_type: PipType, weight: u32, index: char, symbol: char) -> Self {
                Self {
                    weight,
                    pip_type,
                    index,
                    symbol,
                    ..Default::default()
                }
            }

            /// Factory method to update values as needed.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let expected = Pip {
            ///     pip_type: PipType::Rank,
            ///     weight: 12,
            ///     index: 'A',
            ///     symbol: 'A',
            ///     value: 22,
            /// };
            ///
            /// let updated_as: Pip = FrenchRank::ACE.update_value(22);
            ///
            /// assert_eq!(updated_as, expected);
            /// ```
            #[must_use]
            pub fn update_value(&self, value: u32) -> Self {
                Self { value, ..*self }
            }
        }

        impl Default for Pip {
            fn default() -> Self {
                Self {
                    pip_type: PipType::Blank,
                    weight: 0,
                    index: Pip::BLANK_INDEX,
                    symbol: Pip::BLANK_INDEX,
                    value: 0,
                }
            }
        }

        impl Display for Pip {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.symbol)
            }
        }
    }
    pub mod card {
        use crate::basic::types::pips::{Pip, PipType};
        use crate::common::utils::Bit;
        use crate::prelude::{CKCRevised, DeckedBase, Ranged};
        use crate::prelude::{Card, Pile};
        use rand::prelude::SliceRandom;
        use rand::rng;
        use serde::{Deserialize, Serialize};
        use std::cmp::Ordering;
        use std::error::Error;
        use std::fmt;
        use std::fmt::Display;
        use std::fs::File;
        use std::hash::Hash;
        use std::io::Read;

        // region Basic Card

        /// I've created this intermediary struct to make it easier to mix and match cards for related decks.
        /// Whilst [`Card`] needs to be generic so that we can easily share processing code in collections,
        /// the raw data in the [`crate::prelude::BasicCard`] should be simple and flexible.
        ///
        /// The [`crate::prelude::BasicCard`] struct is organized so that the suit [`Pip`] is first, followed by the rank
        /// [`Pip`] so that the default sorting for a collection is done suit first.
        ///
        /// **NOTE:**  The [`Ord`] and [`PartialOrd`] are customize so that the sorts are done in reverse
        /// order. This may be a mistake, since vectors are suboptimal taking from the beginning.
        ///
        /// TODO RF: Structure the code so that the end of the vector is treated as the top of the deck
        /// in terms of how it is interacted with. So when you call `draw()` on a deck you are taking from
        /// the bottom of the vector.
        ///
        /// [Playing cards in Unicode](https://en.wikipedia.org/wiki/Playing_cards_in_Unicode)
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
        pub struct BasicCard {
            pub suit: Pip,
            pub rank: Pip,
        }

        impl BasicCard {
            /// Reads in a YAML file version of `BasicCard` data at the passed in location and returns a vector of `BasicCards`. See the
            /// [`Razz`](crate::pack::decks::razz::Razz) deck for an example of how to use this method.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let cards = BasicCard::cards_from_yaml_file("src/basic/decks/yaml/french.yaml").unwrap();
            ///
            /// assert_eq!(cards.len(), 54);
            /// assert_eq!(cards, Pile::<French>::base_vec());
            /// ```
            ///
            /// # Errors
            ///
            /// Throws an error for an invalid path or invalid data.
            pub fn cards_from_yaml_file(file_path: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
                let mut file = File::open(file_path)?;
                let mut contents = String::new();

                file.read_to_string(&mut contents)?;

                BasicCard::cards_from_yaml_str(&contents)
            }

            /// Takes in a YAML string and returns a vector of `BasicCards`.
            ///
            /// # Errors
            ///
            /// Throws an error for an invalid path or invalid data.
            pub fn cards_from_yaml_str(yaml_str: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
                let cards: Vec<BasicCard> = serde_yml::from_str(yaml_str)?;

                Ok(cards)
            }

            /// The index is the most basic way to represent a `Card` as a `String` using
            /// only basic characters. It is made up of the rank [`Pip`] index followed by the
            /// suit [`Pip`] index.
            ///
            /// For example, the Jack of Diamonds index value is `JD`, while it's
            /// display value is `J♦`:
            ///
            /// ```rust
            /// use cardpack::prelude::*;
            ///
            /// assert_eq!(FrenchBasicCard::JACK_DIAMONDS.index(), "JD");
            /// assert_eq!(FrenchBasicCard::JACK_DIAMONDS.to_string(), "J♦");
            /// ```
            #[must_use]
            pub fn index(&self) -> String {
                format!("{}{}", self.rank.index, self.suit.index)
            }

            /// Returns true if either the rank [`Pip`] or the suit [`Pip`] has a value of `_`,
            #[must_use]
            pub fn is_blank(&self) -> bool {
                self.rank.index == Pip::BLANK_INDEX || self.suit.index == Pip::BLANK_INDEX
            }
        }

        impl CKCRevised for BasicCard {
            fn get_ckc_number(&self) -> u32 {
                if self.is_blank() {
                    return 0;
                }
                self.ckc_rank_number() + self.ckc_suit_number()
            }

            fn ckc_rank_number(&self) -> u32 {
                self.ckc_rank_bits() | self.ckc_rank_shift8() | self.ckc_get_prime()
            }

            // TODO: This needs to be moved out of Basic. Maybe a trait? Maybe just move it out altogether?
            fn ckc_suit_number(&self) -> u32 {
                if self.suit.pip_type == PipType::Joker {
                    return 0;
                }
                match self.suit.value {
                    1..=4 => 1 << (Bit::SUIT_FLAG_SHIFT + self.suit.value),
                    _ => 0,
                }
            }

            fn ckc_rank_bits(&self) -> u32 {
                1 << (Bit::RANK_FLAG_SHIFT + self.rank.weight)
            }

            fn ckc_get_prime(&self) -> u32 {
                if self.rank.weight as usize >= Pip::PRIMES.len() {
                    0
                } else {
                    Pip::PRIMES[(self.rank.weight) as usize]
                }
            }

            fn ckc_rank_shift8(&self) -> u32 {
                self.rank.weight << 8
            }
        }

        impl Display for BasicCard {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}{}", self.rank.symbol, self.suit.symbol)
            }
        }

        impl<DeckType: DeckedBase> From<Card<DeckType>> for BasicCard {
            fn from(card: Card<DeckType>) -> Self {
                Self {
                    suit: card.base_card.suit,
                    rank: card.base_card.rank,
                }
            }
        }

        /// Inverts the order so that the highest card comes first.
        impl Ord for BasicCard {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .suit
                    .cmp(&self.suit)
                    .then_with(|| other.rank.cmp(&self.rank))
            }
        }

        impl PartialOrd for BasicCard {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // endregion

        // region BasicPile
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

            /// Returns a new `BasicPile` with the `BasicCards` sorted.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let hand = BasicPile::from(vec![FrenchBasicCard::KING_SPADES, FrenchBasicCard::ACE_DIAMONDS]);
            ///
            /// assert_eq!(hand.sorted_by_rank().to_string(), "A♦ K♠");
            /// ```
            #[must_use]
            pub fn sorted_by_rank(self) -> Self {
                let mut pile = self.clone();
                pile.0.sort_by(|a, b| b.rank.cmp(&a.rank));
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
        // endregion BasicPile
    }
    pub mod gto {
        use crate::basic::types::card::{BasicCard, BasicPile};
        use crate::basic::types::pips::Pip;
        use crate::traits::Ranged;
        use std::fmt::{Display, Formatter};

        #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        pub struct Combos(Vec<BasicPile>);

        impl Combos {
            /// OK, this refactoring is hot AF. This is what I am looking for when I refactor.
            ///
            /// 1. Find a problem.
            /// 2. Define it in a test.
            /// 3. Solve the problem no matter how ugly the code is.
            /// 4. Refactor into a something prettier.
            /// 5. ...
            /// 6. Profile
            ///
            /// Now this is what we call the `Red -> Green -> Refactor` TDD loop. When I am coding
            /// in a new language, one of the things I look out for is ways to find these idiomatic
            /// refactoring opportunities.
            ///
            /// When you code, over time you start to start to develop an accent based on the language you
            /// are most comfortable with. For me, at first, that was Perl. Later on, when I tried to level
            /// up from a `Web Master` to an `Enterprise Architect`, it became Java.
            ///
            /// ## Funny story:
            ///
            /// The second job I got for a startup was for this company called `Katmango`. OK,
            /// I know the name is trash, but I respected the guy who reached out to get me to join,
            /// and the tech seemed to be a good idea. (In fact, the idea of having a unified sign-on was
            /// so good, that Google and Apple introduced the tech themselves, making our efforts
            /// a complete waste of time.) I must say, I don't miss those days being razzed for not being
            /// enough of a team player for only putting in 60 hours a week, and overhearing the CTO and CEO
            /// sitting behind me in the South of Market repurposed sweat shop, talking about how if they
            /// didn't secure another round of funding we wouldn't miss payroll.
            ///
            /// So, anyway. One day, I was told that I was going to be the companies "Web Master". I was
            /// so excited. This was like the coolest sounding title there was in the great days
            /// of the dot.com boom. A little while later, one of my colleagues clued me in. Web Master
            /// was the job they gave to the dev on the team with the least amount of experience. It was
            /// the equivalent to Nate's original job in Ted Lasso. It was the job where you dealt with
            /// the most annoying aspect of any web business: customer complaints. That was a good level
            /// set.
            ///
            /// ## Back to the point
            ///
            /// I learned this lesson in the most wonderful way from my friend Jim Prior. TODO:
            ///
            /// ---------------------------------------------------------
            ///
            /// The main lesson I've gotten from all that is to strive to learn the basic idiomatic ways
            /// to work within a language.
            ///
            /// TODO: RF - This code is ugly AF.
            #[must_use]
            pub fn connectors(&self) -> Self {
                self.iter()
                    .filter(|pile| pile.is_connector())
                    .cloned()
                    .collect::<Combos>()
                    .iter()
                    .map(|pile| pile.clone().sorted_by_rank())
                    .collect::<Combos>()
            }

            /// I love how the `CoPilot` version recommends functions that don't exist instead of
            /// the one that does.
            ///
            /// ```txt
            /// self.0
            ///             .iter()
            ///             .filter(|pile| pile.contains_rank(rank))
            ///             .cloned()
            ///             .collect()
            /// ```
            #[must_use]
            pub fn of_rank(&self, rank: Pip) -> Self {
                self.iter()
                    .filter(|pile| pile.all_of_rank(rank))
                    .cloned()
                    .collect()
            }

            /// TODO: RF Should be able to create a common function that accepts a closure.
            #[must_use]
            pub fn of_same_rank(&self) -> Self {
                self.iter()
                    .filter(|pile| pile.all_of_same_rank())
                    .cloned()
                    .collect()
            }

            #[must_use]
            pub fn of_same_rank_or_above(&self, rank: Pip) -> Self {
                self.of_same_rank()
                    .iter()
                    .filter(|pile| pile.v().first().unwrap_or(&BasicCard::default()).rank >= rank)
                    .cloned()
                    .collect()
            }

            #[must_use]
            pub fn suited(&self) -> Self {
                self.0
                    .iter()
                    .filter(|pile| pile.all_of_same_suit())
                    .cloned()
                    .collect()
            }

            #[must_use]
            pub fn unsuited(&self) -> Self {
                self.0
                    .iter()
                    .filter(|pile| !pile.all_of_same_suit())
                    .cloned()
                    .collect()
            }

            //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
            // region vector functions

            #[must_use]
            pub fn get(&self, position: usize) -> Option<&BasicPile> {
                self.0.get(position)
            }

            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn iter(&self) -> std::slice::Iter<BasicPile> {
                self.0.iter()
            }

            #[must_use]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn push(&mut self, pile: BasicPile) {
                self.0.push(pile);
            }

            pub fn pop(&mut self) -> Option<BasicPile> {
                self.0.pop()
            }

            pub fn reverse(&mut self) {
                self.0.reverse();
            }

            pub fn sort(&mut self) {
                self.0.sort();
            }

            #[must_use]
            pub fn v(&self) -> &Vec<BasicPile> {
                &self.0
            }

            // endregion
        }

        impl Display for Combos {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }

        impl From<Vec<BasicPile>> for Combos {
            fn from(v: Vec<BasicPile>) -> Self {
                Self(v)
            }
        }

        impl From<&Vec<BasicPile>> for Combos {
            fn from(v: &Vec<BasicPile>) -> Self {
                Self(v.clone())
            }
        }

        impl FromIterator<BasicPile> for Combos {
            fn from_iter<T: IntoIterator<Item = BasicPile>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }

        impl Iterator for Combos {
            type Item = BasicPile;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.pop()
            }
        }

        impl<'a> IntoIterator for &'a Combos {
            type Item = &'a BasicPile;
            type IntoIter = std::slice::Iter<'a, BasicPile>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    }
}

pub mod cards {
    pub mod french {
        use crate::basic::types::card::BasicCard;
        use crate::basic::types::pips::{Pip, PipType};

        pub struct FrenchBasicCard;
        pub struct FrenchSuit;
        pub struct FrenchRank;

        pub const FLUENT_KEY_BASE_NAME_FRENCH: &str = "french";

        impl FrenchBasicCard {
            pub const BIG_JOKER: BasicCard = BasicCard {
                suit: FrenchSuit::JOKER,
                rank: FrenchRank::BIG_JOKER,
            };
            pub const LITTLE_JOKER: BasicCard = BasicCard {
                suit: FrenchSuit::JOKER,
                rank: FrenchRank::LITTLE_JOKER,
            };
            pub const ACE_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::ACE,
            };
            pub const KING_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::KING,
            };
            pub const QUEEN_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::QUEEN,
            };
            pub const JACK_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::JACK,
            };
            pub const TEN_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::TEN,
            };
            pub const NINE_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::NINE,
            };
            pub const EIGHT_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::EIGHT,
            };
            pub const SEVEN_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::SEVEN,
            };
            pub const SIX_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::SIX,
            };
            pub const FIVE_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::FIVE,
            };
            pub const FOUR_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::FOUR,
            };
            pub const TREY_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::TREY,
            };
            pub const DEUCE_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: FrenchRank::DEUCE,
            };
            pub const ACE_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::ACE,
            };
            pub const KING_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::KING,
            };
            pub const QUEEN_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::QUEEN,
            };
            pub const JACK_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::JACK,
            };
            pub const TEN_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::TEN,
            };
            pub const NINE_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::NINE,
            };
            pub const EIGHT_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::EIGHT,
            };
            pub const SEVEN_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::SEVEN,
            };
            pub const SIX_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::SIX,
            };
            pub const FIVE_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::FIVE,
            };
            pub const FOUR_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::FOUR,
            };
            pub const TREY_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::TREY,
            };
            pub const DEUCE_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: FrenchRank::DEUCE,
            };
            pub const ACE_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::ACE,
            };
            pub const KING_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::KING,
            };
            pub const QUEEN_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::QUEEN,
            };
            pub const JACK_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::JACK,
            };
            pub const TEN_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::TEN,
            };
            pub const NINE_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::NINE,
            };
            pub const EIGHT_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::EIGHT,
            };
            pub const SEVEN_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::SEVEN,
            };
            pub const SIX_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::SIX,
            };
            pub const FIVE_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::FIVE,
            };
            pub const FOUR_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::FOUR,
            };
            pub const TREY_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::TREY,
            };
            pub const DEUCE_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: FrenchRank::DEUCE,
            };
            pub const ACE_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::ACE,
            };
            pub const KING_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::KING,
            };
            pub const QUEEN_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::QUEEN,
            };
            pub const JACK_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::JACK,
            };
            pub const TEN_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::TEN,
            };
            pub const NINE_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::NINE,
            };
            pub const EIGHT_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::EIGHT,
            };
            pub const SEVEN_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::SEVEN,
            };
            pub const SIX_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::SIX,
            };
            pub const FIVE_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::FIVE,
            };
            pub const FOUR_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::FOUR,
            };
            pub const TREY_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::TREY,
            };
            pub const DEUCE_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: FrenchRank::DEUCE,
            };
        }

        impl FrenchSuit {
            pub const JOKER: Pip = Pip {
                pip_type: PipType::Joker,
                weight: 4,
                index: 'J',
                symbol: '🃟',
                value: 5,
            };
            pub const SPADES: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 3,
                index: 'S',
                symbol: '♠',
                value: 4,
            };
            pub const HEARTS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 2,
                index: 'H',
                symbol: '♥',
                value: 3,
            };
            pub const DIAMONDS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 1,
                index: 'D',
                symbol: '♦',
                value: 2,
            };
            pub const CLUBS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 0,
                index: 'C',
                symbol: '♣',
                value: 1,
            };
        }

        impl FrenchRank {
            pub const BIG_JOKER: Pip = Pip {
                pip_type: PipType::Joker,
                weight: 14,
                index: 'B',
                symbol: 'B',
                value: 13,
            };
            pub const LITTLE_JOKER: Pip = Pip {
                pip_type: PipType::Joker,
                weight: 13,
                index: 'L',
                symbol: 'L',
                value: 12,
            };

            pub const ACE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 12,
                index: 'A',
                symbol: 'A',
                value: 11,
            };
            pub const KING: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 11,
                index: 'K',
                symbol: 'K',
                value: 10,
            };
            pub const QUEEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 10,
                index: 'Q',
                symbol: 'Q',
                value: 10,
            };
            pub const JACK: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 9,
                index: 'J',
                symbol: 'J',
                value: 10,
            };
            pub const TEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 8,
                index: 'T',
                symbol: 'T',
                value: 10,
            };
            pub const NINE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 7,
                index: '9',
                symbol: '9',
                value: 9,
            };
            pub const EIGHT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 6,
                index: '8',
                symbol: '8',
                value: 8,
            };
            pub const SEVEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 5,
                index: '7',
                symbol: '7',
                value: 7,
            };
            pub const SIX: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 4,
                index: '6',
                symbol: '6',
                value: 6,
            };
            pub const FIVE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 3,
                index: '5',
                symbol: '5',
                value: 5,
            };
            pub const FOUR: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 2,
                index: '4',
                symbol: '4',
                value: 4,
            };
            pub const TREY: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 1,
                index: '3',
                symbol: '3',
                value: 3,
            };
            pub const DEUCE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 0,
                index: '2',
                symbol: '2',
                value: 2,
            };
        }
    }
    pub mod canasta {
        use crate::prelude::{BasicCard, FrenchRank, Pip, PipType};

        pub struct CanastaBasicCard;
        pub struct CanastaSuit;
        pub struct CanastaRank;

        pub const FLUENT_KEY_BASE_NAME_CANASTA: &str = "canasta";

        /// Use the `Canasta` `Deck` as a way to illustrate system evolution.
        impl CanastaBasicCard {
            pub const TREY_HEARTS: BasicCard = BasicCard {
                suit: CanastaSuit::TREY_HEARTS,
                rank: CanastaRank::RED_TREY,
            };
            pub const TREY_DIAMONDS: BasicCard = BasicCard {
                suit: CanastaSuit::TREY_DIAMONDS,
                rank: CanastaRank::RED_TREY,
            };
            pub const BIG_JOKER: BasicCard = BasicCard {
                suit: CanastaSuit::JOKER,
                rank: FrenchRank::BIG_JOKER,
            };
            pub const LITTLE_JOKER: BasicCard = BasicCard {
                suit: CanastaSuit::JOKER,
                rank: FrenchRank::LITTLE_JOKER,
            };
            pub const DEUCE_SPADES: BasicCard = BasicCard {
                suit: CanastaSuit::DEUCE_SPADES,
                rank: CanastaRank::DEUCE,
            };
            pub const DEUCE_HEARTS: BasicCard = BasicCard {
                suit: CanastaSuit::DEUCE_HEARTS,
                rank: CanastaRank::DEUCE,
            };
            pub const DEUCE_DIAMONDS: BasicCard = BasicCard {
                suit: CanastaSuit::DEUCE_DIAMONDS,
                rank: CanastaRank::DEUCE,
            };
            pub const DEUCE_CLUBS: BasicCard = BasicCard {
                suit: CanastaSuit::DEUCE_CLUBS,
                rank: CanastaRank::DEUCE,
            };
        }

        impl CanastaRank {
            pub const RED_TREY: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 15,
                index: '3',
                symbol: '3',
                value: 3,
            };
            pub const DEUCE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 13,
                index: '2',
                symbol: '2',
                value: 2,
            };
        }

        impl CanastaSuit {
            pub const TREY_HEARTS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 11,
                index: 'H',
                symbol: '♥',
                value: 1,
            };
            pub const TREY_DIAMONDS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 10,
                index: 'D',
                symbol: '♦',
                value: 2,
            };
            pub const JOKER: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 9,
                index: 'J',
                symbol: '🃟',
                value: 4,
            };
            pub const DEUCE_SPADES: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 8,
                index: 'S',
                symbol: '♠',
                value: 3,
            };
            pub const DEUCE_HEARTS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 7,
                index: 'H',
                symbol: '♥',
                value: 1,
            };
            pub const DEUCE_DIAMONDS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 6,
                index: 'D',
                symbol: '♦',
                value: 2,
            };
            pub const DEUCE_CLUBS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 5,
                index: 'C',
                symbol: '♣',
                value: 4,
            };
        }
    }
    pub mod pinochle {
        use crate::prelude::{BasicCard, FrenchSuit, Pip, PipType};

        pub struct PinochleBasicCard;
        pub struct PinochleRank;

        pub const FLUENT_KEY_BASE_NAME_PINOCHLE: &str = "pinochle";

        impl PinochleBasicCard {
            pub const TEN_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: PinochleRank::TEN,
            };
            pub const KING_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: PinochleRank::KING,
            };
            pub const QUEEN_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: PinochleRank::QUEEN,
            };
            pub const JACK_SPADES: BasicCard = BasicCard {
                suit: FrenchSuit::SPADES,
                rank: PinochleRank::JACK,
            };
            pub const TEN_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: PinochleRank::TEN,
            };
            pub const KING_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: PinochleRank::KING,
            };
            pub const QUEEN_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: PinochleRank::QUEEN,
            };
            pub const JACK_HEARTS: BasicCard = BasicCard {
                suit: FrenchSuit::HEARTS,
                rank: PinochleRank::JACK,
            };
            pub const TEN_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: PinochleRank::TEN,
            };
            pub const KING_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: PinochleRank::KING,
            };
            pub const QUEEN_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: PinochleRank::QUEEN,
            };
            pub const JACK_DIAMONDS: BasicCard = BasicCard {
                suit: FrenchSuit::DIAMONDS,
                rank: PinochleRank::JACK,
            };
            pub const TEN_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: PinochleRank::TEN,
            };
            pub const KING_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: PinochleRank::KING,
            };
            pub const QUEEN_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: PinochleRank::QUEEN,
            };
            pub const JACK_CLUBS: BasicCard = BasicCard {
                suit: FrenchSuit::CLUBS,
                rank: PinochleRank::JACK,
            };
        }

        impl PinochleRank {
            pub const TEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 11,
                index: 'T',
                symbol: 'T',
                value: 10,
            };
            pub const KING: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 10,
                index: 'K',
                symbol: 'K',
                value: 10,
            };
            pub const QUEEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 9,
                index: 'Q',
                symbol: 'Q',
                value: 10,
            };
            pub const JACK: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 8,
                index: 'J',
                symbol: 'J',
                value: 10,
            };
        }
    }
    pub mod skat {
        use crate::prelude::{BasicCard, Pip, PipType};

        pub struct SkatBasicCard;
        pub struct SkatSuit;
        pub struct SkatRank;

        pub const FLUENT_KEY_BASE_NAME_SKAT: &str = "skat";

        impl SkatBasicCard {
            pub const DAUSE_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::DAUSE,
            };
            pub const ZHEN_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::ZHEN,
            };
            pub const KÖNIG_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::KÖNIG,
            };
            pub const OBER_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::OBER,
            };
            pub const UNTER_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::UNTER,
            };
            pub const NEUN_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::NEUN,
            };
            pub const ACHT_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::ACHT,
            };
            pub const SIEBEN_EICHEL: BasicCard = BasicCard {
                suit: SkatSuit::EICHEL,
                rank: SkatRank::SIEBEN,
            };

            pub const DAUSE_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::DAUSE,
            };
            pub const ZHEN_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::ZHEN,
            };
            pub const KÖNIG_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::KÖNIG,
            };
            pub const OBER_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::OBER,
            };
            pub const UNTER_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::UNTER,
            };
            pub const NEUN_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::NEUN,
            };
            pub const ACHT_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::ACHT,
            };
            pub const SIEBEN_LAUB: BasicCard = BasicCard {
                suit: SkatSuit::LAUB,
                rank: SkatRank::SIEBEN,
            };

            pub const DAUSE_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::DAUSE,
            };
            pub const ZHEN_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::ZHEN,
            };
            pub const KÖNIG_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::KÖNIG,
            };
            pub const OBER_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::OBER,
            };
            pub const UNTER_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::UNTER,
            };
            pub const NEUN_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::NEUN,
            };
            pub const ACHT_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::ACHT,
            };
            pub const SIEBEN_HERZ: BasicCard = BasicCard {
                suit: SkatSuit::HERZ,
                rank: SkatRank::SIEBEN,
            };

            pub const DAUSE_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::DAUSE,
            };
            pub const ZHEN_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::ZHEN,
            };
            pub const KÖNIG_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::KÖNIG,
            };
            pub const OBER_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::OBER,
            };
            pub const UNTER_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::UNTER,
            };
            pub const NEUN_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::NEUN,
            };
            pub const ACHT_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::ACHT,
            };
            pub const SIEBEN_SHELLEN: BasicCard = BasicCard {
                suit: SkatSuit::SHELLEN,
                rank: SkatRank::SIEBEN,
            };
        }

        impl SkatSuit {
            pub const EICHEL: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 3,
                index: 'E',
                symbol: '♣',
                value: 4,
            };
            pub const LAUB: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 2,
                index: 'L',
                symbol: '♠',
                value: 3,
            };
            pub const HERZ: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 1,
                index: 'H',
                symbol: '♥',
                value: 2,
            };
            pub const SHELLEN: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 0,
                index: 'S',
                symbol: '♦',
                value: 1,
            };
        }

        impl SkatRank {
            pub const DAUSE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 7,
                index: 'D',
                symbol: 'D',
                value: 0,
            };
            pub const ZHEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 6,
                index: 'Z',
                symbol: 'Z',
                value: 0,
            };
            pub const KÖNIG: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 5,
                index: 'K',
                symbol: 'K',
                value: 0,
            };
            pub const OBER: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 4,
                index: 'O',
                symbol: 'O',
                value: 0,
            };
            pub const UNTER: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 3,
                index: 'U',
                symbol: 'U',
                value: 0,
            };
            pub const NEUN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 2,
                index: '9',
                symbol: '9',
                value: 2,
            };
            pub const ACHT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 1,
                index: '8',
                symbol: '8',
                value: 0,
            };
            pub const SIEBEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 0,
                index: '7',
                symbol: '7',
                value: 0,
            };
        }
    }
    pub mod tarot {
        use crate::prelude::{BasicCard, Pip, PipType};

        pub struct TarotBasicCard;
        pub struct TarotSuit;
        pub struct TarotRank;

        pub const FLUENT_KEY_BASE_NAME_TAROT: &str = "tarot";

        impl TarotBasicCard {
            pub const FOOL: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::FOOL,
            };
            pub const MAGICIAN: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::MAGICIAN,
            };
            pub const HIGH_PRIESTESS: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::HIGH_PRIESTESS,
            };
            pub const EMPRESS: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::EMPRESS,
            };
            pub const EMPEROR: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::EMPEROR,
            };
            pub const HIEROPHANT: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::HIEROPHANT,
            };
            pub const LOVERS: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::LOVERS,
            };
            pub const CHARIOT: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::CHARIOT,
            };
            pub const STRENGTH: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::STRENGTH,
            };
            pub const HERMIT: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::HERMIT,
            };
            pub const WHEEL_OF_FORTUNE: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::WHEEL_OF_FORTUNE,
            };
            pub const JUSTICE: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::JUSTICE,
            };
            pub const HANGED_MAN: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::HANGED_MAN,
            };
            pub const DEATH: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::DEATH,
            };
            pub const TEMPERANCE: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::TEMPERANCE,
            };
            pub const DEVIL: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::DEVIL,
            };
            pub const TOWER: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::TOWER,
            };
            pub const STAR: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::STAR,
            };
            pub const MOON: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::MOON,
            };
            pub const SUN: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::SUN,
            };
            pub const JUDGEMENT: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::JUDGEMENT,
            };
            pub const WORLD: BasicCard = BasicCard {
                suit: TarotSuit::MAJOR_ARCANA,
                rank: TarotRank::WORLD,
            };

            pub const KING_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::KING,
            };
            pub const QUEEN_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::QUEEN,
            };
            pub const KNIGHT_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::KNIGHT,
            };
            pub const PAGE_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::PAGE,
            };
            pub const TEN_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::TEN,
            };
            pub const NINE_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::NINE,
            };
            pub const EIGHT_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::EIGHT,
            };
            pub const SEVEN_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::SEVEN,
            };
            pub const SIX_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::SIX,
            };
            pub const FIVE_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::FIVE,
            };
            pub const FOUR_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::FOUR,
            };
            pub const THREE_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::THREE,
            };
            pub const TWO_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::TWO,
            };
            pub const ACE_WANDS: BasicCard = BasicCard {
                suit: TarotSuit::WANDS,
                rank: TarotRank::ACE,
            };
            pub const KING_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::KING,
            };
            pub const QUEEN_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::QUEEN,
            };
            pub const KNIGHT_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::KNIGHT,
            };
            pub const PAGE_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::PAGE,
            };
            pub const TEN_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::TEN,
            };
            pub const NINE_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::NINE,
            };
            pub const EIGHT_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::EIGHT,
            };
            pub const SEVEN_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::SEVEN,
            };
            pub const SIX_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::SIX,
            };
            pub const FIVE_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::FIVE,
            };
            pub const FOUR_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::FOUR,
            };
            pub const THREE_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::THREE,
            };
            pub const TWO_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::TWO,
            };
            pub const ACE_CUPS: BasicCard = BasicCard {
                suit: TarotSuit::CUPS,
                rank: TarotRank::ACE,
            };
            pub const KING_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::KING,
            };
            pub const QUEEN_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::QUEEN,
            };
            pub const KNIGHT_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::KNIGHT,
            };
            pub const PAGE_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::PAGE,
            };
            pub const TEN_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::TEN,
            };
            pub const NINE_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::NINE,
            };
            pub const EIGHT_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::EIGHT,
            };
            pub const SEVEN_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::SEVEN,
            };
            pub const SIX_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::SIX,
            };
            pub const FIVE_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::FIVE,
            };
            pub const FOUR_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::FOUR,
            };
            pub const THREE_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::THREE,
            };
            pub const TWO_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::TWO,
            };
            pub const ACE_SWORDS: BasicCard = BasicCard {
                suit: TarotSuit::SWORDS,
                rank: TarotRank::ACE,
            };
            pub const KING_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::KING,
            };
            pub const QUEEN_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::QUEEN,
            };
            pub const KNIGHT_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::KNIGHT,
            };
            pub const PAGE_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::PAGE,
            };
            pub const TEN_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::TEN,
            };
            pub const NINE_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::NINE,
            };
            pub const EIGHT_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::EIGHT,
            };
            pub const SEVEN_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::SEVEN,
            };
            pub const SIX_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::SIX,
            };
            pub const FIVE_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::FIVE,
            };
            pub const FOUR_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::FOUR,
            };
            pub const THREE_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::THREE,
            };
            pub const TWO_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::TWO,
            };
            pub const ACE_PENTACLES: BasicCard = BasicCard {
                suit: TarotSuit::PENTACLES,
                rank: TarotRank::ACE,
            };
        }

        impl TarotSuit {
            pub const MAJOR_ARCANA: Pip = Pip {
                pip_type: PipType::Special,
                weight: 4,
                index: 'M',
                symbol: '🔮',
                value: 5,
            };
            pub const WANDS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 3,
                index: 'W',
                symbol: '🪄',
                value: 4,
            };
            pub const CUPS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 2,
                index: 'C',
                symbol: '🍷',
                value: 3,
            };
            pub const SWORDS: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 1,
                index: 'S',
                symbol: '⚔',
                value: 2,
            };
            pub const PENTACLES: Pip = Pip {
                pip_type: PipType::Suit,
                weight: 0,
                index: 'P',
                symbol: '☆',
                value: 1,
            };
        }

        impl TarotRank {
            // ABC___GHIJKL_NO_QR__UVWXYZ
            // Major Arcana
            pub const FOOL: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 22,
                index: 'F',
                symbol: '🤡',
                value: 23,
            };
            pub const MAGICIAN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 21,
                index: 'M',
                symbol: '🎩',
                value: 22,
            };
            pub const HIGH_PRIESTESS: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 20,
                index: 'P',
                symbol: '😇',
                value: 21,
            };
            pub const EMPRESS: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 19,
                index: 'E',
                symbol: '👸',
                value: 20,
            };
            pub const EMPEROR: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 18,
                index: 'R',
                symbol: '🤴',
                value: 19,
            };
            pub const HIEROPHANT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 17,
                index: 'H',
                symbol: '👑',
                value: 18,
            };
            pub const LOVERS: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 16,
                index: 'L',
                symbol: '💑',
                value: 17,
            };
            pub const CHARIOT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 15,
                index: 'C',
                symbol: '🏎',
                value: 16,
            };
            pub const STRENGTH: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 14,
                index: 'S',
                symbol: '💪',
                value: 15,
            };
            pub const HERMIT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 13,
                index: 'H',
                symbol: '🕵',
                value: 14,
            };
            pub const WHEEL_OF_FORTUNE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 12,
                index: 'W',
                symbol: '🎡',
                value: 13,
            };
            pub const JUSTICE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 11,
                index: 'J',
                symbol: '⚖',
                value: 12,
            };
            pub const HANGED_MAN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 10,
                index: 'H',
                symbol: '🙃',
                value: 11,
            };

            pub const DEATH: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 9,
                index: 'D',
                symbol: '💀',
                value: 10,
            };
            pub const TEMPERANCE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 8,
                index: 'A',
                symbol: '🚭',
                value: 9,
            };
            pub const DEVIL: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 7,
                index: 'S',
                symbol: '😈',
                value: 8,
            };
            pub const TOWER: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 6,
                index: 'O',
                symbol: '🗼',
                value: 7,
            };
            pub const STAR: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 5,
                index: 'S',
                symbol: '⭐',
                value: 6,
            };
            pub const MOON: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 4,
                index: 'M',
                symbol: '🌜',
                value: 5,
            };
            pub const SUN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 3,
                index: 'S',
                symbol: '☀',
                value: 4,
            };
            pub const JUDGEMENT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 2,
                index: 'J',
                symbol: '🔔',
                value: 3,
            };
            pub const WORLD: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 0,
                index: 'W',
                symbol: '🌍',
                value: 1,
            };

            // Minor Arcana
            pub const KING: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 13,
                index: 'K',
                symbol: 'K',
                value: 14,
            };
            pub const QUEEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 12,
                index: 'Q',
                symbol: 'Q',
                value: 13,
            };
            pub const KNIGHT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 11,
                index: 'N',
                symbol: '🃏',
                value: 12,
            };
            pub const PAGE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 10,
                index: 'P',
                symbol: 'P',
                value: 11,
            };
            pub const TEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 9,
                index: 'T',
                symbol: 'T',
                value: 10,
            };
            pub const NINE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 8,
                index: '9',
                symbol: '9',
                value: 9,
            };
            pub const EIGHT: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 7,
                index: '8',
                symbol: '8',
                value: 8,
            };
            pub const SEVEN: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 6,
                index: '7',
                symbol: '7',
                value: 7,
            };
            pub const SIX: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 5,
                index: '6',
                symbol: '6',
                value: 6,
            };
            pub const FIVE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 4,
                index: '5',
                symbol: '5',
                value: 5,
            };
            pub const FOUR: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 3,
                index: '4',
                symbol: '4',
                value: 4,
            };
            pub const THREE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 2,
                index: '3',
                symbol: '3',
                value: 3,
            };
            pub const TWO: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 1,
                index: '2',
                symbol: '2',
                value: 2,
            };
            pub const ACE: Pip = Pip {
                pip_type: PipType::Rank,
                weight: 0,
                index: 'A',
                symbol: 'A',
                value: 1,
            };
        }

        // endregion Tarot
    }
    pub mod tiny {
        use crate::pack::decks::standard52::Standard52;
        use crate::prelude::{
            BasicCard, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard, Pip,
        };
        use colored::Color;
        use std::collections::HashMap;

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

        // Let's you call Decked methods directly on the Tiny type:
        impl Decked<Tiny> for Tiny {}
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__pips_tests {
    use super::*;
    use crate::basic::types::pips::{Pip, PipType};

    #[test]
    fn pip__default() {
        let pip = Pip::default();
        assert_eq!(pip.pip_type, PipType::Blank);
        assert_eq!(pip.weight, 0);
        assert_eq!(pip.index, Pip::BLANK_INDEX);
        assert_eq!(pip.symbol, Pip::BLANK_INDEX);
        assert_eq!(pip.value, 0);
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__card__basic_card_tests {
    use super::*;
    use crate::prelude::{BasicCard, Decked, French, Pile};
    use crate::prelude::{Card, Standard52};
    use crate::traits::{CKCRevised, DeckedBase};
    use ckc_rs::CardNumber;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn cards_from_yaml_file() {
        let cards = BasicCard::cards_from_yaml_file("src/yaml/french.yaml").unwrap();

        assert_eq!(cards.len(), 54);
        assert_eq!(cards, Pile::<French>::base_vec())
    }

    #[test]
    fn is_blank() {
        let base_card = BasicCard::default();
        assert!(base_card.is_blank());
    }

    #[rstest]
    #[case("A♠", CardNumber::ACE_SPADES)]
    #[case("ks", CardNumber::KING_SPADES)]
    #[case("QS", CardNumber::QUEEN_SPADES)]
    #[case("J♠", CardNumber::JACK_SPADES)]
    #[case("TS", CardNumber::TEN_SPADES)]
    #[case("9s", CardNumber::NINE_SPADES)]
    #[case("8♠", CardNumber::EIGHT_SPADES)]
    #[case("7S", CardNumber::SEVEN_SPADES)]
    #[case("6♠", CardNumber::SIX_SPADES)]
    #[case("5S", CardNumber::FIVE_SPADES)]
    #[case("4♠", CardNumber::FOUR_SPADES)]
    #[case("3s", CardNumber::TREY_SPADES)]
    #[case("2S", CardNumber::DEUCE_SPADES)]
    #[case("A♥", CardNumber::ACE_HEARTS)]
    #[case("k♥", CardNumber::KING_HEARTS)]
    #[case("QH", CardNumber::QUEEN_HEARTS)]
    #[case("jh", CardNumber::JACK_HEARTS)]
    #[case("T♥", CardNumber::TEN_HEARTS)]
    #[case("9♥", CardNumber::NINE_HEARTS)]
    #[case("8h", CardNumber::EIGHT_HEARTS)]
    #[case("7H", CardNumber::SEVEN_HEARTS)]
    #[case("6h", CardNumber::SIX_HEARTS)]
    #[case("5H", CardNumber::FIVE_HEARTS)]
    #[case("4♥", CardNumber::FOUR_HEARTS)]
    #[case("3♥", CardNumber::TREY_HEARTS)]
    #[case("2h", CardNumber::DEUCE_HEARTS)]
    #[case("A♦", CardNumber::ACE_DIAMONDS)]
    #[case("k♦", CardNumber::KING_DIAMONDS)]
    #[case("Q♦", CardNumber::QUEEN_DIAMONDS)]
    #[case("Jd", CardNumber::JACK_DIAMONDS)]
    #[case("tD", CardNumber::TEN_DIAMONDS)]
    #[case("9♦", CardNumber::NINE_DIAMONDS)]
    #[case("8D", CardNumber::EIGHT_DIAMONDS)]
    #[case("7♦", CardNumber::SEVEN_DIAMONDS)]
    #[case("6D", CardNumber::SIX_DIAMONDS)]
    #[case("5D", CardNumber::FIVE_DIAMONDS)]
    #[case("4♦", CardNumber::FOUR_DIAMONDS)]
    #[case("3♦", CardNumber::TREY_DIAMONDS)]
    #[case("2d", CardNumber::DEUCE_DIAMONDS)]
    #[case("a♣", CardNumber::ACE_CLUBS)]
    #[case("k♣", CardNumber::KING_CLUBS)]
    #[case("QC", CardNumber::QUEEN_CLUBS)]
    #[case("jc", CardNumber::JACK_CLUBS)]
    #[case("tC", CardNumber::TEN_CLUBS)]
    #[case("9♣", CardNumber::NINE_CLUBS)]
    #[case("8♣", CardNumber::EIGHT_CLUBS)]
    #[case("7c", CardNumber::SEVEN_CLUBS)]
    #[case("6♣", CardNumber::SIX_CLUBS)]
    #[case("5C", CardNumber::FIVE_CLUBS)]
    #[case("4c", CardNumber::FOUR_CLUBS)]
    #[case("3C", CardNumber::TREY_CLUBS)]
    #[case("2C", CardNumber::DEUCE_CLUBS)]
    #[case("__", 0u32)]
    fn card__get_ckc_number(#[case] input: &str, #[case] expected_ckc: u32) {
        let card = Card::<Standard52>::from_str(input).unwrap();

        let base_card: BasicCard = card.into();

        assert_eq!(base_card.get_ckc_number(), expected_ckc);
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__card__pile_tests {
    use super::*;
    use crate::basic::types::card::BasicPile;
    use crate::prelude::{
        Decked, French, FrenchRank, FrenchSuit, Pile, PipType, Ranged, Standard52, Tarot,
    };
    use crate::traits::DeckedBase;
    use std::str::FromStr;

    fn from_str(s: &str) -> BasicPile {
        BasicPile::from(&Pile::<Standard52>::from_str(s).unwrap())
    }

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
        let mut pile = from_str("2♠ 8♣ 4♠");
        let mut pile2 = pile.clone();

        pile.sort();
        pile2.sort_by_rank();

        assert_eq!(pile.to_string(), "4♠ 2♠ 8♣");
        assert_eq!(pile2.to_string(), "8♣ 4♠ 2♠");
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

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__gto__combos_tests {
    use super::*;
    use crate::basic::types::card::BasicPile;
    use crate::basic::types::gto::Combos;
    use crate::prelude::{Decked, DeckedBase, French, FrenchRank, Pile, Standard52};
    use crate::traits::Ranged;

    #[test]
    fn connectors() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).connectors();

        assert_eq!(combos.len(), 192);
        assert_eq!(
            combos.to_string(),
            "A♠ K♠, A♠ K♥, A♠ K♦, A♠ K♣, K♠ Q♠, K♠ Q♥, K♠ Q♦, K♠ Q♣, Q♠ J♠, Q♠ J♥, Q♠ J♦, Q♠ J♣, J♠ T♠, J♠ T♥, J♠ T♦, J♠ T♣, T♠ 9♠, T♠ 9♥, T♠ 9♦, T♠ 9♣, 9♠ 8♠, 9♠ 8♥, 9♠ 8♦, 9♠ 8♣, 8♠ 7♠, 8♠ 7♥, 8♠ 7♦, 8♠ 7♣, 7♠ 6♠, 7♠ 6♥, 7♠ 6♦, 7♠ 6♣, 6♠ 5♠, 6♠ 5♥, 6♠ 5♦, 6♠ 5♣, 5♠ 4♠, 5♠ 4♥, 5♠ 4♦, 5♠ 4♣, 4♠ 3♠, 4♠ 3♥, 4♠ 3♦, 4♠ 3♣, 3♠ 2♠, 3♠ 2♥, 3♠ 2♦, 3♠ 2♣, A♥ K♠, A♥ K♥, A♥ K♦, A♥ K♣, K♥ Q♠, K♥ Q♥, K♥ Q♦, K♥ Q♣, Q♥ J♠, Q♥ J♥, Q♥ J♦, Q♥ J♣, J♥ T♠, J♥ T♥, J♥ T♦, J♥ T♣, T♥ 9♠, T♥ 9♥, T♥ 9♦, T♥ 9♣, 9♥ 8♠, 9♥ 8♥, 9♥ 8♦, 9♥ 8♣, 8♥ 7♠, 8♥ 7♥, 8♥ 7♦, 8♥ 7♣, 7♥ 6♠, 7♥ 6♥, 7♥ 6♦, 7♥ 6♣, 6♥ 5♠, 6♥ 5♥, 6♥ 5♦, 6♥ 5♣, 5♥ 4♠, 5♥ 4♥, 5♥ 4♦, 5♥ 4♣, 4♥ 3♠, 4♥ 3♥, 4♥ 3♦, 4♥ 3♣, 3♥ 2♠, 3♥ 2♥, 3♥ 2♦, 3♥ 2♣, A♦ K♠, A♦ K♥, A♦ K♦, A♦ K♣, K♦ Q♠, K♦ Q♥, K♦ Q♦, K♦ Q♣, Q♦ J♠, Q♦ J♥, Q♦ J♦, Q♦ J♣, J♦ T♠, J♦ T♥, J♦ T♦, J♦ T♣, T♦ 9♠, T♦ 9♥, T♦ 9♦, T♦ 9♣, 9♦ 8♠, 9♦ 8♥, 9♦ 8♦, 9♦ 8♣, 8♦ 7♠, 8♦ 7♥, 8♦ 7♦, 8♦ 7♣, 7♦ 6♠, 7♦ 6♥, 7♦ 6♦, 7♦ 6♣, 6♦ 5♠, 6♦ 5♥, 6♦ 5♦, 6♦ 5♣, 5♦ 4♠, 5♦ 4♥, 5♦ 4♦, 5♦ 4♣, 4♦ 3♠, 4♦ 3♥, 4♦ 3♦, 4♦ 3♣, 3♦ 2♠, 3♦ 2♥, 3♦ 2♦, 3♦ 2♣, A♣ K♠, A♣ K♥, A♣ K♦, A♣ K♣, K♣ Q♠, K♣ Q♥, K♣ Q♦, K♣ Q♣, Q♣ J♠, Q♣ J♥, Q♣ J♦, Q♣ J♣, J♣ T♠, J♣ T♥, J♣ T♦, J♣ T♣, T♣ 9♠, T♣ 9♥, T♣ 9♦, T♣ 9♣, 9♣ 8♠, 9♣ 8♥, 9♣ 8♦, 9♣ 8♣, 8♣ 7♠, 8♣ 7♥, 8♣ 7♦, 8♣ 7♣, 7♣ 6♠, 7♣ 6♥, 7♣ 6♦, 7♣ 6♣, 6♣ 5♠, 6♣ 5♥, 6♣ 5♦, 6♣ 5♣, 5♣ 4♠, 5♣ 4♥, 5♣ 4♦, 5♣ 4♣, 4♣ 3♠, 4♣ 3♥, 4♣ 3♦, 4♣ 3♣, 3♣ 2♠, 3♣ 2♥, 3♣ 2♦, 3♣ 2♣"
        );
    }

    #[test]
    fn of_rank() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_rank(FrenchRank::ACE);

        assert_eq!(combos.len(), 6);
        assert_eq!(
            combos.to_string(),
            "A♠ A♥, A♠ A♦, A♠ A♣, A♥ A♦, A♥ A♣, A♦ A♣"
        );
    }

    #[test]
    fn of_same_rank() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_same_rank();

        assert_eq!(combos.len(), 78);
    }

    #[test]
    fn of_same_rank_or_above() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_same_rank_or_above(FrenchRank::KING);

        assert_eq!(
            combos.to_string(),
            "A♠ A♥, A♠ A♦, A♠ A♣, K♠ K♥, K♠ K♦, K♠ K♣, A♥ A♦, A♥ A♣, K♥ K♦, K♥ K♣, A♦ A♣, K♦ K♣"
        );
    }

    #[test]
    fn suited() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        let suited = combos.suited();
        let connectors = suited.connectors();

        assert_eq!(suited.len(), 312);
        assert_eq!(connectors.len(), 48);
    }

    // TODO: Why are these in the reverse order?
    #[test]
    fn unsuited() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).unsuited();
        let mut connectors = combos.connectors();
        connectors.sort();

        assert_eq!(combos.len(), 1014);
        assert_eq!(connectors.len(), 144);
    }

    #[test]
    fn from_vec() {
        let from = vec![
            Pile::<French>::basic_pile(),
            Pile::<French>::basic_pile(),
            Pile::<French>::basic_pile(),
        ];

        let pile = Pile::<French>::basic_pile();

        let piles = Combos::from(from);

        assert_eq!(3, piles.len());
        for p in piles {
            assert_eq!(pile, p);
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards__french__tests {
    use crate::basic::types::card::BasicCard;
    use crate::basic::types::pips::Pip;
    use crate::prelude::French;
    use crate::prelude::FrenchRank;
    use crate::traits::Decked;

    #[test]
    fn serde() {
        let pips = vec![FrenchRank::ACE];
        let yml = serde_yml::to_string(&pips).unwrap();

        let pip2: Vec<Pip> = serde_yml::from_str(&yml).unwrap();
        assert_eq!(pips, pip2);
    }

    #[test]
    fn serde__deck() {
        let pile = French::deck().into_basic_cards();
        let yml = serde_yml::to_string(&pile).unwrap();

        let from_yml: Vec<BasicCard> = BasicCard::cards_from_yaml_str(&yml).unwrap();

        assert_eq!(pile, from_yml);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards__pinochle__tests {
    use crate::basic::types::pips::Pip;
    use crate::prelude::PinochleRank;

    #[test]
    fn serde() {
        let pips = vec![PinochleRank::KING];
        let yml = serde_yml::to_string(&pips).unwrap();

        // println!("{yml}");

        let pip2: Vec<Pip> = serde_yml::from_str(&yml).unwrap();
        assert_eq!(pips, pip2);
    }
}
