use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

pub struct TarotBasicCard;
pub struct TarotSuit;
pub struct TarotRank;

pub const FLUENT_KEY_BASE_NAME_TAROT: &str = "tarot";

impl TarotBasicCard {
    pub const FOOL: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::FOOL,
    };
    pub const MAGICIAN: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::MAGICIAN,
    };
    pub const HIGH_PRIESTESS: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HIGH_PRIESTESS,
    };
    pub const EMPRESS: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::EMPRESS,
    };
    pub const EMPEROR: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::EMPEROR,
    };
    pub const HIEROPHANT: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HIEROPHANT,
    };
    pub const LOVERS: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::LOVERS,
    };
    pub const CHARIOT: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::CHARIOT,
    };
    pub const STRENGTH: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::STRENGTH,
    };
    pub const HERMIT: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HERMIT,
    };
    pub const WHEEL_OF_FORTUNE: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::WHEEL_OF_FORTUNE,
    };
    pub const JUSTICE: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::JUSTICE,
    };
    pub const HANGED_MAN: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::HANGED_MAN,
    };
    pub const DEATH: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::DEATH,
    };
    pub const TEMPERANCE: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::TEMPERANCE,
    };
    pub const DEVIL: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::DEVIL,
    };
    pub const TOWER: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::TOWER,
    };
    pub const STAR: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::STAR,
    };
    pub const MOON: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::MOON,
    };
    pub const SUN: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::SUN,
    };
    pub const JUDGEMENT: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::JUDGEMENT,
    };
    pub const WORLD: BasicCard = BasicCard {
        suit: TarotSuit::MAJOR_ARCANA,
        rank: TarotRank::WORLD,
    };

    pub const KING_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::KING,
    };
    pub const QUEEN_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::QUEEN,
    };
    pub const KNIGHT_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::KNIGHT,
    };
    pub const PAGE_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::PAGE,
    };
    pub const TEN_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::TEN,
    };
    pub const NINE_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::NINE,
    };
    pub const EIGHT_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::EIGHT,
    };
    pub const SEVEN_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::SEVEN,
    };
    pub const SIX_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::SIX,
    };
    pub const FIVE_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::FIVE,
    };
    pub const FOUR_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::FOUR,
    };
    pub const THREE_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::THREE,
    };
    pub const TWO_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::TWO,
    };
    pub const ACE_WANDS: BasicCard = BasicCard {
        suit: TarotSuit::WANDS,
        rank: TarotRank::ACE,
    };
    pub const KING_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::KING,
    };
    pub const QUEEN_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::QUEEN,
    };
    pub const KNIGHT_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::KNIGHT,
    };
    pub const PAGE_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::PAGE,
    };
    pub const TEN_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::TEN,
    };
    pub const NINE_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::NINE,
    };
    pub const EIGHT_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::EIGHT,
    };
    pub const SEVEN_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::SEVEN,
    };
    pub const SIX_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::SIX,
    };
    pub const FIVE_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::FIVE,
    };
    pub const FOUR_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::FOUR,
    };
    pub const THREE_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::THREE,
    };
    pub const TWO_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::TWO,
    };
    pub const ACE_CUPS: BasicCard = BasicCard {
        suit: TarotSuit::CUPS,
        rank: TarotRank::ACE,
    };
    pub const KING_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::KING,
    };
    pub const QUEEN_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::QUEEN,
    };
    pub const KNIGHT_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::KNIGHT,
    };
    pub const PAGE_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::PAGE,
    };
    pub const TEN_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::TEN,
    };
    pub const NINE_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::NINE,
    };
    pub const EIGHT_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::EIGHT,
    };
    pub const SEVEN_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::SEVEN,
    };
    pub const SIX_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::SIX,
    };
    pub const FIVE_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::FIVE,
    };
    pub const FOUR_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::FOUR,
    };
    pub const THREE_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::THREE,
    };
    pub const TWO_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::TWO,
    };
    pub const ACE_SWORDS: BasicCard = BasicCard {
        suit: TarotSuit::SWORDS,
        rank: TarotRank::ACE,
    };
    pub const KING_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::KING,
    };
    pub const QUEEN_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::QUEEN,
    };
    pub const KNIGHT_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::KNIGHT,
    };
    pub const PAGE_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::PAGE,
    };
    pub const TEN_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::TEN,
    };
    pub const NINE_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::NINE,
    };
    pub const EIGHT_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::EIGHT,
    };
    pub const SEVEN_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::SEVEN,
    };
    pub const SIX_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::SIX,
    };
    pub const FIVE_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::FIVE,
    };
    pub const FOUR_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::FOUR,
    };
    pub const THREE_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::THREE,
    };
    pub const TWO_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::TWO,
    };
    pub const ACE_PENTACLES: BasicCard = BasicCard {
        suit: TarotSuit::PENTACLES,
        rank: TarotRank::ACE,
    };
}

