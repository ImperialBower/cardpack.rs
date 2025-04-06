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
