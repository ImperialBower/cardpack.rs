pub mod basic {
    use crate::funky::types::buffoon_card::BuffoonCard;

    pub struct Deck {}

    impl Deck {
        pub const DECK_SIZE: usize = 52;

        pub const DECK: [BuffoonCard; Deck::DECK_SIZE] = [
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
    }

    pub mod card {
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

pub mod tarot {
    use crate::funky::types::buffoon_card::BuffoonCard;

    pub struct MajorArcana {}

    impl MajorArcana {
        pub const DECK_SIZE: usize = 22;
        pub const DECK: [BuffoonCard; MajorArcana::DECK_SIZE] = [
            card::FOOL,
            card::MAGICIAN,
            card::HIGH_PRIESTESS,
            card::EMPRESS,
            card::EMPEROR,
            card::HIEROPHANT,
            card::LOVERS,
            card::THE_CHARIOT,
            card::STRENGTH,
            card::HERMIT,
            card::WHEEL_OF_FORTUNE,
            card::JUSTICE,
            card::HANGED_MAN,
            card::DEATH,
            card::TEMPERANCE,
            card::DEVIL,
            card::TOWER,
            card::STAR,
            card::MOON,
            card::SUN,
            card::JUDGEMENT,
            card::WORLD,
        ];
    }

    pub mod card {
        use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
        use crate::funky::types::mpip::MPip;
        use crate::prelude::{TarotRank, TarotSuit};

        pub const FOOL: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::FOOL,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const MAGICIAN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::MAGICIAN,
            card_type: BCardType::Tarot,
            enhancement: MPip::LUCKY,
        };
        pub const HIGH_PRIESTESS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HIGH_PRIESTESS,
            card_type: BCardType::Tarot,
            enhancement: MPip::PLANET,
        };
        pub const EMPRESS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::EMPRESS,
            card_type: BCardType::Tarot,
            enhancement: MPip::MOD_MULT_PLUS4,
        };
        pub const EMPEROR: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::EMPEROR,
            card_type: BCardType::Tarot,
            enhancement: MPip::RANDOM_TAROT,
        };
        pub const HIEROPHANT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HIEROPHANT,
            card_type: BCardType::Tarot,
            enhancement: MPip::BONUS,
        };
        pub const LOVERS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::LOVERS,
            card_type: BCardType::Tarot,
            enhancement: MPip::WILD_SUIT,
        };
        pub const THE_CHARIOT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::CHARIOT,
            card_type: BCardType::Tarot,
            enhancement: MPip::STEEL,
        };
        pub const STRENGTH: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::STRENGTH,
            card_type: BCardType::Tarot,
            enhancement: MPip::STRENGTH,
        };
        pub const HERMIT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HERMIT,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const WHEEL_OF_FORTUNE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::WHEEL_OF_FORTUNE,
            card_type: BCardType::Tarot,
            enhancement: MPip::WHEEL_OF_FORTUNE,
        };
        pub const JUSTICE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::JUSTICE,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const HANGED_MAN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HANGED_MAN,
            card_type: BCardType::Tarot,
            enhancement: MPip::HANGED,
        };
        pub const DEATH: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::DEATH,
            card_type: BCardType::Tarot,
            enhancement: MPip::DEATH,
        };
        pub const TEMPERANCE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::TEMPERANCE,
            card_type: BCardType::Tarot,
            enhancement: MPip::TEMPERANCE,
        };
        pub const DEVIL: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::DEVIL,
            card_type: BCardType::Tarot,
            enhancement: MPip::DEVIL,
        };
        pub const TOWER: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::TOWER,
            card_type: BCardType::Tarot,
            enhancement: MPip::TOWER,
        };
        pub const STAR: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::STAR,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const MOON: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::MOON,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const SUN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::SUN,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const JUDGEMENT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::JUDGEMENT,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
        pub const WORLD: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::WORLD,
            card_type: BCardType::Tarot,
            enhancement: MPip::BLANK,
        };
    }
}
