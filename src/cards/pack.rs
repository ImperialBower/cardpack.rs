/*  CardPack - A generic pack of cards library written in Rust.
Copyright (C) <2020>  Christoph Baker

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use crate::cards::pile::Pile;

/// A Pack is an immutable pile of cards. Packs are designed to be a flexible representation of
/// a deck, stack, discard pile, or hand.
///
/// Packs available are for a traditional 52 card French Deck, pinochle, spades, skat and tarot.
///
/// # Usage:
/// ```
/// let pack = cardpack::Pack::french_deck();
///
/// let mut shuffled = pack.cards().shuffle();
/// let sb = shuffled.draw(2).unwrap();
/// let bb = shuffled.draw(2).unwrap();
///
/// println!("small blind: {}", sb.by_symbol_index());
/// println!("big blind:   {}", bb);
///
/// println!();
/// println!("flop : {}", shuffled.draw(3).unwrap());
/// println!("turn : {}", shuffled.draw(1).unwrap());
/// println!("river: {}", shuffled.draw(1).unwrap());
///
/// ```
///

pub struct Pack {
    cards: Pile,
}

impl Pack {
    fn new(cards: Pile) -> Pack {
        Pack { cards }
    }

    /// Returns a reference to the cards in the Pack.
    pub fn cards(&self) -> &Pile {
        &self.cards
    }

    ///
    pub fn french_deck() -> Pack {
        Pack::new(Pile::french_deck())
    }

    pub fn pinochle_deck() -> Pack {
        Pack::new(Pile::pinochle_deck())
    }

    pub fn skat_deck() -> Pack {
        Pack::new(Pile::skat_deck())
    }

    pub fn spades_deck() -> Pack {
        Pack::new(Pile::spades_deck())
    }
}
