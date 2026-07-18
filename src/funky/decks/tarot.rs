use crate::funky::types::buffoon_card::BuffoonCard;

pub struct MajorArcana {}

impl MajorArcana {
    pub const DECK_SIZE: usize = 22;
    pub const DECK: [BuffoonCard; Self::DECK_SIZE] = [
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
    use crate::funky::types::edition::Edition;
    use crate::funky::types::mpip::MPip;
    use crate::prelude::{PipType, TarotRank, TarotSuit};

    pub const FOOL: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::FOOL,
        card_type: BCardType::Tarot,
        enhancement: MPip::Blank,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const MAGICIAN: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::MAGICIAN,
        card_type: BCardType::Tarot,
        enhancement: MPip::Lucky(5, 15),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const HIGH_PRIESTESS: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HIGH_PRIESTESS,
        card_type: BCardType::Tarot,
        enhancement: MPip::Planet(2),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const EMPRESS: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::EMPRESS,
        card_type: BCardType::Tarot,
        enhancement: MPip::MultPlus(4),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const EMPEROR: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::EMPEROR,
        card_type: BCardType::Tarot,
        enhancement: MPip::RandomTarot(2),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const HIEROPHANT: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HIEROPHANT,
        card_type: BCardType::Tarot,
        enhancement: MPip::Chips(30),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const LOVERS: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::LOVERS,
        card_type: BCardType::Tarot,
        enhancement: MPip::Wild(PipType::Suit),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const THE_CHARIOT: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::CHARIOT,
        card_type: BCardType::Tarot,
        enhancement: MPip::STEEL,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const STRENGTH: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::STRENGTH,
        card_type: BCardType::Tarot,
        enhancement: MPip::Strength,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const HERMIT: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HERMIT,
        card_type: BCardType::Tarot,
        enhancement: MPip::DoubleMoney(20),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const WHEEL_OF_FORTUNE: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::WHEEL_OF_FORTUNE,
        card_type: BCardType::Tarot,
        enhancement: MPip::WHEEL_OF_FORTUNE,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const JUSTICE: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::JUSTICE,
        card_type: BCardType::Tarot,
        enhancement: MPip::Glass(2, 4),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const HANGED_MAN: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HANGED_MAN,
        card_type: BCardType::Tarot,
        enhancement: MPip::Hanged(2),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const DEATH: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::DEATH,
        card_type: BCardType::Tarot,
        enhancement: MPip::Death(1),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const TEMPERANCE: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::TEMPERANCE,
        card_type: BCardType::Tarot,
        enhancement: MPip::TEMPERANCE,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const DEVIL: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::DEVIL,
        card_type: BCardType::Tarot,
        enhancement: MPip::DEVIL,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const TOWER: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::TOWER,
        card_type: BCardType::Tarot,
        enhancement: MPip::TOWER,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const STAR: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::STAR,
        card_type: BCardType::Tarot,
        enhancement: MPip::Diamonds(3),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const MOON: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::MOON,
        card_type: BCardType::Tarot,
        enhancement: MPip::Clubs(3),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const SUN: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::SUN,
        card_type: BCardType::Tarot,
        enhancement: MPip::Hearts(3),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const JUDGEMENT: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::JUDGEMENT,
        card_type: BCardType::Tarot,
        enhancement: MPip::JUDGEMENT,
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
    pub const WORLD: BuffoonCard = BuffoonCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::WORLD,
        card_type: BCardType::Tarot,
        enhancement: MPip::Spades(3),
        resell_value: 1,
        edition: Edition::None,
        debuffed: false,
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod funky__decks__tarot_tests {
    use super::*;
    use crate::funky::decks::basic::card::{ACE_SPADES, KING_SPADES};
    use crate::funky::decks::tarot::card;
    use crate::funky::types::buffoon_card::BCardType;
    use crate::funky::types::mpip::MPip;
    use std::collections::HashSet;

    // ---- Deck-level invariants -------------------------------------------

    #[test]
    fn deck__size_matches_declaration() {
        assert_eq!(MajorArcana::DECK.len(), MajorArcana::DECK_SIZE);
    }

    #[test]
    fn deck__all_cards_are_tarot() {
        for tarot in MajorArcana::DECK {
            assert_eq!(
                tarot.card_type,
                BCardType::Tarot,
                "{tarot} in DECK is not a Tarot card"
            );
        }
    }

    #[test]
    fn deck__all_cards_are_distinct_major_arcana() {
        let cards: HashSet<_> = MajorArcana::DECK.iter().collect();
        assert_eq!(cards.len(), MajorArcana::DECK_SIZE, "DECK has duplicates");
        for tarot in MajorArcana::DECK {
            assert_eq!(tarot.resell_value, 1, "{tarot} should resell for $1");
        }
    }

    // ---- The three tarot-application shapes, via `BuffoonCard::enhance` ----
    //
    // `enhance` is the card-application seam every card-targeting tarot goes
    // through (see `BuffoonBoard::use_consumable`). It has exactly three
    // behaviours, and these tests pin one of each.

    /// Rank mutator: Strength steps a card's rank up by one (King -> Ace).
    #[test]
    fn strength__steps_the_rank_up_by_one() {
        let promoted = KING_SPADES.enhance(card::STRENGTH);
        assert_eq!(promoted.rank.index, 'A', "Strength should push King to Ace");
        assert_eq!(promoted.suit, KING_SPADES.suit, "suit is untouched");
        assert_eq!(
            promoted.card_type,
            BCardType::Basic,
            "the target stays a basic card"
        );
    }

    /// Suit mutator: the four suit tarots repaint a card without touching its
    /// rank or enhancement.
    #[test]
    fn suit_tarots__repaint_the_card_leaving_rank_alone() {
        for (tarot, suit) in [
            (card::STAR, 'D'),
            (card::MOON, 'C'),
            (card::SUN, 'H'),
            (card::WORLD, 'S'),
        ] {
            let painted = ACE_SPADES.enhance(tarot);
            assert_eq!(painted.suit.index, suit, "{tarot} set the wrong suit");
            assert_eq!(painted.rank, ACE_SPADES.rank, "{tarot} moved the rank");
        }
    }

    /// Enhancement swap: the enhancing tarots stamp their `MPip` onto the card,
    /// leaving rank and suit intact.
    #[test]
    fn enhancing_tarots__stamp_their_enhancement_onto_the_card() {
        for (tarot, expected) in [
            (card::JUSTICE, MPip::Glass(2, 4)),
            (card::THE_CHARIOT, MPip::STEEL),
            (card::HIEROPHANT, MPip::Chips(30)),
            (card::EMPRESS, MPip::MultPlus(4)),
            (card::TOWER, MPip::TOWER),
        ] {
            let enhanced = ACE_SPADES.enhance(tarot);
            assert_eq!(enhanced.enhancement, expected, "{tarot} set wrong effect");
            assert_eq!(enhanced.rank, ACE_SPADES.rank, "{tarot} moved the rank");
            assert_eq!(enhanced.suit, ACE_SPADES.suit, "{tarot} moved the suit");
        }
    }

    /// Run-level tarots (Death, Hermit, Hanged Man, High Priestess, Emperor)
    /// are *not* card enhancements — applied to a card they are a no-op, because
    /// their real effect belongs to a subsystem outside scoring (EPIC-01a 5e).
    #[test]
    fn run_level_tarots__do_not_touch_a_card() {
        for tarot in [
            card::DEATH,
            card::HERMIT,
            card::HANGED_MAN,
            card::HIGH_PRIESTESS,
            card::EMPEROR,
        ] {
            assert_eq!(
                ACE_SPADES.enhance(tarot),
                ACE_SPADES,
                "{tarot} should leave the card unchanged"
            );
        }
    }

    /// Scoring linkage: an enhancement a tarot applies actually reaches the
    /// score. The Hierophant stamps `Chips(30)`; a King then scores its 10 base
    /// chips plus that 30.
    #[test]
    fn hierophant__adds_its_chips_to_the_scored_card() {
        let blessed = KING_SPADES.enhance(card::HIEROPHANT);
        assert_eq!(KING_SPADES.get_chips(), 10, "plain King is 10 chips");
        assert_eq!(blessed.get_chips(), 40, "Hierophant should add +30 chips");
    }
}
