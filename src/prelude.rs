pub use crate::basic::decks::canasta::*;
pub use crate::basic::decks::cards::canasta::*;
pub use crate::basic::decks::cards::french::*;
pub use crate::basic::decks::cards::pinochle::*;
pub use crate::basic::decks::cards::skat::*;
pub use crate::basic::decks::cards::tarot::*;
pub use crate::basic::decks::euchre24::*;
pub use crate::basic::decks::euchre32::*;
pub use crate::basic::decks::french::*;
// I love how CoPilot keeps recommending hand_and_foot::HandAndFoot even though it's no longer there.
// pub use crate::rev6::decks::hand_and_foot::*;
// guess this is a form of hallucination.
pub use crate::basic::decks::pinochle::*;
pub use crate::basic::decks::short::*;
pub use crate::basic::decks::skat::*;
pub use crate::basic::decks::spades::*;
pub use crate::basic::decks::standard52::*;
pub use crate::basic::decks::tarot::*;
pub use crate::basic::types::basic_card::BasicCard;
pub use crate::basic::types::card::Card;
pub use crate::basic::types::deck::Deck;
pub use crate::basic::types::pile::Pile;
pub use crate::basic::types::pips::{Pip, PipType};
pub use crate::basic::types::traits::{Decked, DeckedBase};
pub use crate::common::errors::CardError;
pub use crate::localization::{FluentName, Named};

pub use colored::Colorize;
pub use std::str::FromStr;
