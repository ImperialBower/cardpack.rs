use crate::basic::decks::cards::ganjifa::{
    DashavataraSuit, FLUENT_KEY_BASE_NAME_DASHAVATARA, ganjifa_deck,
};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "colored-display")]
use colored::Color;
#[cfg(feature = "colored-display")]
use std::collections::HashMap;

/// [Dashavatara Ganjifa](https://en.wikipedia.org/wiki/Ganjifa) — 10 avatar
/// suits × 12 = 120 cards. The five weak suits (Matsya through Vamana) use
/// the inverted pip ladder: `A > 2 > … > 9 > 10`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dashavatara {}
#[allow(clippy::module_name_repetitions)]
pub type DashavataraDeck = Pile<Dashavatara>;
#[allow(clippy::module_name_repetitions)]
pub type DashavataraCard = Card<Dashavatara>;

impl Dashavatara {
    pub const DECK_SIZE: usize = 120;

    /// The ten avatar suits in descending-weight (sorted) order.
    pub const SUITS: [Pip; 10] = [
        DashavataraSuit::MATSYA,
        DashavataraSuit::KURMA,
        DashavataraSuit::VARAHA,
        DashavataraSuit::NARASIMHA,
        DashavataraSuit::VAMANA,
        DashavataraSuit::PARASHURAMA,
        DashavataraSuit::RAMA,
        DashavataraSuit::KRISHNA,
        DashavataraSuit::JAGANNATH,
        DashavataraSuit::KALKI,
    ];

    /// `true` = strong suit (pips 10 high), `false` = weak suit (Ace high).
    /// Parallel to [`Self::SUITS`].
    pub const STRONG: [bool; 10] = [
        false, false, false, false, false, true, true, true, true, true,
    ];

    pub const DECK: [BasicCard; Self::DECK_SIZE] = ganjifa_deck(&Self::SUITS, &Self::STRONG);
}

impl DeckedBase for Dashavatara {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(DashavataraSuit::MATSYA, Color::Blue);
        mappie.insert(DashavataraSuit::KURMA, Color::Green);
        mappie.insert(DashavataraSuit::NARASIMHA, Color::Yellow);
        mappie.insert(DashavataraSuit::PARASHURAMA, Color::Red);
        mappie.insert(DashavataraSuit::KRISHNA, Color::BrightBlue);
        mappie.insert(DashavataraSuit::KALKI, Color::BrightBlack);

        mappie
    }

    fn deck_name() -> String {
        "Dashavatara Ganjifa".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
    }
}

impl Decked<Self> for Dashavatara {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__dashavatara_tests {
    use super::*;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::{Decked, Ranged};
    #[cfg(feature = "i18n")]
    use crate::localization::{FluentName, Named};
    use core::str::FromStr;

    /// The load-bearing test: deck ↔ string round-trip plus
    /// seeded-shuffle → sort == original.
    #[test]
    fn decked__validate() {
        assert!(Dashavatara::validate());
    }

    #[test]
    fn decked__deck() {
        let deck = Dashavatara::deck();
        assert_eq!(deck.len(), 120);
        assert_eq!(
            deck.index(),
            "KM VM AM 2M 3M 4M 5M 6M 7M 8M 9M TM KU VU AU 2U 3U 4U 5U 6U 7U 8U 9U TU KB VB AB 2B 3B 4B 5B 6B 7B 8B 9B TB KN VN AN 2N 3N 4N 5N 6N 7N 8N 9N TN KD VD AD 2D 3D 4D 5D 6D 7D 8D 9D TD KP VP TP 9P 8P 7P 6P 5P 4P 3P 2P AP KR VR TR 9R 8R 7R 6R 5R 4R 3R 2R AR KK VK TK 9K 8K 7K 6K 5K 4K 3K 2K AK KJ VJ TJ 9J 8J 7J 6J 5J 4J 3J 2J AJ KC VC TC 9C 8C 7C 6C 5C 4C 3C 2C AC"
        );
        assert_eq!(
            deck.to_string(),
            "K🐟 V🐟 A🐟 2🐟 3🐟 4🐟 5🐟 6🐟 7🐟 8🐟 9🐟 T🐟 K🐢 V🐢 A🐢 2🐢 3🐢 4🐢 5🐢 6🐢 7🐢 8🐢 9🐢 T🐢 K🐗 V🐗 A🐗 2🐗 3🐗 4🐗 5🐗 6🐗 7🐗 8🐗 9🐗 T🐗 K🦁 V🦁 A🦁 2🦁 3🦁 4🦁 5🦁 6🦁 7🦁 8🦁 9🦁 T🦁 K☂ V☂ A☂ 2☂ 3☂ 4☂ 5☂ 6☂ 7☂ 8☂ 9☂ T☂ K🪓 V🪓 T🪓 9🪓 8🪓 7🪓 6🪓 5🪓 4🪓 3🪓 2🪓 A🪓 K🏹 V🏹 T🏹 9🏹 8🏹 7🏹 6🏹 5🏹 4🏹 3🏹 2🏹 A🏹 K🐄 V🐄 T🐄 9🐄 8🐄 7🐄 6🐄 5🐄 4🐄 3🐄 2🐄 A🐄 K☸ V☸ T☸ 9☸ 8☸ 7☸ 6☸ 5☸ 4☸ 3☸ 2☸ A☸ K🐎 V🐎 T🐎 9🐎 8🐎 7🐎 6🐎 5🐎 4🐎 3🐎 2🐎 A🐎"
        );
    }

