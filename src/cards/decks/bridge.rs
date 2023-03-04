use std::fmt;

use crate::cards::card::Card;
use crate::cards::pack::Pack;
use crate::cards::pile::Pile;
use crate::cards::suit::{Suit, CLUBS, DIAMONDS, HEARTS, SPADES};
use std::collections::HashMap;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum BridgeDirection {
    N,
    E,
    S,
    W,
    Unknown,
}

impl BridgeDirection {
    #[must_use]
    pub fn to(c: char) -> BridgeDirection {
        match c {
            'S' | 's' => BridgeDirection::S,
            'N' | 'n' => BridgeDirection::N,
            'E' | 'e' => BridgeDirection::E,
            'W' | 'w' => BridgeDirection::W,
            _ => BridgeDirection::Unknown,
        }
    }

    fn next(&self) -> BridgeDirection {
        match self {
            BridgeDirection::S => BridgeDirection::W,
            BridgeDirection::W => BridgeDirection::N,
            BridgeDirection::N => BridgeDirection::E,
            BridgeDirection::E => BridgeDirection::S,
            BridgeDirection::Unknown => BridgeDirection::Unknown,
        }
    }
}

/// `BridgeBoard` is a French Deck Pack that sorts and validates the hands dealt as a part
/// of a Bridge hand.
#[derive(Clone, Debug, Hash, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct BridgeBoard {
    pack: Pack,
    pub south: Pile,
    pub west: Pile,
    pub north: Pile,
    pub east: Pile,
}

impl BridgeBoard {
    /// Parses a Portable Bridge Notation deal string and converts it into a
    /// `BridgeBoard` struct.
    ///
    /// # Panics
    ///
    /// Will panic if an invalid PBN deal string is passed in.
    #[must_use]
    pub fn from_pbn_deal(deal: &str) -> Self {
        let (direction, pbn) = BridgeBoard::split_on_direction(deal);

        let mut dir_iter = pbn.split_whitespace();

        let mut board = BridgeBoard::default();
        board.fold_in(&direction, board.to_pile(dir_iter.next().unwrap()));
        board.fold_in(&direction.next(), board.to_pile(dir_iter.next().unwrap()));
        board.fold_in(
            &direction.next().next(),
            board.to_pile(dir_iter.next().unwrap()),
        );
        board.fold_in(
            &direction.next().next().next(),
            board.to_pile(dir_iter.next().unwrap()),
        );

        board
    }

