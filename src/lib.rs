pub use karten::*;
pub use karten::anzug::*;

mod fluent;
mod karten;

use std::collections::HashMap;

use crate::fluent::{LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;

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

    // pub fn french_deck() -> Kartendeck {
    //     let suits = ["spades", "hearts", "diamonds", "clubs"];
    //     let ranks = ["ace", "king", "queen", "jack", "ten", "nine", "eight", "seven", "six", "five", "four", "three", "two"];
    //
    //
    //     let mut karten: Vec<Karte> = Vec(Karte);
    //
    // }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("♠", LOCALES.lookup(&US_ENGLISH, "spades-symbol"));
        assert_eq!("♤", LOCALES.lookup(&US_ENGLISH, "spades-light-symbol"));
    }

    #[test]
    fn hashmap() {
        // let mut _deck<String, Karte> = HashMap::new();


    }
}
