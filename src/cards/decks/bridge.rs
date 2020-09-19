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

use crate::cards::pack::Pack;
use crate::cards::pile::Pile;

pub struct BridgeDeck {
    pack: Pack,
    pub south: Pile,
    pub west: Pile,
    pub north: Pile,
    pub east: Pile,
}

impl BridgeDeck {
    pub fn get_pack(&self) -> &Pack {
        return &self.pack
    }

    pub fn is_valid(&self) -> bool {
        let piles = &[self.south.clone(), self.west.clone(), self.north.clone(), self.east.clone()];
        let pile = Pile::pile_on(piles);
        self.pack.is_complete(&[pile])
    }
}

impl Default for BridgeDeck {
    fn default() -> Self {
        BridgeDeck {
            pack: Pack::french_deck(),
            south: Pile::default(),
            west: Pile::default(),
            north: Pile::default(),
            east: Pile::default(),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_deck_tests {
    use super::*;

    #[test]
    fn is_valid() {
        let mut deck = BridgeDeck::default();
        let mut cards = deck.pack.cards().shuffle();
        deck.south = cards.draw(13).unwrap();
        deck.west = cards.draw(13).unwrap();
        deck.north = cards.draw(13).unwrap();
        deck.east = cards.draw(13).unwrap();

        assert!(deck.is_valid())
    }
}
