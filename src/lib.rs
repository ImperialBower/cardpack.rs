pub use karten::*;

mod fluent;
mod karten;

use crate::fluent::{LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;

/// Deck of Cards (Kartendeck) that includes rank of suites (anzug_rang) and values (rangfolge).
#[derive(Clone, Debug, PartialEq)]
pub struct Kartendeck {
    pub karten: Vec<Karte>,
    pub anzug_rang: Vec<Anzug>,
    pub rangfolge: Vec<Rang>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("♠", LOCALES.lookup(&US_ENGLISH, "spades-symbol"));
        assert_eq!("♤", LOCALES.lookup(&US_ENGLISH, "spades-light-symbol"));
    }
}
