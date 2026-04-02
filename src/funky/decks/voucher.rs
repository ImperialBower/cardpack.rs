/// Balatro Wiki > [Vouchers](https://balatrogame.fandom.com/wiki/Vouchers)
///
/// Vouchers come in 16 base/upgrade pairs (32 total). Purchasing a base voucher
/// unlocks its upgrade in the shop for future antes.
use crate::funky::types::buffoon_card::BuffoonCard;

pub struct Voucher {}

impl Voucher {
    pub const BASE_SIZE: usize = 16;
    pub const UPGRADED_SIZE: usize = 14;

    pub const BASE_VOUCHERS: [BuffoonCard; Self::BASE_SIZE] = [
        card::OVERSTOCK,
        card::CLEARANCE_SALE,
        card::HONE,
        card::REROLL_SURPLUS,
        card::CRYSTAL_BALL,
        card::TELESCOPE,
        card::GRABBER,
        card::WASTEFUL,
        card::TAROT_MERCHANT,
        card::PLANET_MERCHANT,
        card::MAGIC_TRICK,
        card::HIEROGLYPH,
        card::DIRECTORS_CUT,
        card::PAINT_BRUSH,
        card::BLANK,
        card::ANTIMATTER,
    ];

    pub const UPGRADED_VOUCHERS: [BuffoonCard; Self::UPGRADED_SIZE] = [
        card::OVERSTOCK_PLUS,
        card::LIQUIDATION,
        card::GLOW_UP,
        card::REROLL_GLUT,
        card::OMEN_GLOBE,
        card::OBSERVATORY,
        card::NACHO_TONG,
        card::RECYCLOMANCER,
        card::TAROT_TYCOON,
        card::PLANET_TYCOON,
        card::ILLUSION,
        card::PETROGLYPH,
        card::RETCON,
        card::PALETTE,
    ];
}

pub mod card {
    use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
    use crate::funky::types::mpip::MPip;
    use crate::prelude::{Pip, PipType};

    pub const VOUCHER_SUIT: Pip = Pip {
        pip_type: PipType::Special,
        weight: 1,
        index: 'V',
        symbol: '◈',
        value: 0,
    };

    // ── Base vouchers ─────────────────────────────────────────────────────────

