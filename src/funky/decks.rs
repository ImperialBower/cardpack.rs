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

/// Balatro Wiki > [Planet Cards](https://balatrogame.fandom.com/wiki/Planet_Cards)
pub mod planet {
    use crate::funky::types::buffoon_card::BuffoonCard;
    use crate::preludes::funky::{BCardType, MPip};

    pub struct Planet {}

    impl Planet {
        pub const DECK_SIZE: usize = 8;

        pub const DECK: [BuffoonCard; Planet::DECK_SIZE] = [
            card::PLUTO,
            card::VENUS,
            card::EARTH,
            card::MARS,
            card::JUPITER,
            card::SATURN,
            card::URANUS,
            card::NEPTUNE,
        ];

        /// Crude but effective. I would rather have a tight data type that's a pain to access
        /// than a fat one that easy to get the data out of.
        #[must_use]
        pub fn same_planet(original: BuffoonCard, add: BuffoonCard) -> bool {
            if original.card_type != BCardType::Planet || add.card_type != BCardType::Planet {
                return false;
            }
            if let MPip::ChipsMultPlusOnHand(_, _, hand_type) = original.enhancement {
                if let MPip::ChipsMultPlusOnHand(_, _, hand_type2) = add.enhancement {
                    hand_type == hand_type2
                } else {
                    false
                }
            } else {
                false
            }
        }

        #[must_use]
        pub fn add_planets(original: BuffoonCard, add: BuffoonCard) -> BuffoonCard {
            if Planet::same_planet(original, add) {
                if let MPip::ChipsMultPlusOnHand(chips1, mult1, hand_type) = original.enhancement {
                    if let MPip::ChipsMultPlusOnHand(chips2, mult2, _) = add.enhancement {
                        let chips = chips1 + chips2;
                        let mult = mult1 + mult2;
                        let mut new_card = original;
                        new_card.enhancement = MPip::ChipsMultPlusOnHand(chips, mult, hand_type);
                        new_card
                    } else {
                        original
                    }
                } else {
                    original
                }
            } else {
                original
            }
        }
    }

    pub mod card {
        use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
        use crate::funky::types::hands::HandType;
        use crate::funky::types::mpip::MPip;
        use crate::prelude::{Pip, PipType};

        pub const PLANET_SUIT: Pip = Pip {
            pip_type: PipType::Special,
            weight: 1,   // Weight is not used for planets in this context
            index: 'P',  // Arbitrary index for the planet suit
            symbol: '✦', // Arbitrary symbol for the planet suit
            value: 0,    // Value is not used for planets in this context
        };

        pub const PLUTO: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 1,
                index: 'P',
                symbol: '♇',
                value: 1,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(10, 1, HandType::HighCard), // +1 Mult and +10 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const MERCURY: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 2,
                index: 'M',
                symbol: '☿',
                value: 2,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(15, 1, HandType::Pair), // +1 Mult and +15 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const URANUS: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 3,
                index: 'U',
                symbol: '♅',
                value: 3,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(20, 1, HandType::TwoPair), // +1 Mult and +20 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const VENUS: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 4,
                index: 'V',
                symbol: '♀',
                value: 4,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(20, 2, HandType::ThreeOfAKind), // +2 Mult and +20 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const SATURN: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 5,
                index: 'S',
                symbol: '♄',
                value: 5,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(30, 4, HandType::Straight), // +3 Mult and +30 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const JUPITER: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 6, // Weight for JUPITER
                index: 'J',
                symbol: '♃',
                value: 6,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(35, 4, HandType::Flush), // +2 Mult and +35 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const EARTH: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 7,
                index: 'E',
                symbol: '♁',
                value: 7,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(40, 4, HandType::FullHouse), // +2 Mult and +25 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const MARS: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 8,  // Weight for MARS
                index: 'A', // Arbitrary index for MARS
                symbol: '♂',
                value: 8,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(60, 7, HandType::FourOfAKind), // +3 Mult and +30 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const NEPTUNE: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 9,
                index: 'N',
                symbol: '♆',
                value: 9,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(40, 4, HandType::StraightFlush), // +4 Mult and +40 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const PLANET_X: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 10,  // Weight for PLANET_X
                index: '❌', // Arbitrary index for PLANET_X
                symbol: '❌',
                value: 10,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(35, 3, HandType::FiveOfAKind), // +3 Mult and +35 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const CERES: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 11, // Weight for CERES
                index: '⚳', // Arbitrary index for CERES
                symbol: '⚳',
                value: 11,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(40, 4, HandType::FlushFive), // +4 Mult and +40 Chips
            resell_value: 1,
            debuffed: false,
        };
        pub const ERIS: BuffoonCard = BuffoonCard {
            suit: PLANET_SUIT,
            rank: Pip {
                pip_type: PipType::Special,
                weight: 12, // Weight for ERIS
                index: 'ς',
                symbol: 'ς', // Arbitrary symbol for ERIS
                value: 12,
            },
            card_type: BCardType::Planet,
            enhancement: MPip::ChipsMultPlusOnHand(50, 3, HandType::FlushFive), // +3 Mult and +50 Chips
            resell_value: 1,
            debuffed: false,
        };
    }
}

// 	Eris	+3 Mult and +50 Chips	Flush Five	16 Mult x 160 Chips

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
        pub const WALKIE_TALKIE: BuffoonCard = BuffoonCard {
            suit: FrenchSuit::JOKER,
            rank: Pip {
                weight: 800,
                pip_type: PipType::Joker,
                index: '📻',
                symbol: '📻',
                value: 5,
            },
            card_type: BCardType::CommonJoker,
            enhancement: MPip::Blank,
            resell_value: 0,
            debuffed: false,
        };
    }
}

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
