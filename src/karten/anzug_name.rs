use std::fmt;

use fluent_templates::{Loader};
use unic_langid::{LanguageIdentifier};
use crate::fluent::{LOCALES, ToLocaleString, GERMAN, US_ENGLISH};

/// Karten Anzug Name (Card Suit Name) - Single field struct representing the name of a card suit.
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnzugName(String);

impl AnzugName {
    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    fn new<S>(name: S) -> AnzugName
    where
        S: Into<String>,
    {
        AnzugName(name.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for AnzugName {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-name";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for AnzugName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suite_name_tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!("Anzug: Hearts", format!("Anzug: {}", AnzugName::new("hearts")));
    }

    #[test]
    fn as_str() {
        assert_eq!(AnzugName::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(AnzugName::new("diamonds").to_string(), "Diamonds".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(AnzugName("from".to_string()), AnzugName::new(from_string));
        assert_eq!(AnzugName("from".to_string()), AnzugName::new("from"));
    }

    #[test]
    fn fluent() {
        let clubs = AnzugName::new("clubs");

        assert_eq!(clubs.to_string(), "Clubs".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = AnzugName::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "Klee".to_string());
    }
}
