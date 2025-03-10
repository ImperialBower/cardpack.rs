pub mod types {
    pub mod card {
        use std::collections::{HashMap, HashSet};
        use std::fmt::{Display, Formatter};
        use std::hash::Hash;
        use std::marker::PhantomData;
        use std::str::FromStr;
        use std::vec::IntoIter;
        use colored::{Color, Colorize};
        use fluent_templates::LanguageIdentifier;
        use rand::prelude::SliceRandom;
        use rand::{rng, Rng};
        use serde::{Deserialize, Serialize};
        use crate::bussin::types::card::{BasicCard, BasicPile};
        use crate::bussin::types::pips::{Pip, PipType};
        use crate::localization::{FluentName, Named};
        use crate::traits::DeckedBase;
        // region Card

        /// A `Card` is a struct that's a generic wrapper around a [`BasicCard`] providing it with additional
        /// deck specific superpowers, many of which are at the pile level. The ones at the `Card` level
        /// are:
        ///
        /// - `color()` - returns the color of the card based on what's configured at the type parameter's implementation of the `DeckedBase` trait.
        /// - `fluent_name()` - returns the long name of the card from the `Named` trait's use of fluent templates.
        /// - `from_str()` - allows you to create a `Card` for the specific deck with a string representation of the card.
        #[derive(
            Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
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
                crate::traits::basic_pile().contains(&self.base_card)
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

        impl<DeckType: DeckedBase> DeckedBase for crate::prelude::Card<DeckType> {
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

        impl<DeckType: DeckedBase> From<BasicCard> for crate::prelude::Card<DeckType> {
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
        /// collection of [`Cards`](crate::prelude::Card) that are bound by a
        /// specific deck  type parameter.
        ///
        /// The magic behind all this is enabled by implementing the [`Decked`] and [`DeckedBase`] traits.
        /// [`DeckedBase`] defines the [`BasicCards`](crate::prelude::BasicCard) that
        /// hold the data that is in the [`Cards`](crate::prelude::Card) of the `Pile`, and the
        /// [`Decked`] trait that ensures that only [`Cards`](crate::prelude::Card) that fit
        /// the contract defined in the specific deck implementation trait, such as
        /// [`French`](crate::basic::decks::french::French) for a traditional pack of cards with jokers, or
        /// [`Pinochle`](crate::basic::decks::pinochle::Pinochle). This makes it possible for the users
        /// to define a `Pile` of [`Cards`](crate::prelude::Card) through simple strings. Here's
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
            fn from_iter<I: IntoIterator<Item =Card<Decked>>>(iter: I) -> Self {
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