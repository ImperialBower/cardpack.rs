use crate::preludes::funky::{BuffoonCard, BuffoonPile};

pub struct Joker {}

impl Joker {
    pub const COMMON_JOKERS_SIZE: usize = 23;

    pub const COMMON_JOKERS: [BuffoonCard; Self::COMMON_JOKERS_SIZE] = [
        card::JOKER,
        card::GREEDY_JOKER,
        card::LUSTY_JOKER,
        card::WRATHFUL_JOKER,
        card::GLUTTONOUS_JOKER,
        card::JOLLY_JOKER,
        card::ZANY_JOKER,
        card::MAD_JOKER,
        card::CRAZY_JOKER,
        card::DROLL_JOKER,
        card::SLY_JOKER,
        card::WILY_JOKER,
        card::CLEVER_JOKER,
        card::DEVIOUS_JOKER,
        card::CRAFTY_JOKER,
        card::HALF_JOKER,
        card::CREDIT_CARD,
        card::BANNER,
        card::EIGHT_BALL,
        card::MISPRINT,
        card::RAISED_FIST,
        card::CHAOS_THE_CLOWN,
        card::HANGING_CHAD,
    ];

    #[must_use]
    pub fn pile_common() -> BuffoonPile {
        BuffoonPile::from(&Self::COMMON_JOKERS[..])
    }

    pub const UNCOMMON_JOKERS_SIZE: usize = 17;

    pub const UNCOMMON_JOKERS: [BuffoonCard; Self::UNCOMMON_JOKERS_SIZE] = [
        card::JOKER_STENCIL,
        card::FOUR_FINGERS,
        card::MIME,
        card::CEREMONIAL_DAGGER,
        card::MYSTIC_SUMMIT,
        card::MARBLE_JOKER,
        card::LOYALTY_CARD,
        card::DUSK,
        card::FIBONACCI,
        card::STEEL_JOKER,
        card::HACK,
        card::PAREIDOLIA,
        card::SOCK_AND_BUSKIN,
        card::SMEARED_JOKER,
        card::OOPS_ALL_6S,
        card::EROSION,
        card::STONE_JOKER,
    ];

    #[must_use]
    pub fn pile_uncommon() -> BuffoonPile {
        BuffoonPile::from(&Self::UNCOMMON_JOKERS[..])
    }

    pub const RARE_JOKERS_SIZE: usize = 6;

    pub const RARE_JOKERS: [BuffoonCard; Self::RARE_JOKERS_SIZE] = [
        card::THE_DUO,
        card::THE_TRIO,
        card::THE_FAMILY,
        card::THE_ORDER,
        card::THE_TRIBE,
        card::BARON,
    ];

    #[must_use]
    pub fn pile_rare() -> BuffoonPile {
        BuffoonPile::from(&Self::RARE_JOKERS[..])
    }

    pub const LEGENDARY_JOKERS_SIZE: usize = 5;

    pub const LEGENDARY_JOKERS: [BuffoonCard; Self::LEGENDARY_JOKERS_SIZE] = [
        card::CANIO,
        card::TRIBOULET,
        card::YORICK,
        card::CHICOT,
        card::PERKEO,
    ];

