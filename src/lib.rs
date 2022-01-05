#![warn(clippy::pedantic)]

extern crate rand;

pub mod cards;
pub mod fluent;

pub use crate::cards::card::Card;
pub use crate::cards::decks::bridge::BridgeBoard;
pub use crate::cards::decks::standard52::Standard52;
pub use crate::cards::pack::Pack;
pub use crate::cards::pile::Pile;
pub use crate::cards::rank::*;
pub use crate::cards::suit::*;
pub use fluent::named::*;
pub use fluent::*;
