use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, US_ENGLISH};
use crate::karten::rang_kurz::RangKurz;
use crate::karten::rang_name::RangName;

#[derive(Clone, Debug, PartialEq)]
pub struct Rang {
    pub name: RangName,
    pub kurz: RangKurz,
}

impl Rang {
    pub fn new<S: std::clone::Clone>(name: S) -> Rang
    where
        S: Into<String>,
    {
        Rang {
            name: RangName::new(name.clone()),
            kurz: RangKurz::new(name),
        }
    }

    pub fn to_vec(s: &[&str]) -> Vec<Rang> {
        let mut v: Vec<Rang>  = Vec::new();

        for (_, &elem) in s.into_iter().enumerate() {
            v.push(Rang::new(elem));
        }
        v
    }
}

impl ToLocaleString for Rang {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        self.kurz.to_locale_string(lid)
    }
}

impl fmt::Display for Rang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kurz.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!("Rang: A", format!("Rang: {}", Rang::new("ace")));
    }

    #[test]
    fn as_str() {
        assert_eq!(Rang::new("ace").to_string().as_str(), "A");
        assert_eq!(Rang::new("two").to_string().as_str(), "2");
    }

    #[test]
    fn to_string() {
        assert_eq!(Rang::new("king").to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rang {
            name: RangName::new("nine"),
            kurz: RangKurz::new("nine"),
        };

        assert_eq!(expected, Rang::new("nine"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Rang::new("queen");

        assert_eq!(clubs.to_locale_string(&GERMAN), "D".to_string());
    }


    #[test]
    fn to_vec() {
        let mut expected: Vec<Rang>  = Vec::new();
        expected.push(Rang::new("king"));
        expected.push(Rang::new("queen"));

        assert_eq!(expected, Rang::to_vec(&["king", "queen"]));
    }
}
