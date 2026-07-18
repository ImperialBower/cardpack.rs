/// Balatro Wiki > [Planet Cards](https://balatrogame.fandom.com/wiki/Planet_Cards)
use crate::funky::types::buffoon_card::BuffoonCard;
use crate::preludes::funky::{BCardType, MPip};

pub struct Planet {}

impl Planet {
    pub const DECK_SIZE: usize = 9;

    pub const DECK: [BuffoonCard; Self::DECK_SIZE] = [
        card::PLUTO,
        card::MERCURY,
        card::URANUS,
        card::VENUS,
        card::SATURN,
        card::JUPITER,
        card::EARTH,
        card::MARS,
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
        if Self::same_planet(original, add) {
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
    use crate::funky::types::edition::Edition;
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
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
        edition: Edition::None,
        debuffed: false,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__planet_tests {
    use super::*;
    use crate::funky::types::hands::HandType;
    use std::collections::HashSet;

    /// The nine base poker hands Balatro ships a Planet card for. (Royal Flush
    /// has no Planet of its own — it levels through the Straight Flush entry —
    /// and the three secret Planets, Planet X / Ceres / Eris, are not in the
    /// base deck.)
    const BASE_HANDS: [HandType; 9] = [
        HandType::HighCard,
        HandType::Pair,
        HandType::TwoPair,
        HandType::ThreeOfAKind,
        HandType::Straight,
        HandType::Flush,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::StraightFlush,
    ];

    fn hand_of(card: BuffoonCard) -> HandType {
        match card.enhancement {
            MPip::ChipsMultPlusOnHand(_, _, hand) => hand,
            other => panic!("{card} carries {other} rather than ChipsMultPlusOnHand"),
        }
    }

    #[test]
    fn deck__size_matches_declaration() {
        assert_eq!(Planet::DECK.len(), Planet::DECK_SIZE);
    }

    #[test]
    fn deck__all_cards_are_planets() {
        for card in Planet::DECK {
            assert_eq!(
                card.card_type,
                BCardType::Planet,
                "{card} in DECK is not a Planet"
            );
        }
    }

    #[test]
    fn deck__every_planet_carries_a_chips_mult_plus_on_hand_effect() {
        for card in Planet::DECK {
            assert!(
                matches!(card.enhancement, MPip::ChipsMultPlusOnHand(..)),
                "{card} does not level a poker hand"
            );
        }
    }

    #[test]
    fn deck__each_planet_targets_a_distinct_hand_type() {
        let hands: HashSet<_> = Planet::DECK.iter().map(|c| hand_of(*c)).collect();
        assert_eq!(
            hands.len(),
            Planet::DECK_SIZE,
            "two Planets level the same hand"
        );
    }

    #[test]
    fn deck__covers_every_base_poker_hand() {
        let hands: HashSet<_> = Planet::DECK.iter().map(|c| hand_of(*c)).collect();
        for hand in BASE_HANDS {
            assert!(
                hands.contains(&hand),
                "no Planet levels {hand:?} — the base deck is missing one"
            );
        }
    }

    #[test]
    fn deck__resell_value_is_one() {
        for card in Planet::DECK {
            assert_eq!(card.resell_value, 1, "{card} should resell for $1");
        }
    }

    #[test]
    fn same_planet__is_true_only_for_the_same_hand_type() {
        assert!(Planet::same_planet(card::PLUTO, card::PLUTO));
        assert!(!Planet::same_planet(card::PLUTO, card::MERCURY));
    }

    #[test]
    fn same_planet__is_false_across_card_types() {
        let not_a_planet = crate::funky::decks::tarot::card::EMPRESS;
        assert!(!Planet::same_planet(card::PLUTO, not_a_planet));
    }

    #[test]
    fn add_planets__sums_chips_and_mult_of_matching_planets() {
        let doubled = Planet::add_planets(card::PLUTO, card::PLUTO);
        let MPip::ChipsMultPlusOnHand(chips, mult, hand) = doubled.enhancement else {
            panic!("expected a leveled Pluto");
        };
        assert_eq!(chips, 20, "two Plutos should stack to +20 chips");
        assert_eq!(mult, 2, "two Plutos should stack to +2 mult");
        assert_eq!(hand, HandType::HighCard);
    }

    #[test]
    fn add_planets__leaves_a_mismatched_planet_untouched() {
        let unchanged = Planet::add_planets(card::PLUTO, card::MERCURY);
        assert_eq!(unchanged.enhancement, card::PLUTO.enhancement);
    }
}
