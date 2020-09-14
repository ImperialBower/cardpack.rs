use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};
use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

/// Karten Anzug Name (Card Suit Letter) - Single field struct representing the letter of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SuitLetter(String);

impl SuitLetter {
    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    pub fn new<S>(name: S) -> SuitLetter
    where
        S: Into<String>,
    {
        SuitLetter(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for SuitLetter {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-letter";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for SuitLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_letter_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "AnzugBuchstabe: H",
            format!("AnzugBuchstabe: {}", SuitLetter::new("hearts"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(SuitLetter::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(SuitLetter::new("diamonds").to_string(), "D".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(SuitLetter("from".to_string()), SuitLetter::new(from_string));
        assert_eq!(SuitLetter("from".to_string()), SuitLetter::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = SuitLetter::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "K".to_string());
    }
}
