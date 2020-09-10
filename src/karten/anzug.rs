use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, US_ENGLISH};
use crate::karten::anzug_buchstabe::AnzugBuchstabe;
use crate::karten::anzug_name::AnzugName;
use crate::karten::anzug_symbol::AnzugSymbol;

/// Suit (Anzug) struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Debug, PartialEq)]
pub struct Anzug {
    pub name: AnzugName,
    pub buchstabe: AnzugBuchstabe,
    pub symbol: AnzugSymbol,
}

impl Anzug {
    pub fn new<S: std::clone::Clone>(name: S) -> Anzug
    where
        S: Into<String>,
    {
        Anzug {
            name: AnzugName::new(name.clone()),
            buchstabe: AnzugBuchstabe::new(name.clone()),
            symbol: AnzugSymbol::new(name),
        }
    }

    pub fn to_vec(s: &[&str]) -> Vec<Anzug> {
        let mut v: Vec<Anzug> = Vec::new();

        for (_, &elem) in s.into_iter().enumerate() {
            v.push(Anzug::new(elem));
        }
        v
    }

    pub fn generate_french_suits() -> Vec<Anzug> {
        Anzug::to_vec(&["spades", "hearts", "diamonds", "clubs"])
    }

    pub fn generate_major_arcana_suits() -> Vec<Anzug> {
        Anzug::to_vec(&["major-arcana"])
    }

    pub fn generate_minor_arcana_suits() -> Vec<Anzug> {
        Anzug::to_vec(&["wands", "cups", "swords", "pentacles"])
    }
}

impl ToLocaleString for Anzug {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        self.symbol.to_locale_string(lid)
    }
}

impl fmt::Display for Anzug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn as_str() {
        assert_eq!(Anzug::new("diamonds").to_string().as_str(), "♦");
        assert_eq!(Anzug::new("spades").to_string().as_str(), "♠");
    }

    #[test]
    fn display() {
        assert_eq!("Anzug: ♥", format!("Anzug: {}", Anzug::new("hearts")));
    }

    #[test]
    fn new() {
        let expected = Anzug {
            name: AnzugName::new("spades"),
            buchstabe: AnzugBuchstabe::new("spades"),
            symbol: AnzugSymbol::new("spades"),
        };

        assert_eq!(expected, Anzug::new("spades"));
    }

    #[test]
    fn to_string() {
        assert_eq!(Anzug::new("clubs").to_string(), "♣".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Anzug::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "♣".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Anzug> = Vec::new();
        expected.push(Anzug::new("clubs"));
        expected.push(Anzug::new("spades"));

        assert_eq!(expected, Anzug::to_vec(&["clubs", "spades"]));
    }

    #[test]
    fn generate_french_suits() {
        let mut expected: Vec<Anzug> = Vec::new();
        expected.push(Anzug::new("spades"));
        expected.push(Anzug::new("hearts"));
        expected.push(Anzug::new("diamonds"));
        expected.push(Anzug::new("clubs"));

        assert_eq!(expected, Anzug::generate_french_suits());
    }

    #[test]
    fn generate_minor_arcana_suits() {
        let mut expected: Vec<Anzug> = Vec::new();
        expected.push(Anzug::new("wands"));
        expected.push(Anzug::new("cups"));
        expected.push(Anzug::new("swords"));
        expected.push(Anzug::new("pentacles"));

        assert_eq!(expected, Anzug::generate_minor_arcana_suits());
    }
}