    #[test]
    fn sorted() {
        let deck = Dashavatara::deck();

        let shuffled = deck.shuffled_with_seed(42);
        let sorted = shuffled.sorted();

        assert_eq!(
            sorted.to_string(),
            "K🐟 V🐟 A🐟 2🐟 3🐟 4🐟 5🐟 6🐟 7🐟 8🐟 9🐟 T🐟 K🐢 V🐢 A🐢 2🐢 3🐢 4🐢 5🐢 6🐢 7🐢 8🐢 9🐢 T🐢 K🐗 V🐗 A🐗 2🐗 3🐗 4🐗 5🐗 6🐗 7🐗 8🐗 9🐗 T🐗 K🦁 V🦁 A🦁 2🦁 3🦁 4🦁 5🦁 6🦁 7🦁 8🦁 9🦁 T🦁 K☂ V☂ A☂ 2☂ 3☂ 4☂ 5☂ 6☂ 7☂ 8☂ 9☂ T☂ K🪓 V🪓 T🪓 9🪓 8🪓 7🪓 6🪓 5🪓 4🪓 3🪓 2🪓 A🪓 K🏹 V🏹 T🏹 9🏹 8🏹 7🏹 6🏹 5🏹 4🏹 3🏹 2🏹 A🏹 K🐄 V🐄 T🐄 9🐄 8🐄 7🐄 6🐄 5🐄 4🐄 3🐄 2🐄 A🐄 K☸ V☸ T☸ 9☸ 8☸ 7☸ 6☸ 5☸ 4☸ 3☸ 2☸ A☸ K🐎 V🐎 T🐎 9🐎 8🐎 7🐎 6🐎 5🐎 4🐎 3🐎 2🐎 A🐎"
        );
    }

    /// Weak/strong boundary: Matsya (weak, Ace high) vs Kalki (strong, Ten high).
    #[test]
    fn weak_suit__inversion() {
        let weak = DashavataraDeck::from_str("TM AM").unwrap().sorted();
        assert_eq!(weak.index(), "AM TM");

        let strong = DashavataraDeck::from_str("AC TC").unwrap().sorted();
        assert_eq!(strong.index(), "TC AC");
    }

    /// 2 courts + 10 strong pips + 10 weak pips = 22 distinct rank `Pip`s.
    #[test]
    fn ranks__distinct_pips() {
        assert_eq!(Dashavatara::deck().ranks().len(), 22);
        assert_eq!(Dashavatara::deck().suits().len(), 10);
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked__colors() {
        assert_eq!(Dashavatara::colors().len(), 6);
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Dashavatara::deck_name(), "Dashavatara Ganjifa");
    }

    #[test]
    fn decked__fluent_deck_key() {
        assert_eq!(
            Dashavatara::fluent_deck_key(),
            FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
        );
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent__name() {
        let king_matsya = DashavataraCard::from_str("KM").unwrap();
        assert_eq!(
            king_matsya.fluent_rank_name(&FluentName::US_ENGLISH),
            "Raja"
        );
        assert_eq!(
            king_matsya.fluent_suit_name(&FluentName::US_ENGLISH),
            "Matsya"
        );
        assert_eq!(king_matsya.fluent_name_default(), "Raja of Matsya");

        let weak_ace = DashavataraCard::from_str("AM").unwrap();
        assert_eq!(weak_ace.fluent_name_default(), "Ace of Matsya");

        // German draft locale — avatar proper names stay untranslated.
        assert_eq!(king_matsya.fluent_rank_name(&FluentName::DEUTSCH), "König");
        assert_eq!(king_matsya.fluent_suit_name(&FluentName::DEUTSCH), "Matsya");
    }
}
