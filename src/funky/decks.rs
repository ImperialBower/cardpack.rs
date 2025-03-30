pub mod basic {
    use crate::funky::types::buffoon_card::BuffoonCard;

    // https://www.reddit.com/r/balatro/comments/1b6lito/base_sell_value_calculation/
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
        use crate::prelude::{PipType, TarotRank, TarotSuit};

        pub const FOOL: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::FOOL,
            card_type: BCardType::Tarot,
            enhancement: MPip::Blank,
            resell_value: 1,
            debuffed: false,
        };
        pub const MAGICIAN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::MAGICIAN,
            card_type: BCardType::Tarot,
            enhancement: MPip::Lucky(5, 15),
            resell_value: 1,
            debuffed: false,
        };
        pub const HIGH_PRIESTESS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HIGH_PRIESTESS,
            card_type: BCardType::Tarot,
            enhancement: MPip::Planet(2),
            resell_value: 1,
            debuffed: false,
        };
        pub const EMPRESS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::EMPRESS,
            card_type: BCardType::Tarot,
            enhancement: MPip::MultPlus(4),
            resell_value: 1,
            debuffed: false,
        };
        pub const EMPEROR: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::EMPEROR,
            card_type: BCardType::Tarot,
            enhancement: MPip::RandomTarot(2),
            resell_value: 1,
            debuffed: false,
        };
        pub const HIEROPHANT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HIEROPHANT,
            card_type: BCardType::Tarot,
            enhancement: MPip::Chips(30),
            resell_value: 1,
            debuffed: false,
        };
        pub const LOVERS: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::LOVERS,
            card_type: BCardType::Tarot,
            enhancement: MPip::Wild(PipType::Suit),
            resell_value: 1,
            debuffed: false,
        };
        pub const THE_CHARIOT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::CHARIOT,
            card_type: BCardType::Tarot,
            enhancement: MPip::STEEL,
            resell_value: 1,
            debuffed: false,
        };
        pub const STRENGTH: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::STRENGTH,
            card_type: BCardType::Tarot,
            enhancement: MPip::Strength,
            resell_value: 1,
            debuffed: false,
        };
        pub const HERMIT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HERMIT,
            card_type: BCardType::Tarot,
            enhancement: MPip::DoubleMoney(20),
            resell_value: 1,
            debuffed: false,
        };
        pub const WHEEL_OF_FORTUNE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::WHEEL_OF_FORTUNE,
            card_type: BCardType::Tarot,
            enhancement: MPip::WHEEL_OF_FORTUNE,
            resell_value: 1,
            debuffed: false,
        };
        pub const JUSTICE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::JUSTICE,
            card_type: BCardType::Tarot,
            enhancement: MPip::Glass(2, 4),
            resell_value: 1,
            debuffed: false,
        };
        pub const HANGED_MAN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::HANGED_MAN,
            card_type: BCardType::Tarot,
            enhancement: MPip::Hanged(2),
            resell_value: 1,
            debuffed: false,
        };
        pub const DEATH: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::DEATH,
            card_type: BCardType::Tarot,
            enhancement: MPip::Death(1),
            resell_value: 1,
            debuffed: false,
        };
        pub const TEMPERANCE: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::TEMPERANCE,
            card_type: BCardType::Tarot,
            enhancement: MPip::TEMPERANCE,
            resell_value: 1,
            debuffed: false,
        };
        pub const DEVIL: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::DEVIL,
            card_type: BCardType::Tarot,
            enhancement: MPip::DEVIL,
            resell_value: 1,
            debuffed: false,
        };
        pub const TOWER: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::TOWER,
            card_type: BCardType::Tarot,
            enhancement: MPip::TOWER,
            resell_value: 1,
            debuffed: false,
        };
        pub const STAR: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::STAR,
            card_type: BCardType::Tarot,
            enhancement: MPip::Diamonds(3),
            resell_value: 1,
            debuffed: false,
        };
        pub const MOON: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::MOON,
            card_type: BCardType::Tarot,
            enhancement: MPip::Clubs(3),
            resell_value: 1,
            debuffed: false,
        };
        pub const SUN: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::SUN,
            card_type: BCardType::Tarot,
            enhancement: MPip::Hearts(3),
            resell_value: 1,
            debuffed: false,
        };
        pub const JUDGEMENT: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::JUDGEMENT,
            card_type: BCardType::Tarot,
            enhancement: MPip::JUDGEMENT,
            resell_value: 1,
            debuffed: false,
        };
        pub const WORLD: BuffoonCard = BuffoonCard {
            suit: TarotSuit::MAJOR_ARCANA,
            rank: TarotRank::WORLD,
            card_type: BCardType::Tarot,
            enhancement: MPip::Spades(3),
            resell_value: 1,
            debuffed: false,
        };
    }
}

pub mod joker {
    pub struct Joker {}

    pub mod card {
        use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
        use crate::funky::types::mpip::MPip;
        use crate::prelude::{FrenchSuit, Pip, PipType};