impl TarotSuit {
    pub const MAJOR_ARCANA: Pip = Pip {
        pip_type: PipType::Special,
        weight: 4,
        index: 'M',
        symbol: '🔮',
        value: 5,
    };
    pub const WANDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'W',
        symbol: '🪄',
        value: 4,
    };
    pub const CUPS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'C',
        symbol: '🍷',
        value: 3,
    };
    pub const SWORDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'S',
        symbol: '⚔',
        value: 2,
    };
    pub const PENTACLES: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'P',
        symbol: '☆',
        value: 1,
    };
}

impl TarotRank {
    // Major Arcana
    pub const FOOL: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 22,
        index: '0',
        symbol: '🤡',
        value: 23,
    };
    pub const MAGICIAN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 21,
        index: '1',
        symbol: '🎩',
        value: 22,
    };
    pub const HIGH_PRIESTESS: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 20,
        index: '2',
        symbol: '😇',
        value: 21,
    };
    pub const EMPRESS: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 19,
        index: '3',
        symbol: '👸',
        value: 20,
    };
    pub const EMPEROR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 18,
        index: '4',
        symbol: '🤴',
        value: 19,
    };
    pub const HIEROPHANT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 17,
        index: '5',
        symbol: '👑',
        value: 18,
    };
    pub const LOVERS: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 16,
        index: '6',
        symbol: '💑',
        value: 17,
    };
    pub const CHARIOT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 15,
        index: '7',
        symbol: '🏎',
        value: 16,
    };
    pub const STRENGTH: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 14,
        index: '8',
        symbol: '💪',
        value: 15,
    };
    pub const HERMIT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 13,
        index: '9',
        symbol: '🕵',
        value: 14,
    };
    pub const WHEEL_OF_FORTUNE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 12,
        index: 'A',
        symbol: '🎡',
        value: 13,
    };
    pub const JUSTICE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'B',
        symbol: '⚖',
        value: 12,
    };
    pub const HANGED_MAN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'C',
        symbol: '🙃',
        value: 11,
    };

    pub const DEATH: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'D',
        symbol: '💀',
        value: 10,
    };
    pub const TEMPERANCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: 'E',
        symbol: '🚭',
        value: 9,
    };
    pub const DEVIL: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: 'F',
        symbol: '😈',
        value: 8,
    };
    pub const TOWER: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: 'G',
        symbol: '🗼',
        value: 7,
    };
    pub const STAR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: 'H',
        symbol: '⭐',
        value: 6,
    };
    pub const MOON: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: 'I',
        symbol: '🌜',
        value: 5,
    };
    pub const SUN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: 'J',
        symbol: '☀',
        value: 4,
    };
    pub const JUDGEMENT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: 'K',
        symbol: '🔔',
        value: 3,
    };
    pub const WORLD: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'L',
        symbol: '🌍',
        value: 1,
    };

    // Minor Arcana
    pub const KING: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 13,
        index: 'K',
        symbol: 'K',
        value: 14,
    };
    pub const QUEEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 12,
        index: 'Q',
        symbol: 'Q',
        value: 13,
    };
    pub const KNIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'N',
        symbol: '🃏',
        value: 12,
    };
    pub const PAGE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'P',
        symbol: 'P',
        value: 11,
    };
    pub const TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'T',
        symbol: 'T',
        value: 10,
    };
    pub const NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const THREE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const TWO: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '2',
        symbol: '2',
        value: 2,
    };
    pub const ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'A',
        symbol: 'A',
        value: 1,
    };
}