    /// +1 card slot in the shop
    pub const OVERSTOCK: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 1,
            index: 'A',
            symbol: '⊕',
            value: 1,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddShopSlot(1),
        resell_value: 0,
        debuffed: false,
    };

    /// All cards and packs in the shop cost 50% less
    pub const CLEARANCE_SALE: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 2,
            index: 'B',
            symbol: '⊖',
            value: 2,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::ShopDiscount(50),
        resell_value: 0,
        debuffed: false,
    };

    /// Foil, Holographic, and Polychrome cards appear 2× as often
    pub const HONE: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 3,
            index: 'C',
            symbol: '⊗',
            value: 3,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::EditionChanceMultiplier(2),
        resell_value: 0,
        debuffed: false,
    };

    /// Shop rerolls cost $2 less
    pub const REROLL_SURPLUS: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 4,
            index: 'D',
            symbol: '↺',
            value: 4,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::RerollCostReduction(2),
        resell_value: 0,
        debuffed: false,
    };

    /// +1 consumable card slot
    pub const CRYSTAL_BALL: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 5,
            index: 'E',
            symbol: '◉',
            value: 5,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddConsumableSlot(1),
        resell_value: 0,
        debuffed: false,
    };

    /// Celestial Packs always contain the planet card for most-played hand
    pub const TELESCOPE: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 6,
            index: 'F',
            symbol: '◎',
            value: 6,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::TelescopeFocusMostPlayedHand,
        resell_value: 0,
        debuffed: false,
    };

    /// +1 hand per round
    pub const GRABBER: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 7,
            index: 'G',
            symbol: '⊞',
            value: 7,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddHandsPerRound(1),
        resell_value: 0,
        debuffed: false,
    };

    /// +1 discard per round
    pub const WASTEFUL: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 8,
            index: 'H',
            symbol: '⊟',
            value: 8,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddDiscardsPerRound(1),
        resell_value: 0,
        debuffed: false,
    };

    /// Tarot cards appear 2× as often in the shop
    pub const TAROT_MERCHANT: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 9,
            index: 'I',
            symbol: '⊠',
            value: 9,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::BoostCardTypeInShop(2, BCardType::Tarot),
        resell_value: 0,
        debuffed: false,
    };

    /// Planet cards appear 2× as often in the shop
    pub const PLANET_MERCHANT: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 10,
            index: 'J',
            symbol: '⊡',
            value: 10,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::BoostCardTypeInShop(2, BCardType::Planet),
        resell_value: 0,
        debuffed: false,
    };

    /// Playing cards may appear in the shop
    pub const MAGIC_TRICK: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 11,
            index: 'K',
            symbol: '⊢',
            value: 11,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AllowPlayingCardsInShop,
        resell_value: 0,
        debuffed: false,
    };

    /// -1 hand per round, -1 ante
    pub const HIEROGLYPH: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 12,
            index: 'L',
            symbol: '⊣',
            value: 12,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::ReduceHandsAndAnte(1, 1),
        resell_value: 0,
        debuffed: false,
    };

    /// 1 free Boss Blind reroll per ante
    pub const DIRECTORS_CUT: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 13,
            index: 'M',
            symbol: '⊤',
            value: 13,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::FreeReroll(1),
        resell_value: 0,
        debuffed: false,
    };

    /// +1 hand size
    pub const PAINT_BRUSH: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 14,
            index: 'N',
            symbol: '⊥',
            value: 14,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddHandSize(1),
        resell_value: 0,
        debuffed: false,
    };

    /// No effect (but can be upgraded to Credit Card)
    pub const BLANK: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 15,
            index: 'O',
            symbol: '□',
            value: 15,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::Blank,
        resell_value: 0,
        debuffed: false,
    };

    /// +1 Joker slot
    pub const ANTIMATTER: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 16,
            index: 'P',
            symbol: '⊗',
            value: 16,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddJokerSlot(1),
        resell_value: 0,
        debuffed: false,
    };

    // ── Upgraded vouchers ─────────────────────────────────────────────────────

    /// +2 card slots in the shop (upgrade of Overstock)
    pub const OVERSTOCK_PLUS: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 17,
            index: 'Q',
            symbol: '⊕',
            value: 17,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddShopSlot(2),
        resell_value: 0,
        debuffed: false,
    };

    /// All cards and packs in the shop cost 75% less (upgrade of Clearance Sale)
    pub const LIQUIDATION: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 18,
            index: 'R',
            symbol: '⊘',
            value: 18,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::ShopDiscount(75),
        resell_value: 0,
        debuffed: false,
    };

    /// Editions appear 2× as often and Negative editions join the pool (upgrade of Hone)
    pub const GLOW_UP: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 19,
            index: 'S',
            symbol: '⊙',
            value: 19,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::EditionChanceMultiplierWithNegative(2),
        resell_value: 0,
        debuffed: false,
    };

    /// Shop rerolls are free (upgrade of Reroll Surplus)
    pub const REROLL_GLUT: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 20,
            index: 'T',
            symbol: '⊚',
            value: 20,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::RerollCostReduction(0),
        resell_value: 0,
        debuffed: false,
    };

    /// Standard packs may contain any Tarot card (upgrade of Crystal Ball)
    pub const OMEN_GLOBE: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 21,
            index: 'U',
            symbol: '⊛',
            value: 21,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::OmenGlobeEffect,
        resell_value: 0,
        debuffed: false,
    };

    /// ×1.5 mult for each planet used for most-played hand this run (upgrade of Telescope)
    pub const OBSERVATORY: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 22,
            index: 'W',
            symbol: '⊜',
            value: 22,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::MultTimesPerPlanetUsedForBestHand(5), // 5 → ×1.5 (same scale as MultTimes1Dot)
        resell_value: 0,
        debuffed: false,
    };

    /// +2 hands per round (upgrade of Grabber)
    pub const NACHO_TONG: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 23,
            index: 'X',
            symbol: '⊝',
            value: 23,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddHandsPerRound(2),
        resell_value: 0,
        debuffed: false,
    };

    /// +1 discard per round and discarded cards return to the deck (upgrade of Wasteful)
    pub const RECYCLOMANCER: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 24,
            index: 'Y',
            symbol: '↻',
            value: 24,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::RecyclomancerEffect(1),
        resell_value: 0,
        debuffed: false,
    };

    /// Tarot cards appear 4× as often in the shop (upgrade of Tarot Merchant)
    pub const TAROT_TYCOON: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 25,
            index: 'Z',
            symbol: '⊻',
            value: 25,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::BoostCardTypeInShop(4, BCardType::Tarot),
        resell_value: 0,
        debuffed: false,
    };

    /// Planet cards appear 4× as often in the shop (upgrade of Planet Merchant)
    pub const PLANET_TYCOON: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 26,
            index: '1',
            symbol: '⊼',
            value: 26,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::BoostCardTypeInShop(4, BCardType::Planet),
        resell_value: 0,
        debuffed: false,
    };

    /// Playing cards in the shop may have an edition (upgrade of Magic Trick)
    pub const ILLUSION: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 27,
            index: '2',
            symbol: '⊽',
            value: 27,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::PlayingCardsInShopHaveEdition,
        resell_value: 0,
        debuffed: false,
    };

    /// -1 hand per round, -2 antes (upgrade of Hieroglyph)
    pub const PETROGLYPH: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 28,
            index: '3',
            symbol: '⋄',
            value: 28,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::ReduceHandsAndAnte(1, 2),
        resell_value: 0,
        debuffed: false,
    };

    /// Reroll the Boss Blind for $10 per use (upgrade of Director's Cut)
    pub const RETCON: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 29,
            index: '4',
            symbol: '⋅',
            value: 29,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::RerollBossBlind(10),
        resell_value: 0,
        debuffed: false,
    };

    /// +2 hand size (upgrade of Paint Brush)
    pub const PALETTE: BuffoonCard = BuffoonCard {
        suit: VOUCHER_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 30,
            index: '5',
            symbol: '⋆',
            value: 30,
        },
        card_type: BCardType::Voucher,
        enhancement: MPip::AddHandSize(2),
        resell_value: 0,
        debuffed: false,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__voucher_tests {
    use super::*;
    use crate::funky::types::buffoon_card::BCardType;

    #[test]
    fn array_sizes_correct() {
        assert_eq!(Voucher::BASE_VOUCHERS.len(), Voucher::BASE_SIZE);
        assert_eq!(Voucher::UPGRADED_VOUCHERS.len(), Voucher::UPGRADED_SIZE);
        assert_eq!(Voucher::BASE_SIZE, 16);
        assert_eq!(Voucher::UPGRADED_SIZE, 14);
    }

    #[test]
    fn all_correct_type() {
        for card in Voucher::BASE_VOUCHERS
            .iter()
            .chain(Voucher::UPGRADED_VOUCHERS.iter())
        {
            assert_eq!(
                card.card_type,
                BCardType::Voucher,
                "{card:?} should be BCardType::Voucher"
            );
        }
    }

    #[test]
    fn unique_rank_indices() {
        let all: Vec<_> = Voucher::BASE_VOUCHERS
            .iter()
            .chain(Voucher::UPGRADED_VOUCHERS.iter())
            .collect();
        let indices: Vec<char> = all.iter().map(|c| c.rank.index).collect();
        let unique: std::collections::HashSet<char> = indices.iter().copied().collect();
        assert_eq!(
            indices.len(),
            unique.len(),
            "rank index chars should be unique across all vouchers"
        );
    }

    #[test]
    fn upgrades_are_strictly_stronger() {
        use crate::funky::types::mpip::MPip;
        // Upgraded shop-slot voucher should grant more slots than base
        assert!(matches!(card::OVERSTOCK_PLUS.enhancement, MPip::AddShopSlot(n) if n > 1));
        // Upgraded discount should be deeper
        assert!(matches!(card::LIQUIDATION.enhancement, MPip::ShopDiscount(pct) if pct > 50));
        // Upgraded hand-size should be larger
        assert!(matches!(card::PALETTE.enhancement, MPip::AddHandSize(n) if n > 1));
    }
}
