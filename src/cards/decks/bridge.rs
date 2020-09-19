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

use crate::cards::card::Card;
use crate::cards::pack::Pack;
use crate::cards::pile::Pile;

/// BridgeBoard is a French Deck Pack that sorts and validates the hands dealt as a part
/// of a Bridge hand.
pub struct BridgeBoard {
    pack: Pack,
    pub south: Pile,
    pub west: Pile,
    pub north: Pile,
    pub east: Pile,
}

impl BridgeBoard {
    /// Parses a Portable Bridge Notation deal string and converts it into a
    /// BridgeBoard struct.
    pub fn from_pbn_deal(deal: &str) -> BridgeBoard {
        let (_, pbn) = BridgeBoard::split_on_direction(deal);
        let mut dir_iter = pbn.split_whitespace();

        let mut board = BridgeBoard::default();
        board.south = board.to_pile(dir_iter.next().unwrap());
        board.west = board.to_pile(dir_iter.next().unwrap());
        board.north = board.to_pile(dir_iter.next().unwrap());
        board.east = board.to_pile(dir_iter.next().unwrap());

        board
    }

    pub fn deal() -> BridgeBoard {
        let mut board = BridgeBoard::default();
        let mut cards = board.pack.cards().shuffle();
        board.south = cards.draw(13).unwrap();
        board.west = cards.draw(13).unwrap();
        board.north = cards.draw(13).unwrap();
        board.east = cards.draw(13).unwrap();
        board
    }

    pub fn demo(&self) {
        println!("S: {}", self.south.sort().by_symbol_index());
        println!("W: {}", self.west.sort().by_symbol_index());
        println!("N: {}", self.north.sort().by_symbol_index());
        println!("E: {}", self.east.sort().by_symbol_index());
    }

    pub fn get_pack(&self) -> &Pack {
        &self.pack
    }

    pub fn is_valid(&self) -> bool {
        let piles = &[
            self.south.clone(),
            self.west.clone(),
            self.north.clone(),
            self.east.clone(),
        ];
        let pile = Pile::pile_on(piles);
        self.pack.is_complete(&[pile])
    }

    pub fn to_pbn_deal(&self) -> String {
        "".to_string()
    }

    fn to_pile(&self, s: &str) -> Pile {
        let rawsuits: Vec<&str> = s.split('.').collect();

        let mut v: Vec<String> = Vec::new();
        v.append(&mut BridgeBoard::splice_suit_in(
            &rawsuits.get(0).unwrap(),
            'S',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            &rawsuits.get(1).unwrap(),
            'H',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            &rawsuits.get(2).unwrap(),
            'D',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            &rawsuits.get(3).unwrap(),
            'C',
        ));

        let coll: Vec<Card> = v
            .iter()
            .map(|s| self.pack.cards().card_by_index(s.as_str()).unwrap().clone())
            .collect();

        Pile::new_from_vector(coll)
    }

    fn splice_suit_in(s: &str, suit: char) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();

        for c in s.chars() {
            v.push(format!("{}{}", c, suit));
        }
        v
    }

    fn split_on_direction(deal: &str) -> (char, &str) {
        let direction = deal.chars().next().unwrap();
        let remainder = &deal[2..];

        (direction, remainder)
    }
}

impl Default for BridgeBoard {
    fn default() -> Self {
        BridgeBoard {
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
mod bridge_board_tests {
    use super::*;

    const PBN_TEST_STRING: &str =
        "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

    #[test]
    fn from_pbn_deal() {
        let deck = Pile::french_deck();
        let south = deck.pile_by_index(&[
            "QS", "4S", "2S", "QH", "5H", "2H", "AD", "QD", "TD", "9D", "4D", "3D", "QC",
        ]);
        let west = deck.pile_by_index(&[
            "9S", "7S", "AH", "TH", "9H", "3H", "6D", "5D", "2D", "TC", "7C", "4C", "3C",
        ]);
        let north = deck.pile_by_index(&[
            "AS", "JS", "TS", "8S", "5S", "JH", "7H", "6H", "KD", "JD", "AC", "6C", "5C",
        ]);
        let east = deck.pile_by_index(&[
            "KS", "6S", "3S", "KH", "8H", "4H", "8D", "7D", "KC", "JC", "9C", "8C", "2C",
        ]);

        let deal = BridgeBoard::from_pbn_deal(PBN_TEST_STRING);

        assert_eq!(south.unwrap().by_index(), deal.south.by_index());
        assert_eq!(west.unwrap().by_index(), deal.west.by_index());
        assert_eq!(north.unwrap().by_index(), deal.north.by_index());
        assert_eq!(east.unwrap().by_index(), deal.east.by_index());
        assert!(deal.is_valid())
    }

    #[test]
    fn is_valid() {
        let deck = BridgeBoard::deal();

        assert!(deck.is_valid())
    }

    #[test]
    fn is_valid_ne() {
        let mut deck = BridgeBoard::default();
        let mut cards = deck.pack.cards().shuffle();
        deck.south = cards.draw(13).unwrap();
        deck.west = cards.draw(13).unwrap();
        deck.north = cards.draw(13).unwrap();
        deck.east = cards.draw(12).unwrap();

        assert!(!deck.is_valid())
    }

    #[test]
    fn splice_suit_in() {
        let expected = vec!["QS".to_string(), "4S".to_string()];

        let actual = BridgeBoard::splice_suit_in("Q4", 'S');

        assert_eq!(expected, actual)
    }

    #[test]
    fn split_on_direction() {
        let expected_remainder =
            "Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

        let (char, remainder) = BridgeBoard::split_on_direction(PBN_TEST_STRING);

        assert_eq!('S', char);
        assert_eq!(expected_remainder, remainder);
    }

    #[test]
    fn to_pbn_deal() {
        assert_eq!(
            PBN_TEST_STRING.to_string(),
            BridgeBoard::from_pbn_deal(PBN_TEST_STRING).to_pbn_deal()
        )
    }
}