    #[must_use]
    pub fn pile_legendary() -> BuffoonPile {
        BuffoonPile::from(&Self::LEGENDARY_JOKERS[..])
    }
}

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
        enhancement: MPip::MultTimesPlusPerFullDeckSteel(2),
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
        // Scary Face: played face cards give +30 chips when scored.
        enhancement: MPip::ChipsPlusPerScoredFace(30),
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
        // Abstract Joker: +3 mult for each joker on the board.
        enhancement: MPip::MultPlusPerJoker(3),
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
        // Delayed Gratification: earn $2 per discard at end of round, but
        // only if no discard was used this round.
        enhancement: MPip::CashPerDiscardIfNoneUsed(2),
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
        enhancement: MPip::RetriggerPlayedRanks(1, ['2', '3', '4', '5']),
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
        enhancement: MPip::AllCardsAreFaces,
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
        // Gros Michel: +15 Mult, and a 1-in-6 chance of being destroyed at end
        // of round. The mult is the whole reason to play it; the const carried
        // only the destruction until EPIC-01a Phase 0b's audit caught it.
        enhancement: MPip::MultPlusChanceDestroyed(15, 1, 6),
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
        enhancement: MPip::ChipsPlusOn5Ranks(31, ['A', '9', '7', '5', '3']),
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
    /// **DIARY** I am constantly debating in my head how reasonable this all is.
    ///
    /// On the one hand I love how I can have a constant that represents the state, but I can also mutate
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
            weight: 916,
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
            weight: 917,
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
            weight: 918,
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
            weight: 919,
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
            weight: 896,
            pip_type: PipType::Joker,
            index: '∠',
            symbol: '∠',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Blackboard: ×3 mult if all cards held in hand are Spades or Clubs.
        enhancement: MPip::MultTimesIfHeldAllSuits(3, ['S', 'C']),
        resell_value: 3,
        debuffed: false,
    };
    pub const RUNNER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 891,
            pip_type: PipType::Joker,
            index: '∡',
            symbol: '∡',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Runner: +15 chips for each hand played containing a Straight.
        enhancement: MPip::GainChipsPerStraightHand(15),
        resell_value: 2,
        debuffed: false,
    };
    pub const ICE_CREAM: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 886,
            pip_type: PipType::Joker,
            index: '∢',
            symbol: '∢',
            value: 6,
        },
        card_type: BCardType::CommonJoker,
        // Ice Cream: +100 chips, −5 per hand played; floors at 0.
        enhancement: MPip::LoseChipsPerHand(100, 5),
        resell_value: 2,
        debuffed: false,
    };
    pub const DNA: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 881,
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
            weight: 876,
            pip_type: PipType::Joker,
            index: '∤',
            symbol: '∤',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::AllPlayedCardsScore,
        resell_value: 0,
        debuffed: false,
    };
    pub const BLUE_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 871,
            pip_type: PipType::Joker,
            index: '∥',
            symbol: '∥',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Blue Joker: +2 chips for each card remaining in the deck.
        enhancement: MPip::ChipsPerDeckCard(2),
        resell_value: 0,
        debuffed: false,
    };
    pub const SIXTH_SENSE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 866,
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
            weight: 861,
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
            weight: 856,
            pip_type: PipType::Joker,
            index: '∨',
            symbol: '∨',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Hiker: every played card permanently gains +4 chips when scored.
        enhancement: MPip::GainChipsOnScored(4),
        resell_value: 0,
        debuffed: false,
    };
    pub const FACELESS_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 851,
            pip_type: PipType::Joker,
            index: '∩',
            symbol: '∩',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Faceless Joker: earn $5 when 3 or more face cards are discarded at
        // the same time.
        enhancement: MPip::CashOnFacesDiscarded(5, 3),
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
        // Green Joker: +1 Mult per hand played, −1 per discard (net, floored ≥0).
        enhancement: MPip::GainMultPerHandLessDiscard(1),
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
        // Cavendish: ×3 Mult (unconditional), and a 1-in-1000 chance of being
        // destroyed at end of round — the Gros Michel compound shape on the
        // ×mult side. The const carried only the mult until EPIC-01a Phase 1c.
        enhancement: MPip::MultTimesChanceDestroyed(3, 1, 1000),
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
        // Square Joker: +4 chips for each hand played with exactly 4 cards.
        enhancement: MPip::GainChipsPerCardCountHand(4, 4),
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
        enhancement: MPip::GappedStraight,
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
            value: 8,
        },
        card_type: BCardType::RareJoker,
        // Baron: each King held in hand gives ×1.5 mult (compounds).
        enhancement: MPip::MultTimesPerHeldRank(15, 'K'),
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
        // Cloud 9: earn $1 for each 9 in the full deck at end of round.
        enhancement: MPip::CashPerFullDeckRank(1, '9'),
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
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::MultPlusPerMissingDeckCard(4),
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
        // To the Moon: earn $1 extra interest per $5 held at end of round,
        // capped at the base interest cap ($5).
        enhancement: MPip::ExtraInterest(1),
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
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::ChipsPerFullDeckStone(25),
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
        // Golden Joker: earn $4 at end of round — economy, not hand score.
        // Paid by `on_round_end`. (It was once mislabelled `Chips(4)`, which
        // made it silently add 0 chips.)
        enhancement: MPip::CashOnRoundEnd(4),
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
        // Baseball Card: Uncommon jokers each give ×1.5 mult (compounds).
        enhancement: MPip::MultTimesPerUncommonJoker(15),
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
        enhancement: MPip::ChipsPerDollar(2),
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
        // Spare Trousers: +2 Mult for each hand played containing a Two Pair.
        enhancement: MPip::GainMultPerTwoPairHand(2),
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
            weight: 801,
            pip_type: PipType::Joker,
            index: '🍜',
            symbol: '🍜',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Ramen: ×2 Mult, loses ×0.01 per card discarded; floors at ×1.
        enhancement: MPip::LoseMultTimesPerDiscard(200, 1),
        resell_value: 0,
        debuffed: false,
    };
    pub const WALKIE_TALKIE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 802,
            pip_type: PipType::Joker,
            index: '📻',
            symbol: '📻',
            value: 5,
        },
        card_type: BCardType::CommonJoker,
        // Walkie Talkie: each played 10 or 4 gives +10 chips and +4 mult.
        enhancement: MPip::ChipsMultPlusPerScoredRanks(10, 4, ['T', '4']),
        resell_value: 0,
        debuffed: false,
    };

    // 109 Sock and Buskin — Uncommon, $6. Retrigger all played face cards.
    pub const SOCK_AND_BUSKIN: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 803,
            pip_type: PipType::Joker,
            index: '🧦',
            symbol: '🧦',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::RetriggerPlayedFaces(1),
        resell_value: 3,
        debuffed: false,
    };

    // 113 Smeared Joker — Uncommon, $7. Hearts≡Diamonds, Spades≡Clubs for flushes.
    pub const SMEARED_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 806,
            pip_type: PipType::Joker,
            index: '🖍',
            symbol: '🖍',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::SmearedSuits,
        resell_value: 3,
        debuffed: false,
    };

    // 126 Oops! All 6s — Uncommon, $4. Doubles all listed probabilities.
    pub const OOPS_ALL_6S: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 807,
            pip_type: PipType::Joker,
            index: '🎲',
            symbol: '🎲',
            value: 4,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::DoubleOdds,
        resell_value: 2,
        debuffed: false,
    };

    // 115 Hanging Chad — Common, $4. Retrigger first played card 2 extra times.
    pub const HANGING_CHAD: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 804,
            pip_type: PipType::Joker,
            index: '🗳',
            symbol: '🗳',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::RetriggerFirstPlayed(2),
        resell_value: 2,
        debuffed: false,
    };

    // The "family" of Rare jokers that give ×Mult when the played hand contains
    // a given category (Balatro #131–135). "Contains" matches Balatro: e.g. The
    // Duo fires on any hand with at least a pair (two pair, trips, full house,
    // quads all count).
    pub const THE_DUO: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 665,
            pip_type: PipType::Joker,
            index: '👯',
            symbol: '👯',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::MultTimesOnPair(2),
        resell_value: 4,
        debuffed: false,
    };

    pub const THE_TRIO: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 660,
            pip_type: PipType::Joker,
            index: '🔱',
            symbol: '🔱',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::MultTimesOnTrips(3),
        resell_value: 4,
        debuffed: false,
    };

    pub const THE_FAMILY: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 655,
            pip_type: PipType::Joker,
            index: '👪',
            symbol: '👪',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::MultTimesOn4OfAKind(4),
        resell_value: 4,
        debuffed: false,
    };

    pub const THE_ORDER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 650,
            pip_type: PipType::Joker,
            index: '📏',
            symbol: '📏',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::MultTimesOnStraight(3),
        resell_value: 4,
        debuffed: false,
    };

    pub const THE_TRIBE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 645,
            pip_type: PipType::Joker,
            index: '🪶',
            symbol: '🪶',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::MultTimesOnFlush(2),
        resell_value: 4,
        debuffed: false,
    };

    // Legendary jokers (Balatro #146–150), only obtainable from The Soul card,
    // so they have no shop cost. Only Triboulet is a pure scoring effect; the
    // rest depend on systems not modelled yet (card destruction, discard
    // counters, boss blinds, consumables) and stay `Blank` for now.
    pub const CANIO: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 640,
            pip_type: PipType::Joker,
            index: '🎭',
            symbol: '🎭',
            value: 0,
        },
        card_type: BCardType::LegendaryJoker,
        enhancement: MPip::Blank, // gains ×1 mult when a face card is destroyed
        resell_value: 0,
        debuffed: false,
    };

    pub const TRIBOULET: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 635,
            pip_type: PipType::Joker,
            index: '👑',
            symbol: '👑',
            value: 0,
        },
        card_type: BCardType::LegendaryJoker,
        // Played Kings and Queens each give ×2 mult when scored.
        enhancement: MPip::MultTimesPerScoredRank(2, ['K', 'Q']),
        resell_value: 0,
        debuffed: false,
    };

    pub const YORICK: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 630,
            pip_type: PipType::Joker,
            index: '💀',
            symbol: '💀',
            value: 0,
        },
        card_type: BCardType::LegendaryJoker,
        enhancement: MPip::Blank, // gains ×1 mult every 23 cards discarded
        resell_value: 0,
        debuffed: false,
    };

    pub const CHICOT: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 625,
            pip_type: PipType::Joker,
            index: '🎪',
            symbol: '🎪',
            value: 0,
        },
        card_type: BCardType::LegendaryJoker,
        enhancement: MPip::Blank, // disables the effect of every Boss Blind
        resell_value: 0,
        debuffed: false,
    };

    pub const PERKEO: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 620,
            pip_type: PipType::Joker,
            index: '🧪',
            symbol: '🧪',
            value: 0,
        },
        card_type: BCardType::LegendaryJoker,
        enhancement: MPip::Blank, // creates a Negative copy of a consumable
        resell_value: 0,
        debuffed: false,
    };
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

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__joker_tests {
    use super::*;
    use crate::bcards;
    use crate::funky::decks::basic::card as basic;
    use crate::funky::decks::tarot::MajorArcana;
    use crate::funky::types::board::BuffoonBoard;
    use crate::funky::types::draws::Draws;
    use crate::funky::types::mpip::MPip;
    use crate::preludes::funky::{BCardType, BuffoonPile, Deck};
    use std::collections::HashSet;

    /// Every joker const defined in this file, in declaration order. The single
    /// iteration point for crate-wide data invariants (weight uniqueness, the
    /// scoring-reachability guard). A joker is only protected by those guards
    /// once it is listed here; `all_jokers__is_superset_of_every_pile` keeps the
    /// four rarity piles from drifting out of it.
    const ALL_JOKERS: [BuffoonCard; 109] = [
        card::JOKER,
        card::GREEDY_JOKER,
        card::LUSTY_JOKER,
        card::WRATHFUL_JOKER,
        card::GLUTTONOUS_JOKER,
        card::JOLLY_JOKER,
        card::ZANY_JOKER,
        card::MAD_JOKER,
        card::CRAZY_JOKER,
        card::DROLL_JOKER,
        card::SLY_JOKER,
        card::WILY_JOKER,
        card::CLEVER_JOKER,
        card::DEVIOUS_JOKER,
        card::CRAFTY_JOKER,
        card::HALF_JOKER,
        card::JOKER_STENCIL,
        card::FOUR_FINGERS,
        card::MIME,
        card::CREDIT_CARD,
        card::CEREMONIAL_DAGGER,
        card::BANNER,
        card::MYSTIC_SUMMIT,
        card::MARBLE_JOKER,
        card::LOYALTY_CARD,
        card::EIGHT_BALL,
        card::MISPRINT,
        card::DUSK,
        card::RAISED_FIST,
        card::CHAOS_THE_CLOWN,
        card::FIBONACCI,
        card::STEEL_JOKER,
        card::SCARY_FACE,
        card::ABSTRACT_JOKER,
        card::DELAYED_GRATIFICATION,
        card::HACK,
        card::PAREIDOLIA,
        card::GROS_MICHEL,
        card::EVEN_STEVEN,
        card::ODD_TODD,
        card::SCHOLAR,
        card::BUSINESS_CARD,
        card::SUPERNOVA,
        card::RIDE_THE_BUS,
        card::SPACE_JOKER,
        card::EGG,
        card::BURGLAR,
        card::BLACKBOARD,
        card::RUNNER,
        card::ICE_CREAM,
        card::DNA,
        card::SPLASH,
        card::BLUE_JOKER,
        card::SIXTH_SENSE,
        card::CONSTELLATION,
        card::HIKER,
        card::FACELESS_JOKER,
        card::GREEN_JOKER,
        card::SUPERPOSITION,
        card::TO_DO_LIST,
        card::CAVENDISH,
        card::CARD_SHARP,
        card::RED_CARD,
        card::MADNESS,
        card::SQUARE_JOKER,
        card::SEANCE,
        card::RIFF_RAFF,
        card::VAMPIRE,
        card::SHORTCUT,
        card::HOLOGRAM,
        card::VAGABOND,
        card::BARON,
        card::CLOUD_9,
        card::ROCKET,
        card::EROSION,
        card::RESERVED_PARKING,
        card::MAIL_IN_REBATE,
        card::TO_THE_MOON,
        card::HALLUCINATION,
        card::FORTUNE_TELLER,
        card::JUGGLER,
        card::DRUNKARD,
        card::STONE_JOKER,
        card::GOLDEN_JOKER,
        card::LUCKY_CAT,
        card::BASEBALL_CARD,
        card::BULL,
        card::DIET_COLA,
        card::TRADING_CARD,
        card::FLASH_CARD,
        card::POPCORN,
        card::SPARE_TROUSERS,
        card::ANCIENT_JOKER,
        card::RAMEN,
        card::WALKIE_TALKIE,
        card::SOCK_AND_BUSKIN,
        card::HANGING_CHAD,
        card::SMEARED_JOKER,
        card::OOPS_ALL_6S,
        card::THE_DUO,
        card::THE_TRIO,
        card::THE_FAMILY,
        card::THE_ORDER,
        card::THE_TRIBE,
        card::CANIO,
        card::TRIBOULET,
        card::YORICK,
        card::CHICOT,
        card::PERKEO,
    ];

    /// Every joker in the four rarity piles must appear in [`ALL_JOKERS`], so the
    /// crate-wide guards can never silently skip an in-game joker.
    #[test]
    fn all_jokers__is_superset_of_every_pile() {
        let registry: HashSet<_> = ALL_JOKERS.iter().collect();
        for pile in [
            &Joker::COMMON_JOKERS[..],
            &Joker::UNCOMMON_JOKERS[..],
            &Joker::RARE_JOKERS[..],
            &Joker::LEGENDARY_JOKERS[..],
        ] {
            for card in pile {
                assert!(
                    registry.contains(card),
                    "{card} is in a rarity pile but missing from ALL_JOKERS"
                );
            }
        }
    }

    /// `weight` is a joker's sort key; distinct jokers must not share one, or
    /// their display/sort order is ambiguous.
    #[test]
    fn all_jokers__weights_are_unique() {
        let mut seen = HashSet::new();
        let mut collisions: Vec<(usize, String)> = Vec::new();
        for card in ALL_JOKERS {
            let weight = card.rank.weight;
            if !seen.insert(weight) {
                collisions.push((weight, format!("{card}")));
            }
        }
        collisions.sort();
        assert!(
            collisions.is_empty(),
            "{} jokers share a weight with an earlier joker: {collisions:?}",
            collisions.len()
        );
    }

    /// Baron is Rare / $8 in Balatro, not a $5 Common. It must carry the right
    /// rarity and cost and live in the Rare pile, not adrift.
    #[test]
    fn baron__is_rare_dollar_eight_and_piled() {
        assert_eq!(
            card::BARON.card_type,
            BCardType::RareJoker,
            "Baron is a Rare joker in Balatro"
        );
        assert_eq!(card::BARON.rank.value, 8, "Baron costs $8 in Balatro");
        assert!(
            Joker::RARE_JOKERS.contains(&card::BARON),
            "Baron belongs in RARE_JOKERS"
        );
    }

    #[test]
    fn erosion_and_stone_joker__are_uncommon_dollar_six_and_piled() {
        // Both were tagged Common / $5 and left out of every rarity pile, the
        // same drift the Baron fix corrected.
        for (joker, name) in [
            (card::EROSION, "Erosion"),
            (card::STONE_JOKER, "Stone Joker"),
        ] {
            assert_eq!(
                joker.card_type,
                BCardType::UncommonJoker,
                "{name} is an Uncommon joker in Balatro"
            );
            assert_eq!(joker.rank.value, 6, "{name} costs $6 in Balatro");
            assert!(
                Joker::UNCOMMON_JOKERS.contains(&joker),
                "{name} belongs in UNCOMMON_JOKERS"
            );
        }
    }

    /// Does this `MPip` variant *intend* to add a deterministic contribution to
    /// the hand score — a pure function of the current played hand, held cards,
    /// or on-board resources (money, discards, deck, jokers)? This is the
    /// **intent** oracle, independent of whether a scoring arm exists yet, so the
    /// reachability guard can catch a variant that is assigned to a joker but
    /// never wired (the Banner / Mystic Summit silent-zero class).
    ///
    /// `false` covers everything that legitimately contributes nothing to the
    /// *current* hand score: the `Blank` sentinel, economy, per-run counters
    /// (their base is 0), retriggers, card create/destroy, detection-only rule
    /// modifiers, and every probabilistic effect (those resolve through the
    /// seeded-RNG path, not deterministic scoring). The match is exhaustive on
    /// purpose: a new `MPip` variant will not compile until it is classified.
    // One arm per variant — a long but flat exhaustive match, like `Display`.
    #[allow(clippy::too_many_lines)]
    fn scores_hand(mpip: MPip) -> bool {
        match mpip {
            // --- deterministic hand-score contributions ---
            MPip::AddBaseChips(_)
            | MPip::Chips(_)
            | MPip::ChipsMultPlus(_, _)
            | MPip::ChipsMultPlusOnHand(_, _, _)
            | MPip::ChipsMultPlusPerScoredRanks(_, _, _)
            | MPip::ChipsOnFlush(_)
            | MPip::ChipsOnPair(_)
            | MPip::ChipsOn2Pair(_)
            | MPip::ChipsOnStraight(_)
            | MPip::ChipsOnTrips(_)
            | MPip::ChipsPerDeckCard(_)
            | MPip::ChipsPerFullDeckStone(_)
            | MPip::ChipsPerDollar(_)
            | MPip::ChipsPerRemainingDiscard(_)
            | MPip::ChipsPlusOn5Ranks(_, _)
            | MPip::ChipsPlusPerScoredFace(_)
            | MPip::MultPlus(_)
            // Gros Michel scores its +mult unconditionally; the destruction half
            // is a separate, end-of-round concern that does not gate it.
            // Cavendish is the same compound shape on the ×mult side.
            | MPip::MultPlusChanceDestroyed(_, _, _)
            | MPip::MultTimesChanceDestroyed(_, _, _)
            | MPip::MultPlusChipsOnRank(_, _, _)
            | MPip::MultPlusOn5Ranks(_, _)
            | MPip::MultPlusOnFlush(_)
            | MPip::MultPlusOnPair(_)
            | MPip::MultPlusOn2Pair(_)
            | MPip::MultPlusOnStraight(_)
            | MPip::MultPlusOnTrips(_)
            | MPip::MultPlusOnSuit(_, _)
            | MPip::MultPlusOnUpToXCards(_, _)
            | MPip::MultPlusOnZeroDiscards(_)
            | MPip::MultPlusZeroDiscards(_)
            | MPip::MultPlusPerJoker(_)
            | MPip::MultPlusPerMissingDeckCard(_)
            | MPip::MultPlusXOnLowestRankInHand(_)
            | MPip::MultTimes(_)
            | MPip::MultTimes1Dot(_)
            | MPip::MultTimesOnEmptyJokerSlots(_)
            | MPip::MultTimesOnPair(_)
            | MPip::MultTimesOnTrips(_)
            | MPip::MultTimesOn4OfAKind(_)
            | MPip::MultTimesOnStraight(_)
            | MPip::MultTimesOnFlush(_)
            | MPip::MultTimesPerScoredRank(_, _)
            | MPip::MultTimesPerHeldRank(_, _)
            | MPip::MultTimesPerUncommonJoker(_)
            | MPip::MultTimesPlusPerFullDeckSteel(_)
            | MPip::MultTimesIfHeldAllSuits(_, _)
            | MPip::GainMultPerHandLessDiscard(_)
            | MPip::LoseMultTimesPerDiscard(_, _)
            | MPip::LoseChipsPerHand(_, _)
            | MPip::GainChipsPerCardCountHand(_, _)
            | MPip::GainMultPerTwoPairHand(_)
            | MPip::GainChipsPerStraightHand(_)
            // Hiker fattens the cards rather than itself, but the boost lands on
            // the hand that triggers it, so it does change the score.
            | MPip::GainChipsOnScored(_)
            // Retriggers re-score cards, so they score (Hack: each played 2-5;
            // Sock and Buskin: each played face; Hanging Chad: the first played
            // card twice more; Mime: held-card abilities). Dusk retrigger stays
            // non-scoring below until its final-round path lands.
            | MPip::RetriggerPlayedRanks(_, _)
            | MPip::RetriggerPlayedFaces(_)
            | MPip::RetriggerFirstPlayed(_)
            | MPip::RetriggerCardsInHand(_)
            // Rule modifiers change hand classification (Four Fingers: 4-card
            // straights/flushes; Shortcut: gapped straights; Smeared: merged-suit
            // flushes), so they change the base hand score and can enable
            // straight/flush jokers.
            | MPip::FourFlushAndStraight
            | MPip::GappedStraight
            | MPip::SmearedSuits
            // Glass card: xn mult when scored. The 1-in-N destruction rides the
            // same variant but does not gate the mult, so this scores flatly --
            // the Gros Michel shape, on a card instead of a joker.
            | MPip::Glass(_, _)
            // Stone card: +n chips when scored (and no rank/suit for detection,
            // which is why it is still in KNOWN_UNWIRED_CARD_ENHANCEMENTS).
            | MPip::Stone(_) => true,

            // --- sentinel / non-scoring (economy, counters, retrigger, create,
            //     detection, probabilistic) ---
            MPip::Blank
            | MPip::AddCardTypeWhenBlindSelected(_)
            | MPip::ChanceDestroyed(_, _)
            // The `+$` payouts move money at lifecycle events (round end,
            // discard), never the hand score. Bull turns money into chips,
            // but that is Bull's arm, not theirs.
            | MPip::CashOnRoundEnd(_)
            | MPip::CashPerDiscardIfNoneUsed(_)
            | MPip::CashPerFullDeckRank(_, _)
            | MPip::ExtraInterest(_)
            | MPip::CashOnFacesDiscarded(_, _)
            | MPip::CreateCardOnRankPlay(_, _, _)
            | MPip::Credit(_)
            | MPip::Death(_)
            | MPip::DoubleMoney(_)
            // Pareidolia: a detection hook with no standalone score — it only
            // amplifies the face-reading jokers (Scary Face, Sock and Buskin),
            // so on its own it changes nothing and is intentionally non-scoring.
            | MPip::AllCardsAreFaces
            // Splash: inert here — this engine already scores every played card,
            // so "all played cards score" is a no-op, not a silent-zero bug.
            | MPip::AllPlayedCardsScore
            // Oops! All 6s: only affects the probabilistic (seeded-RNG) path, so
            // it is invisible to the pure-score reachability guard — a modifier,
            // not a deterministic scorer.
            | MPip::DoubleOdds
            | MPip::FreeReroll(_)
            | MPip::Gold(_)
            | MPip::Hanged(_)
            | MPip::JokersValue(_)
            | MPip::Lucky(_, _)
            | MPip::MultPlusDoubleValueDestroyJokerOnRight(_)
            | MPip::MultPlusOnConsecutiveHandsNo3Ranks(_, _, _)
            | MPip::MultPlusOnHandPlays
            | MPip::MultPlusRandomTo(_)
            | MPip::MultTimesEveryXHands(_, _)
            | MPip::Planet(_)
            | MPip::RandomJoker(_)
            | MPip::RandomTarot(_)
            | MPip::RetriggerPlayedCardsInFinalRound
            | MPip::SellValueIncrement(_)
            | MPip::Strength
            | MPip::Odds1in(_)
            | MPip::Odds1inCashOn3Ranks(_, _, _)
            | MPip::Odds1inUpgradeHand(_)
            | MPip::Wild(_)
            | MPip::Diamonds(_)
            | MPip::Clubs(_)
            | MPip::Hearts(_)
            | MPip::Spades(_)
            | MPip::Custom(_) => false,
        }
    }

    /// A battery of boards crafted so that **every** wired hand-scoring variant
    /// fires on at least one of them: a rich royal flush (flush, straight, faces,
    /// high ranks, held Kings in Spades/Clubs, money, remaining discards, a
    /// non-empty deck and an Uncommon joker present for Baseball Card), the same
    /// with zero discards, plus two-pair, small-trips and four-of-a-kind hands
    /// covering the pair/trips/quads and low-card-count conditions.
    fn probe_boards() -> Vec<BuffoonBoard> {
        let mk = |played: &str, held: &str, money: isize, discards: usize| {
            let mut b = BuffoonBoard::new(Draws::new(4, discards), Deck::basic_buffoon_pile());
            b.played = bcards!(played);
            b.in_hand = bcards!(held);
            b.money = money;
            // An Uncommon joker so Baseball Card (×per-Uncommon) has something to
            // count; its own effect is present in both baseline and probe, so it
            // cancels out of the marginal.
            b.jokers.push(card::MYSTIC_SUMMIT);
            b
        };
        // A board holding a Steel card (×1.5 while held), so held-card
        // retriggers (Mime) have a non-trivial held op to double and stay
        // reachable. Steel is an enhancement, not a rank/suit change, so the
        // other held-reading jokers (Baron, Blackboard) are unaffected.
        let mut steel_held = mk("AH KH QH JH TH", "KS KC", 0, 3);
        let steel_king = BuffoonCard {
            enhancement: MPip::STEEL,
            ..bcards!("KS").iter().next().copied().unwrap()
        };
        steel_held.in_hand.push(steel_king);
        // A worn deck: cards enhanced to Steel and Stone, and cards destroyed
        // outright. Only the full-deck *roster* moves here — `deck` (the undealt
        // remainder) is left alone, since that is the distinction the "in full
        // deck" jokers (Steel Joker, Stone Joker, Erosion) turn on.
        let mut worn_deck = mk("AH KH QH JH TH", "KS KC", 0, 3);
        let mut wear = |enhancement: MPip, n: usize| {
            for _ in 0..n {
                let card = worn_deck.full_deck.remove(0);
                worn_deck.full_deck.push(BuffoonCard {
                    enhancement,
                    ..card
                });
            }
        };
        wear(MPip::STEEL, 2);
        wear(MPip::TOWER, 2);
        // Destroy three, putting the deck below its starting size for Erosion.
        for _ in 0..3 {
            worn_deck.full_deck.remove(0);
        }
        vec![
            mk("AH KH QH JH TH", "KS KC", 100, 3),
            mk("AH KH QH JH TH", "KS KC", 0, 0),
            mk("KH KS QD QC 4S", "KS KC", 0, 3),
            mk("5H 5S 5D", "KS KC", 0, 3),
            mk("8H 8S 8D 8C", "KS KC", 0, 3),
            steel_held,
            // A bare four-card straight flush (9-T-J-Q of Hearts + an off card):
            // a High Card under vanilla rules, a Straight Flush under Four
            // Fingers — so its rule modifier is reachable.
            mk("9H TH JH QH 2S", "KS KC", 0, 3),
            // A one-gap five-card straight (2-4-6-8-T): a High Card under vanilla
            // rules, a Straight under Shortcut — so its modifier is reachable.
            mk("2C 4D 6H 8S TC", "KS KC", 0, 3),
            // Five red cards across two suits (3 Hearts + 2 Diamonds), no pair or
            // straight: a High Card normally, a Flush under Smeared — so its
            // modifier is reachable.
            mk("AH KH 9H QD JD", "KS KC", 0, 3),
            worn_deck,
        ]
    }

    /// A joker is *reachable* if adding it to some probe board changes the score.
    /// In-round events are fired after the joker is added so counter jokers
    /// (Green Joker, Ramen, …) accumulate — and the scored-card mutations (Hiker)
    /// land — before the score is read.
    fn is_reachable(joker: BuffoonCard) -> bool {
        // Hands that satisfy every growth condition across the slice: a 4-card
        // straight (Square + Runner), a two-pair hand (Spare Trousers), and a
        // generic hand (Green Joker, Ice Cream).
        let growth_hands = [
            bcards!("2C 3C 4C 5C"),
            bcards!("KH KS QD QC 4S"),
            bcards!("AH KH QH JH TH"),
        ];
        probe_boards().into_iter().any(|mut board| {
            let baseline = board.score();
            board.jokers.push(joker);
            for hand in &growth_hands {
                board.on_hand_played(hand);
            }
            board.on_discard(&bcards!("2C 3C 4C"));
            board.on_scored();
            board.score() != baseline
        })
    }

    /// Jokers whose enhancement *intends* to score but has no wiring yet — the
    /// audit output of EPIC-01a Phase 0b. Each is a (near-)pure function of board
    /// state and cheap to wire; they are listed here so the reachability guard
    /// stays green while the debt stays visible. **Remove an entry when you wire
    /// it** — the guard fails if a listed joker starts scoring, forcing the
    /// cleanup, and fails if a *new* unlisted joker silently scores 0.
    const KNOWN_UNWIRED: [BuffoonCard; 1] = [
        // MultTimesOnEmptyJokerSlots(1): ×1 per empty joker slot needs a real
        // joker-slot *limit* on the board (Vec capacity is not the game's 5-slot
        // rule), so it waits on that Phase 3/8 state.
        card::JOKER_STENCIL,
    ];

    /// Crate-wide silent-zero guard. Every joker whose enhancement *intends* to
    /// score (per [`scores_hand`]) must actually change the score on some probe
    /// board — otherwise it is assigned-but-unwired, the exact failure that hid
    /// Banner and Mystic Summit. The still-unwired scorers are tracked in
    /// [`KNOWN_UNWIRED`]; anything outside that set is a fresh bug.
    #[test]
    fn all_jokers__intended_hand_scorers_are_reachable() {
        let known: HashSet<_> = KNOWN_UNWIRED.iter().collect();
        let mut new_silent_zero = Vec::new();
        let mut now_wired = Vec::new();
        let mut misclassified = Vec::new();
        for joker in ALL_JOKERS {
            let intends = scores_hand(joker.enhancement);
            let reachable = is_reachable(joker);
            let listed = known.contains(&joker);
            if intends && !reachable && !listed {
                new_silent_zero.push(format!("{joker}"));
            }
            if listed && reachable {
                now_wired.push(format!("{joker}"));
            }
            if !intends && reachable {
                misclassified.push(format!("{joker}"));
            }
        }
        assert!(
            misclassified.is_empty(),
            "these scored but are classified non-scoring — reclassify in `scores_hand`: {misclassified:?}"
        );
        assert!(
            now_wired.is_empty(),
            "these are wired now — remove them from KNOWN_UNWIRED: {now_wired:?}"
        );
        assert!(
            new_silent_zero.is_empty(),
            "these jokers intend to score but silently add nothing (wire them or, if intentional, adjust the data): {new_silent_zero:?}"
        );
    }

    /// Every `MPip` a **playing card** can end up wearing, derived rather than
    /// hand-listed: stamp each tarot onto a plain card via [`BuffoonCard::enhance`]
    /// and read back what stuck. Tarots that mutate rank or suit (Strength, the
    /// four suit-changers) or that act on the run rather than the card (Death,
    /// Judgement, …) leave the enhancement `Blank` and drop out.
    ///
    /// Deriving it means a **new tarot joins the guard automatically** — the
    /// registry cannot silently fall behind the deck the way a hand-written list
    /// would.
    ///
    /// [`BuffoonCard::enhance`]: crate::funky::types::buffoon_card::BuffoonCard::enhance
    fn all_card_enhancements() -> Vec<MPip> {
        let plain = basic::KING_HEARTS;
        let mut found: Vec<MPip> = MajorArcana::DECK
            .iter()
            .map(|tarot| plain.enhance(*tarot).enhancement)
            .filter(|enhancement| *enhancement != MPip::Blank)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        found.sort_unstable();
        found
    }

    /// A card enhancement is *reachable* if a card wearing it scores differently
    /// from the same card plain — in **either** the played or the held position.
    /// Both are probed because the two are wired through different folds: a Bonus
    /// card's chips land in phase 2, a Steel card's ×1.5 only in phase 3, and an
    /// enhancement wired to the wrong one scores nothing where it counts.
    fn is_card_enhancement_reachable(enhancement: MPip) -> bool {
        let plain = basic::KING_HEARTS;
        let enhanced = BuffoonCard {
            enhancement,
            ..plain
        };

        // The board needs a real hand under it: a held ×mult multiplies the
        // running score, so it cannot show up against a score of zero.
        let probe = |plain_card: BuffoonCard, enhanced_card: BuffoonCard, held: bool| {
            let mut board = BuffoonBoard::new(Draws::new(4, 3), Deck::basic_buffoon_pile());
            board.played = bcards!("2S 5D 8C TS");
            let mut with = |card: BuffoonCard| {
                if held {
                    board.in_hand = BuffoonPile::from(vec![card]);
                } else {
                    board.played = bcards!("2S 5D 8C TS");
                    board.played.push(card);
                }
                board.score()
            };
            with(plain_card) != with(enhanced_card)
        };

        probe(plain, enhanced, false) || probe(plain, enhanced, true)
    }

    /// Card enhancements that *intend* to score but have no wiring yet. The
    /// card-level twin of [`KNOWN_UNWIRED`] — same contract: remove an entry when
    /// you wire it, and the guard fails if a listed one starts scoring.
    const KNOWN_UNWIRED_CARD_ENHANCEMENTS: [MPip; 1] = [
        // Stone card (TOWER): +50 chips, and no rank or suit for detection
        // purposes. The chips are trivial, but the rank/suit suppression is not
        // — a Stone card must not count toward a straight or flush, which needs
        // a detection hook rather than a scoring arm. Wiring only the chips
        // would trade a silent zero for a silently *wrong* hand type, so it
        // waits for both halves to land together.
        MPip::TOWER,
    ];

    /// The card-level silent-zero guard — the twin of
    /// [`all_jokers__intended_hand_scorers_are_reachable`], which iterates
    /// `ALL_JOKERS` and therefore cannot see a *card* enhancement at all. That
    /// blind spot is exactly how the Glass card kept its ×2 mult unwired through
    /// the whole of Phase 0b.
    ///
    /// Same contract as the joker guard, against the same intent oracle: an
    /// enhancement a card can wear, whose variant claims to score, must actually
    /// move the score somewhere.
    #[test]
    fn all_card_enhancements__intended_hand_scorers_are_reachable() {
        let known: HashSet<_> = KNOWN_UNWIRED_CARD_ENHANCEMENTS.iter().collect();
        let mut new_silent_zero = Vec::new();
        let mut now_wired = Vec::new();
        let mut misclassified = Vec::new();
        for enhancement in all_card_enhancements() {
            let intends = scores_hand(enhancement);
            let reachable = is_card_enhancement_reachable(enhancement);
            let listed = known.contains(&enhancement);
            if intends && !reachable && !listed {
                new_silent_zero.push(format!("{enhancement}"));
            }
            if listed && reachable {
                now_wired.push(format!("{enhancement}"));
            }
            if !intends && reachable {
                misclassified.push(format!("{enhancement}"));
            }
        }
        assert!(
            misclassified.is_empty(),
            "these card enhancements scored but are classified non-scoring — reclassify in `scores_hand`: {misclassified:?}"
        );
        assert!(
            now_wired.is_empty(),
            "these are wired now — remove them from KNOWN_UNWIRED_CARD_ENHANCEMENTS: {now_wired:?}"
        );
        assert!(
            new_silent_zero.is_empty(),
            "these card enhancements intend to score but silently add nothing: {new_silent_zero:?}"
        );
    }

    /// Shared invariants for a rarity pile: it has the declared size, every card
    /// is a joker tagged with the expected rarity, and no card is duplicated.
    fn assert_rarity_pile(cards: &[BuffoonCard], size: usize, rarity: BCardType) {
        assert_eq!(cards.len(), size, "declared size does not match the array");
        for card in cards {
            assert!(card.is_joker(), "{card} is not a joker");
            assert_eq!(card.card_type, rarity, "{card} is not tagged {rarity:?}");
        }
        let distinct: HashSet<_> = cards.iter().collect();
        assert_eq!(distinct.len(), size, "the pile contains duplicate cards");
    }

    #[test]
    fn common_jokers__data_invariants() {
        assert_rarity_pile(
            &Joker::COMMON_JOKERS,
            Joker::COMMON_JOKERS_SIZE,
            BCardType::CommonJoker,
        );
        assert_eq!(Joker::pile_common().len(), Joker::COMMON_JOKERS_SIZE);
    }

    #[test]
    fn uncommon_jokers__data_invariants() {
        assert_rarity_pile(
            &Joker::UNCOMMON_JOKERS,
            Joker::UNCOMMON_JOKERS_SIZE,
            BCardType::UncommonJoker,
        );
        assert_eq!(Joker::pile_uncommon().len(), Joker::UNCOMMON_JOKERS_SIZE);
    }

    #[test]
    fn rare_jokers__data_invariants() {
        assert_rarity_pile(
            &Joker::RARE_JOKERS,
            Joker::RARE_JOKERS_SIZE,
            BCardType::RareJoker,
        );
        assert_eq!(Joker::pile_rare().len(), Joker::RARE_JOKERS_SIZE);
    }

    #[test]
    fn legendary_jokers__data_invariants() {
        assert_rarity_pile(
            &Joker::LEGENDARY_JOKERS,
            Joker::LEGENDARY_JOKERS_SIZE,
            BCardType::LegendaryJoker,
        );
        assert_eq!(Joker::pile_legendary().len(), Joker::LEGENDARY_JOKERS_SIZE);
    }

    #[test]
    fn rarity_piles__no_card_appears_in_two_piles() {
        let mut all: Vec<BuffoonCard> = Vec::new();
        all.extend(Joker::COMMON_JOKERS);
        all.extend(Joker::UNCOMMON_JOKERS);
        all.extend(Joker::RARE_JOKERS);
        all.extend(Joker::LEGENDARY_JOKERS);

        let distinct: HashSet<_> = all.iter().collect();
        assert_eq!(
            distinct.len(),
            all.len(),
            "a joker appears in more than one rarity pile"
        );
    }
}
