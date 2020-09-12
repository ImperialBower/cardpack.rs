pub use fluent::*;
pub use karten::anzug::*;

pub mod fluent;
pub mod karten;

extern crate rand;

use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;
use crate::karten::{Karte, Karten};

#[allow(unused_imports)]
use fluent_templates::Loader;

/// Deck of Cards (Kartendeck) that includes rank of suites (anzug_rang) and values (rangfolge).
#[derive(Clone, Debug, PartialEq)]
pub struct Kartendeck {
    pub karten: Karten,
    pub anzugrang: Vec<Anzug>,
    pub rangfolge: Vec<Rang>,
}

impl Kartendeck {
    pub fn new(karten: Karten, anzugrang: Vec<Anzug>, rangfolge: Vec<Rang>) -> Kartendeck {
        Kartendeck {
            karten,
            anzugrang,
            rangfolge,
        }
    }

    pub fn french_deck() -> Kartendeck {
        let suits = Anzug::generate_french_suits();
        let ranks = Rang::generate_french_ranks();

        let mut karten: Karten = Karten::neu();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.add(karten::Karte::new_from_structs(rank.clone(), suit.clone()));
            }
        }
        Kartendeck::new(karten, suits, ranks)
    }

    pub fn pinochle_deck() -> Kartendeck {
        let suits = Anzug::generate_french_suits();
        let ranks = Rang::generate_pinochle_ranks();

        let mut karten: Karten = Karten::neu();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.add(karten::Karte::new_from_structs(rank.clone(), suit.clone()));
                karten.add(karten::Karte::new_from_structs(rank.clone(), suit.clone()));
            }
        }
        Kartendeck::new(karten, suits, ranks)
    }

    pub fn spades_deck() -> Kartendeck {
        let mut deck = Kartendeck::french_deck();
        deck.karten.remove_karte(&Karte::neu("two", "clubs"));
        deck.karten.remove_karte(&Karte::neu("two", "diamonds"));
        let jokers = Karten::jokers();

        deck.karten.prepend(&jokers);
        deck
    }

    pub fn tarot_deck() -> Kartendeck {
        let arcana_suits = Anzug::generate_arcana_suits();
        let mut arcana_suits_enumerator = arcana_suits.iter().enumerate();
        let major_arcana_ranks = Rang::generate_major_arcana_ranks();
        let minor_arcana_ranks = Rang::generate_minor_arcana_ranks();

        let mut karten: Karten = Karten::neu();

        let (_, major_arcana_suit) = arcana_suits_enumerator.next().unwrap();

        // Generate Major Arcana
        for (_, rank) in major_arcana_ranks.iter().enumerate() {
            karten.add(karten::Karte::new_from_structs(rank.clone(), major_arcana_suit.clone()));
        }

        // Generate Minor Arcana
        for (_, suit) in arcana_suits_enumerator {
            for (_, rank) in minor_arcana_ranks.iter().enumerate() {
                karten.add(karten::Karte::new_from_structs(rank.clone(), suit.clone()));
            }
        }

        let ranks = [&major_arcana_ranks[..], &minor_arcana_ranks[..]].concat();
        Kartendeck::new(karten, arcana_suits, ranks)
    }

    pub fn mischen(&self) -> Karten {
        self.karten.mischen()
    }

    pub fn sortieren(&self, karten: Karten) -> Karten {
        let mut sortiert = Karten::neu();

        sortiert
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::fluent::{LOCALES, US_ENGLISH};

    #[test]
    fn it_works() {
        assert_eq!("♠", LOCALES.lookup(&US_ENGLISH, "spades-symbol"));
        assert_eq!("♤", LOCALES.lookup(&US_ENGLISH, "spades-light-symbol"));
    }

    #[test]
    fn spades_deck() {
        let deck = Kartendeck::spades_deck();

        assert!(!deck.karten.contains(&Karte::neu("two", "clubs")));
        assert!(!deck.karten.contains(&Karte::neu("two", "diamonds")));
        assert!(deck.karten.contains(&Karte::neu("two", "spades")));
    }
}
