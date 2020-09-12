use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::*;
use crate::karten::anzug_buchstabe::AnzugBuchstabe;
use crate::karten::anzug_name::AnzugName;
use crate::karten::anzug_symbol::AnzugSymbol;

/// Suit (Anzug) struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Anzug {
    pub wert: u8,
    pub name: AnzugName,
    pub buchstabe: AnzugBuchstabe,
    pub symbol: AnzugSymbol,
}

impl Anzug {
    pub fn new<S: std::clone::Clone>(name: S) -> Anzug
    where
        S: Into<String>,
    {
        let n = name.into();
        let wert = get_value_u8(n.clone().as_str());
        Anzug::new_with_value(n, wert)
    }

    pub fn new_with_value<S: std::clone::Clone>(name: S, wert: u8) -> Anzug
        where
            S: Into<String>,
    {
        Anzug {
            wert,
            name: AnzugName::new(name.clone()),
            buchstabe: AnzugBuchstabe::new(name.clone()),
            symbol: AnzugSymbol::new(name),
        }
    }

    pub fn to_vec(s: &[&str]) -> Vec<Anzug> {
        let mut v: Vec<Anzug> = Vec::new();

        for (i, &elem) in s.into_iter().enumerate() {
            let wert = s.len() - i;
            v.push(Anzug::new_with_value(elem, wert as u8));
        }
        v
    }

    pub fn generate_french_suits() -> Vec<Anzug> {
        Anzug::to_vec(&["spades", "hearts", "diamonds", "clubs"])
    }

    pub fn generate_arcana_suits() -> Vec<Anzug> {
        Anzug::to_vec(&["major-arcana", "wands", "cups", "swords", "pentacles"])
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

impl Valuable for Anzug {
    fn revise_value(&mut self, new_value: u8) {
        self.wert = new_value
    }

    fn get_value(&self) -> u8 {
        self.wert
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
            wert: 4,
            name: AnzugName::new("spades"),
            buchstabe: AnzugBuchstabe::new("spades"),
            symbol: AnzugSymbol::new("spades"),
        };

        assert_eq!(expected, Anzug::new("spades"));
    }

    #[test]
    fn new_with_value() {
        let expected = Anzug {
            wert: 4,
            name: AnzugName::new("spades"),
            buchstabe: AnzugBuchstabe::new("spades"),
            symbol: AnzugSymbol::new("spades"),
        };

        assert_eq!(expected, Anzug::new_with_value("spades", 4));
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
        expected.push(Anzug::new_with_value("clubs", 2));
        expected.push(Anzug::new_with_value("spades", 1));

        assert_eq!(expected, Anzug::to_vec(&["clubs", "spades"]));
    }

    #[test]
    fn generate_french_suits() {
        let mut expected: Vec<Anzug> = Vec::new();
        expected.push(Anzug::new_with_value("spades", 4));
        expected.push(Anzug::new_with_value("hearts", 3));
        expected.push(Anzug::new_with_value("diamonds", 2));
        expected.push(Anzug::new_with_value("clubs", 1));

        assert_eq!(expected, Anzug::generate_french_suits());
    }

    #[test]
    fn generate_arcana_suits() {
        let mut expected: Vec<Anzug> = Vec::new();
        expected.push(Anzug::new_with_value("major-arcana", 5));
        expected.push(Anzug::new_with_value("wands", 4));
        expected.push(Anzug::new_with_value("cups", 3));
        expected.push(Anzug::new_with_value("swords", 2));
        expected.push(Anzug::new_with_value("pentacles", 1));

        assert_eq!(expected, Anzug::generate_arcana_suits());
    }

    #[test]
    fn revise_value() {
        let mut wands = Anzug::new("wands");
        assert_eq!(4, wands.get_value());

        wands.revise_value(3);

        assert_eq!(3, wands.get_value());
    }
}
