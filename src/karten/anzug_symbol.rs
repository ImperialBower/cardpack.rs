use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Karten Anzug Symbol (Card Suit Symbol) - Single field struct representing the symbol of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct AnzugSymbol(String);

impl AnzugSymbol {
    // Accepts String or &str
    pub fn new<S>(name: S) -> AnzugSymbol
    where
        S: Into<String>,
    {
        AnzugSymbol(name.into())
    }
}

impl ToLocaleString for AnzugSymbol {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-symbol";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for AnzugSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suite_symbol_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "AnzugSymbol: ♥",
            format!("AnzugSymbol: {}", AnzugSymbol::new("hearts"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(AnzugSymbol::new("diamonds").to_string().as_str(), "♦");
        assert_eq!(AnzugSymbol::new("spades").to_string().as_str(), "♠");
    }

    #[test]
    fn to_string() {
        assert_eq!(AnzugSymbol::new("clubs").to_string(), "♣".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(
            AnzugSymbol("from".to_string()),
            AnzugSymbol::new(from_string)
        );
        assert_eq!(AnzugSymbol("from".to_string()), AnzugSymbol::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = AnzugSymbol::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "♣".to_string());
    }
}
