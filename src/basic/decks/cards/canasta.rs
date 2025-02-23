use crate::basic::decks::cards::french::FrenchRank;
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

pub struct CanastaBasicCard;
pub struct CanastaSuit;
pub struct CanastaRank;

pub const FLUENT_KEY_BASE_NAME_CANASTA: &str = "canasta";

/// Use the `Canasta` `Deck` as a way to illustrate system evolution.
impl CanastaBasicCard {
    pub const TREY_HEARTS: BasicCard = BasicCard {
        suit: CanastaSuit::TREY_HEARTS,
        rank: CanastaRank::RED_TREY,
    };
    pub const TREY_DIAMONDS: BasicCard = BasicCard {
        suit: CanastaSuit::TREY_DIAMONDS,
        rank: CanastaRank::RED_TREY,
    };
    pub const BIG_JOKER: BasicCard = BasicCard {
        suit: CanastaSuit::JOKER,
        rank: FrenchRank::BIG_JOKER,
    };
    pub const LITTLE_JOKER: BasicCard = BasicCard {
        suit: CanastaSuit::JOKER,
        rank: FrenchRank::LITTLE_JOKER,
    };
    pub const DEUCE_SPADES: BasicCard = BasicCard {
        suit: CanastaSuit::DEUCE_SPADES,
        rank: CanastaRank::DEUCE,
    };
    pub const DEUCE_HEARTS: BasicCard = BasicCard {
        suit: CanastaSuit::DEUCE_HEARTS,
        rank: CanastaRank::DEUCE,
    };
    pub const DEUCE_DIAMONDS: BasicCard = BasicCard {
        suit: CanastaSuit::DEUCE_DIAMONDS,
        rank: CanastaRank::DEUCE,
    };
    pub const DEUCE_CLUBS: BasicCard = BasicCard {
        suit: CanastaSuit::DEUCE_CLUBS,
        rank: CanastaRank::DEUCE,
    };
}

impl CanastaRank {
    pub const RED_TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 15,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 13,
        index: '2',
        symbol: '2',
        value: 2,
    };
}

impl CanastaSuit {
    pub const TREY_HEARTS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 11,
        index: 'H',
        symbol: '♥',
        value: 1,
    };
    pub const TREY_DIAMONDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 10,
        index: 'D',
        symbol: '♦',
        value: 2,
    };
    pub const JOKER: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 9,
        index: 'J',
        symbol: '🃟',
        value: 4,
    };
    pub const DEUCE_SPADES: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 8,
        index: 'S',
        symbol: '♠',
        value: 3,
    };
    pub const DEUCE_HEARTS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 7,
        index: 'H',
        symbol: '♥',
        value: 1,
    };
    pub const DEUCE_DIAMONDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 6,
        index: 'D',
        symbol: '♦',
        value: 2,
    };
    pub const DEUCE_CLUBS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 5,
        index: 'C',
        symbol: '♣',
        value: 4,
    };
}
