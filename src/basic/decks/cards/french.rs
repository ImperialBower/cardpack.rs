use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

pub struct FrenchBasicCard;
pub struct FrenchSuit;
pub struct FrenchRank;

pub const FLUENT_KEY_BASE_NAME_FRENCH: &str = "french";

impl FrenchBasicCard {
    pub const BIG_JOKER: BasicCard = BasicCard {
        suit: FrenchSuit::JOKER,
        rank: FrenchRank::BIG_JOKER,
    };
    pub const LITTLE_JOKER: BasicCard = BasicCard {
        suit: FrenchSuit::JOKER,
        rank: FrenchRank::LITTLE_JOKER,
    };
    pub const ACE_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::ACE,
    };
    pub const KING_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::KING,
    };
    pub const QUEEN_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::QUEEN,
    };
    pub const JACK_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::JACK,
    };
    pub const TEN_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::TEN,
    };
    pub const NINE_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::NINE,
    };
    pub const EIGHT_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::EIGHT,
    };
    pub const SEVEN_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::SEVEN,
    };
    pub const SIX_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::SIX,
    };
    pub const FIVE_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::FIVE,
    };
    pub const FOUR_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::FOUR,
    };
    pub const TREY_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::TREY,
    };
    pub const DEUCE_SPADES: BasicCard = BasicCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::DEUCE,
    };
    pub const ACE_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::ACE,
    };
    pub const KING_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::KING,
    };
    pub const QUEEN_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::QUEEN,
    };
    pub const JACK_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::JACK,
    };
    pub const TEN_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::TEN,
    };
    pub const NINE_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::NINE,
    };
    pub const EIGHT_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::EIGHT,
    };
    pub const SEVEN_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::SEVEN,
    };
    pub const SIX_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::SIX,
    };
    pub const FIVE_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::FIVE,
    };
    pub const FOUR_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::FOUR,
    };
    pub const TREY_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::TREY,
    };
    pub const DEUCE_HEARTS: BasicCard = BasicCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::DEUCE,
    };
    pub const ACE_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::ACE,
    };
    pub const KING_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::KING,
    };
    pub const QUEEN_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::QUEEN,
    };
    pub const JACK_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::JACK,
    };
    pub const TEN_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::TEN,
    };
    pub const NINE_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::NINE,
    };
    pub const EIGHT_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::EIGHT,
    };
    pub const SEVEN_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::SEVEN,
    };
    pub const SIX_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::SIX,
    };
    pub const FIVE_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::FIVE,
    };
    pub const FOUR_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::FOUR,
    };
    pub const TREY_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::TREY,
    };
    pub const DEUCE_DIAMONDS: BasicCard = BasicCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::DEUCE,
    };
    pub const ACE_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::ACE,
    };
    pub const KING_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::KING,
    };
    pub const QUEEN_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::QUEEN,
    };
    pub const JACK_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::JACK,
    };
    pub const TEN_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::TEN,
    };
    pub const NINE_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::NINE,
    };
    pub const EIGHT_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::EIGHT,
    };
    pub const SEVEN_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::SEVEN,
    };
    pub const SIX_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::SIX,
    };
    pub const FIVE_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::FIVE,
    };
    pub const FOUR_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::FOUR,
    };
    pub const TREY_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::TREY,
    };
    pub const DEUCE_CLUBS: BasicCard = BasicCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::DEUCE,
    };
}

impl FrenchSuit {
    pub const JOKER: Pip = Pip {
        pip_type: PipType::Joker,
        weight: 4,
        index: 'J',
        symbol: '🃟',
        value: 5,
    };
    pub const SPADES: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'S',
        symbol: '♠',
        value: 4,
    };
    pub const HEARTS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'H',
        symbol: '♥',
        value: 3,
    };
    pub const DIAMONDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'D',
        symbol: '♦',
        value: 2,
    };
    pub const CLUBS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'C',
        symbol: '♣',
        value: 1,
    };
}

impl FrenchRank {
    pub const BIG_JOKER: Pip = Pip {
        pip_type: PipType::Joker,
        weight: 14,
        index: 'B',
        symbol: 'B',
        value: 13,
    };
    pub const LITTLE_JOKER: Pip = Pip {
        pip_type: PipType::Joker,
        weight: 13,
        index: 'L',
        symbol: 'L',
        value: 12,
    };

    pub const ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 12,
        index: 'A',
        symbol: 'A',
        value: 11,
    };
    pub const KING: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'K',
        symbol: 'K',
        value: 10,
    };
    pub const QUEEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'Q',
        symbol: 'Q',
        value: 10,
    };
    pub const JACK: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'J',
        symbol: 'J',
        value: 10,
    };
    pub const TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: 'T',
        symbol: 'T',
        value: 10,
    };
    pub const NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: '2',
        symbol: '2',
        value: 2,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod basic__decks__cards__french__tests {
    use super::*;
    use crate::prelude::{Decked, French};

    #[test]
    fn serde() {
        let pips = vec![FrenchRank::ACE];
        let yml = serde_norway::to_string(&pips).unwrap();

        let pip2: Vec<Pip> = serde_norway::from_str(&yml).unwrap();
        assert_eq!(pips, pip2);
    }

    #[test]
    fn serde__deck() {
        let pile = French::deck().into_basic_cards();
        let yml = serde_norway::to_string(&pile).unwrap();

        let from_yml: Vec<BasicCard> = BasicCard::cards_from_yaml_str(&yml).unwrap();

        assert_eq!(pile, from_yml);
    }
}
