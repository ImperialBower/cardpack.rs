use crate::funky::types::buffoon_card::BuffoonCard;
use crate::preludes::funky::BuffoonPile;

// https://www.reddit.com/r/balatro/comments/1b6lito/base_sell_value_calculation/
pub struct Deck {}

impl Deck {
    #[must_use]
    pub fn basic_buffoon_pile() -> BuffoonPile {
        BuffoonPile::from(Self::DECK.to_vec())
    }

    pub const DECK_SIZE: usize = 52;

    pub const DECK: [BuffoonCard; Self::DECK_SIZE] = [
        card::ACE_SPADES,
        card::KING_SPADES,
        card::QUEEN_SPADES,
        card::JACK_SPADES,
        card::TEN_SPADES,
        card::NINE_SPADES,
        card::EIGHT_SPADES,
        card::SEVEN_SPADES,
        card::SIX_SPADES,
        card::FIVE_SPADES,
        card::FOUR_SPADES,
        card::TREY_SPADES,
        card::DEUCE_SPADES,
        card::ACE_HEARTS,
        card::KING_HEARTS,
        card::QUEEN_HEARTS,
        card::JACK_HEARTS,
        card::TEN_HEARTS,
        card::NINE_HEARTS,
        card::EIGHT_HEARTS,
        card::SEVEN_HEARTS,
        card::SIX_HEARTS,
        card::FIVE_HEARTS,
        card::FOUR_HEARTS,
        card::TREY_HEARTS,
        card::DEUCE_HEARTS,
        card::ACE_DIAMONDS,
        card::KING_DIAMONDS,
        card::QUEEN_DIAMONDS,
        card::JACK_DIAMONDS,
        card::TEN_DIAMONDS,
        card::NINE_DIAMONDS,
        card::EIGHT_DIAMONDS,
        card::SEVEN_DIAMONDS,
        card::SIX_DIAMONDS,
        card::FIVE_DIAMONDS,
        card::FOUR_DIAMONDS,
        card::TREY_DIAMONDS,
        card::DEUCE_DIAMONDS,
        card::ACE_CLUBS,
        card::KING_CLUBS,
        card::QUEEN_CLUBS,
        card::JACK_CLUBS,
        card::TEN_CLUBS,
        card::NINE_CLUBS,
        card::EIGHT_CLUBS,
        card::SEVEN_CLUBS,
        card::SIX_CLUBS,
        card::FIVE_CLUBS,
        card::FOUR_CLUBS,
        card::TREY_CLUBS,
        card::DEUCE_CLUBS,
    ];

    pub const ABANDONED_DECK_SIZE: usize = 40;

    pub const ABANDONED_DECK: [BuffoonCard; Self::ABANDONED_DECK_SIZE] = [
        card::ACE_SPADES,
        card::TEN_SPADES,
        card::NINE_SPADES,
        card::EIGHT_SPADES,
        card::SEVEN_SPADES,
        card::SIX_SPADES,
        card::FIVE_SPADES,
        card::FOUR_SPADES,
        card::TREY_SPADES,
        card::DEUCE_SPADES,
        card::ACE_HEARTS,
        card::TEN_HEARTS,
        card::NINE_HEARTS,
        card::EIGHT_HEARTS,
        card::SEVEN_HEARTS,
        card::SIX_HEARTS,
        card::FIVE_HEARTS,
        card::FOUR_HEARTS,
        card::TREY_HEARTS,
        card::DEUCE_HEARTS,
        card::ACE_DIAMONDS,
        card::TEN_DIAMONDS,
        card::NINE_DIAMONDS,
        card::EIGHT_DIAMONDS,
        card::SEVEN_DIAMONDS,
        card::SIX_DIAMONDS,
        card::FIVE_DIAMONDS,
        card::FOUR_DIAMONDS,
        card::TREY_DIAMONDS,
        card::DEUCE_DIAMONDS,
        card::ACE_CLUBS,
        card::TEN_CLUBS,
        card::NINE_CLUBS,
        card::EIGHT_CLUBS,
        card::SEVEN_CLUBS,
        card::SIX_CLUBS,
        card::FIVE_CLUBS,
        card::FOUR_CLUBS,
        card::TREY_CLUBS,
        card::DEUCE_CLUBS,
    ];

    pub const CHECKERED_DECK_SIZE: usize = 52;

