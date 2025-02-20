use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

pub struct SkatBasicCard;
pub struct SkatSuit;
pub struct SkatRank;

pub const FLUENT_KEY_BASE_NAME_SKAT: &str = "skat";

impl SkatBasicCard {
    pub const DAUSE_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::DAUSE,
    };
    pub const ZHEN_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::ZHEN,
    };
    pub const KÖNIG_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::KÖNIG,
    };
    pub const OBER_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::OBER,
    };
    pub const UNTER_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::UNTER,
    };
    pub const NEUN_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::NEUN,
    };
    pub const ACHT_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::ACHT,
    };
    pub const SIEBEN_EICHEL: BasicCard = BasicCard {
        suit: SkatSuit::EICHEL,
        rank: SkatRank::SIEBEN,
    };

    pub const DAUSE_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::DAUSE,
    };
    pub const ZHEN_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::ZHEN,
    };
    pub const KÖNIG_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::KÖNIG,
    };
    pub const OBER_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::OBER,
    };
    pub const UNTER_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::UNTER,
    };
    pub const NEUN_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::NEUN,
    };
    pub const ACHT_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::ACHT,
    };
    pub const SIEBEN_LAUB: BasicCard = BasicCard {
        suit: SkatSuit::LAUB,
        rank: SkatRank::SIEBEN,
    };

    pub const DAUSE_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::DAUSE,
    };
    pub const ZHEN_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::ZHEN,
    };
    pub const KÖNIG_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::KÖNIG,
    };
    pub const OBER_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::OBER,
    };
    pub const UNTER_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::UNTER,
    };
    pub const NEUN_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::NEUN,
    };
    pub const ACHT_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::ACHT,
    };
    pub const SIEBEN_HERZ: BasicCard = BasicCard {
        suit: SkatSuit::HERZ,
        rank: SkatRank::SIEBEN,
    };

    pub const DAUSE_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::DAUSE,
    };
    pub const ZHEN_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::ZHEN,
    };
    pub const KÖNIG_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::KÖNIG,
    };
    pub const OBER_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::OBER,
    };
    pub const UNTER_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::UNTER,
    };
    pub const NEUN_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::NEUN,
    };
    pub const ACHT_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::ACHT,
    };
    pub const SIEBEN_SHELLEN: BasicCard = BasicCard {
        suit: SkatSuit::SHELLEN,
        rank: SkatRank::SIEBEN,
    };
}

impl SkatSuit {
    pub const EICHEL: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'E',
        symbol: '♣',
        value: 4,
    };
    pub const LAUB: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'L',
        symbol: '♠',
        value: 3,
    };
    pub const HERZ: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'H',
        symbol: '♥',
        value: 2,
    };
    pub const SHELLEN: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'S',
        symbol: '♦',
        value: 1,
    };
}

impl SkatRank {
    pub const DAUSE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: 'D',
        symbol: 'D',
        value: 0,
    };
    pub const ZHEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: 'Z',
        symbol: 'Z',
        value: 0,
    };
    pub const KÖNIG: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: 'K',
        symbol: 'K',
        value: 0,
    };
    pub const OBER: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: 'O',
        symbol: 'O',
        value: 0,
    };
    pub const UNTER: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: 'U',
        symbol: 'U',
        value: 0,
    };
    pub const NEUN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '9',
        symbol: '9',
        value: 2,
    };
    pub const ACHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '8',
        symbol: '8',
        value: 0,
    };
    pub const SIEBEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: '7',
        symbol: '7',
        value: 0,
    };
}
