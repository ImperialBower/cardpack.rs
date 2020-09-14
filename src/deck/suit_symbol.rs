use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Karten Anzug Symbol (Card Suit Symbol) - Single field struct representing the symbol of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SuitSymbol(String);

impl SuitSymbol {
    // Accepts String or &str
    pub fn new<S>(name: S) -> SuitSymbol
    where
        S: Into<String>,
    {
        SuitSymbol(name.into())
    }
}

impl ToLocaleString for SuitSymbol {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-symbol";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for SuitSymbol {
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
            format!("AnzugSymbol: {}", SuitSymbol::new("hearts"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(SuitSymbol::new("diamonds").to_string().as_str(), "♦");
        assert_eq!(SuitSymbol::new("spades").to_string().as_str(), "♠");
    }

    #[test]
    fn to_string() {
        assert_eq!(SuitSymbol::new("clubs").to_string(), "♣".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(
            SuitSymbol("from".to_string()),
            SuitSymbol::new(from_string)
        );
        assert_eq!(SuitSymbol("from".to_string()), SuitSymbol::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = SuitSymbol::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "♣".to_string());
    }
}
