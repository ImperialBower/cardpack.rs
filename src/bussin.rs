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

        /// `Pip` is the smallest unit of a [`BasicCard`](crate::basic::types::basic_card::BasicCard).
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
        /// Each [`BasicCard`](crate::basic::types::basic_card::BasicCard) struct is made up of two `Pips`, one
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
            /// The universal index for a blank `Pip` in a [`Card`](crate::basic::types::card::Card). Blank
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
            pub fn new(
                pip_type: PipType,
                weight: u32,
                index: char,
                symbol: char,
            ) -> Self {
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
        use std::cmp::Ordering;
        use std::error::Error;
        use std::fmt;
        use std::fmt::Display;
        use std::fs::File;
        use std::hash::Hash;
        use std::io::Read;
        use rand::prelude::SliceRandom;
        use rand::rng;
        use serde::{Deserialize, Serialize};
        use crate::bussin::types::pips::{Pip, PipType};
        use crate::common::utils::Bit;
        use crate::prelude::{CKCRevised, Card, DeckedBase, Pile, Ranged};

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
            /// [`Razz`](crate::basic::decks::razz::Razz) deck for an example of how to use this method.
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

            /// Returns n number of [`BasicCards`](crate::basic::types::basic_card::BasicCard) from the
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
            fn from_iter<T: IntoIterator<Item =BasicCard>>(iter: T) -> Self {
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
        use std::fmt::{Display, Formatter};
        use crate::bussin::types::card::{BasicCard, BasicPile};
        use crate::bussin::types::pips::Pip;

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
                    .collect::<crate::basic::types::combos::Combos>()
                    .iter()
                    .map(|pile| pile.clone().sorted_by_rank())
                    .collect::<crate::basic::types::combos::Combos>()
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
