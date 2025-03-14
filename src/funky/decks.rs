pub mod basic {
    use crate::funky::types::buffoon_card::BuffoonCard;

    pub struct Deck {}

    impl Deck {
        pub const DECK_SIZE: usize = 52;

        pub const DECK: [BuffoonCard; Deck::DECK_SIZE] = [
            cards::ACE_SPADES,
            cards::KING_SPADES,
            cards::QUEEN_SPADES,
            cards::JACK_SPADES,
            cards::TEN_SPADES,
            cards::NINE_SPADES,
            cards::EIGHT_SPADES,
            cards::SEVEN_SPADES,
            cards::SIX_SPADES,
            cards::FIVE_SPADES,
            cards::FOUR_SPADES,
            cards::TREY_SPADES,
            cards::DEUCE_SPADES,
            cards::ACE_HEARTS,
            cards::KING_HEARTS,
            cards::QUEEN_HEARTS,
            cards::JACK_HEARTS,
            cards::TEN_HEARTS,
            cards::NINE_HEARTS,
            cards::EIGHT_HEARTS,
            cards::SEVEN_HEARTS,
            cards::SIX_HEARTS,
            cards::FIVE_HEARTS,
            cards::FOUR_HEARTS,
            cards::TREY_HEARTS,
            cards::DEUCE_HEARTS,
            cards::ACE_DIAMONDS,
            cards::KING_DIAMONDS,
            cards::QUEEN_DIAMONDS,
            cards::JACK_DIAMONDS,
            cards::TEN_DIAMONDS,
            cards::NINE_DIAMONDS,
            cards::EIGHT_DIAMONDS,
            cards::SEVEN_DIAMONDS,
            cards::SIX_DIAMONDS,
            cards::FIVE_DIAMONDS,
            cards::FOUR_DIAMONDS,
            cards::TREY_DIAMONDS,
            cards::DEUCE_DIAMONDS,
            cards::ACE_CLUBS,
            cards::KING_CLUBS,
            cards::QUEEN_CLUBS,
            cards::JACK_CLUBS,
            cards::TEN_CLUBS,
            cards::NINE_CLUBS,
            cards::EIGHT_CLUBS,
            cards::SEVEN_CLUBS,
            cards::SIX_CLUBS,
            cards::FIVE_CLUBS,
            cards::FOUR_CLUBS,
            cards::TREY_CLUBS,
            cards::DEUCE_CLUBS,
        ];
    }

    pub mod cards {
        use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
        use crate::funky::types::mpip::MPip;
        use crate::prelude::{FrenchRank, FrenchSuit};

        pub const ACE_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::ACE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const KING_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::KING,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const QUEEN_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::QUEEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const JACK_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::JACK,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TEN_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::TEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const NINE_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::NINE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const EIGHT_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::EIGHT,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SEVEN_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::SEVEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SIX_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::SIX,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FIVE_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::FIVE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FOUR_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::FOUR,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TREY_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::TREY,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const DEUCE_SPADES: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::SPADES,
            rank: FrenchRank::DEUCE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const ACE_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::ACE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const KING_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::KING,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const QUEEN_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::QUEEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const JACK_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::JACK,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TEN_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::TEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const NINE_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::NINE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const EIGHT_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::EIGHT,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SEVEN_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::SEVEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SIX_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::SIX,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FIVE_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::FIVE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FOUR_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::FOUR,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TREY_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::TREY,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const DEUCE_HEARTS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::HEARTS,
            rank: FrenchRank::DEUCE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const ACE_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::ACE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const KING_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::KING,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const QUEEN_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::QUEEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const JACK_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::JACK,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TEN_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::TEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const NINE_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::NINE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const EIGHT_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::EIGHT,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SEVEN_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::SEVEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SIX_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::SIX,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FIVE_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::FIVE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FOUR_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::FOUR,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TREY_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::TREY,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const DEUCE_DIAMONDS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::DIAMONDS,
            rank: FrenchRank::DEUCE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const ACE_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::ACE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const KING_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::KING,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const QUEEN_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::QUEEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const JACK_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::JACK,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TEN_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::TEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const NINE_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::NINE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const EIGHT_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::EIGHT,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SEVEN_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::SEVEN,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const SIX_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::SIX,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FIVE_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::FIVE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const FOUR_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::FOUR,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const TREY_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::TREY,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
        pub const DEUCE_CLUBS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::CLUBS,
            rank: FrenchRank::DEUCE,
            card_type: BCardType::Basic,
            enhancement: MPip::BLANK,
        };
    }
}
