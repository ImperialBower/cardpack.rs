use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};

/// Karten Anzug Name (Card Suit Name) - Single field struct representing the name of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct RankName(String);

impl RankName {
    // Accepts String or &str
    pub fn new<S>(name: S) -> RankName
    where
        S: Into<String>,
    {
        RankName(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for RankName {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-name";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for RankName {
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
            format!("RangName: {}", RankName::new("jack"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(RankName::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(RankName::new("ace").to_string(), "Ace".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(RankName("from".to_string()), RankName::new(from_string));
        assert_eq!(RankName("from".to_string()), RankName::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        assert_eq!(
            RankName::new("queen").to_locale_string(&GERMAN),
            "Dame".to_string()
        );
        assert_eq!(
            RankName::new("ace").to_locale_string(&GERMAN),
            "Ass".to_string()
        );
        assert_eq!(
            RankName::new("jack").to_locale_string(&GERMAN),
            "Bube".to_string()
        );
    }
}
