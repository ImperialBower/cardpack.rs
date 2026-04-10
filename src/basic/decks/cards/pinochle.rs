use crate::basic::decks::cards::french::FrenchSuit;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

pub struct PinochleBasicCard;
pub struct PinochleRank;

pub const FLUENT_KEY_BASE_NAME_PINOCHLE: &str = "pinochle";

impl PinochleBasicCard {
    pub const TEN_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: PinochleRank::TEN,
    };
    pub const KING_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: PinochleRank::KING,
    };
    pub const QUEEN_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: PinochleRank::QUEEN,
    };
    pub const JACK_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: PinochleRank::JACK,
    };
    pub const TEN_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: PinochleRank::TEN,
    };
    pub const KING_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: PinochleRank::KING,
    };
    pub const QUEEN_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: PinochleRank::QUEEN,
    };
    pub const JACK_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: PinochleRank::JACK,
    };
    pub const TEN_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: PinochleRank::TEN,
    };
    pub const KING_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: PinochleRank::KING,
    };
    pub const QUEEN_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: PinochleRank::QUEEN,
    };
    pub const JACK_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: PinochleRank::JACK,
    };
    pub const TEN_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: PinochleRank::TEN,
    };
    pub const KING_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: PinochleRank::KING,
    };
    pub const QUEEN_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: PinochleRank::QUEEN,
    };
    pub const JACK_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: PinochleRank::JACK,
    };
}

impl PinochleRank {
    pub const TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'T',
        symbol: 'T',
        value: 10,
    };
    pub const KING: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'K',
        symbol: 'K',
        value: 10,
    };
    pub const QUEEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'Q',
        symbol: 'Q',
        value: 10,
    };
    pub const JACK: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: 'J',
        symbol: 'J',
        value: 10,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__decks__cards__french__tests {
    use super::*;

    #[test]
    fn serde() {
        let pips = vec![PinochleRank::KING];
        let yml = serde_norway::to_string(&pips).unwrap();

        // println!("{yml}");

        let pip2: Vec<Pip> = serde_norway::from_str(&yml).unwrap();
        assert_eq!(pips, pip2);
    }
}
