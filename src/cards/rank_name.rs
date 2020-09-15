use std::fmt;

use crate::fluent::{ToLocaleString, US_ENGLISH};

/// Card Rank Name - Single field struct representing the name of a card rank.
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
    fn get_fluent_key(&self) -> String {
        self.0.to_owned() + &*"-name".to_owned()
    }

    fn get_raw_name(&self) -> &str {
        self.0.as_str()
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