        // https://symbl.cc/en/unicode-table/#miscellaneous-symbols
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters#Dingbats
        /// For Joker cards, their cost is set by the rank value.
        pub const JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 1_000,
                pip_type: PipType::Joker,
                index: '⚫',
                symbol: '⚫',
                value: 2,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlus(4),
            resell_value: 1,
            debuffed: false,
        };
        pub const GREEDY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 995,
                pip_type: PipType::Joker,
                index: '♦',
                symbol: '♦',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MULT_PLUS3_ON_DIAMONDS,
            resell_value: 2,
            debuffed: false,
        };
        pub const LUSTY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 990,
                pip_type: PipType::Joker,
                index: '♥',
                symbol: '♥',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MULT_PLUS3_ON_HEARTS,
            resell_value: 2,
            debuffed: false,
        };
        pub const WRATHFUL_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 985,
                pip_type: PipType::Joker,
                index: '♠',
                symbol: '♠',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MULT_PLUS3_ON_SPADES,
            resell_value: 2,
            debuffed: false,
        };
        pub const GLUTTONOUS_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 980,
                pip_type: PipType::Joker,
                index: '♣',
                symbol: '♣',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MULT_PLUS3_ON_CLUBS,
            resell_value: 2,
            debuffed: false,
        };
        /// The `Jolly Joker` is one that has no effect on a single card, and only returns mult
        /// on a specific conditions of cards.
        pub const JOLLY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 975,
                pip_type: PipType::Joker,
                index: '☺',
                symbol: '☺',
                value: 3,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnPair(8),
            resell_value: 1,
            debuffed: false,
        };
        pub const ZANY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 970,
                pip_type: PipType::Joker,
                index: '🤪',
                symbol: '🤪',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnTrips(12),
            resell_value: 2,
            debuffed: false,
        };
        pub const MAD_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 965,
                pip_type: PipType::Joker,
                index: '☹',
                symbol: '☹',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOn2Pair(10),
            resell_value: 2,
            debuffed: false,
        };
        pub const CRAZY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 960,
                pip_type: PipType::Joker,
                index: '▦',
                symbol: '▦',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnStraight(12),
            resell_value: 2,
            debuffed: false,
        };
        pub const DROLL_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 955,
                pip_type: PipType::Joker,
                index: '▤',
                symbol: '▤',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnFlush(10),
            resell_value: 2,
            debuffed: false,
        };
        pub const SLY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 950,
                pip_type: PipType::Joker,
                index: '⛄',
                symbol: '⛄',
                value: 3,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOnPair(50),
            resell_value: 1,
            debuffed: false,
        };
        pub const WILY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 945,
                pip_type: PipType::Joker,
                index: '⛕',
                symbol: '⛕',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOnTrips(100),
            resell_value: 2,
            debuffed: false,
        };
        pub const CLEVER_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 940,
                pip_type: PipType::Joker,
                index: '∑',
                symbol: '∑',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOn2Pair(80),
            resell_value: 2,
            debuffed: false,
        };
        pub const DEVIOUS_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 935,
                pip_type: PipType::Joker,
                index: '∫',
                symbol: '∫',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOnStraight(100),
            resell_value: 2,
            debuffed: false,
        };
        pub const CRAFTY_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 930,
                pip_type: PipType::Joker,
                index: '∞',
                symbol: '∞',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOnFlush(80),
            resell_value: 2,
            debuffed: false,
        };
        pub const HALF_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 925,
                pip_type: PipType::Joker,
                index: '½',
                symbol: '½',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnUpToXCards(20, 3),
            resell_value: 2,
            debuffed: false,
        };
        pub const JOKER_STENCIL: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 920,
                pip_type: PipType::Joker,
                index: '∛',
                symbol: '∛',
                value: 8,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::MultTimesOnEmptyJokerSlots(1),
            resell_value: 4,
            debuffed: false,
        };
        pub const FOUR_FINGERS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 915,
                pip_type: PipType::Joker,
                index: '∜',
                symbol: '∜',
                value: 7,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::FourFlushAndStraight,
            resell_value: 3,
            debuffed: false,
        };
        pub const MIME: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 914,
                pip_type: PipType::Joker,
                index: '∝',
                symbol: '∝',
                value: 5,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::RetriggerCardsInHand(1),
            resell_value: 2,
            debuffed: false,
        };
        pub const CREDIT_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 913,
                pip_type: PipType::Joker,
                index: '💳',
                symbol: '💳',
                value: 1,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Credit(20),
            resell_value: 1,
            debuffed: false,
        };
        pub const CEREMONIAL_DAGGER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 912,
                pip_type: PipType::Joker,
                index: '🗡',
                symbol: '🗡',
                value: 6,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::MultPlusDoubleValueDestroyJokerOnRight(0),
            resell_value: 3,
            debuffed: false,
        };
        pub const BANNER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 911,
                pip_type: PipType::Joker,
                index: '🚩',
                symbol: '🚩',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsPerRemainingDiscard(30),
            resell_value: 2,
            debuffed: false,
        };
        pub const MYSTIC_SUMMIT: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 910,
                pip_type: PipType::Joker,
                index: '🏔',
                symbol: '🏔',
                value: 6,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::MultPlusOnZeroDiscards(15),
            resell_value: 2,
            debuffed: false,
        };
        pub const MARBLE_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 909,
                pip_type: PipType::Joker,
                index: '🔮',
                symbol: '🔮',
                value: 6,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::AddCardTypeWhenBlindSelected(BCardType::Stone),
            resell_value: 2,
            debuffed: false,
        };
        pub const LOYALTY_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 908,
                pip_type: PipType::Joker,
                index: '🛍',
                symbol: '🛍',
                value: 5,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::MultTimesEveryXHands(4, 6),
            resell_value: 2,
            debuffed: false,
        };

        // FINISH ME
        pub const EIGHT_BALL: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 907,
                pip_type: PipType::Joker,
                index: '🎱',
                symbol: '🎱',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::CreateCardOnRankPlay(4, '8', BCardType::Tarot),
            resell_value: 2,
            debuffed: false,
        };
        pub const MISPRINT: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 906,
                pip_type: PipType::Joker,
                index: '🃏',
                symbol: '🃏',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusRandomTo(24),
            resell_value: 2,
            debuffed: false,
        };
        pub const DUSK: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 905,
                pip_type: PipType::Joker,
                index: '🌆',
                symbol: '🌆',
                value: 5,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::RetriggerPlayedCardsInFinalRound,
            resell_value: 2,
            debuffed: false,
        };
        pub const RAISED_FIST: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 904,
                pip_type: PipType::Joker,
                index: '✊',
                symbol: '✊',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusXOnLowestRankInHand(2),
            resell_value: 2,
            debuffed: false,
        };
        pub const CHAOS_THE_CLOWN: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 903,
                pip_type: PipType::Joker,
                index: '🤡',
                symbol: '🤡',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::FreeReroll(1),
            resell_value: 2,
            debuffed: false,
        };
        pub const FIBONACCI: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 902,
                pip_type: PipType::Joker,
                index: '🔢',
                symbol: '🔢',
                value: 8,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::MultPlusOn5Ranks(8, ['A', '2', '3', '5', '8']),
            resell_value: 4,
            debuffed: false,
        };
        pub const STEEL_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 901,
                pip_type: PipType::Joker,
                index: '🔩',
                symbol: '🔩',
                value: 7,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::Blank,
            resell_value: 3,
            debuffed: false,
        };
        pub const SCARY_FACE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 900,
                pip_type: PipType::Joker,
                index: '👻',
                symbol: '👻',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 2,
            debuffed: false,
        };
        pub const ABSTRACT_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 895,
                pip_type: PipType::Joker,
                index: '🎨',
                symbol: '🎨',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 2,
            debuffed: false,
        };
        pub const DELAYED_GRATIFICATION: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 890,
                pip_type: PipType::Joker,
                index: '🕰',
                symbol: '🕰',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 2,
            debuffed: false,
        };
        pub const HACK: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 885,
                pip_type: PipType::Joker,
                index: '💻',
                symbol: '💻',
                value: 6,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::Blank,
            resell_value: 3,
            debuffed: false,
        };
        pub const PAREIDOLIA: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 880,
                pip_type: PipType::Joker,
                index: '👁',
                symbol: '👁',
                value: 5,
            },
            card_type: BCardType::UncommonJoker,
            enhancement: MPip::Blank,
            resell_value: 2,
            debuffed: false,
        };
        pub const GROS_MICHEL: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 875,
                pip_type: PipType::Joker,
                index: '🍌',
                symbol: '🍌',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChanceDestroyed(1, 6),
            resell_value: 2,
            debuffed: false,
        };
        pub const EVEN_STEVEN: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 870,
                pip_type: PipType::Joker,
                index: '▤',
                symbol: '▤',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOn5Ranks(4, ['T', '8', '6', '4', '2']),
            resell_value: 2,
            debuffed: false,
        };
        pub const ODD_TODD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 865,
                pip_type: PipType::Joker,
                index: '▲',
                symbol: '▲',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOn5Ranks(4, ['A', '9', '7', '5', '3']),
            resell_value: 2,
            debuffed: false,
        };
        pub const SCHOLAR: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 860,
                pip_type: PipType::Joker,
                index: '🎓',
                symbol: '🎓',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusChipsOnRank(4, 20, 'A'),
            resell_value: 2,
            debuffed: false,
        };
        /// **DIARY** I am constantly debating in my head how reasonable this all is. On the one
        /// hand I love how I can have a constant that represents the state, but I can also mutate
        /// it through game play, just like in the game, without touching the underlying functions
        /// that process it. On the other hand, it's convoluted AF.
        pub const BUSINESS_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 855,
                pip_type: PipType::Joker,
                index: '🪪',
                symbol: '🪪',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Odds1inCashOn3Ranks(2, 2, ['K', 'Q', 'J']),
            resell_value: 2,
            debuffed: false,
        };
        pub const SUPERNOVA: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 850,
                pip_type: PipType::Joker,
                index: '🌌',
                symbol: '🌌',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnHandPlays,
            resell_value: 2,
            debuffed: false,
        };
        pub const RIDE_THE_BUS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 910,
                pip_type: PipType::Joker,
                index: '🚌',
                symbol: '🚌',
                value: 6,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::MultPlusOnConsecutiveHandsNo3Ranks(0, 1, ['K', 'Q', 'J']),
            resell_value: 3,
            debuffed: false,
        };
        pub const SPACE_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 910,
                pip_type: PipType::Joker,
                index: '∝',
                symbol: '∝',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Odds1inUpgradeHand(4),
            resell_value: 2,
            debuffed: false,
        };
        pub const EGG: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 905,
                pip_type: PipType::Joker,
                index: '∞',
                symbol: '∞',
                value: 6,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::SellValueIncrement(3),
            resell_value: 2,
            debuffed: false,
        };
        pub const BURGLAR: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 900,
                pip_type: PipType::Joker,
                index: '∟',
                symbol: '∟',
                value: 6,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 3,
            debuffed: false,
        };
        pub const BLACKBOARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 895,
                pip_type: PipType::Joker,
                index: '∠',
                symbol: '∠',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 3,
            debuffed: false,
        };
        pub const RUNNER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 890,
                pip_type: PipType::Joker,
                index: '∡',
                symbol: '∡',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::ChipsOnStraight(15),
            resell_value: 2,
            debuffed: false,
        };
        pub const ICE_CREAM: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 885,
                pip_type: PipType::Joker,
                index: '∢',
                symbol: '∢',
                value: 6,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Chips(100),
            resell_value: 2,
            debuffed: false,
        };
        pub const DNA: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 880,
                pip_type: PipType::Joker,
                index: '∣',
                symbol: '∣',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 2,
            debuffed: false,
        };
        pub const SPLASH: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 875,
                pip_type: PipType::Joker,
                index: '∤',
                symbol: '∤',
                value: 4,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const BLUE_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 870,
                pip_type: PipType::Joker,
                index: '∥',
                symbol: '∥',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SIXTH_SENSE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 865,
                pip_type: PipType::Joker,
                index: '∦',
                symbol: '∦',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const CONSTELLATION: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 860,
                pip_type: PipType::Joker,
                index: '∧',
                symbol: '∧',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const HIKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 855,
                pip_type: PipType::Joker,
                index: '∨',
                symbol: '∨',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const FACELESS_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 850,
                pip_type: PipType::Joker,
                index: '∩',
                symbol: '∩',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const GREEN_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 845,
                pip_type: PipType::Joker,
                index: '∪',
                symbol: '∪',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SUPERPOSITION: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 840,
                pip_type: PipType::Joker,
                index: '∫',
                symbol: '∫',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const TO_DO_LIST: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 835,
                pip_type: PipType::Joker,
                index: '∬',
                symbol: '∬',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const CAVENDISH: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 830,
                pip_type: PipType::Joker,
                index: '∭',
                symbol: '∭',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const CARD_SHARP: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 825,
                pip_type: PipType::Joker,
                index: '∮',
                symbol: '∮',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const RED_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 820,
                pip_type: PipType::Joker,
                index: '∯',
                symbol: '∯',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const MADNESS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 815,
                pip_type: PipType::Joker,
                index: '∰',
                symbol: '∰',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SQUARE_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 810,
                pip_type: PipType::Joker,
                index: '∱',
                symbol: '∱',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SEANCE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 805,
                pip_type: PipType::Joker,
                index: '∲',
                symbol: '∲',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const RIFF_RAFF: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 800,
                pip_type: PipType::Joker,
                index: '∳',
                symbol: '∳',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const VAMPIRE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 795,
                pip_type: PipType::Joker,
                index: '∴',
                symbol: '∴',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SHORTCUT: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 790,
                pip_type: PipType::Joker,
                index: '∵',
                symbol: '∵',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const HOLOGRAM: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 785,
                pip_type: PipType::Joker,
                index: '∶',
                symbol: '∶',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const VAGABOND: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 780,
                pip_type: PipType::Joker,
                index: '∷',
                symbol: '∷',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const BARON: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 775,
                pip_type: PipType::Joker,
                index: '∸',
                symbol: '∸',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const CLOUD_9: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 770,
                pip_type: PipType::Joker,
                index: '∹',
                symbol: '∹',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const ROCKET: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 765,
                pip_type: PipType::Joker,
                index: '∺',
                symbol: '∺',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const EROSION: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 760,
                pip_type: PipType::Joker,
                index: '∻',
                symbol: '∻',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const RESERVED_PARKING: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 755,
                pip_type: PipType::Joker,
                index: '∼',
                symbol: '∼',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const MAIL_IN_REBATE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 750,
                pip_type: PipType::Joker,
                index: '∽',
                symbol: '∽',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const TO_THE_MOON: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 745,
                pip_type: PipType::Joker,
                index: '∾',
                symbol: '∾',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const HALLUCINATION: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 740,
                pip_type: PipType::Joker,
                index: '∿',
                symbol: '∿',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const FORTUNE_TELLER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 735,
                pip_type: PipType::Joker,
                index: '≀',
                symbol: '≀',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const JUGGLER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 730,
                pip_type: PipType::Joker,
                index: '≁',
                symbol: '≁',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const DRUNKARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 725,
                pip_type: PipType::Joker,
                index: '≂',
                symbol: '≂',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const STONE_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 720,
                pip_type: PipType::Joker,
                index: '≃',
                symbol: '≃',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const GOLDEN_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 715,
                pip_type: PipType::Joker,
                index: '≄',
                symbol: '≄',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Chips(4),
            resell_value: 0,
            debuffed: false,
        };
        pub const LUCKY_CAT: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 710,
                pip_type: PipType::Joker,
                index: '≅',
                symbol: '≅',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const BASEBALL_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 705,
                pip_type: PipType::Joker,
                index: '≆',
                symbol: '≆',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const BULL: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 700,
                pip_type: PipType::Joker,
                index: '≇',
                symbol: '≇',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const DIET_COLA: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 695,
                pip_type: PipType::Joker,
                index: '≈',
                symbol: '≈',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const TRADING_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 690,
                pip_type: PipType::Joker,
                index: '≉',
                symbol: '≉',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const FLASH_CARD: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 685,
                pip_type: PipType::Joker,
                index: '≊',
                symbol: '≊',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const POPCORN: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 680,
                pip_type: PipType::Joker,
                index: '≋',
                symbol: '≋',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const SPARE_TROUSERS: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 675,
                pip_type: PipType::Joker,
                index: '≌',
                symbol: '≌',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const ANCIENT_JOKER: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 670,
                pip_type: PipType::Joker,
                index: '≍',
                symbol: '≍',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
        pub const RAMEN: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 800,
                pip_type: PipType::Joker,
                index: '🍜',
                symbol: '🍜',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
    }
}

