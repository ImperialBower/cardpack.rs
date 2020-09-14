use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Karten Anzug Name (Card Suit Letter) - Single field struct representing the letter of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct RankShort(String);

impl RankShort {
    // Accepts String or &str
    pub fn new<S>(name: S) -> RankShort
    where
        S: Into<String>,
    {
        RankShort(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for RankShort {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-short";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for RankShort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_letter_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "RangKurz: A",
            format!("RangKurz: {}", RankShort::new("ace"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(RankShort::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(RankShort::new("king").to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(RankShort("from".to_string()), RankShort::new(from_string));
        assert_eq!(RankShort("from".to_string()), RankShort::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = RankShort::new("ten");

        assert_eq!(clubs.to_locale_string(&GERMAN), "10".to_string());
    }
}
