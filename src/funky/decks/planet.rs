/// Balatro Wiki > [Planet Cards](https://balatrogame.fandom.com/wiki/Planet_Cards)
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
