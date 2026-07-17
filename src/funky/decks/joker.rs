use crate::preludes::funky::{BuffoonCard, BuffoonPile};

pub struct Joker {}

impl Joker {
    pub const COMMON_JOKERS_SIZE: usize = 56;

    pub const COMMON_JOKERS: [BuffoonCard; Self::COMMON_JOKERS_SIZE] = [
        card::SUPERPOSITION,
        card::RIFF_RAFF,
        card::FORTUNE_TELLER,
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
        card::SCARY_FACE,
        card::ABSTRACT_JOKER,
        card::DELAYED_GRATIFICATION,
        card::GROS_MICHEL,
        card::EVEN_STEVEN,
        card::ODD_TODD,
        card::SCHOLAR,
        card::BUSINESS_CARD,
        card::SUPERNOVA,
        card::RIDE_THE_BUS,
        card::EGG,
        card::RUNNER,
        card::ICE_CREAM,
        card::SPLASH,
        card::BLUE_JOKER,
        card::FACELESS_JOKER,
        card::GREEN_JOKER,
        card::TO_DO_LIST,
        card::CAVENDISH,
        card::RED_CARD,
        card::SQUARE_JOKER,
        card::RESERVED_PARKING,
        card::MAIL_IN_REBATE,
        card::HALLUCINATION,
        card::JUGGLER,
        card::DRUNKARD,
        card::GOLDEN_JOKER,
        card::POPCORN,
        card::WALKIE_TALKIE,
        card::MYSTIC_SUMMIT,
    ];

    #[must_use]
    pub fn pile_common() -> BuffoonPile {
        BuffoonPile::from(&Self::COMMON_JOKERS[..])
    }

    pub const UNCOMMON_JOKERS_SIZE: usize = 41;

    pub const UNCOMMON_JOKERS: [BuffoonCard; Self::UNCOMMON_JOKERS_SIZE] = [
        card::JOKER_STENCIL,
        card::FOUR_FINGERS,
        card::MIME,
        card::CEREMONIAL_DAGGER,
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
        card::SELTZER,
        card::HOLOGRAM,
        card::CONSTELLATION,
        card::VAMPIRE,
        card::LUCHADOR,
        card::MATADOR,
        card::MADNESS,
        card::CARD_SHARP,
        card::ROCKET,
        card::SPACE_JOKER,
        card::BURGLAR,
        card::BLACKBOARD,
        card::SIXTH_SENSE,
        card::HIKER,
        card::SHORTCUT,
        card::CLOUD_9,
        card::TO_THE_MOON,
        card::LUCKY_CAT,
        card::BULL,
        card::DIET_COLA,
        card::TRADING_CARD,
        card::FLASH_CARD,
        card::SPARE_TROUSERS,
        card::RAMEN,
        card::SEANCE,
    ];

    #[must_use]
    pub fn pile_uncommon() -> BuffoonPile {
        BuffoonPile::from(&Self::UNCOMMON_JOKERS[..])
    }

    pub const RARE_JOKERS_SIZE: usize = 10;

