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
        let mut v: Vec<Rang> = Vec::new();

        for (_, &elem) in s.into_iter().enumerate() {
            v.push(Rang::new(elem));
        }
        v
    }

    pub fn generate_french_ranks() -> Vec<Rang> {
        Rang::to_vec(&[
            "ace", "king", "queen", "jack", "ten", "nine", "eight", "seven", "six", "five", "four",
            "three", "two",
        ])
    }

    pub fn generate_pinochle_ranks() -> Vec<Rang> {
        Rang::to_vec(&["ace", "ten", "king", "queen", "jack", "nine"])
    }

    pub fn generate_major_arcana_ranks() -> Vec<Rang> {
        Rang::to_vec(&[
            "fool",
            "magician",
            "priestess",
            "empress",
            "emperor",
            "hierophant",
            "lovers",
            "chariot",
            "strength",
            "hermit",
            "fortune",
            "justice",
            "hanged",
            "death",
            "temperance",
            "devil",
            "tower",
            "star",
            "moon",
            "sun",
            "judgement",
            "world",
        ])
    }

    pub fn generate_minor_arcana_ranks() -> Vec<Rang> {
        Rang::to_vec(&[
            "king", "queen", "knight", "page", "ten", "nine", "eight", "seven", "six", "five",
            "four", "three", "two", "ace",
        ])
    }

    pub fn generate_spades_ranks() -> Vec<Rang> {
        Rang::to_vec(&[
            "big-joker",
            "little-joker",
            "ace",
            "king",
            "queen",
            "knight",
            "page",
            "ten",
            "nine",
            "eight",
            "seven",
            "six",
            "five",
            "four",
            "three",
            "two",
        ])
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
        let mut expected: Vec<Rang> = Vec::new();
        expected.push(Rang::new("king"));
        expected.push(Rang::new("queen"));

        assert_eq!(expected, Rang::to_vec(&["king", "queen"]));
    }

    #[test]
    fn generate_french_ranks() {
        let mut expected: Vec<Rang> = Vec::new();
        expected.push(Rang::new("ace"));
        expected.push(Rang::new("king"));
        expected.push(Rang::new("queen"));
        expected.push(Rang::new("jack"));
        expected.push(Rang::new("ten"));
        expected.push(Rang::new("nine"));
        expected.push(Rang::new("eight"));
        expected.push(Rang::new("seven"));
        expected.push(Rang::new("six"));
        expected.push(Rang::new("five"));
        expected.push(Rang::new("four"));
        expected.push(Rang::new("three"));
        expected.push(Rang::new("two"));

        assert_eq!(expected, Rang::generate_french_ranks());
    }

    #[test]
    fn generate_pinochle_ranks() {
        let mut expected: Vec<Rang> = Vec::new();
        expected.push(Rang::new("ace"));
        expected.push(Rang::new("ten"));
        expected.push(Rang::new("king"));
        expected.push(Rang::new("queen"));
        expected.push(Rang::new("jack"));
        expected.push(Rang::new("nine"));

        assert_eq!(expected, Rang::generate_pinochle_ranks());
    }

    #[test]
    fn generate_major_arcana_ranks() {
        let major = Rang::generate_major_arcana_ranks();

        assert_eq!(22, major.len());
    }

    #[test]
    fn generate_minor_arcana_ranks() {
        let ex: Vec<Rang> = Rang::to_vec(&[
            "king", "queen", "knight", "page", "ten", "nine", "eight", "seven", "six", "five",
            "four", "three", "two", "ace",
        ]);

        assert_eq!(ex, Rang::generate_minor_arcana_ranks());
    }

    #[test]
    fn generate_spades_ranks() {
        assert_eq!(16, Rang::generate_spades_ranks().len());
    }
}
