use cardpack::prelude::{CardError, Decked, FrenchSuit, Pile, Pip, Standard52};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};
use cardpack::basic::types::traits::Ranged;

fn main() {
    env_logger::init();

    println!("First, let's deal out a random bridge hand.");
    println!();
    let bridge_board = BridgeBoard::deal();
    println!("Here it is in Portable Bridge Notation:\n    {bridge_board}");

    println!();
    println!("How does it look as a traditional compass?");
    let s = BridgeCompass::new(bridge_board);
    println!("{s}");

    println!();
    println!("Now, let's take a PBN Deal String and convert it into a bridge hand.");
    let deal = "S:Q42.Q52.AQT943.Q 97.AT93.652.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";
    println!("Here's the original' Portable Bridge Notation:\n    {deal}");

    let bridge_board = BridgeBoard::from_pbn_deal(deal);

    println!();
    println!("As a bridge compass:");
    println!();
    let s = BridgeCompass::new(bridge_board);
    println!("{s}");
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

    fn random() -> BridgeDirection {
        match rand::random::<u8>() % 3 {
            0 => BridgeDirection::S,
            1 => BridgeDirection::W,
            2 => BridgeDirection::N,
            _ => BridgeDirection::E,
        }
    }
}

impl Display for BridgeDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self {
            BridgeDirection::S => "South",
            BridgeDirection::W => "West",
            BridgeDirection::N => "North",
            BridgeDirection::E => "East",
            BridgeDirection::Unknown => "Unknown",
        };

        write!(f, "{}", dir)
    }
}

/// `BridgeBoard` is a French Deck Pack that sorts and validates the hands dealt as a part
/// of a Bridge hand.
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct BridgeBoard {
    pub dealer: BridgeDirection,
    pub pack: Pile<Standard52>,
    pub south: Pile<Standard52>,
    pub west: Pile<Standard52>,
    pub north: Pile<Standard52>,
    pub east: Pile<Standard52>,
    pub nw_vulnerable: bool,
    pub ew_vulnerable: bool,
}

impl BridgeBoard {
    pub fn deal() -> BridgeBoard {
        let mut cards = Pile::<Standard52>::deck().shuffled();
        let pack = cards.clone();

        let dealer = BridgeDirection::random();
        let south = cards.draw(13).unwrap().sort();
        let west = cards.draw(13).unwrap().sort();
        let north = cards.draw(13).unwrap().sort();
        let east = cards.draw(13).unwrap().sort();

        BridgeBoard {
            dealer,
            pack,
            south,
            west,
            north,
            east,

            nw_vulnerable: false,
            ew_vulnerable: false,
        }
    }

    fn calculate_pbn(
        dealer: BridgeDirection,
        south: &Pile<Standard52>,
        west: &Pile<Standard52>,
        north: &Pile<Standard52>,
        east: &Pile<Standard52>,
    ) -> String {
        match dealer {
            BridgeDirection::N => {
                format!(
                    "N:{} {} {} {}",
                    Self::hand_to_pbn_deal_segment(north),
                    Self::hand_to_pbn_deal_segment(east),
                    Self::hand_to_pbn_deal_segment(south),
                    Self::hand_to_pbn_deal_segment(west)
                )
            }
            BridgeDirection::E => {
                format!(
                    "E:{} {} {} {}",
                    Self::hand_to_pbn_deal_segment(east),
                    Self::hand_to_pbn_deal_segment(south),
                    Self::hand_to_pbn_deal_segment(west),
                    Self::hand_to_pbn_deal_segment(north)
                )
            }
            BridgeDirection::S => {
                format!(
                    "S:{} {} {} {}",
                    Self::hand_to_pbn_deal_segment(south),
                    Self::hand_to_pbn_deal_segment(west),
                    Self::hand_to_pbn_deal_segment(north),
                    Self::hand_to_pbn_deal_segment(east)
                )
            }
            _ => {
                format!(
                    "W:{} {} {} {}",
                    Self::hand_to_pbn_deal_segment(west),
                    Self::hand_to_pbn_deal_segment(north),
                    Self::hand_to_pbn_deal_segment(east),
                    Self::hand_to_pbn_deal_segment(south)
                )
            }
        }
    }

