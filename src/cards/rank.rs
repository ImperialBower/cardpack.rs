use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::*;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rank {
    pub weight: isize,
    pub raw: String,
}

impl Rank {
    pub fn new<S: std::clone::Clone>(name: S) -> Rank
    where
        S: Into<String>,
    {
        let n = name.into();
        let weight = get_weight_isize(n.as_str());
        Rank::new_with_value(n, weight)
    }

    pub fn new_with_value<S: std::clone::Clone>(name: S, weight: isize) -> Rank
    where
        S: Into<String>,
    {
        Rank {
            weight,
            raw: name.into(),
        }
    }

    pub fn from_array(s: &[&str]) -> Vec<Rank> {
        let mut v: Vec<Rank> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let weight = (s.len() + 1) - i;
            v.push(Rank::new_with_value(elem, weight as isize));
        }
        v
    }

    pub fn get_short(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-short", self.raw);
        get_value_by_key(key.as_str(), lid)
    }

    pub fn get_default_long(&self) -> String {
        self.get_long(&US_ENGLISH)
    }

    pub fn get_long(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-name", self.raw);
        get_value_by_key(key.as_str(), lid)
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

    pub fn generate_skat_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            "daus", "king", "ober", "unter", "ten", "nine", "eight", "seven",
        ])
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_short(&US_ENGLISH))
    }
}

impl Weighty for Rank {
    fn revise_weight(&mut self, new_value: isize) {
        self.weight = new_value
    }

    fn get_weight(&self) -> isize {
        self.weight
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use crate::fluent::GERMAN;

    #[test]
    fn display() {
        assert_eq!("Rang: A", format!("Rang: {}", Rank::new("ace")));
    }

    #[test]
    fn get_short() {
        let queen = Rank::new("queen");

        assert_eq!("Q".to_string(), queen.get_short(&US_ENGLISH));
        assert_eq!("D".to_string(), queen.get_short(&GERMAN));
    }

    #[test]
    fn get_long() {
        let ace = Rank::new("ace");

        assert_eq!("Ace".to_string(), ace.get_long(&US_ENGLISH));
        assert_eq!("Ass".to_string(), ace.get_long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::new("king").to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rank {
            weight: 9,
            raw: "nine".to_string(),
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

    #[test]
    fn revise_value() {
        let mut ace = Rank::new("ace");
        assert_eq!(14, ace.get_weight());

        ace.revise_weight(3);

        assert_eq!(3, ace.get_weight());
    }
}