    pub const CHECKERED_DECK: [BuffoonCard; Self::CHECKERED_DECK_SIZE] = [
        card::ACE_SPADES,
        card::ACE_SPADES,
        card::KING_SPADES,
        card::KING_SPADES,
        card::QUEEN_SPADES,
        card::QUEEN_SPADES,
        card::JACK_SPADES,
        card::JACK_SPADES,
        card::TEN_SPADES,
        card::TEN_SPADES,
        card::NINE_SPADES,
        card::NINE_SPADES,
        card::EIGHT_SPADES,
        card::EIGHT_SPADES,
        card::SEVEN_SPADES,
        card::SEVEN_SPADES,
        card::SIX_SPADES,
        card::SIX_SPADES,
        card::FIVE_SPADES,
        card::FIVE_SPADES,
        card::FOUR_SPADES,
        card::FOUR_SPADES,
        card::TREY_SPADES,
        card::TREY_SPADES,
        card::DEUCE_SPADES,
        card::DEUCE_SPADES,
        card::ACE_HEARTS,
        card::ACE_HEARTS,
        card::KING_HEARTS,
        card::KING_HEARTS,
        card::QUEEN_HEARTS,
        card::QUEEN_HEARTS,
        card::JACK_HEARTS,
        card::JACK_HEARTS,
        card::TEN_HEARTS,
        card::TEN_HEARTS,
        card::NINE_HEARTS,
        card::NINE_HEARTS,
        card::EIGHT_HEARTS,
        card::EIGHT_HEARTS,
        card::SEVEN_HEARTS,
        card::SEVEN_HEARTS,
        card::SIX_HEARTS,
        card::SIX_HEARTS,
        card::FIVE_HEARTS,
        card::FIVE_HEARTS,
        card::FOUR_HEARTS,
        card::FOUR_HEARTS,
        card::TREY_HEARTS,
        card::TREY_HEARTS,
        card::DEUCE_HEARTS,
        card::DEUCE_HEARTS,
    ];
}

pub mod card {
    use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
    use crate::funky::types::mpip::MPip;
    use crate::prelude::{FrenchRank, FrenchSuit, Pip};

    #[must_use]
    pub fn plus_rank(basic_card: BuffoonCard) -> BuffoonCard {
        let rank = match basic_card.rank.weight {
            12 => FrenchRank::DEUCE,
            11 => FrenchRank::ACE,
            10 => FrenchRank::KING,
            9 => FrenchRank::QUEEN,
            8 => FrenchRank::JACK,
            7 => FrenchRank::TEN,
            6 => FrenchRank::NINE,
            5 => FrenchRank::EIGHT,
            4 => FrenchRank::SEVEN,
            3 => FrenchRank::SIX,
            2 => FrenchRank::FIVE,
            1 => FrenchRank::FOUR,
            0 => FrenchRank::TREY,
            _ => basic_card.rank,
        };
        BuffoonCard { rank, ..basic_card }
    }

    #[must_use]
    pub fn set_suit(basic_card: BuffoonCard, suit: Pip) -> BuffoonCard {
        BuffoonCard { suit, ..basic_card }
    }

    pub const ACE_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::ACE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const KING_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::KING,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const QUEEN_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::QUEEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const JACK_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::JACK,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TEN_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::TEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const NINE_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::NINE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const EIGHT_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::EIGHT,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SEVEN_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::SEVEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SIX_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::SIX,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FIVE_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::FIVE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FOUR_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::FOUR,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TREY_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::TREY,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const DEUCE_SPADES: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::SPADES,
        rank: FrenchRank::DEUCE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const ACE_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::ACE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const KING_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::KING,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const QUEEN_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::QUEEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const JACK_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::JACK,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TEN_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::TEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const NINE_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::NINE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const EIGHT_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::EIGHT,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SEVEN_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::SEVEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SIX_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::SIX,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FIVE_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::FIVE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FOUR_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::FOUR,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TREY_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::TREY,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const DEUCE_HEARTS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::HEARTS,
        rank: FrenchRank::DEUCE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const ACE_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::ACE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const KING_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::KING,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const QUEEN_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::QUEEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const JACK_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::JACK,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TEN_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::TEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const NINE_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::NINE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const EIGHT_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::EIGHT,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SEVEN_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::SEVEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SIX_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::SIX,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FIVE_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::FIVE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FOUR_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::FOUR,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TREY_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::TREY,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const DEUCE_DIAMONDS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::DIAMONDS,
        rank: FrenchRank::DEUCE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const ACE_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::ACE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const KING_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::KING,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const QUEEN_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::QUEEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const JACK_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::JACK,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TEN_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::TEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const NINE_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::NINE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const EIGHT_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::EIGHT,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SEVEN_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::SEVEN,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const SIX_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::SIX,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FIVE_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::FIVE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const FOUR_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::FOUR,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const TREY_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::TREY,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
    pub const DEUCE_CLUBS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::CLUBS,
        rank: FrenchRank::DEUCE,
        card_type: BCardType::Basic,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };
}
