pub mod types {
    pub mod card {
        use crate::basic::types::card::{BasicCard, BasicPile};
        use crate::basic::types::pips::{Pip, PipType};
        use crate::localization::{FluentName, Named};
        use crate::prelude::CardError;
        use crate::traits::Decked;
        use crate::traits::DeckedBase;
        use crate::traits::Ranged;
        use colored::{Color, Colorize};
        use fluent_templates::LanguageIdentifier;
        use rand::prelude::SliceRandom;
        use rand::{Rng, rng};
        use serde::{Deserialize, Serialize};
        use std::collections::{HashMap, HashSet};
        use std::fmt::{Display, Formatter};
        use std::hash::Hash;
        use std::marker::PhantomData;
        use std::str::FromStr;
        use std::vec::IntoIter;
        // region Card

        /// A `Card` is a struct that's a generic wrapper around a [`BasicCard`] providing it with additional
        /// deck specific superpowers, many of which are at the pile level. The ones at the `Card` level
        /// are:
        ///
        /// - `color()` - returns the color of the card based on what's configured at the type parameter's implementation of the `DeckedBase` trait.
        /// - `fluent_name()` - returns the long name of the card from the `Named` trait's use of fluent templates.
        /// - `from_str()` - allows you to create a `Card` for the specific deck with a string representation of the card.
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            PartialEq,
            Ord,
            PartialOrd,
            Serialize,
            Deserialize,
        )]
        pub struct Card<DeckType>
        where
            DeckType: DeckedBase,
        {
            pub base_card: BasicCard,
            pub deck: PhantomData<DeckType>,
        }

        impl<DeckType: DeckedBase> Card<DeckType> {
            #[must_use]
            pub fn new(base_card: BasicCard) -> Self {
                Self::from(base_card)
            }

            /// Returns the underlying [`BasicCard`] struct.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::TREY_DIAMONDS);
            ///
            /// assert_eq!(
            ///     Card::<French>::new(FrenchBasicCard::TREY_DIAMONDS).base(),
            ///     FrenchBasicCard::TREY_DIAMONDS
            /// );
            /// assert_eq!(
            ///     Card::<Pinochle>::new(PinochleBasicCard::TEN_SPADES).base(),
            ///     PinochleBasicCard::TEN_SPADES
            /// );
            /// ```
            #[must_use]
            pub fn base(&self) -> BasicCard {
                self.base_card
            }

            /// Returns the color designated for a Card's specific suit in the deck's configuration.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::TEN_DIAMONDS);
            ///
            /// assert_eq!(card.color(), Color::Red);
            /// ```
            ///
            /// This feels heavy and hackie. It's not important enough to worry about.
            #[must_use]
            pub fn color(&self) -> Color {
                let binding = DeckType::colors();
                let color = binding.get(&self.base_card.suit);

                match color {
                    Some(color) => *color,
                    None => Color::White,
                }
            }

            /// Returns the color designated for a Card's specific suit in the deck's configuration.
            #[must_use]
            pub fn color_index_string(&self) -> String {
                self.color_string(self.base_card.index())
            }

            /// TODO RF: create a `color_index_string()` version with a common implementation.
            ///
            /// DONE!!!
            #[must_use]
            pub fn color_symbol_string(&self) -> String {
                self.color_string(self.base_card.to_string())
            }

            /// Returns a color formatted version of the String based on the settings in the deck's configuration.
            fn color_string(&self, s: String) -> String {
                match self.color() {
                    Color::Red => s.red().to_string(),
                    Color::Blue => s.blue().to_string(),
                    Color::Green => s.green().to_string(),
                    Color::Yellow => s.yellow().to_string(),
                    Color::Magenta => s.magenta().to_string(),
                    Color::Cyan => s.cyan().to_string(),
                    Color::BrightBlack => s.bright_black().to_string(),
                    Color::BrightRed => s.bright_red().to_string(),
                    Color::BrightGreen => s.bright_green().to_string(),
                    Color::BrightYellow => s.bright_yellow().to_string(),
                    Color::BrightBlue => s.bright_blue().to_string(),
                    Color::BrightMagenta => s.bright_magenta().to_string(),
                    Color::BrightCyan => s.bright_cyan().to_string(),
                    _ => s,
                }
            }

            /// Returns the basic, text representation of a `Card`.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<Skat>::new(SkatBasicCard::ZHEN_EICHEL);
            ///
            /// assert_eq!(card.index(), "ZE");
            /// ```
            #[must_use]
            pub fn index(&self) -> String {
                self.base_card.index()
            }

            /// Returns true if the `Card` is blank.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// assert!(Card::<French>::default().is_blank());
            /// assert!(!Card::<French>::new(FrenchBasicCard::ACE_SPADES).is_blank());
            /// ```
            #[must_use]
            pub fn is_blank(&self) -> bool {
                self.base_card.is_blank()
            }

            /// `CoPilot` was completely useless for this. It was surprisingly easy to figure it out
            /// for myself one I got used to the patterns with this hint from the compiler:
            ///
            /// ```txt
            /// error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
            ///    --> src/basic/types/card.rs:151:9
            ///     |
            /// 151 |           DeckedBase::basic_pile().contains(&self.base_card)
            ///     |           ^^^^^^^^^^^^^^^^^^^^^^^^ cannot call associated function of trait
            ///     |
            ///    ::: src/basic/types/traits.rs:17:5
            ///     |
            /// 17  | /     fn basic_pile() -> BasicPile {
            /// 18  | |         BasicPile::from(Self::base_vec())
            /// 19  | |     }
            ///     | |_____- `DeckedBase::basic_pile` defined here
            ///     |
            /// help: use a fully-qualified path to one of the available implementations
            ///     |
            /// 151 |         <Canasta as DeckedBase>::basic_pile().contains(&self.base_card)
            ///     |         +++++++++++           +
            /// 151 |         <Euchre24 as DeckedBase>::basic_pile().contains(&self.base_card)
            ///     |         ++++++++++++           +
            /// 151 |         <Euchre32 as DeckedBase>::basic_pile().contains(&self.base_card)
            ///     |         ++++++++++++           +
            /// 151 |         <French as DeckedBase>::basic_pile().contains(&self.base_card)
            ///     |         ++++++++++           +
            ///       and 10 other candidates
            /// ```
            ///
            /// This is one of the many reasons why I love `Rust`. Even when it doesn't spell it out for you,
            /// it does make your life a lot easier.
            #[must_use]
            pub fn is_valid(&self) -> bool {
                <DeckType as DeckedBase>::basic_pile().contains(&self.base_card)
            }

            /// Returns the default, aka `US_ENGLISH`, version of the  long name of the whole `Card`
            /// from the `Named` trait's use of fluent templates in the rank and suit [`Pip`]s for the `Card`.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::NINE_CLUBS);
            ///
            /// assert_eq!("Nine of Clubs", card.fluent_name_default());
            /// ```
            #[must_use]
            pub fn fluent_name_default(&self) -> String {
                self.fluent_name(&FluentName::US_ENGLISH)
            }

            /// Returns the long name of the whole `Card` from the `Named` trait's use of fluent templates
            /// in the rank and suit [`Pip`]s for the `Card`.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::NINE_CLUBS);
            ///
            /// assert_eq!("Nine of Clubs", card.fluent_name(&FluentName::US_ENGLISH));
            /// assert_eq!("Neun Klee", card.fluent_name(&FluentName::DEUTSCH));
            /// ```
            /// TODO: HACK
            #[must_use]
            pub fn fluent_name(&self, lid: &LanguageIdentifier) -> String {
                match self.base_card.suit.pip_type {
                    PipType::Special => self.fluent_rank_name(lid).to_string(),
                    PipType::Joker => {
                        format!("Joker {}", self.fluent_rank_name(lid))
                    }
                    _ => {
                        format!(
                            "{}{}{}",
                            self.fluent_rank_name(lid),
                            Self::fluent_connector(lid),
                            self.fluent_suit_name(lid)
                        )
                    }
                }
            }

            /// Returns the connector string for the rank and suit [`Pip`]s in the `Card`'s name.
            ///
            /// TODO RF: Need a more configurable way to do this.
            fn fluent_connector(lid: &LanguageIdentifier) -> String {
                match lid {
                    &FluentName::DEUTSCH => " ".to_string(),
                    _ => " of ".to_string(),
                }
            }

            /// Returns the long name of the rank [`Pip`] for the `Card` set in the localization files.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::DEUCE_DIAMONDS);
            ///
            /// assert_eq!("Deuce", card.fluent_rank_name(&FluentName::US_ENGLISH));
            /// assert_eq!("Zwei", card.fluent_rank_name(&FluentName::DEUTSCH));
            /// ```
            ///
            /// TODO: HACK I am feeling like I have begun to outlive my need
            /// for fluent templates. The deck from yaml idea feels like the path.
            #[must_use]
            pub fn fluent_rank_name(&self, lid: &LanguageIdentifier) -> String {
                let s: String = match self.base_card.suit.pip_type {
                    PipType::Special => {
                        format!(
                            "{}-special-{}",
                            DeckType::fluent_name_base(),
                            self.base_card.rank.index.to_lowercase()
                        )
                    }
                    _ => {
                        format!(
                            "{}-{}",
                            DeckType::fluent_name_base(),
                            self.base_card.rank.index.to_lowercase()
                        )
                    }
                };

                FluentName::new("name-rank").fluent_value(s.as_str(), lid)
            }

            /// Returns the long name of the suit [`Pip`] for the `Card` set in the localization files.
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::DEUCE_DIAMONDS);
            ///
            /// assert_eq!("Diamonds", card.fluent_suit_name(&FluentName::US_ENGLISH));
            /// assert_eq!("Diamanten", card.fluent_suit_name(&FluentName::DEUTSCH));
            /// ```
            ///
            #[must_use]
            pub fn fluent_suit_name(&self, lid: &LanguageIdentifier) -> String {
                let s = format!(
                    "{}-{}",
                    DeckType::fluent_name_base(),
                    self.base_card.suit.index.to_lowercase()
                );
                FluentName::new("name-suit").fluent_value(s.as_str(), lid)
            }
        }

        impl<DeckType: DeckedBase> DeckedBase for Card<DeckType> {
            /// Pass through call to the `Card's` underlying type parameter.
            fn base_vec() -> Vec<BasicCard> {
                DeckType::base_vec()
            }

            /// Pass through call to the `Card's` underlying type parameter.
            fn colors() -> HashMap<Pip, Color> {
                DeckType::colors()
            }

            /// Pass through call to the `Card's` underlying type parameter.
            fn deck_name() -> String {
                DeckType::deck_name()
            }

            /// Pass through call to the `Card's` underlying type parameter.
            fn fluent_deck_key() -> String {
                DeckType::fluent_deck_key()
            }
        }

        impl<DeckType: Default + Copy + Ord + DeckedBase> Display for crate::prelude::Card<DeckType> {
            /// Passes through the `Display` result from the underlying [`BasicCard`].
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
            ///
            /// assert_eq!(card.to_string(), card.base_card.to_string());
            /// ```
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "{}", self.base_card)
            }
        }

        impl<DeckType: DeckedBase> From<BasicCard> for Card<DeckType> {
            fn from(pips: BasicCard) -> Self {
                Self {
                    base_card: pips,
                    deck: PhantomData,
                }
            }
        }

        impl<DeckType: DeckedBase> From<&BasicCard> for crate::prelude::Card<DeckType> {
            fn from(pips: &BasicCard) -> Self {
                crate::prelude::Card::<DeckType>::from(*pips)
            }
        }

        impl<DeckType: DeckedBase> FromStr for crate::prelude::Card<DeckType> {
            type Err = CardError;

            /// Cards can be created from strings in any combination of index and symbol strings in upper
            /// and lowercase letters.
            ///
            /// # Indexes
            ///
            /// A `Card's` index is make of the unique char (`Pip.index`) or symbol (`Pip.symbol`) for the
            /// suit `Pip` and a unique char for the rank `Pip`. The implementation of the trait is
            /// designed to be very forgiving. For example:
            ///
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
            ///
            /// let possible = vec!["AS", "as", "aS", "As", "A♠", "a♠"];
            ///
            /// for s in possible {
            ///    assert_eq!(card, Card::<French>::from_str(s).unwrap());
            /// }
            /// ```
            ///
            /// We've changed the contract for index strings in one way: we are adding support for blank
            /// cards, aka `__`. This is so you can represent a collection that includes a blank spot,
            /// such as `J♥ T♥ __`
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.trim().to_uppercase();

                let mut cards = crate::prelude::Card::<DeckType>::base_vec();
                // Add a blank card to the list of cards so that it is considered valid.
                cards.push(BasicCard::default());

                cards
                    .iter()
                    .find(|c| (c.to_string() == s) || (c.index() == s))
                    .copied()
                    .map(crate::prelude::Card::<DeckType>::from)
                    .ok_or(CardError::InvalidCard(s.to_string()))
            }
        }
        // endregion

        // region Pile

        /// A `Pile` is a [generic data type](https://doc.rust-lang.org/book/ch10-01-syntax.html)
        /// collection of [`Cards`](Card) that are bound by a
        /// specific deck  type parameter.
        ///
        /// The magic behind all this is enabled by implementing the [`Decked`] and [`DeckedBase`] traits.
        /// [`DeckedBase`] defines the [`BasicCards`](BasicCard) that
        /// hold the data that is in the [`Cards`](Card) of the `Pile`, and the
        /// [`Decked`] trait that ensures that only [`Cards`](Card) that fit
        /// the contract defined in the specific deck implementation trait, such as
        /// [`French`](crate::pack::decks::french::French) for a traditional pack of cards with jokers, or
        /// [`Pinochle`](crate::pack::decks::pinochle::Pinochle). This makes it possible for the users
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
        /// TODO: fixme
        /// ```txt
        /// use cardpack::prelude::*;
        /// let mut deck: Pile<Standard52> = Standard52::deck();
        ///
        /// assert_eq!(deck(" "), "A K Q J T 9 8 7 6 5 4 3 2");
        /// assert_eq!(deck.suit_symbol_index(), "♠ ♥ ♦ ♣");
        /// assert_eq!(deck.suit_index(), "S H D C");
        /// assert_eq!(deck.draw(5).to_string(), "A♠ K♠ Q♠ J♠ T♠");
        /// assert_eq!(deck.len(), 47);
        /// ```
        ///
        /// ```txt
        /// use cardpack::rev1_prelude::{Decked, Modern, Pile};
        /// let modern_deck: Pile<Modern, Modern> = Modern::deck();
        ///
        /// assert_eq!(modern_deck.rank_index(""), "BLAKQJT98765432");
        /// assert_eq!(modern_deck.suit_symbol_index(), "🃟 ♠ ♥ ♦ ♣");
        /// assert_eq!(modern_deck.suit_index(), "J S H D C");
        /// ```
        #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

            /// Returns a reference to the underlying [`crate::prelude::Card`] vector for the Pile.
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

            /// Returns true if the passed in [`crate::prelude::Card`] is in the `Pile`.
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

            /// Prints out a demonstration of the deck. Used in the `cli` example program.
            pub fn demo_cards(&self, verbose: bool) {
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
                    println!();
                    println!("Long in English and German:");

                    for card in deck {
                        let name = card.fluent_name_default();
                        println!("  {name} ");
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

                let mut cards = Pile::<DeckType>::default();
                for _ in 0..n {
                    cards.push(self.draw_first()?);
                }
                Some(cards)
            }

            /// Draws the first [`crate::prelude::Card`]  if there. Returns `None` if the `Pile` is empty.
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
                Pile::<DeckType>::from_str(index).unwrap_or_else(|_| {
                    log::warn!(
                        "Pile::forgiving_from_str(): {index} is invalid. Returning empty Pile."
                    );
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

            /// Returns the `Pile` as a `HashSet`, an unordered collection of each unique [`Card`].
            ///
            /// ```
            /// use std::collections::HashSet;
            /// use cardpack::prelude::*;
            ///
            /// let pile = Pile::<Standard52>::from_str("2♠ 8♠ 4♠").unwrap();
            /// let mut hs: HashSet<Card<Standard52>> = HashSet::new();
            ///
            /// hs.insert(card!(2S));
            /// hs.insert(card!(8S));
            /// hs.insert(card!(4S));
            ///
            /// assert_eq!(pile.into_hashset(), hs);
            /// ```
            #[must_use]
            pub fn into_hashset(&self) -> HashSet<Card<DeckType>> {
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
            pub fn iter(&self) -> std::slice::Iter<Card<DeckType>> {
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
            /// use cardpack::basic::cards::tiny::Tiny;
            /// use cardpack::prelude::*;
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
            pub fn map_by_suit(&self) -> HashMap<Pip, Pile<DeckType>> {
                let mut map: HashMap<Pip, Pile<DeckType>> = HashMap::new();

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
            /// TODO: delete me
            #[must_use]
            pub fn piles_to_string(piles: &[Self]) -> String {
                piles
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            }

            /// Returns the position of the passed in [`Card`] in the `Pile`. If the [`Card`] isn't there,
            /// it returns `None`.
            ///
            /// ```
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
            pub fn prepend(&mut self, other: &Pile<DeckType>) {
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
            /// ```
            /// use cardpack::prelude::*;
            ///
            /// let pile1 = Razz::deck();
            /// let pile2 = Razz::deck().shuffled();
            ///
            /// assert_ne!(pile1, pile2);
            /// assert!(pile1.same(&pile2));
            /// ```
            #[must_use]
            pub fn same(&self, cards: &Pile<DeckType>) -> bool {
                let left = self.sorted();
                let right = cards.sorted();

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
                self.0
                    .sort_by(|a, b| b.base_card.rank.cmp(&a.base_card.rank));
            }

            /// Returns a String of the `Pile` with the passed in function applied to each [`Card`].
            ///
            /// ```
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
            for Pile<DeckType>
        {
            fn from(cards: HashSet<Card<DeckType>>) -> Self {
                Self(cards.into_iter().collect()).sorted()
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
                let cards = Pile::<DeckType>::into_cards(&cards);
                Self(cards)
            }
        }

        impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> From<BasicPile> for Pile<DeckType> {
            fn from(pile: BasicPile) -> Self {
                let cards = Pile::<DeckType>::into_cards(pile.v());
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

                Ok(Pile::<DeckType>::from(
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

        // endregion
    }
}

pub mod decks {
    pub mod canasta {
        use crate::prelude::{
            BasicCard, CanastaBasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_CANASTA,
            French, FrenchBasicCard, Pile, Pip,
        };
        use colored::Color;
        use std::collections::HashMap;

        /// [Canasta](https://en.wikipedia.org/wiki/Canasta) deck
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Canasta {}
        #[allow(clippy::module_name_repetitions)]
        pub type CanastaDeck = Pile<Canasta>;
        #[allow(clippy::module_name_repetitions)]
        pub type CanastaCard = Card<Canasta>;

        impl Canasta {
            pub const DECK_SIZE: usize = 108;

            pub const DECK: [BasicCard; Canasta::DECK_SIZE] = [
                CanastaBasicCard::TREY_HEARTS,
                CanastaBasicCard::TREY_HEARTS,
                CanastaBasicCard::TREY_DIAMONDS,
                CanastaBasicCard::TREY_DIAMONDS,
                CanastaBasicCard::BIG_JOKER,
                CanastaBasicCard::BIG_JOKER,
                CanastaBasicCard::LITTLE_JOKER,
                CanastaBasicCard::LITTLE_JOKER,
                CanastaBasicCard::DEUCE_SPADES,
                CanastaBasicCard::DEUCE_SPADES,
                CanastaBasicCard::DEUCE_HEARTS,
                CanastaBasicCard::DEUCE_HEARTS,
                CanastaBasicCard::DEUCE_DIAMONDS,
                CanastaBasicCard::DEUCE_DIAMONDS,
                CanastaBasicCard::DEUCE_CLUBS,
                CanastaBasicCard::DEUCE_CLUBS,
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::FIVE_SPADES,
                FrenchBasicCard::FIVE_SPADES,
                FrenchBasicCard::FOUR_SPADES,
                FrenchBasicCard::FOUR_SPADES,
                FrenchBasicCard::TREY_SPADES,
                FrenchBasicCard::TREY_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::FIVE_HEARTS,
                FrenchBasicCard::FIVE_HEARTS,
                FrenchBasicCard::FOUR_HEARTS,
                FrenchBasicCard::FOUR_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::FIVE_DIAMONDS,
                FrenchBasicCard::FIVE_DIAMONDS,
                FrenchBasicCard::FOUR_DIAMONDS,
                FrenchBasicCard::FOUR_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
                FrenchBasicCard::FIVE_CLUBS,
                FrenchBasicCard::FIVE_CLUBS,
                FrenchBasicCard::FOUR_CLUBS,
                FrenchBasicCard::FOUR_CLUBS,
                FrenchBasicCard::TREY_CLUBS,
                FrenchBasicCard::TREY_CLUBS,
            ];
        }

        impl DeckedBase for Canasta {
            fn base_vec() -> Vec<BasicCard> {
                Canasta::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                French::colors()
            }

            fn deck_name() -> String {
                "Canasta".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_CANASTA.to_string()
            }
        }

        impl Decked<Canasta> for Canasta {}
    }
    pub mod euchre24 {
        use crate::prelude::{
            BasicCard, Card, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard, Pile, Pip, Standard52,
        };
        use crate::traits::{Decked, DeckedBase};
        use colored::Color;
        use std::collections::HashMap;

        /// This deck represents the most common 24 card form of
        /// [Euchre](https://en.wikipedia.org/wiki/Euchre) with
        /// `A K Q J T 9` ranks.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Euchre24 {}

        #[allow(clippy::module_name_repetitions)]
        pub type Euchre24Deck = Pile<Euchre24>;
        #[allow(clippy::module_name_repetitions)]
        pub type Euchre24Card = Card<Euchre24>;

        impl Euchre24 {
            pub const DECK_SIZE: usize = 24;

            pub const DECK: [BasicCard; Euchre24::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
            ];
        }

        impl DeckedBase for Euchre24 {
            fn base_vec() -> Vec<BasicCard> {
                Euchre24::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Euchre 24".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Euchre24> for Euchre24 {}
    }
    pub mod euchre32 {
        use crate::prelude::{BasicCard, Card, FrenchBasicCard, Pile, Pip};
        use crate::prelude::{FLUENT_KEY_BASE_NAME_FRENCH, Standard52};
        use crate::traits::{Decked, DeckedBase};
        use colored::Color;
        use std::collections::HashMap;

        /// This deck represents the most 32 card form of
        /// [Euchre](https://en.wikipedia.org/wiki/Euchre) with
        /// `A K Q J T 9 8 7` ranks.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Euchre32 {}

        #[allow(clippy::module_name_repetitions)]
        pub type Euchre32Deck = Pile<Euchre32>;
        #[allow(clippy::module_name_repetitions)]
        pub type Euchre32Card = Card<Euchre32>;

        impl Euchre32 {
            pub const DECK_SIZE: usize = 32;

            pub const DECK: [BasicCard; Euchre32::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
            ];
        }

        impl DeckedBase for Euchre32 {
            fn base_vec() -> Vec<BasicCard> {
                Euchre32::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Euchre 32".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Euchre32> for Euchre32 {}
    }
    pub mod french {
        use crate::prelude::{
            BasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard,
            FrenchSuit, Pile, Pip,
        };
        use colored::Color;
        use std::collections::HashMap;

        /// `French` is the type parameter for the `French Deck` version of the generic
        /// [`Card`] and [`Pile`] structs.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct French {}
        #[allow(clippy::module_name_repetitions)]
        pub type FrenchDeck = Pile<French>;
        #[allow(clippy::module_name_repetitions)]
        pub type FrenchCard = Card<French>;

        impl French {
            pub const DECK_SIZE: usize = 54;

            pub const DECK: [BasicCard; French::DECK_SIZE] = [
                FrenchBasicCard::BIG_JOKER,
                FrenchBasicCard::LITTLE_JOKER,
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::FIVE_SPADES,
                FrenchBasicCard::FOUR_SPADES,
                FrenchBasicCard::TREY_SPADES,
                FrenchBasicCard::DEUCE_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::FIVE_HEARTS,
                FrenchBasicCard::FOUR_HEARTS,
                FrenchBasicCard::TREY_HEARTS,
                FrenchBasicCard::DEUCE_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::FIVE_DIAMONDS,
                FrenchBasicCard::FOUR_DIAMONDS,
                FrenchBasicCard::TREY_DIAMONDS,
                FrenchBasicCard::DEUCE_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
                FrenchBasicCard::FIVE_CLUBS,
                FrenchBasicCard::FOUR_CLUBS,
                FrenchBasicCard::TREY_CLUBS,
                FrenchBasicCard::DEUCE_CLUBS,
            ];
        }

        impl DeckedBase for French {
            fn base_vec() -> Vec<BasicCard> {
                French::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                let mut mappie = HashMap::new();

                mappie.insert(FrenchSuit::JOKER, Color::Blue);
                mappie.insert(FrenchSuit::HEARTS, Color::Red);
                mappie.insert(FrenchSuit::DIAMONDS, Color::Red);

                mappie
            }

            fn deck_name() -> String {
                "French".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<French> for French {}
    }
    pub mod pinochle {
        use crate::prelude::{
            BasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_PINOCHLE, FrenchBasicCard,
            Pile, PinochleBasicCard, Pip, Standard52,
        };
        use colored::Color;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Pinochle {}

        #[allow(clippy::module_name_repetitions)]
        pub type PinochleDeck = Pile<Pinochle>;
        #[allow(clippy::module_name_repetitions)]
        pub type PinochleCard = Card<Pinochle>;

        impl Pinochle {
            pub const DECK_SIZE: usize = 48;

            pub const DECK: [BasicCard; Pinochle::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::ACE_SPADES,
                PinochleBasicCard::TEN_SPADES,
                PinochleBasicCard::TEN_SPADES,
                PinochleBasicCard::KING_SPADES,
                PinochleBasicCard::KING_SPADES,
                PinochleBasicCard::QUEEN_SPADES,
                PinochleBasicCard::QUEEN_SPADES,
                PinochleBasicCard::JACK_SPADES,
                PinochleBasicCard::JACK_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::ACE_HEARTS,
                PinochleBasicCard::TEN_HEARTS,
                PinochleBasicCard::TEN_HEARTS,
                PinochleBasicCard::KING_HEARTS,
                PinochleBasicCard::KING_HEARTS,
                PinochleBasicCard::QUEEN_HEARTS,
                PinochleBasicCard::QUEEN_HEARTS,
                PinochleBasicCard::JACK_HEARTS,
                PinochleBasicCard::JACK_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::ACE_DIAMONDS,
                PinochleBasicCard::TEN_DIAMONDS,
                PinochleBasicCard::TEN_DIAMONDS,
                PinochleBasicCard::KING_DIAMONDS,
                PinochleBasicCard::KING_DIAMONDS,
                PinochleBasicCard::QUEEN_DIAMONDS,
                PinochleBasicCard::QUEEN_DIAMONDS,
                PinochleBasicCard::JACK_DIAMONDS,
                PinochleBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::ACE_CLUBS,
                PinochleBasicCard::TEN_CLUBS,
                PinochleBasicCard::TEN_CLUBS,
                PinochleBasicCard::KING_CLUBS,
                PinochleBasicCard::KING_CLUBS,
                PinochleBasicCard::QUEEN_CLUBS,
                PinochleBasicCard::QUEEN_CLUBS,
                PinochleBasicCard::JACK_CLUBS,
                PinochleBasicCard::JACK_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
            ];
        }

        impl DeckedBase for Pinochle {
            fn base_vec() -> Vec<BasicCard> {
                Pinochle::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Pinochle".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_PINOCHLE.to_string()
            }
        }

        impl Decked<Pinochle> for Pinochle {}
    }
    pub mod razz {
        use crate::prelude::{
            BasicCard, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, Pip, Standard52,
        };
        use colored::Color;
        use std::collections::HashMap;

        /// [`Razz`](https://en.wikipedia.org/wiki/Razz_(poker)) deck where the cards are ordered from low
        /// to high with the `Ace` counting as low.
        ///
        /// This is an example of Deck generation using `BasicCard` configuration in `yaml` instead of
        /// programmatically.
        ///
        /// The yaml file was generated with the help of `CoPilot`, which created a version that didn't
        /// actually work. You can see it in [`razz_bad.yaml`](yaml/razz_bad.yaml). This is why we test.
        /// While the front line `Deck::<Razz>::validate()` didn't catch anything, this time,
        /// the basic `from_str()` test did, after we had to debug. This is why it is always dangerous
        /// to bury errors with just returning default. In a production system, I would add at least
        /// logging in order to have some record of what's going on. In fact, let's add that to
        /// `BasicCard::cards_from_file()` now.
        ///
        /// This is an
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Razz {}

        impl DeckedBase for Razz {
            fn base_vec() -> Vec<BasicCard> {
                BasicCard::cards_from_yaml_file("src/yaml/razz.yaml")
                    .unwrap_or_else(|_| Vec::default())
            }

            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Razz".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Razz> for Razz {}
    }
    pub mod short {
        use crate::prelude::{
            BasicCard, Card, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard, Pile, Pip, Standard52,
        };
        use crate::traits::{Decked, DeckedBase};
        use colored::Color;
        use std::collections::HashMap;

        /// [Manila, aka Six Plus aka Short-deck](https://en.wikipedia.org/wiki/Six-plus_hold_%27em)
        /// is a version of Texas Hold'em where the card Ranks of 2 through 5
        /// are removed from the deck.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Short {}

        #[allow(clippy::module_name_repetitions)]
        pub type ShortDeck = Pile<Short>;
        #[allow(clippy::module_name_repetitions)]
        pub type ShortCard = Card<Short>;

        impl Short {
            pub const DECK_SIZE: usize = 36;

            pub const DECK: [BasicCard; Short::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
            ];
        }

        impl DeckedBase for Short {
            fn base_vec() -> Vec<BasicCard> {
                Short::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                Standard52::colors()
            }

            fn deck_name() -> String {
                "Short".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Short> for Short {}
    }
    pub mod skat {
        use crate::prelude::{
            BasicCard, Card, FLUENT_KEY_BASE_NAME_SKAT, Pile, Pip, SkatBasicCard, SkatSuit,
        };
        use crate::traits::{Decked, DeckedBase};
        use colored::Color;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Skat {}
        #[allow(clippy::module_name_repetitions)]
        pub type SkatDeck = Pile<Skat>;
        #[allow(clippy::module_name_repetitions)]
        pub type SkatCard = Card<Skat>;

        impl Skat {
            pub const DECK_SIZE: usize = 32;

            pub const DECK: [BasicCard; Skat::DECK_SIZE] = [
                SkatBasicCard::DAUSE_EICHEL,
                SkatBasicCard::ZHEN_EICHEL,
                SkatBasicCard::KÖNIG_EICHEL,
                SkatBasicCard::OBER_EICHEL,
                SkatBasicCard::UNTER_EICHEL,
                SkatBasicCard::NEUN_EICHEL,
                SkatBasicCard::ACHT_EICHEL,
                SkatBasicCard::SIEBEN_EICHEL,
                SkatBasicCard::DAUSE_LAUB,
                SkatBasicCard::ZHEN_LAUB,
                SkatBasicCard::KÖNIG_LAUB,
                SkatBasicCard::OBER_LAUB,
                SkatBasicCard::UNTER_LAUB,
                SkatBasicCard::NEUN_LAUB,
                SkatBasicCard::ACHT_LAUB,
                SkatBasicCard::SIEBEN_LAUB,
                SkatBasicCard::DAUSE_HERZ,
                SkatBasicCard::ZHEN_HERZ,
                SkatBasicCard::KÖNIG_HERZ,
                SkatBasicCard::OBER_HERZ,
                SkatBasicCard::UNTER_HERZ,
                SkatBasicCard::NEUN_HERZ,
                SkatBasicCard::ACHT_HERZ,
                SkatBasicCard::SIEBEN_HERZ,
                SkatBasicCard::DAUSE_SHELLEN,
                SkatBasicCard::ZHEN_SHELLEN,
                SkatBasicCard::KÖNIG_SHELLEN,
                SkatBasicCard::OBER_SHELLEN,
                SkatBasicCard::UNTER_SHELLEN,
                SkatBasicCard::NEUN_SHELLEN,
                SkatBasicCard::ACHT_SHELLEN,
                SkatBasicCard::SIEBEN_SHELLEN,
            ];
        }

        impl DeckedBase for Skat {
            fn base_vec() -> Vec<BasicCard> {
                Skat::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                let mut mappie = HashMap::new();

                mappie.insert(SkatSuit::LAUB, Color::Green);
                mappie.insert(SkatSuit::HERZ, Color::Red);
                mappie.insert(SkatSuit::SHELLEN, Color::BrightBlue);

                mappie
            }

            fn deck_name() -> String {
                "Skat".to_string()
            }

            fn fluent_name_base() -> String {
                FLUENT_KEY_BASE_NAME_SKAT.to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_SKAT.to_string()
            }
        }

        impl Decked<Skat> for Skat {}
    }
    pub mod spades {
        use crate::pack::decks::french::French;
        use crate::prelude::{
            BasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard,
            Pile, Pip,
        };
        use colored::Color;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Spades {}
        #[allow(clippy::module_name_repetitions)]
        pub type SpadesDeck = Pile<Spades>;
        #[allow(clippy::module_name_repetitions)]
        pub type SpadesCard = Card<Spades>;

        impl Spades {
            pub const DECK_SIZE: usize = 52;

            pub const DECK: [BasicCard; Spades::DECK_SIZE] = [
                FrenchBasicCard::BIG_JOKER,
                FrenchBasicCard::LITTLE_JOKER,
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::FIVE_SPADES,
                FrenchBasicCard::FOUR_SPADES,
                FrenchBasicCard::TREY_SPADES,
                FrenchBasicCard::DEUCE_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::FIVE_HEARTS,
                FrenchBasicCard::FOUR_HEARTS,
                FrenchBasicCard::TREY_HEARTS,
                FrenchBasicCard::DEUCE_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::FIVE_DIAMONDS,
                FrenchBasicCard::FOUR_DIAMONDS,
                FrenchBasicCard::TREY_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
                FrenchBasicCard::FIVE_CLUBS,
                FrenchBasicCard::FOUR_CLUBS,
                FrenchBasicCard::TREY_CLUBS,
            ];
        }

        impl DeckedBase for Spades {
            fn base_vec() -> Vec<BasicCard> {
                Spades::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                French::colors()
            }

            fn deck_name() -> String {
                "Spades".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Spades> for Spades {}
    }
    pub mod standard52 {
        use crate::prelude::{
            BasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_FRENCH, FrenchBasicCard,
            FrenchSuit, Pile, Pip,
        };
        use colored::Color;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Standard52 {}
        #[allow(clippy::module_name_repetitions)]
        pub type Standard52Deck = Pile<Standard52>;
        #[allow(clippy::module_name_repetitions)]
        pub type Standard52Card = Card<Standard52>;

        impl Standard52 {
            pub const DECK_SIZE: usize = 52;

            pub const DECK: [BasicCard; Standard52::DECK_SIZE] = [
                FrenchBasicCard::ACE_SPADES,
                FrenchBasicCard::KING_SPADES,
                FrenchBasicCard::QUEEN_SPADES,
                FrenchBasicCard::JACK_SPADES,
                FrenchBasicCard::TEN_SPADES,
                FrenchBasicCard::NINE_SPADES,
                FrenchBasicCard::EIGHT_SPADES,
                FrenchBasicCard::SEVEN_SPADES,
                FrenchBasicCard::SIX_SPADES,
                FrenchBasicCard::FIVE_SPADES,
                FrenchBasicCard::FOUR_SPADES,
                FrenchBasicCard::TREY_SPADES,
                FrenchBasicCard::DEUCE_SPADES,
                FrenchBasicCard::ACE_HEARTS,
                FrenchBasicCard::KING_HEARTS,
                FrenchBasicCard::QUEEN_HEARTS,
                FrenchBasicCard::JACK_HEARTS,
                FrenchBasicCard::TEN_HEARTS,
                FrenchBasicCard::NINE_HEARTS,
                FrenchBasicCard::EIGHT_HEARTS,
                FrenchBasicCard::SEVEN_HEARTS,
                FrenchBasicCard::SIX_HEARTS,
                FrenchBasicCard::FIVE_HEARTS,
                FrenchBasicCard::FOUR_HEARTS,
                FrenchBasicCard::TREY_HEARTS,
                FrenchBasicCard::DEUCE_HEARTS,
                FrenchBasicCard::ACE_DIAMONDS,
                FrenchBasicCard::KING_DIAMONDS,
                FrenchBasicCard::QUEEN_DIAMONDS,
                FrenchBasicCard::JACK_DIAMONDS,
                FrenchBasicCard::TEN_DIAMONDS,
                FrenchBasicCard::NINE_DIAMONDS,
                FrenchBasicCard::EIGHT_DIAMONDS,
                FrenchBasicCard::SEVEN_DIAMONDS,
                FrenchBasicCard::SIX_DIAMONDS,
                FrenchBasicCard::FIVE_DIAMONDS,
                FrenchBasicCard::FOUR_DIAMONDS,
                FrenchBasicCard::TREY_DIAMONDS,
                FrenchBasicCard::DEUCE_DIAMONDS,
                FrenchBasicCard::ACE_CLUBS,
                FrenchBasicCard::KING_CLUBS,
                FrenchBasicCard::QUEEN_CLUBS,
                FrenchBasicCard::JACK_CLUBS,
                FrenchBasicCard::TEN_CLUBS,
                FrenchBasicCard::NINE_CLUBS,
                FrenchBasicCard::EIGHT_CLUBS,
                FrenchBasicCard::SEVEN_CLUBS,
                FrenchBasicCard::SIX_CLUBS,
                FrenchBasicCard::FIVE_CLUBS,
                FrenchBasicCard::FOUR_CLUBS,
                FrenchBasicCard::TREY_CLUBS,
                FrenchBasicCard::DEUCE_CLUBS,
            ];
        }

        impl DeckedBase for Standard52 {
            fn base_vec() -> Vec<BasicCard> {
                Standard52::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                let mut mappie = HashMap::new();

                mappie.insert(FrenchSuit::HEARTS, Color::Red);
                mappie.insert(FrenchSuit::DIAMONDS, Color::Red);

                mappie
            }

            fn deck_name() -> String {
                "Standard 52".to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_FRENCH.to_string()
            }
        }

        impl Decked<Standard52> for Standard52 {}
    }
    pub mod tarot {
        use crate::prelude::{
            BasicCard, Card, Decked, DeckedBase, FLUENT_KEY_BASE_NAME_TAROT, Pile, Pip,
            TarotBasicCard, TarotSuit,
        };
        use colored::Color;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Tarot {}

        #[allow(clippy::module_name_repetitions)]
        pub type TarotDeck = Pile<Tarot>;
        #[allow(clippy::module_name_repetitions)]
        pub type TarotCard = Card<Tarot>;

        impl Tarot {
            pub const DECK_SIZE: usize = 78;

            pub const DECK: [BasicCard; Tarot::DECK_SIZE] = [
                TarotBasicCard::FOOL,
                TarotBasicCard::MAGICIAN,
                TarotBasicCard::HIGH_PRIESTESS,
                TarotBasicCard::EMPRESS,
                TarotBasicCard::EMPEROR,
                TarotBasicCard::HIEROPHANT,
                TarotBasicCard::LOVERS,
                TarotBasicCard::CHARIOT,
                TarotBasicCard::STRENGTH,
                TarotBasicCard::HERMIT,
                TarotBasicCard::WHEEL_OF_FORTUNE,
                TarotBasicCard::JUSTICE,
                TarotBasicCard::HANGED_MAN,
                TarotBasicCard::DEATH,
                TarotBasicCard::TEMPERANCE,
                TarotBasicCard::DEVIL,
                TarotBasicCard::TOWER,
                TarotBasicCard::STAR,
                TarotBasicCard::MOON,
                TarotBasicCard::SUN,
                TarotBasicCard::JUDGEMENT,
                TarotBasicCard::WORLD,
                TarotBasicCard::KING_WANDS,
                TarotBasicCard::QUEEN_WANDS,
                TarotBasicCard::KNIGHT_WANDS,
                TarotBasicCard::PAGE_WANDS,
                TarotBasicCard::TEN_WANDS,
                TarotBasicCard::NINE_WANDS,
                TarotBasicCard::EIGHT_WANDS,
                TarotBasicCard::SEVEN_WANDS,
                TarotBasicCard::SIX_WANDS,
                TarotBasicCard::FIVE_WANDS,
                TarotBasicCard::FOUR_WANDS,
                TarotBasicCard::THREE_WANDS,
                TarotBasicCard::TWO_WANDS,
                TarotBasicCard::ACE_WANDS,
                TarotBasicCard::KING_CUPS,
                TarotBasicCard::QUEEN_CUPS,
                TarotBasicCard::KNIGHT_CUPS,
                TarotBasicCard::PAGE_CUPS,
                TarotBasicCard::TEN_CUPS,
                TarotBasicCard::NINE_CUPS,
                TarotBasicCard::EIGHT_CUPS,
                TarotBasicCard::SEVEN_CUPS,
                TarotBasicCard::SIX_CUPS,
                TarotBasicCard::FIVE_CUPS,
                TarotBasicCard::FOUR_CUPS,
                TarotBasicCard::THREE_CUPS,
                TarotBasicCard::TWO_CUPS,
                TarotBasicCard::ACE_CUPS,
                TarotBasicCard::KING_SWORDS,
                TarotBasicCard::QUEEN_SWORDS,
                TarotBasicCard::KNIGHT_SWORDS,
                TarotBasicCard::PAGE_SWORDS,
                TarotBasicCard::TEN_SWORDS,
                TarotBasicCard::NINE_SWORDS,
                TarotBasicCard::EIGHT_SWORDS,
                TarotBasicCard::SEVEN_SWORDS,
                TarotBasicCard::SIX_SWORDS,
                TarotBasicCard::FIVE_SWORDS,
                TarotBasicCard::FOUR_SWORDS,
                TarotBasicCard::THREE_SWORDS,
                TarotBasicCard::TWO_SWORDS,
                TarotBasicCard::ACE_SWORDS,
                TarotBasicCard::KING_PENTACLES,
                TarotBasicCard::QUEEN_PENTACLES,
                TarotBasicCard::KNIGHT_PENTACLES,
                TarotBasicCard::PAGE_PENTACLES,
                TarotBasicCard::TEN_PENTACLES,
                TarotBasicCard::NINE_PENTACLES,
                TarotBasicCard::EIGHT_PENTACLES,
                TarotBasicCard::SEVEN_PENTACLES,
                TarotBasicCard::SIX_PENTACLES,
                TarotBasicCard::FIVE_PENTACLES,
                TarotBasicCard::FOUR_PENTACLES,
                TarotBasicCard::THREE_PENTACLES,
                TarotBasicCard::TWO_PENTACLES,
                TarotBasicCard::ACE_PENTACLES,
            ];
        }

        impl DeckedBase for Tarot {
            fn base_vec() -> Vec<BasicCard> {
                Tarot::DECK.to_vec()
            }

            fn colors() -> HashMap<Pip, Color> {
                let mut mappie = HashMap::new();

                mappie.insert(TarotSuit::MAJOR_ARCANA, Color::Blue);
                mappie.insert(TarotSuit::CUPS, Color::Red);
                mappie.insert(TarotSuit::SWORDS, Color::Red);

                mappie
            }

            fn deck_name() -> String {
                "Tarot".to_string()
            }

            fn fluent_name_base() -> String {
                FLUENT_KEY_BASE_NAME_TAROT.to_string()
            }

            fn fluent_deck_key() -> String {
                FLUENT_KEY_BASE_NAME_TAROT.to_string()
            }
        }

        impl Decked<Tarot> for Tarot {}
    }
    pub mod tiny {}
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__types__card_tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::pack::decks::french::French;
    use crate::pack::types::card::Card;
    use crate::prelude::{FrenchBasicCard, SkatBasicCard};
    use crate::traits::DeckedBase;
    use std::str::FromStr;

    #[test]
    fn new() {
        let card = Card::<French>::new(FrenchBasicCard::ACE_SPADES);
        assert_eq!(FrenchBasicCard::ACE_SPADES, card.base());
        assert_eq!(card.to_string(), "A♠");
    }

    /// This test exposes a flaw with my underlying logic. Going to create a validator
    /// of some sort.
    #[test]
    fn new__invalid_basic_card() {
        let card = Card::<French>::new(SkatBasicCard::KÖNIG_LAUB);
        assert_eq!(SkatBasicCard::KÖNIG_LAUB, card.base());
    }

    #[test]
    fn is_blank() {
        let card = Card::<French>::default();
        assert!(card.is_blank());
    }

    #[test]
    fn is_valid() {
        assert!(Card::<French>::new(FrenchBasicCard::ACE_SPADES).is_valid());
        assert!(!Card::<French>::new(SkatBasicCard::KÖNIG_LAUB).is_valid());
    }

    #[test]
    fn fluent_name() {
        let nine_of_clubs: Card<French> = FrenchBasicCard::NINE_CLUBS.into();

        assert_eq!(
            "Nine of Clubs",
            nine_of_clubs.fluent_name(&FluentName::US_ENGLISH)
        );
        assert_eq!("Neun Klee", nine_of_clubs.fluent_name(&FluentName::DEUTSCH));
    }

    #[test]
    fn fluent_name_default() {
        let eight_of_diamonds: Card<French> = FrenchBasicCard::EIGHT_DIAMONDS.into();

        assert_eq!("Eight of Diamonds", eight_of_diamonds.fluent_name_default());
    }

    #[test]
    fn fluent_rank_name() {
        let card: Card<French> = FrenchBasicCard::NINE_CLUBS.into();
        assert_eq!("Nine", card.fluent_rank_name(&FluentName::US_ENGLISH));
    }

    #[test]
    fn fluent_suit_name() {
        let card: Card<French> = FrenchBasicCard::NINE_CLUBS.into();
        assert_eq!("Clubs", card.fluent_suit_name(&FluentName::US_ENGLISH));
    }

    #[test]
    fn decked_base__vec() {
        let cards = Card::<French>::base_vec();

        let s: String = cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(
            "B🃟, L🃟, A♠, K♠, Q♠, J♠, T♠, 9♠, 8♠, 7♠, 6♠, 5♠, 4♠, 3♠, 2♠, A♥, K♥, Q♥, J♥, T♥, 9♥, 8♥, 7♥, 6♥, 5♥, 4♥, 3♥, 2♥, A♦, K♦, Q♦, J♦, T♦, 9♦, 8♦, 7♦, 6♦, 5♦, 4♦, 3♦, 2♦, A♣, K♣, Q♣, J♣, T♣, 9♣, 8♣, 7♣, 6♣, 5♣, 4♣, 3♣, 2♣",
            s
        );
    }

    #[test]
    fn display() {
        let basecard = FrenchBasicCard::ACE_SPADES;
        let card: Card<French> = basecard.into();

        assert_eq!("A♠", card.to_string());
        assert_eq!("A♠", basecard.to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!("AS", Card::<French>::from_str("as").unwrap().index());
        assert_eq!("__", Card::<French>::from_str("__").unwrap().index());
    }

    #[test]
    fn to_string_from_str() {
        let base_cards = Card::<French>::base_vec();

        for base_card in base_cards {
            let card: Card<French> = Card::<French>::from(base_card);
            let s = card.to_string();
            let index = card.index();

            assert_eq!(card, Card::<French>::from_str(&s).unwrap());
            assert_eq!(card, Card::<French>::from_str(&index).unwrap());
            assert_eq!(card, Card::<French>::from_str(&s.to_lowercase()).unwrap());
            assert_eq!(
                card,
                Card::<French>::from_str(&index.to_lowercase()).unwrap()
            );
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__types__pile_tests {
    use super::*;
    use crate::cards;
    use crate::pack::decks::standard52::Standard52;
    use crate::prelude::{
        BasicCard, Card, FLUENT_KEY_BASE_NAME_FRENCH, French, FrenchBasicCard, FrenchRank,
        FrenchSuit, Pile, Pip,
    };
    use crate::traits::{Decked, DeckedBase, Ranged};
    use colored::Color;
    use std::collections::{HashMap, HashSet};
    use std::str::FromStr;

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
    fn into_hashset() {
        let five_deck = French::decks(5);

        let hashset: HashSet<Card<French>> = five_deck.into_hashset();
        let deck = Pile::<French>::from(hashset);

        assert_eq!(five_deck.len(), 270);
        assert_eq!(deck, French::deck());
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
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__decks__canasta_tests {
    use super::*;
    use crate::prelude::Canasta;
    use crate::traits::{Decked, Ranged};

    #[test]
    fn decked__deck() {
        assert_eq!(
            Canasta::deck().to_string(),
            "3♥ 3♥ 3♦ 3♦ B🃟 B🃟 L🃟 L🃟 2♠ 2♠ 2♥ 2♥ 2♦ 2♦ 2♣ 2♣ A♠ A♠ K♠ K♠ Q♠ Q♠ J♠ J♠ T♠ T♠ 9♠ 9♠ 8♠ 8♠ 7♠ 7♠ 6♠ 6♠ 5♠ 5♠ 4♠ 4♠ 3♠ 3♠ A♥ A♥ K♥ K♥ Q♥ Q♥ J♥ J♥ T♥ T♥ 9♥ 9♥ 8♥ 8♥ 7♥ 7♥ 6♥ 6♥ 5♥ 5♥ 4♥ 4♥ A♦ A♦ K♦ K♦ Q♦ Q♦ J♦ J♦ T♦ T♦ 9♦ 9♦ 8♦ 8♦ 7♦ 7♦ 6♦ 6♦ 5♦ 5♦ 4♦ 4♦ A♣ A♣ K♣ K♣ Q♣ Q♣ J♣ J♣ T♣ T♣ 9♣ 9♣ 8♣ 8♣ 7♣ 7♣ 6♣ 6♣ 5♣ 5♣ 4♣ 4♣ 3♣ 3♣"
        );
    }

    #[test]
    pub fn ranks_index() {
        let pile = Canasta::deck().shuffled();
        let expected = "3~B~L~2~A~K~Q~J~T~9~8~7~6~5~4~3";

        let ranks_index = pile.ranks_index("~");

        assert_eq!(ranks_index, expected);
    }

    /// TODO: WTF??!!
    /// TODO: WTF do I mean by WTF??? Don't do this.
    #[test]
    pub fn suits_index() {
        let pile = Canasta::deck().shuffled();
        let expected = "H~D~J~S~H~D~C~S~H~D~C";

        let ranks_index = pile.suits_index("~");

        assert_eq!(ranks_index, expected);
    }

    #[test]
    fn decked__validate() {
        assert!(Canasta::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__decks__euchre24_tests {
    use super::*;
    use crate::pack::decks::euchre24::Euchre24;
    use crate::traits::Decked;

    #[test]
    fn decked__deck() {
        assert_eq!(
            Euchre24::deck().to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ A♥ K♥ Q♥ J♥ T♥ 9♥ A♦ K♦ Q♦ J♦ T♦ 9♦ A♣ K♣ Q♣ J♣ T♣ 9♣"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Euchre24::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__decks__euchre32_tests {
    use super::*;
    use crate::pack::decks::euchre32::Euchre32;
    use crate::traits::Decked;

    #[test]
    fn decked__deck() {
        assert_eq!(
            Euchre32::deck().to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Euchre32::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__card__french__tests {
    use super::*;
    use crate::pack::decks::french::French;
    use crate::prelude::{Card, FrenchBasicCard, Pile};
    use crate::traits::{Decked, DeckedBase};
    use std::str::FromStr;

    #[test]
    fn from_str__card() {
        assert_eq!(
            Card::<French>::from_str("2c").unwrap(),
            FrenchBasicCard::DEUCE_CLUBS.into()
        );
    }

    #[test]
    fn from_str__pile() {
        let pile = Pile::<French>::from_str("2c 3c 4c").unwrap();

        assert_eq!(pile.len(), 3);
        assert_eq!(pile.to_string(), "2♣ 3♣ 4♣");
    }

    #[test]
    fn decked__validate() {
        assert!(French::validate());
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Pile::<French>::deck_name(), "French");
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__card__pinochle_tests {
    use super::*;
    use crate::pack::decks::pinochle::Pinochle;
    use crate::traits::Decked;

    #[test]
    fn decked__validate() {
        assert!(Pinochle::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__decks__razz_tests {
    use super::*;
    use crate::pack::decks::razz::Razz;
    use crate::prelude::Pile;
    use crate::traits::Decked;

    #[test]
    fn from_str() {
        let deck = Pile::<Razz>::deck();

        assert_eq!(
            deck.to_string(),
            "A♠ 2♠ 3♠ 4♠ 5♠ 6♠ 7♠ 8♠ 9♠ T♠ J♠ Q♠ K♠ A♥ 2♥ 3♥ 4♥ 5♥ 6♥ 7♥ 8♥ 9♥ T♥ J♥ Q♥ K♥ A♦ 2♦ 3♦ 4♦ 5♦ 6♦ 7♦ 8♦ 9♦ T♦ J♦ Q♦ K♦ A♣ 2♣ 3♣ 4♣ 5♣ 6♣ 7♣ 8♣ 9♣ T♣ J♣ Q♣ K♣"
        );
    }

    #[test]
    fn deck__draw() {
        let mut deck = Pile::<Razz>::deck();
        assert_eq!(deck.draw(3).unwrap().to_string(), "A♠ 2♠ 3♠");
    }

    #[test]
    fn decked__validate() {
        assert!(Pile::<Razz>::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod pack__card__short_tests {
    use super::*;
    use crate::pack::decks::short::Short;
    use crate::traits::Decked;

    #[test]
    fn deck() {
        let deck = Short::deck();
        assert_eq!(deck.len(), 36);
        assert_eq!(
            deck.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Short::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__skat_tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::pack::decks::skat::Skat;
    use crate::traits::Decked;

    #[test]
    fn decked__deck() {
        let deck = Skat::deck();
        assert_eq!(
            deck.to_string(),
            "D♣ Z♣ K♣ O♣ U♣ 9♣ 8♣ 7♣ D♠ Z♠ K♠ O♠ U♠ 9♠ 8♠ 7♠ D♥ Z♥ K♥ O♥ U♥ 9♥ 8♥ 7♥ D♦ Z♦ K♦ O♦ U♦ 9♦ 8♦ 7♦"
        );
        assert_eq!(
            deck.index(),
            "DE ZE KE OE UE 9E 8E 7E DL ZL KL OL UL 9L 8L 7L DH ZH KH OH UH 9H 8H 7H DS ZS KS OS US 9S 8S 7S"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Skat::validate());
    }

    #[test]
    fn fluent__name() {
        let mut deck = Skat::deck();
        let dause_eichel = deck.draw_first().unwrap();
        let daus = dause_eichel.fluent_rank_name(&FluentName::DEUTSCH);
        let eichel = dause_eichel.fluent_suit_name(&FluentName::DEUTSCH);
        let deuce = dause_eichel.fluent_rank_name(&FluentName::US_ENGLISH);
        let acorns = dause_eichel.fluent_suit_name(&FluentName::US_ENGLISH);

        assert_eq!(daus, "Daus");
        assert_eq!(eichel, "Eichel");
        assert_eq!(deuce, "Deuce");
        assert_eq!(acorns, "Acorns");
        assert_eq!(dause_eichel.fluent_name_default(), "Deuce of Acorns");
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__spades_tests {
    use super::*;
    use crate::pack::decks::spades::Spades;
    use crate::prelude::{Card, Pile};
    use crate::traits::Decked;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            Spades::deck().to_string(),
            "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣"
        );
    }

    #[test]
    fn from_str__card() {
        assert!(Card::<Spades>::from_str("2c").is_err());
    }

    #[test]
    fn from_str__pile() {
        let pile = Pile::<Spades>::from_str("2c 3c 4c");

        assert!(pile.is_err());
    }

    #[test]
    fn decked__validate() {
        assert!(Spades::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__card__standard52_tests {
    use crate::pack::decks::standard52::Standard52;
    use crate::traits::Decked;

    #[test]
    fn decked__validate() {
        assert!(Standard52::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__tarot_tests {
    use super::*;
    use crate::pack::decks::tarot::{Tarot, TarotCard};
    use crate::traits::Decked;
    use std::str::FromStr;

    #[test]
    fn fluent__fluent_name_default() {
        let magician = TarotCard::from_str("mm").unwrap();

        assert_eq!(magician.index(), "MM");
        assert_eq!(magician.fluent_name_default(), "The Magician");
    }

    #[test]
    fn decked__validate() {
        assert!(Tarot::validate());
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__tiny__tests {
    use super::*;
    use crate::basic::cards::tiny::Tiny;
    use crate::prelude::*;

    // This is
    #[test]
    fn test() {
        let mut deck = Tiny::deck();

        assert_eq!(deck.to_string(), "A♠ K♠ A♥ K♥");

        // Every deck comes with the Ranged trait automatically:
        assert_eq!(
            deck.combos(2).to_string(),
            "A♠ K♠, A♠ A♥, A♠ K♥, K♠ K♥, A♥ K♠, A♥ K♥"
        );

        // Deal from the top of the deck:
        assert_eq!(deck.draw_first().unwrap().to_string(), "A♠");

        // Deal from the bottom of the deck:
        assert_eq!(deck.draw_last().unwrap().to_string(), "K♥");

        // Should be two cards remaining:
        assert_eq!(deck.len(), 2);
        assert_eq!(deck.index(), "KS AH");

        // Draw a remaining card:
        assert_eq!(deck.draw_first().unwrap(), tiny!(KS));

        // Draw the last card:
        assert_eq!(deck.draw_last().unwrap(), tiny!(AH));

        // And now the deck is empty:
        assert!(deck.draw_first().is_none());
        assert!(deck.draw_last().is_none());
    }

    #[test]
    fn validate() {
        assert!(Tiny::validate());
    }
}
