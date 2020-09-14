use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::deck::rank_name::RankName;
use crate::deck::rank_short::RankShort;
use crate::fluent::*;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rank {
    pub value: isize,
    pub name: RankName,
    pub short: RankShort,
}

impl Rank {
    pub fn new<S: std::clone::Clone>(name: S) -> Rank
    where
        S: Into<String>,
    {
        let n = name.into();
        let value = get_value_isize(n.as_str());
        Rank::new_with_value(n, value)
    }

    pub fn new_with_value<S: std::clone::Clone>(name: S, value: isize) -> Rank
    where
        S: Into<String>,
    {
        Rank {
            value,
            name: RankName::new(name.clone()),
            short: RankShort::new(name),
        }
    }

    pub fn from_array(s: &[&str]) -> Vec<Rank> {
        let mut v: Vec<Rank> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let value = (s.len() + 1) - i;
            v.push(Rank::new_with_value(elem, value as isize));
        }
        v
    }

    pub fn generate_french_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            "ace", "king", "queen", "jack", "ten", "nine", "eight", "seven", "six", "five", "four",
            "three", "two",
        ])
    }

    pub fn generate_pinochle_ranks() -> Vec<Rank> {
        Rank::from_array(&["ace", "ten", "king", "queen", "jack", "nine"])
    }

    pub fn generate_major_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
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

    pub fn generate_minor_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            "king", "queen", "knight", "page", "ten", "nine", "eight", "seven", "six", "five",
            "four", "three", "two", "ace",
        ])
    }

    pub fn generate_spades_ranks() -> Vec<Rank> {
        Rank::from_array(&[
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

impl ToLocaleString for Rank {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        self.short.to_locale_string(lid)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!("Rang: A", format!("Rang: {}", Rank::new("ace")));
    }

    #[test]
    fn as_str() {
        assert_eq!(Rank::new("ace").to_string().as_str(), "A");
        assert_eq!(Rank::new("two").to_string().as_str(), "2");
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::new("king").to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rank {
            value: 9,
            name: RankName::new("nine"),
            short: RankShort::new("nine"),
        };

        assert_eq!(expected, Rank::new("nine"));
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Rank::new_with_value("nine", 3),
            Rank::new_with_value("nine", 4)
        );
        assert_ne!(
            Rank::new_with_value("ten", 4),
            Rank::new_with_value("nine", 4)
        );
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Rank::new("queen");

        assert_eq!(clubs.to_locale_string(&GERMAN), "D".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_value("king", 3));
        expected.push(Rank::new_with_value("queen", 2));

        assert_eq!(expected, Rank::from_array(&["king", "queen"]));
    }

    #[test]
    fn generate_french_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new("ace"));
        expected.push(Rank::new("king"));
        expected.push(Rank::new("queen"));
        expected.push(Rank::new("jack"));
        expected.push(Rank::new("ten"));
        expected.push(Rank::new("nine"));
        expected.push(Rank::new("eight"));
        expected.push(Rank::new("seven"));
        expected.push(Rank::new("six"));
        expected.push(Rank::new("five"));
        expected.push(Rank::new("four"));
        expected.push(Rank::new("three"));
        expected.push(Rank::new("two"));

        assert_eq!(expected, Rank::generate_french_ranks());
    }

    #[test]
    fn generate_pinochle_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_value("ace", 7));
        expected.push(Rank::new_with_value("ten", 6));
        expected.push(Rank::new_with_value("king", 5));
        expected.push(Rank::new_with_value("queen", 4));
        expected.push(Rank::new_with_value("jack", 3));
        expected.push(Rank::new_with_value("nine", 2));

        assert_eq!(expected, Rank::generate_pinochle_ranks());
    }

    #[test]
    fn generate_major_arcana_ranks() {
        let major = Rank::generate_major_arcana_ranks();

        assert_eq!(22, major.len());
    }

    #[test]
    fn generate_minor_arcana_ranks() {
        let ex: Vec<Rank> = Rank::from_array(&[
            "king", "queen", "knight", "page", "ten", "nine", "eight", "seven", "six", "five",
            "four", "three", "two", "ace",
        ]);

        assert_eq!(ex, Rank::generate_minor_arcana_ranks());
    }

    #[test]
    fn generate_spades_ranks() {
        assert_eq!(16, Rank::generate_spades_ranks().len());
    }
}
