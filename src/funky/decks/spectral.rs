/// Balatro Wiki > [Spectral Cards](https://balatrogame.fandom.com/wiki/Spectral_Cards)
use crate::funky::types::buffoon_card::BuffoonCard;

pub struct Spectral {}

impl Spectral {
    pub const DECK_SIZE: usize = 18;

    pub const DECK: [BuffoonCard; Self::DECK_SIZE] = [
        card::FAMILIAR,
        card::GRIM,
        card::INCANTATION,
        card::TALISMAN,
        card::AURA,
        card::WRAITH,
        card::SIGIL,
        card::OUIJA,
        card::ECTOPLASM,
        card::IMMOLATE,
        card::ANKH,
        card::DEJA_VU,
        card::HEX,
        card::TRANCE,
        card::MEDIUM,
        card::CRYPTID,
        card::SOUL,
        card::BLACK_HOLE,
    ];
}

pub mod card {
    use crate::funky::types::buffoon_card::{BCardType, BuffoonCard};
    use crate::funky::types::mpip::MPip;
    use crate::prelude::{Pip, PipType};

    pub const SPECTRAL_SUIT: Pip = Pip {
        pip_type: PipType::Special,
        weight: 1,
        index: 'X',
        symbol: '✧',
        value: 0,
    };

    /// Destroy 2 random cards in deck, add 3 random Enhanced face cards
    pub const FAMILIAR: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 1,
            index: 'f',
            symbol: '☽',
            value: 1,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::DestroyAndAddEnhancedFaceCards(2, 3),
        resell_value: 1,
        debuffed: false,
    };

    /// Destroy 2 random cards in deck, add 2 random Enhanced Aces
    pub const GRIM: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 2,
            index: 'g',
            symbol: '☠',
            value: 2,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::DestroyAndAddEnhancedAces(2, 2),
        resell_value: 1,
        debuffed: false,
    };

    /// Destroy 2 random cards in deck, add 4 random Enhanced numbered cards 2–10
    pub const INCANTATION: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 3,
            index: 'i',
            symbol: '⚗',
            value: 3,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::DestroyAndAddEnhancedNumbered(2, 4),
        resell_value: 1,
        debuffed: false,
    };

    /// Add a Gold Seal to a random card in the full deck
    pub const TALISMAN: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 4,
            index: 't',
            symbol: '⚜',
            value: 4,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddGoldSealToRandomCard,
        resell_value: 1,
        debuffed: false,
    };

    /// Add a random edition (Foil/Holographic/Polychrome) to a selected Joker
    pub const AURA: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 5,
            index: 'a',
            symbol: '⚛',
            value: 5,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddEditionToSelectedJoker,
        resell_value: 1,
        debuffed: false,
    };

    /// Create a random Rare Joker and set money to $0
    pub const WRAITH: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 6,
            index: 'w',
            symbol: '⚡',
            value: 6,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::CreateRareJokerSetMoneyZero,
        resell_value: 1,
        debuffed: false,
    };

    /// Convert all cards in hand to a single random suit
    pub const SIGIL: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 7,
            index: 's',
            symbol: '⚕',
            value: 7,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::ConvertHandToRandomSuit,
        resell_value: 1,
        debuffed: false,
    };

    /// Convert all cards in hand to a single random rank, -1 hand size
    pub const OUIJA: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 8,
            index: 'o',
            symbol: '⚆',
            value: 8,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::ConvertHandToRandomRankReduceHandSize,
        resell_value: 1,
        debuffed: false,
    };

    /// Add Negative edition to a random Joker, -1 Joker slot
    pub const ECTOPLASM: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 9,
            index: 'e',
            symbol: '⚇',
            value: 9,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddNegativeToRandomJokerReduceSlots,
        resell_value: 1,
        debuffed: false,
    };

    /// Destroy 5 random cards in deck, gain $20
    pub const IMMOLATE: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 10,
            index: 'm',
            symbol: '⚈',
            value: 10,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::DestroyRandomCardsGainCash(5, 20),
        resell_value: 1,
        debuffed: false,
    };

    /// Copy a random Joker, destroy all other Jokers
    pub const ANKH: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 11,
            index: 'k',
            symbol: '☥',
            value: 11,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::CopyRandomJokerDestroyOthers,
        resell_value: 1,
        debuffed: false,
    };

    /// Add a Red Seal to a selected playing card
    pub const DEJA_VU: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 12,
            index: 'd',
            symbol: '⚉',
            value: 12,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddRedSealToSelectedCard,
        resell_value: 1,
        debuffed: false,
    };

    /// Add Polychrome edition to a random Joker, destroy all other Jokers
    pub const HEX: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 13,
            index: 'h',
            symbol: '⛧',
            value: 13,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddPolychromeToRandomJokerDestroyOthers,
        resell_value: 1,
        debuffed: false,
    };

    /// Add a Blue Seal to a selected playing card
    pub const TRANCE: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 14,
            index: 'r',
            symbol: '☯',
            value: 14,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddBlueSealToSelectedCard,
        resell_value: 1,
        debuffed: false,
    };

    /// Add a Purple Seal to a selected playing card
    pub const MEDIUM: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 15,
            index: 'u',
            symbol: '☮',
            value: 15,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::AddPurpleSealToSelectedCard,
        resell_value: 1,
        debuffed: false,
    };

    /// Create 2 copies of a selected playing card in the full deck
    pub const CRYPTID: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 16,
            index: 'c',
            symbol: '⚿',
            value: 16,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::CreateCopiesOfSelectedCard(2),
        resell_value: 1,
        debuffed: false,
    };

    /// Create a random Legendary Joker
    pub const SOUL: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 17,
            index: 'l',
            symbol: '♾',
            value: 17,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::CreateLegendaryJoker,
        resell_value: 1,
        debuffed: false,
    };

    /// Upgrade every poker hand by 1 level
    pub const BLACK_HOLE: BuffoonCard = BuffoonCard {
        suit: SPECTRAL_SUIT,
        rank: Pip {
            pip_type: PipType::Special,
            weight: 18,
            index: 'b',
            symbol: '⚫',
            value: 18,
        },
        card_type: BCardType::Spectral,
        enhancement: MPip::UpgradeAllPokerHands,
        resell_value: 1,
        debuffed: false,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__spectral_tests {
    use super::*;
    use crate::funky::types::buffoon_card::BCardType;

    #[test]
    fn deck_size_correct() {
        assert_eq!(Spectral::DECK.len(), Spectral::DECK_SIZE);
        assert_eq!(Spectral::DECK_SIZE, 18);
    }

    #[test]
    fn all_correct_type() {
        for card in Spectral::DECK {
            assert_eq!(
                card.card_type,
                BCardType::Spectral,
                "{card:?} should be BCardType::Spectral"
            );
        }
    }

    #[test]
    fn no_blank_enhancements() {
        use crate::funky::types::mpip::MPip;
        for card in Spectral::DECK {
            assert_ne!(
                card.enhancement,
                MPip::Blank,
                "{card:?} should have a non-Blank enhancement"
            );
        }
    }

    #[test]
    fn unique_rank_indices() {
        let indices: Vec<char> = Spectral::DECK.iter().map(|c| c.rank.index).collect();
        let unique: std::collections::HashSet<char> = indices.iter().copied().collect();
        assert_eq!(
            indices.len(),
            unique.len(),
            "rank index chars should be unique"
        );
    }
}
