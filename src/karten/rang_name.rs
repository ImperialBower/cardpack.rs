use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};

/// Karten Anzug Name (Card Suit Name) - Single field struct representing the name of a card suit.
///
#[derive(Clone, Debug, PartialEq)]
pub struct RangName(String);

impl RangName {
    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    pub fn new<S>(name: S) -> RangName
    where
        S: Into<String>,
    {
        RangName(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for RangName {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-name";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for RangName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suite_name_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "RangName: Jack",
            format!("RangName: {}", RangName::new("jack"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(RangName::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(RangName::new("ace").to_string(), "Ace".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(RangName("from".to_string()), RangName::new(from_string));
        assert_eq!(RangName("from".to_string()), RangName::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        assert_eq!(RangName::new("queen").to_locale_string(&GERMAN), "Dame".to_string());
        assert_eq!(RangName::new("ace").to_locale_string(&GERMAN), "Ass".to_string());
        assert_eq!(RangName::new("jack").to_locale_string(&GERMAN), "Bube".to_string());
    }
}
