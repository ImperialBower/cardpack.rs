use crate::basic::decks::cards::ganjifa::{FLUENT_KEY_BASE_NAME_MUGHAL, MughalSuit, ganjifa_deck};
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

/// [Mughal Ganjifa](https://en.wikipedia.org/wiki/Ganjifa) — 8 suits × 12 =
/// 96 cards. The four weak suits (Red Coins, Harps, Bills, Cloth) use the
/// inverted pip ladder: `A > 2 > … > 9 > 10`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mughal {}
#[allow(clippy::module_name_repetitions)]
pub type MughalDeck = Pile<Mughal>;
#[allow(clippy::module_name_repetitions)]
pub type MughalCard = Card<Mughal>;

impl Mughal {
    pub const DECK_SIZE: usize = 96;

    /// The eight suits in descending-weight (sorted) order.
    pub const SUITS: [Pip; 8] = [
        MughalSuit::SLAVES,
        MughalSuit::CROWNS,
        MughalSuit::SWORDS,
        MughalSuit::RED_COINS,
        MughalSuit::HARPS,
        MughalSuit::BILLS,
        MughalSuit::WHITE_COINS,
        MughalSuit::CLOTH,
    ];

    /// `true` = strong suit (pips 10 high), `false` = weak suit (Ace high).
    /// Parallel to [`Self::SUITS`].
    pub const STRONG: [bool; 8] = [true, true, true, false, false, false, true, false];

    pub const DECK: [BasicCard; Self::DECK_SIZE] = ganjifa_deck(&Self::SUITS, &Self::STRONG);
}

impl DeckedBase for Mughal {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(MughalSuit::CROWNS, Color::Yellow);
        mappie.insert(MughalSuit::SWORDS, Color::BrightBlue);
        mappie.insert(MughalSuit::RED_COINS, Color::Red);
        mappie.insert(MughalSuit::HARPS, Color::Magenta);
        mappie.insert(MughalSuit::BILLS, Color::Cyan);
        mappie.insert(MughalSuit::CLOTH, Color::Green);

        mappie
    }

    fn deck_name() -> String {
        "Mughal Ganjifa".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
    }
}

impl Decked<Self> for Mughal {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__mughal_tests {
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
        assert!(Mughal::validate());
    }

    #[test]
    fn decked__deck() {
        let deck = Mughal::deck();
        assert_eq!(deck.len(), 96);
        assert_eq!(
            deck.index(),
            "KG VG TG 9G 8G 7G 6G 5G 4G 3G 2G AG KT VT TT 9T 8T 7T 6T 5T 4T 3T 2T AT KS VS TS 9S 8S 7S 6S 5S 4S 3S 2S AS KR VR AR 2R 3R 4R 5R 6R 7R 8R 9R TR KH VH AH 2H 3H 4H 5H 6H 7H 8H 9H TH KB VB AB 2B 3B 4B 5B 6B 7B 8B 9B TB KW VW TW 9W 8W 7W 6W 5W 4W 3W 2W AW KQ VQ AQ 2Q 3Q 4Q 5Q 6Q 7Q 8Q 9Q TQ"
        );
        assert_eq!(
            deck.to_string(),
            "K👤 V👤 T👤 9👤 8👤 7👤 6👤 5👤 4👤 3👤 2👤 A👤 K👑 V👑 T👑 9👑 8👑 7👑 6👑 5👑 4👑 3👑 2👑 A👑 K⚔ V⚔ T⚔ 9⚔ 8⚔ 7⚔ 6⚔ 5⚔ 4⚔ 3⚔ 2⚔ A⚔ K🔴 V🔴 A🔴 2🔴 3🔴 4🔴 5🔴 6🔴 7🔴 8🔴 9🔴 T🔴 K🎵 V🎵 A🎵 2🎵 3🎵 4🎵 5🎵 6🎵 7🎵 8🎵 9🎵 T🎵 K📜 V📜 A📜 2📜 3📜 4📜 5📜 6📜 7📜 8📜 9📜 T📜 K⚪ V⚪ T⚪ 9⚪ 8⚪ 7⚪ 6⚪ 5⚪ 4⚪ 3⚪ 2⚪ A⚪ K🧵 V🧵 A🧵 2🧵 3🧵 4🧵 5🧵 6🧵 7🧵 8🧵 9🧵 T🧵"
        );
    }

    /// The Ganjifa signature rule, asserted through `sorted()` output
    /// (`BasicCard::Ord` is inverted — never assert with `<`/`>`).
    #[test]
    fn weak_suit__inversion() {
        // Red Coins is weak: Ace outranks Ten.
        let weak = MughalDeck::from_str("TR AR").unwrap().sorted();
        assert_eq!(weak.index(), "AR TR");

        // Slaves is strong: Ten outranks Ace.
        let strong = MughalDeck::from_str("AG TG").unwrap().sorted();
        assert_eq!(strong.index(), "TG AG");
    }

    /// 2 courts + 10 strong pips + 10 weak pips = 22 distinct rank `Pip`s
    /// (`Pip` equality covers all five fields) — NOT 12.
    #[test]
    fn ranks__distinct_pips() {
        assert_eq!(Mughal::deck().ranks().len(), 22);
        assert_eq!(Mughal::deck().suits().len(), 8);
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked__colors() {
        assert_eq!(Mughal::colors().len(), 6);
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Mughal::deck_name(), "Mughal Ganjifa");
    }

    #[test]
    fn decked__fluent_deck_key() {
        assert_eq!(
            Mughal::fluent_deck_key(),
            FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
        );
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent__name() {
        let king_slaves = MughalCard::from_str("KG").unwrap();
        assert_eq!(
            king_slaves.fluent_rank_name(&FluentName::US_ENGLISH),
            "King"
        );
        assert_eq!(
            king_slaves.fluent_suit_name(&FluentName::US_ENGLISH),
            "Slaves"
        );
        assert_eq!(king_slaves.fluent_name_default(), "King of Slaves");

        // Weak-ladder cards resolve through the same shared rank keys.
        let weak_ace = MughalCard::from_str("AR").unwrap();
        assert_eq!(weak_ace.fluent_name_default(), "Ace of Red Coins");

        // German draft locale.
        assert_eq!(king_slaves.fluent_rank_name(&FluentName::DEUTSCH), "König");
        assert_eq!(
            king_slaves.fluent_suit_name(&FluentName::DEUTSCH),
            "Sklaven"
        );
    }
}
