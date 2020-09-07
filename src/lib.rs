pub use fluent::*;
pub use karten::anzug::*;
pub use karten::*;

pub mod fluent;
mod karten;

extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;
#[allow(unused_imports)]
use fluent_templates::Loader;

/// Deck of Cards (Kartendeck) that includes rank of suites (anzug_rang) and values (rangfolge).
#[derive(Clone, Debug, PartialEq)]
pub struct Kartendeck {
    pub karten: Vec<Karte>,
    pub anzugrang: Vec<Anzug>,
    pub rangfolge: Vec<Rang>,
}

impl Kartendeck {
    pub fn new(karten: Vec<Karte>, anzugrang: Vec<Anzug>, rangfolge: Vec<Rang>) -> Kartendeck {
        Kartendeck {
            karten,
            anzugrang,
            rangfolge,
        }
    }

    pub fn french_deck() -> Kartendeck {
        let suits = Anzug::generate_french_suits();
        let ranks = Rang::generate_french_ranks();

        let mut karten: Vec<Karte> = Vec::new();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.push(Karte::new_from_structs(rank.clone(), suit.clone()));
            }
        };
        Kartendeck::new(karten, suits, ranks)
    }

    pub fn pinochle_deck() -> Kartendeck {
        let suits = Anzug::generate_french_suits();
        let ranks = Rang::generate_pinochle_ranks();

        let mut karten: Vec<Karte> = Vec::new();
        for (_, suit) in suits.iter().enumerate() {
            for (_, rank) in ranks.iter().enumerate() {
                karten.push(Karte::new_from_structs(rank.clone(), suit.clone()));
                karten.push(Karte::new_from_structs(rank.clone(), suit.clone()));
            }
        };
        Kartendeck::new(karten, suits, ranks)
    }

    pub fn shuffle(&self) -> Vec<Karte> {
        let mut c = self.karten.clone();
        c.shuffle(&mut thread_rng());
        c
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
}
