pub use crate::bussin::cards::canasta::*;
pub use crate::bussin::cards::french::*;
pub use crate::bussin::cards::pinochle::*;
pub use crate::bussin::cards::skat::*;
pub use crate::bussin::cards::tarot::*;
pub use crate::bussin::types::card::BasicCard;
pub use crate::bussin::types::card::BasicPile;
pub use crate::bussin::types::pips::{Pip, PipType};
pub use crate::pack::decks::canasta::*;
pub use crate::pack::decks::euchre24::*;
pub use crate::pack::decks::euchre32::*;
pub use crate::pack::decks::french::*;
pub use crate::pack::decks::pinochle::*;
pub use crate::pack::decks::razz::*;
pub use crate::pack::decks::short::*;
pub use crate::pack::decks::skat::*;
pub use crate::pack::decks::spades::*;
pub use crate::pack::decks::standard52::*;
pub use crate::pack::decks::tarot::*;
pub use crate::pack::decks::tiny::*;
// I love how CoPilot keeps recommending hand_and_foot::HandAndFoot even though it's no longer there.
// pub use crate::rev6::decks::hand_and_foot::*;
// guess this is a form of hallucination.
pub use crate::common::errors::CardError;
pub use crate::localization::{FluentName, Named};
pub use crate::pack::types::card::Card;
pub use crate::pack::types::card::Pile;
pub use crate::traits::{CKCRevised, Decked, DeckedBase, Ranged};

// Macros
pub use crate::card;
pub use crate::cards;
pub use crate::french_cards;
pub use crate::tiny;

pub use colored::{Color, Colorize};
pub use std::str::FromStr;
