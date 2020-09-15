use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::cards::suit_letter::SuitLetter;
use crate::cards::suit_name::SuitName;
use crate::cards::suit_symbol::SuitSymbol;
use crate::fluent::*;

/// Suit struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Suit {
    pub value: isize,
    pub name: SuitName,
    pub letter: SuitLetter,
    pub symbol: SuitSymbol,
}

impl Suit {
    pub fn new<S: std::clone::Clone>(name: S) -> Suit
    where
        S: Into<String>,
    {
        let n = name.into();
        let value = get_value_isize(n.as_str());
        Suit::new_with_value(n, value)
    }

    pub fn new_with_value<S: std::clone::Clone>(name: S, value: isize) -> Suit
    where
        S: Into<String>,
    {
        Suit {
            value,
            name: SuitName::new(name.clone()),
            letter: SuitLetter::new(name.clone()),
            symbol: SuitSymbol::new(name),
        }
    }

    fn bottom_up_value(_len: usize, i: usize) -> isize {
        (i + 1) as isize
    }

    fn top_down_value(len: usize, i: usize) -> isize {
        (len - i) as isize
    }

    fn from_array_gen(s: &[&str], f: impl Fn(usize, usize) -> isize) -> Vec<Suit> {
        let mut v: Vec<Suit> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let value = f(s.len(), i);
            v.push(Suit::new_with_value(elem, value));
        }
        v
    }

    pub fn from_array(s: &[&str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::top_down_value)
    }

    pub fn from_array_bottom_up(s: &[&str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::bottom_up_value)
    }

    pub fn generate_french_suits() -> Vec<Suit> {
        Suit::from_array(&["spades", "hearts", "diamonds", "clubs"])
    }

    pub fn generate_arcana_suits() -> Vec<Suit> {
        Suit::from_array(&["major-arcana", "wands", "cups", "swords", "pentacles"])
    }

    pub fn generate_skat_suits() -> Vec<Suit> {
        Suit::from_array(&["eichel", "laub", "herz", "schellen"])
    }
}

impl ToLocaleString for Suit {
    fn get_fluent_key(&self) -> String {
        unimplemented!()
    }

    fn get_raw_name(&self) -> &str {
        self.name.get_raw_name()
    }

    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        self.symbol.to_locale_string(lid)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol.to_locale_string(&US_ENGLISH))
    }
}

impl Valuable for Suit {
    fn revise_value(&mut self, new_value: isize) {
        self.value = new_value
    }

    fn get_value(&self) -> isize {
        self.value
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn as_str() {
        assert_eq!(Suit::new("diamonds").to_string().as_str(), "♦");
        assert_eq!(Suit::new("spades").to_string().as_str(), "♠");
    }

    #[test]
    fn display() {
        assert_eq!("Suit: ♥", format!("Suit: {}", Suit::new("hearts")));
    }

    #[test]
    fn new() {
        let expected = Suit {
            value: 4,
            name: SuitName::new("spades"),
            letter: SuitLetter::new("spades"),
            symbol: SuitSymbol::new("spades"),
        };

        assert_eq!(expected, Suit::new("spades"));
    }

    #[test]
    fn new_with_value() {
        let expected = Suit {
            value: 4,
            name: SuitName::new("spades"),
            letter: SuitLetter::new("spades"),
            symbol: SuitSymbol::new("spades"),
        };

        assert_eq!(expected, Suit::new_with_value("spades", 4));
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Suit::new_with_value("spades", 3),
            Suit::new_with_value("spades", 4)
        );
        assert_ne!(
            Suit::new_with_value("hearts", 4),
            Suit::new_with_value("spades", 4)
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(Suit::new("clubs").to_string(), "♣".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Suit::new("clubs");

        assert_eq!(clubs.to_locale_string(&GERMAN), "♣".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("clubs", 2));
        expected.push(Suit::new_with_value("spades", 1));

        assert_eq!(expected, Suit::from_array(&["clubs", "spades"]));
    }

    #[test]
    fn to_vec_bottom_up() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("clubs", 1));
        expected.push(Suit::new_with_value("spades", 2));

        assert_eq!(expected, Suit::from_array_bottom_up(&["clubs", "spades"]));
    }

    #[test]
    fn generate_french_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("spades", 4));
        expected.push(Suit::new_with_value("hearts", 3));
        expected.push(Suit::new_with_value("diamonds", 2));
        expected.push(Suit::new_with_value("clubs", 1));

        assert_eq!(expected, Suit::generate_french_suits());
    }

    #[test]
    fn generate_arcana_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_value("major-arcana", 5));
        expected.push(Suit::new_with_value("wands", 4));
        expected.push(Suit::new_with_value("cups", 3));
        expected.push(Suit::new_with_value("swords", 2));
        expected.push(Suit::new_with_value("pentacles", 1));

        assert_eq!(expected, Suit::generate_arcana_suits());
    }

    #[test]
    fn revise_value() {
        let mut wands = Suit::new("wands");
        assert_eq!(4, wands.get_value());

        wands.revise_value(3);

        assert_eq!(3, wands.get_value());
    }
}
