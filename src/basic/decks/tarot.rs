use crate::prelude::*;
use colored::Color;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tarot {}

#[allow(clippy::module_name_repetitions)]
pub type TarotDeck = Deck<Tarot>;
#[allow(clippy::module_name_repetitions)]
pub type TarotCard = Card<Tarot>;

impl Tarot {
    pub const DECK_SIZE: usize = 78;

    pub const DECK: [BasicCard; Tarot::DECK_SIZE] = [
        TarotBasicCard::FOOL,
        TarotBasicCard::MAGICIAN,
        TarotBasicCard::HIGH_PRIESTESS,
        TarotBasicCard::EMPRESS,
        TarotBasicCard::EMPEROR,
        TarotBasicCard::HIEROPHANT,
        TarotBasicCard::LOVERS,
        TarotBasicCard::CHARIOT,
        TarotBasicCard::STRENGTH,
        TarotBasicCard::HERMIT,
        TarotBasicCard::WHEEL_OF_FORTUNE,
        TarotBasicCard::JUSTICE,
        TarotBasicCard::HANGED_MAN,
        TarotBasicCard::DEATH,
        TarotBasicCard::TEMPERANCE,
        TarotBasicCard::DEVIL,
        TarotBasicCard::TOWER,
        TarotBasicCard::STAR,
        TarotBasicCard::MOON,
        TarotBasicCard::SUN,
        TarotBasicCard::JUDGEMENT,
        TarotBasicCard::WORLD,
        TarotBasicCard::KING_WANDS,
        TarotBasicCard::QUEEN_WANDS,
        TarotBasicCard::KNIGHT_WANDS,
        TarotBasicCard::PAGE_WANDS,
        TarotBasicCard::TEN_WANDS,
        TarotBasicCard::NINE_WANDS,
        TarotBasicCard::EIGHT_WANDS,
        TarotBasicCard::SEVEN_WANDS,
        TarotBasicCard::SIX_WANDS,
        TarotBasicCard::FIVE_WANDS,
        TarotBasicCard::FOUR_WANDS,
        TarotBasicCard::THREE_WANDS,
        TarotBasicCard::TWO_WANDS,
        TarotBasicCard::ACE_WANDS,
        TarotBasicCard::KING_CUPS,
        TarotBasicCard::QUEEN_CUPS,
        TarotBasicCard::KNIGHT_CUPS,
        TarotBasicCard::PAGE_CUPS,
        TarotBasicCard::TEN_CUPS,
        TarotBasicCard::NINE_CUPS,
        TarotBasicCard::EIGHT_CUPS,
        TarotBasicCard::SEVEN_CUPS,
        TarotBasicCard::SIX_CUPS,
        TarotBasicCard::FIVE_CUPS,
        TarotBasicCard::FOUR_CUPS,
        TarotBasicCard::THREE_CUPS,
        TarotBasicCard::TWO_CUPS,
        TarotBasicCard::ACE_CUPS,
        TarotBasicCard::KING_SWORDS,
        TarotBasicCard::QUEEN_SWORDS,
        TarotBasicCard::KNIGHT_SWORDS,
        TarotBasicCard::PAGE_SWORDS,
        TarotBasicCard::TEN_SWORDS,
        TarotBasicCard::NINE_SWORDS,
        TarotBasicCard::EIGHT_SWORDS,
        TarotBasicCard::SEVEN_SWORDS,
        TarotBasicCard::SIX_SWORDS,
        TarotBasicCard::FIVE_SWORDS,
        TarotBasicCard::FOUR_SWORDS,
        TarotBasicCard::THREE_SWORDS,
        TarotBasicCard::TWO_SWORDS,
        TarotBasicCard::ACE_SWORDS,
        TarotBasicCard::KING_PENTACLES,
        TarotBasicCard::QUEEN_PENTACLES,
        TarotBasicCard::KNIGHT_PENTACLES,
        TarotBasicCard::PAGE_PENTACLES,
        TarotBasicCard::TEN_PENTACLES,
        TarotBasicCard::NINE_PENTACLES,
        TarotBasicCard::EIGHT_PENTACLES,
        TarotBasicCard::SEVEN_PENTACLES,
        TarotBasicCard::SIX_PENTACLES,
        TarotBasicCard::FIVE_PENTACLES,
        TarotBasicCard::FOUR_PENTACLES,
        TarotBasicCard::THREE_PENTACLES,
        TarotBasicCard::TWO_PENTACLES,
        TarotBasicCard::ACE_PENTACLES,
    ];
}

impl DeckedBase for Tarot {
    fn base_vec() -> Vec<BasicCard> {
        Tarot::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(TarotSuit::MAJOR_ARCANA, Color::Blue);
        mappie.insert(TarotSuit::CUPS, Color::Red);
        mappie.insert(TarotSuit::SWORDS, Color::Red);

        mappie
    }

    fn deck_name() -> String {
        "Tarot".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_TAROT.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_TAROT.to_string()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__tarot_tests {
    use super::*;
    use crate::basic::types::deck::Deck;
    use crate::basic::types::traits::Decked;
    use std::str::FromStr;

    #[test]
    fn fluent__fluent_name_default() {
        let magician = TarotCard::from_str("mm").unwrap();

        assert_eq!(magician.index(), "MM");
        assert_eq!(magician.fluent_name_default(), "The Magician");
    }

    #[test]
    fn decked__validate() {
        assert!(Deck::<Tarot>::validate());
    }
}
