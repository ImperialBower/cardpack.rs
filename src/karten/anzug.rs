use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, US_ENGLISH};
use crate::karten::anzug_buchstabe::AnzugBuchstabe;
use crate::karten::anzug_name::AnzugName;
use crate::karten::anzug_symbol::AnzugSymbol;

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
mod suite_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!("Anzug: ♥", format!("Anzug: {}", Anzug::new("hearts")));
    }

    #[test]
    fn as_str() {
        assert_eq!(Anzug::new("diamonds").to_string().as_str(), "♦");
        assert_eq!(Anzug::new("spades").to_string().as_str(), "♠");
    }

    #[test]
    fn to_string() {
        assert_eq!(Anzug::new("clubs").to_string(), "♣".to_string());
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
    fn to_string_by_locale() {
        let clubs = Anzug::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "♣".to_string());
    }
}