    pub const RARE_JOKERS: [BuffoonCard; Self::RARE_JOKERS_SIZE] = [
        card::VAGABOND,
        card::ANCIENT_JOKER,
        card::THE_DUO,
        card::THE_TRIO,
        card::THE_FAMILY,
        card::THE_ORDER,
        card::THE_TRIBE,
        card::BARON,
        card::DNA,
        card::BASEBALL_CARD,
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
            value: 5,
        },
        card_type: BCardType::CommonJoker,
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
        resell_value: 3,
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
            value: 4,
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
            value: 4,
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
            value: 4,
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
        card_type: BCardType::UncommonJoker,
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
            value: 4,
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
        card_type: BCardType::UncommonJoker,
        // Burglar: when a Blind is selected, gain +3 hands and lose all
        // discards (wiping any discard bonus, e.g. Drunkard's).
        enhancement: MPip::GainHandsLoseDiscardsWhenBlindSelected(3),
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
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
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
            value: 5,
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
            value: 8,
        },
        card_type: BCardType::RareJoker,
        enhancement: MPip::Blank,
        resell_value: 4,
        debuffed: false,
    };
    pub const SPLASH: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 876,
            pip_type: PipType::Joker,
            index: '∤',
            symbol: '∤',
            value: 3,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::AllPlayedCardsScore,
        resell_value: 1,
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
        resell_value: 2,
        debuffed: false,
    };
    pub const SIXTH_SENSE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 866,
            pip_type: PipType::Joker,
            index: '∦',
            symbol: '∦',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };
    // 55 Constellation — Uncommon, $6. Was tagged Common / $5 and unpiled.
    pub const CONSTELLATION: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 861,
            pip_type: PipType::Joker,
            index: '∧',
            symbol: '∧',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        // Constellation: gains ×0.1 mult per Planet card used.
        enhancement: MPip::GainMultTimesPerPlanetUsed(10),
        resell_value: 3,
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
        card_type: BCardType::UncommonJoker,
        // Hiker: every played card permanently gains +4 chips when scored.
        enhancement: MPip::GainChipsOnScored(4),
        resell_value: 2,
        debuffed: false,
    };
    pub const FACELESS_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 851,
            pip_type: PipType::Joker,
            index: '∩',
            symbol: '∩',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Faceless Joker: earn $5 when 3 or more face cards are discarded at
        // the same time.
        enhancement: MPip::CashOnFacesDiscarded(5, 3),
        resell_value: 2,
        debuffed: false,
    };
    pub const GREEN_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 845,
            pip_type: PipType::Joker,
            index: '∪',
            symbol: '∪',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Green Joker: +1 Mult per hand played, −1 per discard (net, floored ≥0).
        enhancement: MPip::GainMultPerHandLessDiscard(1),
        resell_value: 2,
        debuffed: false,
    };
    pub const SUPERPOSITION: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 840,
            pip_type: PipType::Joker,
            index: '∫',
            symbol: '∫',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Superposition: create a Tarot when the hand is a Straight holding an Ace.
        enhancement: MPip::CreateTarotOnAceStraight,
        resell_value: 2,
        debuffed: false,
    };
    pub const TO_DO_LIST: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 835,
            pip_type: PipType::Joker,
            index: '∬',
            symbol: '∬',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::Blank,
        resell_value: 2,
        debuffed: false,
    };
    pub const CAVENDISH: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 830,
            pip_type: PipType::Joker,
            index: '∭',
            symbol: '∭',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Cavendish: ×3 Mult (unconditional), and a 1-in-1000 chance of being
        // destroyed at end of round — the Gros Michel compound shape on the
        // ×mult side. The const carried only the mult until EPIC-01a Phase 1c.
        enhancement: MPip::MultTimesChanceDestroyed(3, 1, 1000),
        resell_value: 2,
        debuffed: false,
    };
    pub const CARD_SHARP: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 825,
            pip_type: PipType::Joker,
            index: '∮',
            symbol: '∮',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        // Card Sharp: x3 mult if this hand type was already played this round.
        enhancement: MPip::MultTimesOnRepeatedHandThisRound(3),
        resell_value: 3,
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
        resell_value: 2,
        debuffed: false,
    };
    pub const MADNESS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 815,
            pip_type: PipType::Joker,
            index: '∰',
            symbol: '∰',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        // Madness: on a Small or Big Blind (never a Boss), gain x0.5 mult and
        // destroy a random other joker.
        enhancement: MPip::GainMultTimesOnNonBossBlindDestroyingJoker(50),
        resell_value: 3,
        debuffed: false,
    };
    pub const SQUARE_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 810,
            pip_type: PipType::Joker,
            index: '∱',
            symbol: '∱',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Square Joker: +4 chips for each hand played with exactly 4 cards.
        enhancement: MPip::GainChipsPerCardCountHand(4, 4),
        resell_value: 2,
        debuffed: false,
    };
    pub const SEANCE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 805,
            pip_type: PipType::Joker,
            index: '∲',
            symbol: '∲',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };
    pub const RIFF_RAFF: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 800,
            pip_type: PipType::Joker,
            index: '∳',
            symbol: '∳',
            value: 6,
        },
        card_type: BCardType::CommonJoker,
        // Riff-Raff: create 2 Common Jokers when a Blind is selected.
        enhancement: MPip::CreateJokersWhenBlindSelected(2, BCardType::CommonJoker),
        resell_value: 3,
        debuffed: false,
    };
    // 68 Vampire — Uncommon, $7. The Baron/Erosion data-fix pattern again: it was
    // tagged Common / $5 and sat in no rarity pile.
    pub const VAMPIRE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 795,
            pip_type: PipType::Joker,
            index: '∴',
            symbol: '∴',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        // Vampire: gains ×0.1 mult per enhanced card played, eating the
        // enhancement as it goes.
        enhancement: MPip::GainMultTimesPerEnhancedPlayed(10),
        resell_value: 3,
        debuffed: false,
    };
    pub const SHORTCUT: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 790,
            pip_type: PipType::Joker,
            index: '∵',
            symbol: '∵',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::GappedStraight,
        resell_value: 3,
        debuffed: false,
    };
    // 70 Hologram — Uncommon, $7. The Baron/Erosion data-fix pattern: it was
    // tagged Common / $5 and sat in no rarity pile.
    pub const HOLOGRAM: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 785,
            pip_type: PipType::Joker,
            index: '∶',
            symbol: '∶',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        // Hologram: gains ×0.25 mult per playing card added to the deck.
        enhancement: MPip::GainMultTimesPerCardAdded(25),
        resell_value: 3,
        debuffed: false,
    };
    pub const VAGABOND: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 780,
            pip_type: PipType::Joker,
            index: '∷',
            symbol: '∷',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        // Vagabond: create a Tarot when a hand is played holding $4 or less.
        enhancement: MPip::CreateTarotOnLowMoney(4),
        resell_value: 4,
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
        resell_value: 4,
        debuffed: false,
    };
    pub const CLOUD_9: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 770,
            pip_type: PipType::Joker,
            index: '∹',
            symbol: '∹',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        // Cloud 9: earn $1 for each 9 in the full deck at end of round.
        enhancement: MPip::CashPerFullDeckRank(1, '9'),
        resell_value: 3,
        debuffed: false,
    };
    pub const ROCKET: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 765,
            pip_type: PipType::Joker,
            index: '∺',
            symbol: '∺',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        // Rocket: $1 at end of round, +$2 more per Boss Blind defeated.
        enhancement: MPip::CashOnRoundEndGrowingOnBossDefeat(1, 2),
        resell_value: 3,
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
        resell_value: 3,
        debuffed: false,
    };
    pub const RESERVED_PARKING: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 755,
            pip_type: PipType::Joker,
            index: '∼',
            symbol: '∼',
            value: 6,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };
    pub const MAIL_IN_REBATE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 750,
            pip_type: PipType::Joker,
            index: '∽',
            symbol: '∽',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::Blank,
        resell_value: 2,
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
        card_type: BCardType::UncommonJoker,
        // To the Moon: earn $1 extra interest per $5 held at end of round,
        // capped at the base interest cap ($5).
        enhancement: MPip::ExtraInterest(1),
        resell_value: 2,
        debuffed: false,
    };
    pub const HALLUCINATION: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 740,
            pip_type: PipType::Joker,
            index: '∿',
            symbol: '∿',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        enhancement: MPip::Blank,
        resell_value: 2,
        debuffed: false,
    };
    // 86 Fortune Teller — Common, $6. Cost was $5.
    pub const FORTUNE_TELLER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 735,
            pip_type: PipType::Joker,
            index: '≀',
            symbol: '≀',
            value: 6,
        },
        card_type: BCardType::CommonJoker,
        // Fortune Teller: +1 mult per Tarot used this run — retroactive, so it
        // reads the board's run-wide tally rather than its own counter.
        enhancement: MPip::MultPlusPerTarotUsedThisRun(1),
        resell_value: 3,
        debuffed: false,
    };
    pub const JUGGLER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 730,
            pip_type: PipType::Joker,
            index: '≁',
            symbol: '≁',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Juggler: +1 hand size while held — round config, not hand score.
        enhancement: MPip::HandSizeIncrement(1),
        resell_value: 2,
        debuffed: false,
    };
    pub const DRUNKARD: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 725,
            pip_type: PipType::Joker,
            index: '≂',
            symbol: '≂',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Drunkard: +1 discard each round while held.
        enhancement: MPip::DiscardIncrement(1),
        resell_value: 2,
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
        resell_value: 3,
        debuffed: false,
    };
    pub const GOLDEN_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 715,
            pip_type: PipType::Joker,
            index: '≄',
            symbol: '≄',
            value: 6,
        },
        card_type: BCardType::CommonJoker,
        // Golden Joker: earn $4 at end of round — economy, not hand score.
        // Paid by `on_round_end`. (It was once mislabelled `Chips(4)`, which
        // made it silently add 0 chips.)
        enhancement: MPip::CashOnRoundEnd(4),
        resell_value: 3,
        debuffed: false,
    };
    pub const LUCKY_CAT: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 710,
            pip_type: PipType::Joker,
            index: '≅',
            symbol: '≅',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };
    pub const BASEBALL_CARD: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 705,
            pip_type: PipType::Joker,
            index: '≆',
            symbol: '≆',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        // Baseball Card: Uncommon jokers each give ×1.5 mult (compounds).
        enhancement: MPip::MultTimesPerUncommonJoker(15),
        resell_value: 4,
        debuffed: false,
    };
    pub const BULL: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 700,
            pip_type: PipType::Joker,
            index: '≇',
            symbol: '≇',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::ChipsPerDollar(2),
        resell_value: 3,
        debuffed: false,
    };
    pub const DIET_COLA: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 695,
            pip_type: PipType::Joker,
            index: '≈',
            symbol: '≈',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };
    pub const TRADING_CARD: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 690,
            pip_type: PipType::Joker,
            index: '≉',
            symbol: '≉',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
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
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 2,
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
        // Popcorn: +20 mult, losing 4 per round played — spent after 5 rounds,
        // when the round that empties it destroys it.
        enhancement: MPip::LoseMultPerRound(20, 4),
        resell_value: 2,
        debuffed: false,
    };
    pub const SPARE_TROUSERS: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 675,
            pip_type: PipType::Joker,
            index: '≌',
            symbol: '≌',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        // Spare Trousers: +2 Mult for each hand played containing a Two Pair.
        enhancement: MPip::GainMultPerTwoPairHand(2),
        resell_value: 3,
        debuffed: false,
    };
    pub const ANCIENT_JOKER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 670,
            pip_type: PipType::Joker,
            index: '≍',
            symbol: '≍',
            value: 8,
        },
        card_type: BCardType::RareJoker,
        // Ancient Joker: x1.5 mult per played card of the round's ancient suit,
        // which re-rolls at every round end.
        enhancement: MPip::MultTimesPerScoredAncientSuit(15),
        resell_value: 4,
        debuffed: false,
    };
    pub const RAMEN: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 801,
            pip_type: PipType::Joker,
            index: '🍜',
            symbol: '🍜',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        // Ramen: ×2 Mult, loses ×0.01 per card discarded; floors at ×1.
        enhancement: MPip::LoseMultTimesPerDiscard(200, 1),
        resell_value: 3,
        debuffed: false,
    };
    pub const WALKIE_TALKIE: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 802,
            pip_type: PipType::Joker,
            index: '📻',
            symbol: '📻',
            value: 4,
        },
        card_type: BCardType::CommonJoker,
        // Walkie Talkie: each played 10 or 4 gives +10 chips and +4 mult.
        enhancement: MPip::ChipsMultPlusPerScoredRanks(10, 4, ['T', '4']),
        resell_value: 2,
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

    // 77 Luchador — Uncommon, $5. Sell it to disable the current Boss Blind.
    pub const LUCHADOR: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 809,
            pip_type: PipType::Joker,
            index: '🤼',
            symbol: '🤼',
            value: 5,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::DisableBossBlindOnSell,
        resell_value: 2,
        debuffed: false,
    };

    // 129 Matador — Uncommon, $7. "Earn $8 if played hand triggers the Boss
    // Blind ability." Deliberately left `Blank`: this engine models the Boss
    // Blinds whose ability is a `Draws` mutation applied once at blind select
    // (The Needle, The Water, The Manacle), and none of them is *triggered by a
    // played hand* — so there is no event for Matador to read. Paying on every
    // boss hand instead would be the plausible-but-wrong value EPIC-01a exists
    // to keep out. It needs per-hand boss abilities, which are their own EPIC.
    pub const MATADOR: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 811,
            pip_type: PipType::Joker,
            index: '🐂',
            symbol: '🐂',
            value: 7,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::Blank,
        resell_value: 3,
        debuffed: false,
    };

    // 102 Seltzer — Uncommon, $6. Retrigger all cards played for the next 10
    // hands, then it is destroyed ("Drank!").
    pub const SELTZER: BuffoonCard = BuffoonCard {
        suit: FrenchSuit::JOKER,
        rank: Pip {
            weight: 808,
            pip_type: PipType::Joker,
            index: '🥤',
            symbol: '🥤',
            value: 6,
        },
        card_type: BCardType::UncommonJoker,
        enhancement: MPip::RetriggerAllPlayedForHands(1, 10),
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
        // Canio: gains ×1 mult when a face card is destroyed.
        enhancement: MPip::GainMultTimesPerFaceDestroyed(100),
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
        // Yorick: gains ×1 mult every 23 cards discarded.
        enhancement: MPip::GainMultTimesPerDiscardedCards(100, 23),
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
        // Chicot: disables the ability of every Boss Blind while held.
        enhancement: MPip::DisablesAllBossBlinds,
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
    use crate::funky::decks::planet::card as planet_card;
    use crate::funky::decks::tarot::MajorArcana;
    use crate::funky::decks::tarot::card as tarot_card;
    use crate::funky::types::board::BuffoonBoard;
    use crate::funky::types::draws::Draws;
    use crate::funky::types::mpip::MPip;
    use crate::preludes::funky::{BCardType, Blind, BuffoonPile, Deck, Score};
    use std::collections::{HashMap, HashSet};

    /// Every joker const defined in this file, in declaration order. The single
    /// iteration point for crate-wide data invariants (weight uniqueness, the
    /// scoring-reachability guard). A joker is only protected by those guards
    /// once it is listed here; `all_jokers__is_superset_of_every_pile` keeps the
    /// four rarity piles from drifting out of it.
    const ALL_JOKERS: [BuffoonCard; 112] = [
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
        card::LUCHADOR,
        card::MATADOR,
        card::SELTZER,
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

    /// The inverse of the superset guard: every joker in [`ALL_JOKERS`] must
    /// sit in the rarity pile its own `card_type` names. An adrift joker is
    /// invisible to everything that draws from the piles (Riff-Raff already
    /// mints from `COMMON_JOKERS`; the shop will draw stock the same way) —
    /// and `CommonJoker`/$5/`resell_value: 0` with no pile is exactly what
    /// stale default data looks like (EPIC-01a §Data fixes).
    #[test]
    fn all_jokers__every_joker_is_piled_by_its_rarity() {
        let mut adrift: Vec<String> = Vec::new();
        for card in ALL_JOKERS {
            let pile: &[BuffoonCard] = match card.card_type {
                BCardType::CommonJoker => &Joker::COMMON_JOKERS,
                BCardType::UncommonJoker => &Joker::UNCOMMON_JOKERS,
                BCardType::RareJoker => &Joker::RARE_JOKERS,
                BCardType::LegendaryJoker => &Joker::LEGENDARY_JOKERS,
                other => {
                    adrift.push(format!("{card} has non-joker card_type {other:?}"));
                    continue;
                }
            };
            if !pile.contains(&card) {
                adrift.push(format!("{card}"));
            }
        }
        assert!(
            adrift.is_empty(),
            "{} jokers missing from their rarity's pile:\n{}",
            adrift.len(),
            adrift.join("\n")
        );
    }

    /// Balatro's sell value is half the buy cost, rounded down, floored at $1
    /// (a $2 Joker sells for $1, a $7 Vampire for $3). `sell_joker` pays
    /// `resell_value` out for real now, so a stale 0 is money silently lost.
    /// Legendary jokers are excluded: they cannot be bought, carry cost 0, and
    /// the engine models no sell price for them.
    #[test]
    fn all_jokers__resell_value_is_half_cost_floored_at_one() {
        let mut wrong: Vec<String> = Vec::new();
        for card in ALL_JOKERS {
            if card.card_type == BCardType::LegendaryJoker {
                continue;
            }
            let expected = (card.rank.value / 2).max(1);
            if card.resell_value != expected {
                wrong.push(format!(
                    "{card}: cost ${} should sell ${expected}, has ${}",
                    card.rank.value, card.resell_value
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "{} jokers with a wrong resell_value:\n{}",
            wrong.len(),
            wrong.join("\n")
        );
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

    #[test]
    fn hologram_and_vampire__are_uncommon_dollar_seven_and_piled() {
        // The same drift again, found while wiring them in Phase 3: both were
        // tagged Common / $5 and left out of every rarity pile. Balatro has both
        // at Uncommon / $7 (sell $3).
        for (joker, name) in [(card::HOLOGRAM, "Hologram"), (card::VAMPIRE, "Vampire")] {
            assert_eq!(
                joker.card_type,
                BCardType::UncommonJoker,
                "{name} is an Uncommon joker in Balatro"
            );
            assert_eq!(joker.rank.value, 7, "{name} costs $7 in Balatro");
            assert_eq!(joker.resell_value, 3, "{name} sells for $3 in Balatro");
            assert!(
                Joker::UNCOMMON_JOKERS.contains(&joker),
                "{name} belongs in UNCOMMON_JOKERS"
            );
        }
    }

    #[test]
    fn phase_8_jokers__have_their_balatro_rarity_and_cost() {
        // Madness and Rocket were tagged Common / $5 and unpiled; Luchador and
        // Matador are new consts. Balatro: Madness U/$7, Rocket U/$6,
        // Luchador U/$5, Matador U/$7.
        for (joker, name, cost) in [
            (card::MADNESS, "Madness", 7),
            (card::ROCKET, "Rocket", 6),
            (card::LUCHADOR, "Luchador", 5),
            (card::MATADOR, "Matador", 7),
        ] {
            assert_eq!(
                joker.card_type,
                BCardType::UncommonJoker,
                "{name} is an Uncommon joker in Balatro"
            );
            assert_eq!(joker.rank.value, cost, "{name} costs ${cost} in Balatro");
            assert!(
                Joker::UNCOMMON_JOKERS.contains(&joker),
                "{name} belongs in UNCOMMON_JOKERS"
            );
        }
    }

    #[test]
    fn matador__stays_blank_because_no_boss_ability_fires_on_a_played_hand() {
        // A characterization, not an aspiration. Matador pays "if played hand
        // triggers the Boss Blind ability", and every boss this engine models
        // applies its ability once at blind select as a `Draws` mutation — none
        // is triggered by a hand. There is no event to read, so it stays `Blank`
        // rather than take a plausible-but-wrong value (paying on every boss
        // hand). Wire it when per-hand boss abilities land, and delete this.
        assert_eq!(card::MATADOR.enhancement, MPip::Blank);
        assert!(
            !scores_hand(card::MATADOR.enhancement),
            "so the reachability guard rightly ignores it"
        );
    }

    #[test]
    fn seltzer__is_uncommon_dollar_six_and_piled() {
        assert_eq!(card::SELTZER.card_type, BCardType::UncommonJoker);
        assert_eq!(card::SELTZER.rank.value, 6, "Seltzer costs $6 in Balatro");
        assert_eq!(card::SELTZER.resell_value, 3);
        assert!(Joker::UNCOMMON_JOKERS.contains(&card::SELTZER));
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
            // Card Sharp reads the round's per-hand-type tally; Ancient Joker
            // reads the run's current suit. Both are board state rather than
            // counters, and both are ×mult conditionals once driven.
            | MPip::MultTimesOnRepeatedHandThisRound(_)
            | MPip::MultTimesPerScoredAncientSuit(_)
            | MPip::MultTimesPerUncommonJoker(_)
            | MPip::MultTimesPlusPerFullDeckSteel(_)
            | MPip::MultTimesIfHeldAllSuits(_, _)
            | MPip::GainMultPerHandLessDiscard(_)
            | MPip::LoseMultTimesPerDiscard(_, _)
            | MPip::LoseChipsPerHand(_, _)
            | MPip::GainChipsPerCardCountHand(_, _)
            | MPip::GainMultPerTwoPairHand(_)
            | MPip::GainChipsPerStraightHand(_)
            // The remaining per-run counters. Each reads its accumulator at
            // scoring time, so a grown one changes the score: Popcorn's mult
            // decays per round (Ice Cream's shape), and Yorick / Hologram /
            // Canio gain xmult per cards discarded / card added / face
            // destroyed. Ungrown they are inert (+0 mult, x1) -- the probe
            // boards drive their events so the guard sees them grown.
            | MPip::LoseMultPerRound(_, _)
            | MPip::GainMultTimesPerDiscardedCards(_, _)
            | MPip::GainMultTimesPerCardAdded(_)
            | MPip::GainMultTimesPerFaceDestroyed(_)
            // Vampire grows on the `Scored` event, i.e. before the fold reads
            // it, so a hand of enhanced cards moves the score on that same hand.
            | MPip::GainMultTimesPerEnhancedPlayed(_)
            // Constellation is a plain counter (xmult per Planet used); Fortune
            // Teller reads the board's run-wide Tarot tally instead, which is
            // what makes it retroactive. Both add mult once driven.
            | MPip::GainMultTimesPerPlanetUsed(_)
            | MPip::MultPlusPerTarotUsedThisRun(_)
            // Madness gains its xmult on every non-boss blind selected, so a
            // grown one scores. (The joker it destroys is a side effect, not its
            // contribution.)
            | MPip::GainMultTimesOnNonBossBlindDestroyingJoker(_)
            // Hiker fattens the cards rather than itself, but the boost lands on
            // the hand that triggers it, so it does change the score.
            | MPip::GainChipsOnScored(_)
            // Retriggers re-score cards, so they score (Hack: each played 2-5;
            // Sock and Buskin: each played face; Hanging Chad: the first played
            // card twice more; Mime: held-card abilities; Dusk: every played
            // card on the round's final hand; Seltzer: every played card, for
            // its first 10 hands).
            | MPip::RetriggerPlayedRanks(_, _)
            | MPip::RetriggerPlayedFaces(_)
            | MPip::RetriggerFirstPlayed(_)
            | MPip::RetriggerCardsInHand(_)
            | MPip::RetriggerPlayedCardsInFinalRound
            | MPip::RetriggerAllPlayedForHands(_, _)
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
            // The creators add a card or a joker to the run; none of them adds
            // anything to the hand in front of them. Marble Joker's Stone card
            // and Riff-Raff's Common Jokers may well score *later* — but that is
            // the created card's arm, not the creator's, the same way Pareidolia
            // only ever scores through the jokers it amplifies.
            | MPip::AddCardTypeWhenBlindSelected(_)
            | MPip::CreateJokersWhenBlindSelected(_, _)
            | MPip::CreateTarotOnAceStraight
            | MPip::CreateTarotOnLowMoney(_)
            // The Boss Blind pair act on the *blind*, never on the hand.
            // Luchador and Chicot switch a boss's ability off, which reaches the
            // score only through what that ability was doing to the round's
            // draws — Banner and Mystic Summit read the result, through their own
            // arms, exactly as they do for the draw-modifier jokers.
            | MPip::DisableBossBlindOnSell
            | MPip::DisablesAllBossBlinds
            // Rocket pays money at round end; Bull is what turns money into
            // chips, and that is Bull's arm.
            | MPip::CashOnRoundEndGrowingOnBossDefeat(_, _)
            | MPip::ChanceDestroyed(_, _)
            // The `+$` payouts move money at lifecycle events (round end,
            // discard), never the hand score. Bull turns money into chips,
            // but that is Bull's arm, not theirs.
            | MPip::CashOnRoundEnd(_)
            | MPip::CashPerDiscardIfNoneUsed(_)
            | MPip::CashPerFullDeckRank(_, _)
            | MPip::ExtraInterest(_)
            | MPip::CashOnFacesDiscarded(_, _)
            // The draw modifiers change how many hands/discards/cards a round
            // grants (`on_blind_selected` recomputes `Draws` from them), never
            // the score of the hand in front of them. Banner / Mystic Summit
            // read the *result*, through their own arms.
            | MPip::HandSizeIncrement(_)
            | MPip::DiscardIncrement(_)
            | MPip::GainHandsLoseDiscardsWhenBlindSelected(_)
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
            //
            // It must be one that reads **only the played hand** — Fibonacci
            // (+8 mult per played A/2/3/5/8) does. A resident that reads round
            // state would silently make every *modifier* look like a scorer: with
            // Mystic Summit here (+15 mult at zero discards), Burglar and
            // Drunkard move the score through it and the guard reports them as
            // misclassified, when in truth they are modifiers of exactly
            // Pareidolia's kind.
            b.jokers.push(card::FIBONACCI);
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
        // A board **playing** enhanced cards. Every other probe plays plain
        // cards, so this is the only one where Vampire has an enhancement to
        // count and eat. Bonus (+30 chips) is enough: Vampire keys off the
        // presence of the enhancement, not what it does.
        let mut enhanced_played = mk("AH KH QH JH TH", "KS KC", 0, 3);
        for index in 0..2 {
            let card = enhanced_played.played.remove(index);
            enhanced_played.played.insert(
                index,
                BuffoonCard {
                    enhancement: MPip::BONUS,
                    ..card
                },
            );
        }
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
            enhanced_played,
            worn_deck,
        ]
    }

    /// Drive one probe board through every growth event, optionally with `joker`
    /// on it, and return what it scores.
    ///
    /// The event order is chosen so the board ends up **mid-round on its final
    /// hand** — the state Dusk needs — so anything that resets the round has to
    /// come first.
    fn drive_events(board: &mut BuffoonBoard, joker: Option<BuffoonCard>) -> Score {
        // Hands that satisfy every growth condition across the slice: a 4-card
        // straight (Square + Runner), a two-pair hand (Spare Trousers), and a
        // generic hand (Green Joker, Ice Cream).
        let growth_hands = [
            bcards!("2C 3C 4C 5C"),
            bcards!("KH KS QD QC 4S"),
            bcards!("AH KH QH JH TH"),
        ];
        // One discard action carrying 23 cards: enough for Yorick to complete a
        // block, while staying a *single* action so Green Joker's net (hands −
        // discards) is unchanged.
        let big_discard =
            bcards!("2C 3C 4C 5C 6C 7C 8C 9C TC JC QC KC AC 2D 3D 4D 5D 6D 7D 8D 9D TD JD");

        if let Some(joker) = joker {
            board.push_joker(joker);
        }
        // A Small Blind is selected (Madness, which refuses to grow on a boss).
        // The pure hook, so Madness's random destruction pass never fires and
        // cannot eat the joker being probed.
        board.blind = Blind::Small;
        board.on_blind_selected();
        // A rolled ancient suit (Ancient Joker). Poked rather than rolled,
        // because the roll lives on the `_with_rng` path and this battery drives
        // the pure hooks — the probe boards play Hearts, so this is a suit their
        // hands actually match.
        board.ancient_suit = Some('H');
        // Exactly one round end: enough for Popcorn to decay (+20 → +16), and
        // fewer than the five that would empty and destroy it.
        board.on_round_end();
        // A card joins the run's deck (Hologram) …
        board.add_card_to_deck(bcards!("7D").iter().next().copied().unwrap());
        // … and a face card leaves it (Canio). Skipped on a board whose roster no
        // longer holds that King (the worn deck enhanced it), which is fine —
        // reachability is an `any` across the battery.
        if let Some(king) = board.full_deck_index_of(bcards!("KS").iter().next().copied().unwrap())
        {
            board.destroy_deck_card(king);
        }
        board.on_discard(&big_discard);
        // One Planet and one Tarot spent (Constellation, Fortune Teller). The
        // Tarot targets nothing, so it only registers as used — all either joker
        // reads.
        board.create_consumable(planet_card::PLUTO);
        board.use_consumable(0, &[]);
        board.create_consumable(tarot_card::JUSTICE);
        board.use_consumable(0, &[]);
        // Three hands into a four-hand round: leaves `hands_played == 3`, so the
        // hand being scored is the fourth and last (Dusk), and Seltzer still has
        // 7 of its 10 unspent.
        for hand in &growth_hands {
            board.on_hand_played(hand);
        }
        board.on_scored();
        board.score()
    }

    /// A joker is *reachable* if adding it to some probe board changes the score.
    ///
    /// **Both** sides of the comparison are driven through the identical event
    /// sequence — the baseline board simply has no probed joker on it. That is
    /// what makes the difference attributable to the joker and nothing else.
    /// Taking the baseline *before* the events instead would fold every
    /// event's own effect into the marginal: using a Planet levels the hand type
    /// for everyone, so every joker on the board would look "reachable" and the
    /// guard would pass vacuously for all of them.
    fn is_reachable(joker: BuffoonCard) -> bool {
        probe_boards().into_iter().any(|mut board| {
            let baseline = drive_events(&mut board.clone(), None);
            let probed = drive_events(&mut board, Some(joker));
            probed != baseline
        })
    }

    /// Jokers whose enhancement *intends* to score but has no wiring yet — the
    /// audit output of EPIC-01a Phase 0b. Each is a (near-)pure function of board
    /// state and cheap to wire; they are listed here so the reachability guard
    /// stays green while the debt stays visible. **Remove an entry when you wire
    /// it** — the guard fails if a listed joker starts scoring, forcing the
    /// cleanup, and fails if a *new* unlisted joker silently scores 0.
    ///
    /// **Empty, as of EPIC-01a Phase 8** — every joker that intends to score now
    /// does. Its last entry was Joker Stencil, which waited on a real joker-slot
    /// *limit* (a `Vec`'s capacity is not the game's 5-slot rule); Phase 5c added
    /// `BuffoonBoard::joker_slots` for Riff-Raff's "(Must have room)", which
    /// unblocked it for free.
    ///
    /// Keep the list, and keep it empty: a joker that stops scoring lands here
    /// with a reason, or it is a bug. Jokers that legitimately do *not* score are
    /// classified in [`scores_hand`] instead, and never belong here.
    const KNOWN_UNWIRED: [BuffoonCard; 0] = [];

    /// Every joker still carrying [`MPip::Blank`], each with the reason it does.
    ///
    /// The third guard, and the one that closes the last silent failure mode in
    /// the crate. Its two siblings ([`KNOWN_UNWIRED`] and
    /// `KNOWN_UNWIRED_CARD_ENHANCEMENTS`) catch a card that *intends* to score
    /// and doesn't — but a `Blank` joker legitimately scores nothing, so neither
    /// guard can see it. That left "`Blank` because we haven't got to it" and
    /// "`Blank` because it needs a subsystem that does not exist"
    /// indistinguishable, and EPIC-01a's whole premise is that the difference
    /// must be written down.
    ///
    /// The reason is **data, not a comment**, on purpose: a comment cannot be
    /// asserted, and rots silently. This one is checked — see
    /// [`all_jokers__every_blank_joker_has_a_stated_reason`], which fails if a
    /// `Blank` joker is missing from this list, if a listed joker gets wired, or
    /// if a reason is too thin to be a reason.
    ///
    /// **Not a to-do list.** An entry here is a *decision*, and several of these
    /// are permanent-ish: they wait on subsystems (spectral cards, packs, the
    /// shop) that are deliberately outside this crate's current scope. Delete an
    /// entry only when the joker is wired.
    const BLANK_WITH_REASON: [(BuffoonCard, &str); 14] = [
        // --- Blocked on subsystems that do not exist (EPIC-01a item 5e) ---
        (
            card::SIXTH_SENSE,
            "Spectral cards do not exist — `BCardType::Spectral` is a bare tag with no deck (EPIC-01 Story 3)",
        ),
        (
            card::SEANCE,
            "Spectral cards do not exist — same blocker as Sixth Sense (EPIC-01 Story 3)",
        ),
        (
            card::HALLUCINATION,
            "needs booster packs, which do not exist",
        ),
        (
            card::RED_CARD,
            "+3 mult per pack skipped: needs booster packs, which do not exist",
        ),
        (
            card::PERKEO,
            "needs the shop and Negative editions, neither of which exists",
        ),
        (
            card::FLASH_CARD,
            "+2 mult per reroll: needs the shop, which does not exist",
        ),
        (
            card::DNA,
            "needs a draw step (it copies a card into hand); its other half, \
             'first hand of round', is expressible now via `hands_played`",
        ),
        // --- Blocked on state this EPIC deliberately scoped out (item 1c) ---
        (
            card::TO_DO_LIST,
            "needs a per-round random target hand type",
        ),
        (card::MAIL_IN_REBATE, "needs a per-round random target rank"),
        (card::TRADING_CARD, "needs card destruction on discard"),
        (
            card::RESERVED_PARKING,
            "a probabilistic held-card payout — deferred rather than blocked; it \
             could ride the existing seeded-RNG path today",
        ),
        // --- Blocked on the pure-fold boundary ---
        (
            card::LUCKY_CAT,
            "needs scoring to be mutating: it gains x0.25 per Lucky *proc*, which \
             happens inside the pure `&self` fold, so no `&mut` hook can see it \
             without re-rolling the RNG. The same gap `on_scored` already \
             characterizes for Hiker",
        ),
        // --- Blocked on boss blind abilities (item 8) ---
        (
            card::MATADOR,
            "pays when a played hand triggers the Boss Blind ability; every boss \
             modelled here applies its ability once at blind select, so there is \
             no per-hand trigger to read",
        ),
        // --- Blocked on Tags, which do not exist ---
        (
            card::DIET_COLA,
            "sell it to create a free Double Tag: needs Tags, which do not exist \
             — there is no object to create, and Double Tag's own effect ('copy \
             the next selected Tag') needs the skip-blind tag-selection flow too. \
             Its `selling_self` trigger also wants the shop. Tags first",
        ),
    ];

    /// Every `Blank` joker must say why it is `Blank`.
    ///
    /// This is the guard the other two structurally cannot be: a `Blank` joker
    /// scores nothing *correctly*, so reachability has nothing to catch it with.
    /// Without this, "not got to it yet" and "waiting on spectral cards" look
    /// identical in the source.
    #[test]
    fn all_jokers__every_blank_joker_has_a_stated_reason() {
        let listed: HashMap<_, _> = BLANK_WITH_REASON.iter().copied().collect();
        let mut unexplained = Vec::new();
        let mut wired_but_listed = Vec::new();

        for joker in ALL_JOKERS {
            let is_blank = joker.enhancement == MPip::Blank;
            let has_reason = listed.contains_key(&joker);
            if is_blank && !has_reason {
                unexplained.push(format!("{joker}"));
            }
            if !is_blank && has_reason {
                wired_but_listed.push(format!("{joker}"));
            }
        }

        assert!(
            wired_but_listed.is_empty(),
            "these are wired now — remove them from BLANK_WITH_REASON: {wired_but_listed:?}"
        );
        assert!(
            unexplained.is_empty(),
            "these jokers are Blank with no stated reason. Wire them, or add them \
             to BLANK_WITH_REASON saying what they are waiting on — `Blank` by \
             omission is indistinguishable from `Blank` by decision, and no other \
             guard can tell them apart: {unexplained:?}"
        );

        for (joker, reason) in BLANK_WITH_REASON {
            assert!(
                reason.len() > 20,
                "{joker} needs a real reason, not `{reason}`"
            );
        }
    }

    /// Every `Blank` joker's reason names something concrete.
    ///
    /// The reason guard only checks that a reason exists and is not a stub; this
    /// checks it is a *decision* rather than a shrug. Three jokers (Card Sharp,
    /// Diet Cola, Ancient Joker) were once `Blank` purely by omission — never
    /// assigned to a phase, no reason recorded — and the fix for two of them was
    /// simply to wire them, because nothing had ever been blocking them. This is
    /// what stops that state coming back under a vaguer name.
    #[test]
    fn blank_jokers__every_reason_names_a_blocker() {
        // A real reason says what is missing. This is a coarse check — it cannot
        // tell a true reason from a plausible one — but it does catch "TODO",
        // "not done yet", and their relatives, which is the failure that let
        // three jokers sit untriaged through eight phases.
        const BLOCKER_WORDS: [&str; 8] = [
            "needs",
            "not exist",
            "deferred",
            "no ",
            "cannot",
            "wants",
            "blocked",
            "first hand",
        ];
        for (joker, reason) in BLANK_WITH_REASON {
            let lower = reason.to_lowercase();
            assert!(
                BLOCKER_WORDS.iter().any(|word| lower.contains(word)),
                "{joker}'s reason does not name what is missing: `{reason}`"
            );
            assert!(
                !lower.contains("untriaged") && !lower.contains("todo"),
                "{joker} is not triaged — decide what it needs, then wire it or say why not: `{reason}`"
            );
        }
    }

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
    ///
    /// **Empty.** Its last entry was the Stone card (`TOWER`), which waited on
    /// *both* halves landing together: +50 chips **and** rank/suit suppression,
    /// because chips alone would have traded a silent zero for a silently wrong
    /// hand type. Both are in — `BuffoonCard::is_stone` masks the chips flat and
    /// `BuffoonPile::detectable` drops it from classification — so a Stone card
    /// scores its 50 and pairs, connects and flushes with nothing.
    const KNOWN_UNWIRED_CARD_ENHANCEMENTS: [MPip; 0] = [];

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