    /// Parses a Portable Bridge Notation deal string and converts it into a
    /// `BridgeBoard` struct.
    ///
    /// # Panics
    ///
    /// Will panic if an invalid PBN deal string is passed in.
    #[must_use]
    pub fn from_pbn_deal(deal: &str) -> Self {
        let (mut direction, pbn) = BridgeBoard::split_on_direction(deal);

        let mut board = BridgeBoard::default();
        board.dealer = direction;

        for s in pbn.split_whitespace() {
            board.fold_in(&direction, board.to_pile(s));
            direction = direction.next();
        }

        board
    }

    pub fn hand_to_pbn_deal_segment(hand: &Pile<Standard52>) -> String {
        let spades = BridgeBoard::get_suit_string(FrenchSuit::SPADES, &hand);
        let hearts = BridgeBoard::get_suit_string(FrenchSuit::HEARTS, &hand);
        let diamonds = BridgeBoard::get_suit_string(FrenchSuit::DIAMONDS, &hand);
        let clubs = BridgeBoard::get_suit_string(FrenchSuit::CLUBS, &hand);

        format!("{spades}.{hearts}.{diamonds}.{clubs}")
    }

    fn get_suit_string(suit: Pip, hand: &Pile<Standard52>) -> String {
        hand
            .ranks_index_by_suit(suit, "")
            .unwrap_or_else(|| String::new())
    }

    /// NOTE: index string is a really horrible name for something used in code. Index has too
    /// many implications.
    pub fn pile_by_index(index: &str) -> Result<Pile<Standard52>, CardError> {
        Pile::<Standard52>::from_str(index)
    }

    pub fn as_pile(&self) -> Pile<Standard52> {
        let mut pile = Pile::<Standard52>::default();
        pile.prepend(&self.south);
        pile.prepend(&self.west);
        pile.prepend(&self.north);
        pile.prepend(&self.east);

        pile
    }

    fn fold_in(&mut self, direction: &BridgeDirection, hand: Pile<Standard52>) {
        match direction {
            BridgeDirection::S => self.south = hand.sort(),
            BridgeDirection::W => self.west = hand.sort(),
            BridgeDirection::N => self.north = hand.sort(),
            BridgeDirection::E => self.east = hand.sort(),
            BridgeDirection::Unknown => self.east = hand,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.as_pile().into_hashset().len() == 52
    }

    fn splice_suit_in(s: &str, suit: char) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();

        for c in s.chars() {
            v.push(format!("{c}{suit}"));
        }
        v
    }

    fn split_on_direction(deal: &str) -> (BridgeDirection, &str) {
        let direction = BridgeDirection::to(deal.chars().next().unwrap());
        let remainder = &deal[2..];

        (direction, remainder)
    }

    fn to_pile(&self, s: &str) -> Pile<Standard52> {
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

        v.iter()
            .map(|s| self.pack.card_by_index(s).unwrap().clone())
            .collect()
    }
}

impl Default for BridgeBoard {
    fn default() -> Self {
        BridgeBoard {
            dealer: BridgeDirection::S,
            pack: Pile::<Standard52>::deck(),
            south: Pile::default(),
            west: Pile::default(),
            north: Pile::default(),
            east: Pile::default(),
            nw_vulnerable: false,
            ew_vulnerable: false,
        }
    }
}

impl Display for BridgeBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig = Self::calculate_pbn(
            self.dealer,
            &self.south,
            &self.west,
            &self.north,
            &self.east,
        );
        write!(f, "{sig}")
    }
}

struct BridgeCompass;