// 1 Joker 	+4 Mult 	$2 	Common 	Available from start. 	+m 	Indep.
// 2 Greedy Joker 	Played cards with  Diamond suit give +3 Mult when scored 	$5 	Common 	Available from start. 	+m 	On Scored
// 3 Lusty Joker 	Played cards with  Heart suit give +3 Mult when scored 	$5 	Common 	Available from start. 	+m 	On Scored
// 4 Wrathful Joker 	Played cards with  Spade suit give +3 Mult when scored 	$5 	Common 	Available from start. 	+m 	On Scored
// 5 Gluttonous Joker 	Played cards with  Club suit give +3 Mult when scored 	$5 	Common 	Available from start. 	+m 	On Scored
// 6 Jolly Joker 	+8 Mult if played hand contains a Pair 	$3 	Common 	Available from start. 	+m 	Indep.
// 7 Zany Joker 	+12 Mult if played hand contains a Three of a Kind 	$4 	Common 	Available from start. 	+m 	Indep.
// 8 Mad Joker 	+10 Mult if played hand contains a Two Pair 	$4 	Common 	Available from start. 	+m 	Indep.
// 9 Crazy Joker 	+12 Mult if played hand contains a Straight 	$4 	Common 	Available from start. 	+m 	Indep.
// 10 Droll Joker 	+10 Mult if played hand contains a Flush 	$4 	Common 	Available from start. 	+m 	Indep.
// 11 Sly Joker 	+50 Chips if played hand contains a Pair 	$3 	Common 	Available from start. 	+c 	Indep.
// 12 Wily Joker 	+100 Chips if played hand contains a Three of a Kind 	$4 	Common 	Available from start. 	+c 	Indep.
// 13 Clever Joker 	+80 Chips if played hand contains a Two Pair 	$4 	Common 	Available from start. 	+c 	Indep.
// 14 	Devious Joker 	+100 Chips if played hand contains a Straight 	$4 	Common 	Available from start. 	+c 	Indep.
// 15 	Crafty Joker 	+80 Chips if played hand contains a Flush 	$4 	Common 	Available from start. 	+c 	Indep.
// 16 	Half Joker 	+20 Mult if played hand contains 3 or fewer cards 	$5 	Common 	Available from start. 	+m 	Indep.
// 17 	Joker Stencil 	X1 Mult for each empty Joker slot. Joker Stencil included (Curently X1) 	$8 	Uncommon 	Available from start. 	Xm 	Indep.
// 18 	Four Fingers 	All Flushes and Straights can be made with 4 cards 	$7 	Uncommon 	Available from start. 	!! 	N/A
// 19 	Mime 	Retrigger all card held in hand abilities 	$5 	Uncommon 	Available from start. 	... 	On Held
// 20 	Credit Card 	Go up to -$20 in debt 	$1 	Common 	Available from start. 	+$ 	N/A
// 21 	Ceremonial Dagger 	When Blind is selected, destroy Joker to the right and permanently add double its sell value to this Mult (Currently +0 Mult) 	$6 	Uncommon 	Available from start. 	+m 	Indep.
// 22 	Banner 	+30 Chips for each remaining discard 	$5 	Common 	Available from start. 	+c 	Indep.
// 23 	Mystic Summit 	+15 Mult when 0 discards remaining 	$5 	Common 	Available from start. 	+m 	Indep.
// 24 	Marble Joker 	Adds one Stone card to the deck when Blind is selected 	$6 	Uncommon 	Available from start. 	!! 	N/A
// 25 	Loyalty Card 	X4 Mult every 6 hands played 5 reaining 	$5 	Uncommon 	Available from start. 	Xm 	Indep.
// 26 	8 Ball 	1 in 4 chance for each played 8 to create a Tarot card when scored (Mus have room) 	$5 	Common 	Available from start. 	!! 	On Scored
// 27 	Misprint 	+0-23 Mult 	$4 	Common 	Available from start. 	+m 	Indep.
// 28 	Dusk 	Retrigger all played cards in final hand of the round 	$5 	Uncommon 	Available from start. 	... 	On Scored
// 29 	Raised Fist 	Adds double the rank of lowest ranked card held in hand to Mult 	$5 	Common 	Available from start. 	+m 	On Held
// 30 	Chaos the Clown 	1 free Reroll per shop 	$4 	Common 	Available from start. 	!! 	N/A
// 31 	Fibonacci 	Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored 	$8 	Uncommon 	Available from start. 	+m 	On Scored
// 32 	Steel Joker 	Gives X0.2 Mult for each Steel Card in your full deck (Currently X1 Mult) 	$7 	Uncommon 	Available from start. (Can only appear in the shop when there is a Steel Card in the deck.) 	Xm 	Indep.
// 33 	Scary Face 	Played face cards give +30 Chips when scored 	$4 	Common 	Available from start. 	+c 	On Scored
// 34 	Abstract Joker 	+3 Mult for each Joker card (Curently +0 Mult) 	$4 	Common 	Available from start. 	+m 	Indep.
// 35 	Delayed Gratification 	Earn $2 per discard if no discards are used by end of the round 	$4 	Common 	Available from start. 	+$ 	N/A
// 36 	Hack 	Retrigger each played 2, 3, 4, or 5 	$6 	Uncommon 	Available from start. 	... 	On Scored
// 37 	Pareidolia 	All cards are considered face cards 	$5 	Uncommon 	Available from start. 	!! 	N/A
// 38 	Gros Michel 	+15 Mult 1 in6 chance this is destroyed at the end of round. 	$5 	Common 	Available from start. 	+m 	Indep.
// 39 	Even Steven 	Played cards with even rank give +4 Mult when scored (10, 8, 6, 4, 2) 	$4 	Common 	Available from start. 	+m 	On Scored
// 40 	Odd Todd 	Played cards with odd rank give +31 Chips when scored (A, 9, 7, 5, 3) 	$4 	Common 	Available from start. 	+c 	On Scored
// 41 	Scholar 	Played Aces give +20 Chips and +4 Mult when scored 	$4 	Common 	Available from start. 	++ 	On Scored
// 42 	Business Card 	Played face cards have a 1 in 2 chance to give $2 when scored 	$4 	Common 	Available from start. 	+$ 	On Scored
// 43 	Supernova 	Adds the number of times poker hand has been played this run to Mult 	$5 	Common 	Available from start. 	+m 	Indep.
// 44 	Ride the Bus 	This Joker gains +1 Mult per consecutive hand played without a scoring face card (Currently +0 Mult) 	$6 	Common 	Available from start. 	+m 	Mixed
// 45 	Space Joker 	1 in 4 chance to upgrade level of played poker hand 	$5 	Uncommon 	Available from start. 	!! 	On Played
// 46 	Egg 	Gains $3 of sell value at end of round 	$4 	Common 	Available from start. 	+$ 	N/A
// 47 	Burglar 	When Blind is selected, gain +3 Hands and lose all discards 	$6 	Uncommon 	Available from start. 	!! 	N/A
// 48 	Blackboard 	X3 Mult if all cards held in hand are  Spades or  Clubs 	$6 	Uncommon 	Available from start. 	Xm 	Indep.
// 49 	Runner 	Gains +15 Chips if played hand contains a Straight (Currently +0 Chips) 	$5 	Common 	Available from start. 	+c 	Mixed
// 50 	Ice Cream 	+100 Chips -5 Chips for every hand played 	$5 	Common 	Available from start. 	+c 	Indep.
// 51 	DNA 	If first hand of round has only 1 card, add a permanent copy to deck and draw it to hand 	$8 	Rare 	Available from start. 	!! 	On Played
// 52 	Splash 	Every played card counts in scoring 	$3 	Common 	Available from start. 	!! 	N/A
// 53 	Blue Joker 	+2 Chips for each remaining card in deck
// (Currently +104 Chips) 	$5 	Common 	Available from start. 	+c 	Indep.
// 54 	Sixth Sense 	If first hand of round is a single 6, destroy it and create a Spectral card
// (Must have room) 	$6 	Uncommon 	Available from start. 	!! 	N/A
// 55 	Constellation 	This Joker gains X0.1 Mult every time a Planet card is used
// (Currently X1 Mult) 	$6 	Uncommon 	Available from start. 	Xm 	Indep.
// 56 	Hiker 	Every played card permanently gains +5 Chips when scored 	$5 	Uncommon 	Available from start. 	+c 	On Scored
// 57 	Faceless Joker 	Earn $5 if 3 or more face cards are discarded at the same time 	$4 	Common 	Available from start. 	+$ 	On Discard
// 58 	Green Joker 	+1 Mult per hand played
// -1 Mult per discard
// (Currently +0 Mult) 	$4 	Common 	Available from start. 	+m 	Mixed
// 59 	Superposition 	Create a Tarot card if poker hand contains an Ace and a Straight
// (Must have room) 	$4 	Common 	Available from start. 	!! 	Indep.
// 60 	To Do List 	Earn $4 if poker hand is a [Poker Hand], poker hand changes at end of round 	$4 	Common 	Available from start. 	+$ 	On Played
// 61 	Cavendish 	X3 Mult
// 1 in 1000 chance this card is destroyed at the end of round 	$4 	Common 	Available from start. (Can only appear in the shop when Gros Michel has destroyed itself in the current run.) 	Xm 	Indep.
// 62 	Card Sharp 	X3 Mult if played poker hand has already been played this round 	$6 	Uncommon 	Available from start. 	Xm 	Indep.
// 63 	Red Card 	This Joker gains +3 Mult when any Booster Pack is skipped
// (Curently +0 Mult) 	$5 	Common 	Available from start. 	+m 	Indep.
// 64 	Madness 	When Small Blind or Big Blind is selected, gain X0.5 Mult and destroy a random Joker
// (Currently X1 Mult) 	$7 	Uncommon 	Available from start. 	Xm 	Indep.
// 65 	Square Joker 	This Joker gains +4 Chips if played hand has exactly 4 cards
// (Currently 0 Chips) 	$4 	Common 	Available from start. 	+c 	Mixed
// 66 	Séance 	If poker hand is a Straight Flush, create a random Spectral card
// (Must have room) 	$6 	Uncommon 	Available from start. 	!! 	Indep.
// 67 	Riff-Raff 	When Blind is selected, create 2 Common Jokers
// (Must have room) 	$6 	Common 	Available from start. 	!! 	N/A
// 68 	Vampire 	This Joker gains X0.1 Mult per scoring Enhanced card played, removes card Enhancement
// (Currently X1 Mult) 	$7 	Uncommon 	Available from start. 	Xm 	Mixed
// 69 	Shortcut 	Allows Straights to be made with gaps of 1 rank
// (ex: 10 8 6 5 3) 	$7 	Uncommon 	Available from start. 	!! 	N/A
// 70 	Hologram 	This Joker gains X0.25 Mult every time a playing card is added to your deck
// (Currently X1 Mult) 	$7 	Uncommon 	Available from start. 	Xm 	Indep.
// 71 	Vagabond 	Create a Tarot card if hand is played with $4 or less 	$8 	Rare 	Available from start. 	!! 	Indep.
// 72 	Baron 	Each King held in hand gives X1.5 Mult 	$8 	Rare 	Available from start. 	Xm 	On Held
// 73 	Cloud 9 	Earn $1 for each 9 in your full deck at end of round
// (Currently $4) 	$7 	Uncommon 	Available from start. 	+$ 	N/A
// 74 	Rocket 	Earn $1 at end of round. Payout increases by $2 when Boss Blind is defeated 	$6 	Uncommon 	Available from start. 	+$ 	N/A
// 75 	Obelisk 	This Joker gains X0.2 Mult per consecutive hand played without playing your most played poker hand
// (Currently X1 Mult) 	$8 	Rare 	Available from start. 	Xm 	Mixed
// 76 	Midas Mask 	All played face cards become Gold cards when scored 	$7 	Uncommon 	Available from start. 	!! 	On Played
// 77 	Luchador 	Sell this card to disable the current Boss Blind 	$5 	Uncommon 	Available from start. 	!! 	N/A
// 78 	Photograph 	First played face card gives X2 Mult when scored 	$5 	Common 	Available from start. 	Xm 	On Scored
// 79 	Gift Card 	Add $1 of sell value to every Joker and Consumable card at end of round 	$6 	Uncommon 	Available from start. 	+$ 	N/A
// 80 	Turtle Bean 	+5 hand size, reduces by 1 each round 	$6 	Uncommon 	Available from start. 	!! 	N/A
// 81 	Erosion 	+4 Mult for each card below [the deck's starting size] in your full deck
// (Currently +0 Mult) 	$6 	Uncommon 	Available from start. 	+m 	Indep.
// 82 	Reserved Parking 	Each face card held in hand has a 1 in 2 chance to give $1 	$6 	Common 	Available from start. 	+$ 	On Held
// 83 	Mail-In Rebate 	Earn $5 for each discarded [rank], rank changes every round 	$4 	Common 	Available from start. 	+$ 	On Discard
// 84 	To the Moon 	Earn an extra $1 of interest for every $5 you have at end of round 	$5 	Uncommon 	Available from start. 	+$ 	N/A
// 85 	Hallucination 	1 in 2 chance to create a Tarot card when any Booster Pack is opened
// (Must have room) 	$4 	Common 	Available from start. 	!! 	N/A
// 86 	Fortune Teller 	+1 Mult per Tarot card used this run
// (Currently +0) 	$6 	Common 	Available from start. 	+m 	Indep.
// 87 	Juggler 	+1 hand size 	$4 	Common 	Available from start. 	!! 	N/A
// 88 	Drunkard 	+1 discard each round 	$4 	Common 	Available from start. 	!! 	N/A
// 89 	Stone Joker 	Gives +25 Chips for each Stone Card in your full deck
// (Currently +0 Chips) 	$6 	Uncommon 	Available from start. (Can only appear in the shop when there is a Stone Card in the deck.) 	+c 	Indep.
// 90 	Golden Joker 	Earn $4 at end of round 	$6 	Common 	Available from start. 	+$ 	N/A
// 91 	Lucky Cat 	This Joker gains X0.25 Mult every time a Lucky card successfully triggers
// (Currently X1 Mult) 	$6 	Uncommon 	Available from start. (Can only appear in the shop when there is a Lucky Card in the deck.) 	Xm 	Mixed
// 92 	Baseball Card 	Uncommon Jokers each give X1.5 Mult 	$8 	Rare 	Available from start. 	Xm 	On Other Jokers
// 93 	Bull 	+2 Chips for each $1 you have
// (Currently +0 Chips) 	$6 	Uncommon 	Available from start. 	+c 	Indep.
// 94 	Diet Cola 	Sell this card to create a free Double Tag 	$6 	Uncommon 	Available from start. 	!! 	N/A
// 95 	Trading Card 	If first discard of round has only 1 card, destroy it and earn $3 	$6 	Uncommon 	Available from start. 	+$ 	On Discard
// 96 	Flash Card 	This Joker gains +2 Mult per reroll in the shop
// (Currently +0 Mult) 	$5 	Uncommon 	Available from start. 	+m 	Indep.
// 97 	Popcorn 	+20 Mult
// -4 Mult per round played 	$5 	Common 	Available from start. 	+m 	Indep.
// 98 	Spare Trousers 	This Joker gains +2 Mult if played hand contains a Two Pair
// (Currently +0 Mult) 	$6 	Uncommon 	Available from start. 	+m 	Mixed
// 99 	Ancient Joker 	Each played card with [suit] gives X1.5 Mult when scored,
// suit changes at end of round 	$8 	Rare 	Available from start. 	Xm 	On Scored
// 100 Ramen 	X2 Mult, loses X0.01 Mult per card discarded 	$6 	Uncommon 	Available from start. 	Xm 	Mixed
// 101 Walkie Talkie 	Each played 10 or 4 gives +10 Chips and +4 Mult when scored 	$4 	Common 	Available from start. 	++ 	On Scored
// 102 Seltzer 	Retrigger all cards played for the next 10 hands 	$6 	Uncommon 	Available from start. 	... 	On Scored
// 103 Castle 	This Joker gains +3 Chips per discarded [suit] card, suit changes every round
// (Currently +0 Chips) 	$6 	Uncommon 	Available from start. 	+c 	Mixed
// 104 Smiley Face 	Played face cards give +5 Mult when scored 	$4 	Common 	Available from start. 	+m 	On Scored
// 105 Campfire 	This Joker gains X0.25 Mult for each card sold, resets when Boss Blind is defeated
// (Currently X1 Mult) 	$9 	Rare 	Available from start. 	Xm 	Indep.
// 106 Golden Ticket 	Played Gold cards earn $4 when scored 	$5 	Common 	Play a 5 card hand that contains only Gold cards. (Can only appear in the shop when there is a Gold Card in the deck.) 	+$ 	On Scored
// 107 Mr. Bones 	Prevents Death if chips scored are at least 25% of required chips
// self destructs 	$5 	Uncommon 	Lose five runs. 	!! 	N/A
// 108 Acrobat 	X3 Mult on final hand of round 	$6 	Uncommon 	Play 200 hands 	Xm 	Indep.
// 109 Sock and Buskin 	Retrigger all played face cards 	$6 	Uncommon 	Play 300 face cards across all runs. 	... 	On Scored
// 110 Swashbuckler 	Adds the sell value of all other owned Jokers to Mult
// (Currently +1 Mult) 	$4 	Common 	Sell 20 Jokers. 	+m 	Indep.
// 111 Troubadour 	+2 hand size,
// -1 hand per round 	$6 	Uncommon 	Win 5 consecutive rounds by playing only a single hand in each. (Discards are fine.) 	!! 	N/A
// 112 Certificate 	When round begins, add a random playing card with a random seal to your hand 	$6 	Uncommon 	Have a Gold card with a Gold Seal. 	!! 	N/A
// 113 Smeared Joker 	 Hearts and  Diamonds count as the same suit,  Spades and  Clubs count as the same suit 	$7 	Uncommon 	Have 3 or more Wild Cards in your deck. 	!! 	N/A
// 114 Throwback 	X0.25 Mult for each Blind skipped this run
// (Currently X1 Mult) 	$6 	Uncommon 	Continue a run from the Main Menu. 	Xm 	Indep.
// 115 Hanging Chad 	Retrigger first played card used in scoring 2 additional times 	$4 	Common 	Beat a Boss Blind with a High Card hand. 	... 	On Scored
// 116 Rough Gem 	Played cards with  Diamond suit earn $1 when scored 	$7 	Uncommon 	Have at least 30 Diamonds in your deck 	+$ 	On Scored
// 117 Bloodstone 	1 in 2 chance for played cards with  Heart suit to give X1.5 Mult when scored 	$7 	Uncommon 	Have at least 30 Hearts in your deck. 	Xm 	On Scored
// 118 Arrowhead 	Played cards with  Spade suit give +50 Chips when scored 	$7 	Uncommon 	Have at least 30 Spades in your deck. 	+c 	On Scored
// 119 Onyx Agate 	Played cards with  Club suit give +7 Mult when scored 	$7 	Uncommon 	Have at least 30 Clubs in your deck 	+m 	On Scored
// 120 Glass Joker 	This Joker gains X0.75 Mult for every Glass Card that is destroyed
// (Currently X1 Mult) 	$6 	Uncommon 	Have 5 or more Glass cards in your deck. (Can only appear in the shop when there is a Glass Card in the deck.) 	Xm 	Indep.
// 121 Showman 	Joker, Tarot, Planet, and Spectral cards may appear multiple times 	$5 	Uncommon 	Reach Ante level 4 	!! 	N/A
// 122 Flower Pot 	X3 Mult if poker hand contains a  Diamond card,  Club card,  Heart card, and  Spade card 	$6 	Uncommon 	Reach Ante Level 8 	Xm 	Indep.
// 123 Blueprint 	Copies ability of Joker to the right 	$10 	Rare 	Win 1 run. 	!!
// 124 Wee Joker 	This Joker gains +8 Chips when each played 2 is scored
// (Currently +0  Chips) 	$8 	Rare 	Win a run in 18 or fewer rounds. 	+c 	Mixed
// 125 Merry Andy 	+3 discards each round,
// -1 hand size 	$7 	Uncommon 	Win a run in 12 or fewer rounds 	!! 	N/A
// 126 Oops! All 6s 	Doubles all listed probabilities
// (ex: 1 in 3 -> 2 in 3) 	$4 	Uncommon 	Earn at least 10,000 Chips in a single hand. 	!! 	N/A
// 127 The Idol 	Each played [rank] of [suit] gives X2 Mult when scored
// Card changes every round 	$6 	Uncommon 	In one hand, earn at least 1,000,000 Chips. 	Xm 	On Scored
// 128 Seeing Double 	X2 Mult if played hand has a scoring  Club card and a scoring card of any other suit 	$6 	Uncommon 	Play a hand that contains four 7 of Clubs.
// Other suits that count as clubs (e.g. wild suits) with rank 7 will also count. 	Xm 	Indep.
// 129 Matador 	Earn $8 if played hand triggers the Boss Blind ability 	$7 	Uncommon 	Defeat a Boss Blind in one hand, without using discards. 	+$ 	Indep.
// 130 Hit the Road 	This Joker gains X0.5 Mult for every Jack discarded this round
// (Currently X1 Mult) 	$8 	Rare 	Discard 5 Jacks at the same time. 	Xm 	Mixed
// 131 The Duo 	X2 Mult if played hand contains a Pair 	$8 	Rare 	Win a run without playing a Pair. 	Xm 	Indep.
// 132 The Trio 	X3 Mult if played hand contains a Three of a Kind 	$8 	Rare 	Win a run without playing a Three of a Kind. 	Xm 	Indep.
// 133 The Family 	X4 Mult if played hand contains a Four of a Kind 	$8 	Rare 	Win a run without playing a Four of a Kind. 	Xm 	Indep.
// 134 The Order 	X3 Mult if played hand contains a Straight 	$8 	Rare 	Win a run without playing a Straight. 	Xm 	Indep.
// 135 The Tribe 	X2 Mult if played hand contains a Flush 	$8 	Rare 	Win a run without playing a Flush. 	Xm 	Indep.
// 136 Stuntman 	+250 Chips,
// -2 hand size 	$7 	Rare 	Earn at least 100 million (100,000,000) Chips in a single hand. 	+c 	Indep.
// 137 Invisible Joker 	After 2 rounds, sell this card to Duplicate a random Joker
// (Currently 0/2)
// (Removes Negative from copy) 	$8 	Rare 	Win a game while never having more than 4 jokers. 	!! 	N/A
// 138 Brainstorm 	Copies the ability of leftmost Joker 	$10 	Rare 	Discard a Royal Flush. 	!!
// 139 Satellite 	Earn $1 at end of round per unique Planet card used this run 	$6 	Uncommon 	Have at least $400. 	+$ 	N/A
// 140 Shoot the Moon 	Each Queen held in hand gives +13 Mult 	$5 	Common 	Play every Heart card in your deck in one round. 	+m 	On Held
// 141 Driver's License 	X3 Mult if you have at least 16 Enhanced cards in your full deck
// (Currently 0) 	$7 	Rare 	Enhance 16 cards in your deck 	Xm 	Indep.
// 142 Cartomancer 	Create a Tarot card when Blind is selected
// (Must have room) 	$6 	Uncommon 	Discover every Tarot Card. 	!! 	N/A
// 143 Astronomer 	All Planet cards and Celestial Packs in the shop are free 	$8 	Uncommon 	Discover all Planet cards. 	!! 	N/A
// 144 Burnt Joker 	Upgrade the level of the first discarded poker hand each round 	$8 	Rare 	Sell 50 cards. 	!! 	On Discard
// 145 Bootstraps 	+2 Mult for every $5 you have
// (Currently +0 Mult) 	$7 	Uncommon 	Have at least 2 Polychrome Jokers at the same time. 	+m 	Indep.
// 146 Canio 	This Joker gains X1 Mult when a face card is destroyed
// (Currently X1 Mult) 	N/A 	Legendary 	Find this Joker from the Soul card. 	Xm 	Indep.
// 147 Triboulet 	Played Kings and Queens each give X2 Mult when scored 	N/A 	Legendary 	Find this Joker from the Soul card. 	Xm 	On Scored
// 148 Yorick 	This Joker gains X1 Mult every 23 [23] cards discarded
// (Currently X1 Mult) 	N/A 	Legendary 	Find this Joker from the Soul card. 	Xm 	Mixed
// 149 Chicot 	Disables effect of every Boss Blind 	N/A 	Legendary 	Find this Joker from the Soul card. 	!! 	N/A
// 150 Perkeo 	Creates a Negative copy of 1 random consumable card in your possession at the end of the shop 	N/A 	Legendary 	Find this Joker from the Soul card. 	!! 	N/A
// Trivia
