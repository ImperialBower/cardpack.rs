#![warn(clippy::pedantic)]
#![allow(clippy::needless_doctest_main)]

//! [Cardpack](https://crates.io/crates/cardpack) is a library to represent various decks of playing
//! cards. The library is designed to support the following features:
//!
//! - Custom `Rank` and `Suit` [`Pips`](basic::types::pips::Pip).
//! - Ability to sort a [`Deck`](basic::types::deck::Deck) of [`Cards`](basic::types::card::Card) in various ways.
//! - Localization of card names using [fluent-templates](https://github.com/XAMPPRocky/fluent-templates).
//!
//! ## Overview
//!
//! The structure of the library is the following:
//!
//! - [`Deck`](basic::types::deck::Deck) - A generic collection of [`Cards`](basic::types::card::Card) that implement the [`DeckedBase`](common::traits::DeckedBase) trait
//!   - [`Card`](basic::types::card::Card) - A generic wrapper around [`BasicCard`](basic::types::basic_card::BasicCard) that implements the [`DeckedBase`](common::traits::DeckedBase) trait.
//!     - [`BasicCard`](basic::types::basic_card::BasicCard) - The basic data of a [`Card`](basic::types::card::Card) without any generic constraints. Made up of a `Rank` and `Suit` [`Pip`](basic::types::pips::Pip).
//!       - [`Pip`](basic::types::pips::Pip) - The basic data of a `Rank` and `Suit`, used for sorting, evaluating, and displaying [`Cards`](basic::types::card::Card).
//!
//! The library supports the following decks:
//!
//! ## French Deck
//!
//! The [`French`](basic::decks::french::French) deck is the foundation [`Deck`](basic::types::deck::Deck)
//! of playing cards. It is made up of a collection of 54 `Cards` with 13 ranks in each of the four suits,
//! and two jokers. Most of the other decks are made up on the [`French BasicCards`](basic::decks::cards::french::FrenchBasicCard).
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! let mut french_deck = Deck::<French>::deck();
//!
//! // It's also possible to call the deck method directly on the specific generic implementing type:
//! let mut french_deck = French::deck();
//!
//! assert_eq!(french_deck.len(), 54);
//! assert_eq!(
//!     french_deck.to_string(),
//!     "B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
//! );
//! assert!(french_deck.contains(&Card::<French>::new(FrenchBasicCard::ACE_SPADES)));
//!
//! let shuffled = french_deck.shuffled();
//!
//! // Use the `french_cards!` macro to parse the shuffled deck as a string:
//! let parsed = french_cards!(shuffled.to_string().as_str());
//!
//! // Verify that the cards, in any order, are the same:
//! assert!(french_deck.same(&parsed));
//!
//! // When sorted, they should be exactly the same:
//! assert_eq!(parsed.sort(), french_deck);
//!
//! // For a joker card's index string, `B` stands for the Big or Full-Color Joker and `L` for the
//! // Little or One-Color Joker, with `🃟` being the symbol character for the joker suit.
//! let jokers = french_deck.draw(2).unwrap();
//! assert_eq!(jokers.to_string(), "B🃟 L🃟");
//!
//! let royal_flush = french_deck.draw(5).unwrap();
//! assert_eq!(royal_flush.to_string(), "A♠ K♠ Q♠ J♠ T♠");
//! assert_eq!(royal_flush.index(), "AS KS QS JS TS");
//!
//! // The original deck should now have five cards less:
//! assert_eq!(french_deck.len(), 47);
//!
//! // Cards can provide a longer description in English and German:
//! assert_eq!(Card::<French>::new(FrenchBasicCard::ACE_SPADES).fluent_name_default(), "Ace of Spades");
//! assert_eq!(Card::<French>::new(FrenchBasicCard::QUEEN_HEARTS).fluent_name(&FluentName::DEUTSCH), "Dame Herzen");
//! ```
//!
//! At some point I would love to add support for more languages.
//!
//! ## Standard 52 Card Deck
//!
//! A [`Standard52`](basic::decks::standard52::Standard52) deck is a
//! [`French`](basic::decks::french::French) deck without the two jokers.
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! let mut standard52_deck = Deck::<Standard52>::deck();
//!
//! assert_eq!(standard52_deck.len(), 52);
//! assert_eq!(
//!     standard52_deck.to_string(),
//!     "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
//! );
//!
//! // It includes the card! and cards! macros for easy Standard52 card creation:
//! assert_eq!(card!(AS), Card::<Standard52>::new(FrenchBasicCard::ACE_SPADES));
//! assert_eq!(cards!("AS KS QS JS TS"), standard52_deck.draw(5).unwrap());
//! ```
//!
//! By default, a [`Deck`](basic::types::deck::Deck) displays the suit symbols when you display the
//! values. It also has the ability to return the letter values, or what are called "index strings".
//!
//! ```rust
//! use cardpack::prelude::*;
//!
//! assert_eq!(
//!     Deck::<Standard52>::deck().index(),
//!     "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C"
//! );
//! ```
//!
//! An important thing to remember about the decks is that the cards have their weight inside them
//! to facilitate sorting. If you wanted a deck for a game of poker where the lowest hand wins, you
//! would need to create a separate deck file with the card's `Rank` weights inverted. The
//! [`Razz Deck`](basic::decks::razz::Razz) is an example of this. It is also an example of
//! how you can create a [`Deck`](basic::types::deck::Deck)  where the
//! [`BasicCard`](basic::types::basic_card::BasicCard) for the deck are generated programmatically
//! in YAML instead using the power of [Serde](https://serde.rs/)
//!
//! ```
//! use cardpack::prelude::*;
//! assert_eq!(Deck::<Razz>::deck().draw(5).unwrap().to_string(), "A♠ 2♠ 3♠ 4♠ 5♠");
//! assert_eq!(Deck::<Standard52>::deck().draw(5).unwrap().to_string(), "A♠ K♠ Q♠ J♠ T♠");
//! ```
//!
//! The raw YAML that was used to create the [`Razz Deck`](basic::decks::razz::Razz) is available
//! in the source code.
//!
//!
//! Other decks include:
//!
//! - [`Canasta`](basic::decks::canasta::Canasta) - 2 Modern decks with the red 3s made jokers.
//! - [`Euchre24`](basic::decks::euchre24::Euchre24) - A 24 card version of a Euchre deck.
//! - [`Euchre32`](basic::decks::euchre32::Euchre32) - A 32 card version of a Euchre deck.
//! - [`ShortDeck`](basic::decks::short::Short) - A 36 card deck with ranks 6 through Ace.
//! - [`Pinochle`](basic::decks::pinochle::Pinochle) - A 48 card deck with two copies of the 9 through Ace ranks.
//! - [`Skat`](basic::decks::skat::Skat) - A 32 card German card game with different suits and ranks.
//! - [`Spades`](basic::decks::spades::Spades) - A Modern deck with the 2 of Clubs and 2 of Diamonds removed.
//! - [`Tarot`](basic::decks::tarot::Tarot) - A 78 card deck with 22 Major Arcana and 56 Minor Arcana cards.
//!
//! In past versions of the library there was a [Hand and Foot](https://gamerules.com/rules/hand-and-foot-card-game/)
//! deck. This has been removed because it can simply be created using a
//! [`French`](basic::decks::french::French) and what functionality is available in the Decked trait:
//!
//! ```
//! use cardpack::prelude::*;
//!
//! let hand_and_foot_4players = Deck::<French>::decks(4);
//! assert_eq!(hand_and_foot_4players.len(), 216);
//!
//! let hand_and_foot_5players = Deck::<French>::decks(5);
//! assert_eq!(hand_and_foot_5players.len(), 270);
//! ```
//!
//!
//! ## Custom Deck example:
//!
//! Here's a very simple example where we create a tiny deck with only the ace and kink ranks,
//! and only the spades and hearts suits. Just for fun, we'll include a `tiny!` macro for one `Tiny` card.
//!
//! ```rust
//! use std::collections::HashMap;
//! use colored::Color;
//! use cardpack::prelude::*;
//!
//! #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
//! pub struct Tiny {}
//!
//! impl Tiny {
//!    pub const DECK_SIZE: usize = 4;
//!
//!     pub const DECK: [BasicCard; Tiny::DECK_SIZE] = [
//!         FrenchBasicCard::ACE_SPADES,
//!         FrenchBasicCard::KING_SPADES,
//!         FrenchBasicCard::ACE_HEARTS,
//!         FrenchBasicCard::KING_HEARTS,
//!     ];
//! }
//!
//! impl DeckedBase for Tiny {
//!     fn base_vec() -> Vec<BasicCard> {
//!         Tiny::DECK.to_vec()
//!     }
//!
//!     fn colors() -> HashMap<Pip, Color> {
//!         Standard52::colors()
//!     }
//!
//!     fn deck_name() -> String {
//!         "Tiny".to_string()
//!     }
//!
//!     fn fluent_deck_key() -> String {
//!         FLUENT_KEY_BASE_NAME_FRENCH.to_string()
//!     }
//! }
//!
//! // Let's you call Decked methods directly on the Tiny type:
//! impl Decked<Tiny> for Tiny {}
//!
//! macro_rules! tiny {
//!     (AS) => {
//!         Card::<Tiny>::new(FrenchBasicCard::ACE_SPADES)
//!     };
//!     (KS) => {
//!         Card::<Tiny>::new(FrenchBasicCard::KING_SPADES)
//!     };
//!     (AH) => {
//!         Card::<Tiny>::new(FrenchBasicCard::ACE_HEARTS)
//!     };
//!     (KH) => {
//!         Card::<Tiny>::new(FrenchBasicCard::KING_HEARTS)
//!     };
//!     (__) => {
//!         Card::<Tiny>::default()
//!     };
//! }
//!
//! let mut deck = Tiny::deck();
//!
//! assert_eq!(deck.to_string(), "A♠ K♠ A♥ K♥");
//!
//! // Deal from the top of the deck:
//! assert_eq!(deck.draw_first().unwrap().to_string(), "A♠");
//!
//! // Deal from the bottom of the deck:
//! assert_eq!(deck.draw_last().unwrap().to_string(), "K♥");
//!
//! // Should be two cards remaining:
//! assert_eq!(deck.len(), 2);
//! assert_eq!(deck.index(), "KS AH");
//!
//! // Draw a remaining card:
//! assert_eq!(deck.draw_first().unwrap(), tiny!(KS));
//!
//! // Draw the last card:
//! assert_eq!(deck.draw_last().unwrap(), tiny!(AH));
//!
//! // And now the deck is empty:
//! assert!(deck.draw_first().is_none());
//! assert!(deck.draw_last().is_none());
//! ```

extern crate rand;

pub mod basic;
pub mod common;
pub mod localization;
pub mod old;
pub mod prelude;
pub mod prelude_old;
