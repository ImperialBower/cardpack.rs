use crate::basic::decks::cards::skat::{SkatBasicCard, SkatSuit, FLUENT_KEY_BASE_NAME_SKAT};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::deck::Deck;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::DeckedBase;
use colored::Color;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Skat {}
#[allow(clippy::module_name_repetitions)]
pub type SkatDeck = Deck<Skat>;
#[allow(clippy::module_name_repetitions)]
pub type SkatCard = Card<Skat>;

impl Skat {
    pub const DECK_SIZE: usize = 32;

    pub const DECK: [BasicCard; Skat::DECK_SIZE] = [
        SkatBasicCard::DAUSE_EICHEL,
        SkatBasicCard::ZHEN_EICHEL,
        SkatBasicCard::KÖNIG_EICHEL,
        SkatBasicCard::OBER_EICHEL,
        SkatBasicCard::UNTER_EICHEL,
        SkatBasicCard::NEUN_EICHEL,
        SkatBasicCard::ACHT_EICHEL,
        SkatBasicCard::SIEBEN_EICHEL,
        SkatBasicCard::DAUSE_LAUB,
        SkatBasicCard::ZHEN_LAUB,
        SkatBasicCard::KÖNIG_LAUB,
        SkatBasicCard::OBER_LAUB,
        SkatBasicCard::UNTER_LAUB,
        SkatBasicCard::NEUN_LAUB,
        SkatBasicCard::ACHT_LAUB,
        SkatBasicCard::SIEBEN_LAUB,
        SkatBasicCard::DAUSE_HERZ,
        SkatBasicCard::ZHEN_HERZ,
        SkatBasicCard::KÖNIG_HERZ,
        SkatBasicCard::OBER_HERZ,
        SkatBasicCard::UNTER_HERZ,
        SkatBasicCard::NEUN_HERZ,
        SkatBasicCard::ACHT_HERZ,
        SkatBasicCard::SIEBEN_HERZ,
        SkatBasicCard::DAUSE_SHELLEN,
        SkatBasicCard::ZHEN_SHELLEN,
        SkatBasicCard::KÖNIG_SHELLEN,
        SkatBasicCard::OBER_SHELLEN,
        SkatBasicCard::UNTER_SHELLEN,
        SkatBasicCard::NEUN_SHELLEN,
        SkatBasicCard::ACHT_SHELLEN,
        SkatBasicCard::SIEBEN_SHELLEN,
    ];
}

impl DeckedBase for Skat {
    fn base_vec() -> Vec<BasicCard> {
        Skat::DECK.to_vec()
    }

    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(SkatSuit::LAUB, Color::Green);
        mappie.insert(SkatSuit::HERZ, Color::Red);
        mappie.insert(SkatSuit::SHELLEN, Color::BrightBlue);

        mappie
    }

    fn deck_name() -> String {
        "Skat".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_SKAT.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_SKAT.to_string()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__skat_tests {
    use super::*;
    use crate::basic::types::deck::Deck;
    use crate::basic::types::traits::Decked;
    use crate::localization::{FluentName, Named};

    #[test]
    fn decked__deck() {
        let deck = Deck::<Skat>::deck();
        assert_eq!(
            deck.to_string(),
            "D♣ Z♣ K♣ O♣ U♣ 9♣ 8♣ 7♣ D♠ Z♠ K♠ O♠ U♠ 9♠ 8♠ 7♠ D♥ Z♥ K♥ O♥ U♥ 9♥ 8♥ 7♥ D♦ Z♦ K♦ O♦ U♦ 9♦ 8♦ 7♦"
        );
        assert_eq!(
            deck.index(),
            "DE ZE KE OE UE 9E 8E 7E DL ZL KL OL UL 9L 8L 7L DH ZH KH OH UH 9H 8H 7H DS ZS KS OS US 9S 8S 7S"
        );
    }

    #[test]
    fn decked__validate() {
        assert!(Deck::<Skat>::validate());
    }

    #[test]
    fn fluent__name() {
        let mut deck = Deck::<Skat>::deck();
        let dause_eichel = deck.draw_first().unwrap();
        let daus = dause_eichel.fluent_rank_name(&FluentName::DEUTSCH);
        let eichel = dause_eichel.fluent_suit_name(&FluentName::DEUTSCH);
        let deuce = dause_eichel.fluent_rank_name(&FluentName::US_ENGLISH);
        let acorns = dause_eichel.fluent_suit_name(&FluentName::US_ENGLISH);

        assert_eq!(daus, "Daus");
        assert_eq!(eichel, "Eichel");
        assert_eq!(deuce, "Deuce");
        assert_eq!(acorns, "Acorns");
        assert_eq!(dause_eichel.fluent_name_default(), "Deuce of Acorns");
    }
}