impl BridgeCompass {
    pub fn new(board: BridgeBoard) -> String {
        let north = BridgeCompass::cell_string(board.north);
        let west = BridgeCompass::cell_string(board.west);
        let east = BridgeCompass::cell_string(board.east);
        let south = BridgeCompass::cell_string(board.south);

        format!(
            "{}",
            BridgeCompass::compass(
                BridgeCompass::compass_cell("NORTH", north.as_str()),
                BridgeCompass::compass_cell("WEST", west.as_str()),
                BridgeCompass::compass_cell("EAST", east.as_str()),
                BridgeCompass::compass_cell("SOUTH", south.as_str()),
            )
        )
    }

    fn cell_string(cards: Pile<Standard52>) -> String {
        let mut v = Vec::<String>::new();

        match cards
            .ranks_index_by_suit(FrenchSuit::SPADES, " ")
        {
            Some(index) => {
                v.push(format!("♠ {index}"));
            }
            None => {}
        }
        match cards
            .ranks_index_by_suit(FrenchSuit::HEARTS, " ")
        {
            Some(index) => {
                v.push(format!("♥ {index}"));
            }
            None => {}
        }
        match cards
            .ranks_index_by_suit(FrenchSuit::DIAMONDS, " ")
        {
            Some(index) => {
                v.push(format!("♦ {index}"));
            }
            None => {}
        }
        match cards
            .ranks_index_by_suit(FrenchSuit::CLUBS, " ")
        {
            Some(index) => {
                v.push(format!("♣ {index}"));
            }
            None => {}
        }

        v.join("\n")
    }

    fn compass_cell(direction: &str, index: &str) -> String {
        let contents = "   ".to_owned() + direction + "\n" + index;

        let mut table = Table::new();
        table.has_top_boarder = false;
        table.has_bottom_boarder = false;
        table.separate_rows = false;
        table.style = TableStyle::empty();
        table.add_row(Row::new(vec![
            TableCell::builder(contents)
                .col_span(2)
                .alignment(Alignment::Left)
                .build(),
        ]));
        table.render()
    }

    fn compass(north: String, west: String, east: String, south: String) -> String {
        let mut table = Table::new();
        table.has_top_boarder = false;
        table.has_bottom_boarder = false;
        table.separate_rows = false;
        table.style = TableStyle::empty();
        table.add_row(Row::new(vec![
            TableCell::builder(north)
                .col_span(2)
                .alignment(Alignment::Center)
                .build(),
        ]));

        table.add_row(Row::new(vec![
            TableCell::builder(west)
                .col_span(1)
                .alignment(Alignment::Left)
                .build(),
            TableCell::builder(east)
                .col_span(1)
                .alignment(Alignment::Left)
                .build(),
        ]));
        table.add_row(Row::new(vec![
            TableCell::builder(south)
                .col_span(2)
                .alignment(Alignment::Center)
                .build(),
        ]));
        table.render()
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
        // let deck = French::deck();
        let south = BridgeBoard::pile_by_index("QS 4S 2S QH 5H 2H AD QD TD 9D 4D 3D QC");
        let west = BridgeBoard::pile_by_index("9S 7S AH TH 9H 3H 6D 5D 2D TC 7C 4C 3C");
        let north = BridgeBoard::pile_by_index("AS JS TS 8S 5S JH 7H 6H KD JD AC 6C 5C");
        let east = BridgeBoard::pile_by_index("KS 6S 3S KH 8H 4H 8D 7D KC JC 9C 8C 2C");

        let deal = BridgeBoard::from_pbn_deal(PBN_TEST_STRING);

        println!("{}", deal.pack.index());

        assert_eq!(south.unwrap().index(), deal.south.index());
        assert_eq!(west.unwrap().index(), deal.west.index());
        assert_eq!(north.unwrap().index(), deal.north.index());
        assert_eq!(east.unwrap().index(), deal.east.index());
        // assert!(deal.pack.index())
    }

    #[test]
    fn from_pbn_deal__unsorted() {
        let unsorted = "S:4Q2.5Q2.Q94T3A.Q 79.AT93.562.T743 AJT85.J76.KJ.A65 K63.K84.87.KJ982";

        let sorted = BridgeBoard::from_pbn_deal(unsorted).to_string();

        assert_eq!(PBN_TEST_STRING, sorted.as_str());
    }