    fn fold_in(&mut self, direction: &BridgeDirection, hand: Pile) {
        match direction {
            BridgeDirection::S => self.south = hand.sort(),
            BridgeDirection::W => self.west = hand.sort(),
            BridgeDirection::N => self.north = hand.sort(),
            BridgeDirection::E => self.east = hand.sort(),
            BridgeDirection::Unknown => self.east = hand,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn deal() -> BridgeBoard {
        let mut board = BridgeBoard::default();
        let mut cards = board.pack.cards().shuffle();
        board.south = cards.draw(13).unwrap().sort();
        board.west = cards.draw(13).unwrap().sort();
        board.north = cards.draw(13).unwrap().sort();
        board.east = cards.draw(13).unwrap().sort();
        board
    }

    pub fn demo(&self) {
        println!("S: {}", self.south.sort().to_symbol_index());
        println!("W: {}", self.west.sort().to_symbol_index());
        println!("N: {}", self.north.sort().to_symbol_index());
        println!("E: {}", self.east.sort().to_symbol_index());
    }

    pub fn demo_compass(&self) {
        let north = self.north.short_suit_indexes_to_string();
        let west = self.west.short_suit_indexes_to_string();
        let east = self.east.short_suit_indexes_to_string();
        let south = self.south.short_suit_indexes_to_string();

        println!(
            "{}",
            BridgeBoard::compass(
                BridgeBoard::compass_cell("NORTH", north.as_str()),
                BridgeBoard::compass_cell("WEST", west.as_str()),
                BridgeBoard::compass_cell("EAST", east.as_str()),
                BridgeBoard::compass_cell("SOUTH", south.as_str()),
            )
        );
    }

    fn compass_cell(direction: &str, index: &str) -> String {
        let contents = "   ".to_owned() + direction + "\n" + index;

        let mut table = Table::new();
        table.has_top_boarder = false;
        table.has_bottom_boarder = false;
        table.separate_rows = false;
        table.style = TableStyle::empty();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            contents,
            2,
            Alignment::Left,
        )]));
        table.render()
    }

    fn compass(north: String, west: String, east: String, south: String) -> String {
        let mut table = Table::new();
        table.has_top_boarder = false;
        table.has_bottom_boarder = false;
        table.separate_rows = false;
        table.style = TableStyle::empty();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            north,
            2,
            Alignment::Center,
        )]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(west, 1, Alignment::Left),
            TableCell::new_with_alignment(east, 1, Alignment::Left),
        ]));
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            south,
            2,
            Alignment::Center,
        )]));
        table.render()
    }

    #[must_use]
    pub fn get_pack(&self) -> &Pack {
        &self.pack
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        let piles = &[
            self.south.clone(),
            self.west.clone(),
            self.north.clone(),
            self.east.clone(),
        ];
        let pile = Pile::pile_on(piles.to_vec());
        self.pack.is_complete(&[pile])
    }

    /// Returns a Portable Bridge Notation deal string from a Bridge Board.
    #[must_use]
    pub fn to_pbn_deal(&self) -> String {
        let south = BridgeBoard::hand_to_pbn_deal_segment(&self.south);
        let west = BridgeBoard::hand_to_pbn_deal_segment(&self.west);
        let north = BridgeBoard::hand_to_pbn_deal_segment(&self.north);
        let east = BridgeBoard::hand_to_pbn_deal_segment(&self.east);
        format!("S:{} {} {} {}", south, west, north, east)
    }

    fn hand_to_pbn_deal_segment(hand: &Pile) -> String {
        let mappie = hand.map_by_suit();
        let spades = BridgeBoard::get_suit_string(&Suit::new(SPADES), &mappie);
        let hearts = BridgeBoard::get_suit_string(&Suit::new(HEARTS), &mappie);
        let diamonds = BridgeBoard::get_suit_string(&Suit::new(DIAMONDS), &mappie);
        let clubs = BridgeBoard::get_suit_string(&Suit::new(CLUBS), &mappie);

        format!("{}.{}.{}.{}", spades, hearts, diamonds, clubs)
    }

    fn get_suit_string(suit: &Suit, mappie: &HashMap<Suit, Pile>) -> String {
        let indexes = mappie.get(suit);
        match indexes {
            Some(hand) => hand.rank_indexes(),
            None => String::new(),
        }
    }

    fn to_pile(&self, s: &str) -> Pile {
        let rawsuits: Vec<&str> = s.split('.').collect();

        let mut v: Vec<String> = Vec::new();
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.first().unwrap(),
            'S',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(1).unwrap(),
            'H',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(2).unwrap(),
            'D',
        ));
        v.append(&mut BridgeBoard::splice_suit_in(
            rawsuits.get(3).unwrap(),
            'C',
        ));

        let coll: Vec<Card> = v
            .iter()
            .map(|s| self.pack.cards().card_by_index(s.as_str()).unwrap().clone())
            .collect();

        Pile::from_vector(coll)
    }

    fn splice_suit_in(s: &str, suit: char) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();

        for c in s.chars() {
            v.push(format!("{}{}", c, suit));
        }
        v
    }

    fn split_on_direction(deal: &str) -> (BridgeDirection, &str) {
        let direction = BridgeDirection::to(deal.chars().next().unwrap());
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

impl fmt::Display for BridgeBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = self.to_pbn_deal();
        write!(f, "{}", sig)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bridge_tests {
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

        assert_eq!(south.unwrap().to_index(), deal.south.to_index());
        assert_eq!(west.unwrap().to_index(), deal.west.to_index());
        assert_eq!(north.unwrap().to_index(), deal.north.to_index());
        assert_eq!(east.unwrap().to_index(), deal.east.to_index());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_pbn_deal__unsorted() {
        let unsorted = "S:4Q2.5Q2.Q94T3A.Q 79.AT93.562.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

        let sorted = BridgeBoard::from_pbn_deal(unsorted).to_pbn_deal();

        assert_eq!(PBN_TEST_STRING, sorted.as_str());
    }

    #[test]
    fn from_pbn_deal__west() {
        let deck = Pile::french_deck();
        let pbn = "W:A94.K2.T876.QT53 Q75.AQJT976.9.42 KT62.3.AK2.AK986 J83.854.QJ543.J7";
        let west = deck.pile_by_index(&[
            "AS", "9S", "4S", "KH", "2H", "TD", "8D", "7D", "6D", "QC", "TC", "5C", "3C",
        ]);

        let deal = BridgeBoard::from_pbn_deal(pbn);

        assert_eq!(west.unwrap().to_index(), deal.west.to_index());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_to_pbn_deal() {
        let bb = BridgeBoard::deal();

        let pbn = bb.to_pbn_deal();
        let from = BridgeBoard::from_pbn_deal(pbn.as_str()).to_pbn_deal();

        assert_eq!(pbn, from)
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

        assert_eq!(BridgeDirection::S, char);
        assert_eq!(expected_remainder, remainder);
    }

    #[test]
    fn to_pbn_deal() {
        assert_eq!(
            PBN_TEST_STRING.to_string(),
            BridgeBoard::from_pbn_deal(PBN_TEST_STRING).to_pbn_deal()
        )
    }

    #[test]
    fn hand_to_pbn_deal_segment() {
        let deal = BridgeBoard::from_pbn_deal(PBN_TEST_STRING);
        let hand = deal
            .pack
            .cards()
            .pile_by_index(&[
                "QS", "4S", "2S", "QH", "5H", "2H", "AD", "QD", "TD", "9D", "4D", "3D", "QC",
            ])
            .unwrap();
        let expected = "Q42.Q52.AQT943.Q";

        let actual = BridgeBoard::hand_to_pbn_deal_segment(&hand);

        assert_eq!(expected, actual);
    }

    #[test]
    fn hand_to_pbn_deal_segment__unbalanced() {
        let all_spades = Pile::french_deck().draw(13).unwrap();
        let expected = "AKQJT98765432...";

        let actual = BridgeBoard::hand_to_pbn_deal_segment(&all_spades);

        assert_eq!(expected, actual);
    }

    #[test]
    fn to() {
        assert_eq!(BridgeDirection::S, BridgeDirection::to('S'));
        assert_eq!(BridgeDirection::S, BridgeDirection::to('s'));
        assert_eq!(BridgeDirection::E, BridgeDirection::to('E'));
        assert_eq!(BridgeDirection::E, BridgeDirection::to('e'));
        assert_eq!(BridgeDirection::N, BridgeDirection::to('N'));
        assert_eq!(BridgeDirection::N, BridgeDirection::to('n'));
        assert_eq!(BridgeDirection::W, BridgeDirection::to('W'));
        assert_eq!(BridgeDirection::W, BridgeDirection::to('w'));
        assert_eq!(BridgeDirection::Unknown, BridgeDirection::to(' '));
    }

    #[test]
    fn next() {
        assert_eq!(
            BridgeDirection::W,
            BridgeDirection::next(&BridgeDirection::S)
        );
        assert_eq!(
            BridgeDirection::N,
            BridgeDirection::next(&BridgeDirection::W)
        );
        assert_eq!(
            BridgeDirection::E,
            BridgeDirection::next(&BridgeDirection::N)
        );
        assert_eq!(
            BridgeDirection::S,
            BridgeDirection::next(&BridgeDirection::E)
        );
        assert_eq!(
            BridgeDirection::Unknown,
            BridgeDirection::next(&BridgeDirection::Unknown)
        );
    }
}
