use fluent_templates::Loader;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, LOCALES, US_ENGLISH};

/// Card Suit Name - Single field struct representing the name of a card suit.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SuitName(String);

impl SuitName {
    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    pub fn new<S>(name: S) -> SuitName
    where
        S: Into<String>,
    {
        SuitName(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for SuitName {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let var = "-name";
        let id = format!("{}{}", &self.0, var);
        LOCALES.lookup(lid, id.as_str())
    }
}

impl fmt::Display for SuitName {
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
            "Anzug: Hearts",
            format!("Anzug: {}", SuitName::new("hearts"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(SuitName::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(
            SuitName::new("diamonds").to_string(),
            "Diamonds".to_string()
        );
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(SuitName("from".to_string()), SuitName::new(from_string));
        assert_eq!(SuitName("from".to_string()), SuitName::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = SuitName::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "Klee".to_string());
    }
}