    #[test]
    fn from_pbn_deal__west() {
        // let deck = French::deck();
        let pbn = "W:A94.K2.T876.QT53 Q75.AQJT976.9.42 KT62.3.AK2.AK986 J83.854.QJ543.J7";
        let west = BridgeBoard::pile_by_index("AS 9S 4S KH 2H TD 8D 7D 6D QC TC 5C 3C");

        let deal = BridgeBoard::from_pbn_deal(pbn);

        assert_eq!(west.unwrap().index(), deal.west.index());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_pbn_deal__north() {
        // let deck = French::deck();
        let pbn = "N:A94.K2.T876.QT53 Q75.AQJT976.9.42 KT62.3.AK2.AK986 J83.854.QJ543.J7";
        let north = BridgeBoard::pile_by_index("AS 9S 4S KH 2H TD 8D 7D 6D QC TC 5C 3C");

        let deal = BridgeBoard::from_pbn_deal(pbn);

        assert_eq!(north.unwrap().index(), deal.north.index());
        assert_eq!(pbn, deal.to_string());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_pbn_deal__east() {
        // let deck = French::deck();
        let pbn = "E:A94.K2.T876.QT53 Q75.AQJT976.9.42 KT62.3.AK2.AK986 J83.854.QJ543.J7";
        let east = BridgeBoard::pile_by_index("AS 9S 4S KH 2H TD 8D 7D 6D QC TC 5C 3C");

        let deal = BridgeBoard::from_pbn_deal(pbn);

        assert_eq!(east.unwrap().index(), deal.east.index());
        assert_eq!(pbn, deal.to_string());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_pbn_deal__south() {
        // let deck = French::deck();
        let pbn = "S:A94.K2.T876.QT53 Q75.AQJT976.9.42 KT62.3.AK2.AK986 J83.854.QJ543.J7";
        let south = BridgeBoard::pile_by_index("AS 9S 4S KH 2H TD 8D 7D 6D QC TC 5C 3C");

        let deal = BridgeBoard::from_pbn_deal(pbn);

        assert_eq!(south.unwrap().index(), deal.south.index());
        assert_eq!(pbn, deal.to_string());
        assert!(deal.is_valid())
    }

    #[test]
    fn from_to_pbn_deal() {
        let bb = BridgeBoard::deal();

        let pbn = bb.to_string();
        let from = BridgeBoard::from_pbn_deal(pbn.as_str()).to_string();

        assert_eq!(pbn, from)
    }

    #[test]
    fn is_valid() {
        let deck = BridgeBoard::deal();

        assert!(deck.is_valid())
    }

    #[test]
    fn is_valid_ne() {
        let mut deck = BridgeBoard::deal();

        deck.south = deck.south.draw(1).unwrap();

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
    fn hand_to_pbn_deal_segment() {
        let deal = BridgeBoard::from_pbn_deal(PBN_TEST_STRING);
        let expected = "Q42.Q52.AQT943.Q";

        let actual = BridgeBoard::hand_to_pbn_deal_segment(&deal.south);

        assert_eq!(expected, actual);
    }

    #[test]
    fn hand_to_pbn_deal_segment__unbalanced() {
        let all_spades = Pile::<Standard52>::deck().draw(13).unwrap();
        let expected = "AKQJT98765432...";

        let actual = BridgeBoard::hand_to_pbn_deal_segment(&all_spades);

        assert_eq!(expected, actual);
    }

    #[test]
    fn display() {
        let deck = BridgeBoard::from_pbn_deal(PBN_TEST_STRING);

        assert_eq!(deck.to_string(), PBN_TEST_STRING);
    }

    #[test]
    fn bridge_direction__to() {
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
    fn bridge_direction__next() {
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

    /// **HACK**
    #[test]
    fn bridge_direction__random() {
        for _ in 0..100 {
            assert_ne!(BridgeDirection::random(), BridgeDirection::Unknown);
        }
    }
}
