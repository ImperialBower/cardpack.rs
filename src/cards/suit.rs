use std::fmt;

use crate::fluent_name::FluentName;
use crate::Named;
use crate::FLUENT_SYMBOL_SECTION;
use crate::US_ENGLISH;

// French Deck Suit Fluent Identifiers
pub const SPADES: &str = "spades";
pub const HEARTS: &str = "hearts";
pub const DIAMONDS: &str = "diamonds";
pub const CLUBS: &str = "clubs";
// Tarot Deck Suit Fluent Identifiers
pub const MAJOR_ARCANA: &str = "major-arcana";
pub const WANDS: &str = "wands";
pub const CUPS: &str = "cups";
pub const SWORDS: &str = "swords";
pub const PENTACLES: &str = "pentacles";
// Skat Suit Fluent Identifiers
pub const EICHEL: &str = "eichel"; // Acorns
pub const LAUB: &str = "laub"; // Leaves
pub const HERZ: &str = "herz"; // Hearts
pub const SHELLEN: &str = "schellen"; // Bells
                                      // Special Suits
pub const TRUMP: &str = "trump";
pub const BLANK: &str = "_";

/// Suit struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Suit {
    pub weight: isize,
    pub name: FluentName,
}

impl Suit {
    pub fn new(name: &'static str) -> Suit {
        let name = FluentName::new(name);
        Suit {
            weight: name.default_weight(),
            name,
        }
    }

    pub fn new_with_weight(name: &'static str, weight: isize) -> Suit {
        Suit {
            weight,
            name: FluentName::new(name),
        }
    }

    pub fn symbol(&self) -> String {
        self.name.fluent_value(FLUENT_SYMBOL_SECTION, &US_ENGLISH)
    }

    fn top_down_value(len: usize, i: usize) -> isize {
        (len - i) as isize
    }

    fn from_array_gen(s: &[&'static str], f: impl Fn(usize, usize) -> isize) -> Vec<Suit> {
        let mut v: Vec<Suit> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let value = f(s.len(), i);
            v.push(Suit::new_with_weight(elem, value));
        }
        v
    }

    pub fn from_array(s: &[&'static str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::top_down_value)
    }

    /// Returns a Suit from its symbol string.
    pub fn from_french_deck_index(symbol: &'static str) -> Suit {
        match symbol {
            "♠" => Suit::new(SPADES),
            "S" => Suit::new(SPADES),
            "♥" => Suit::new(HEARTS),
            "H" => Suit::new(HEARTS),
            "♦" => Suit::new(DIAMONDS),
            "D" => Suit::new(DIAMONDS),
            "♣" => Suit::new(CLUBS),
            "C" => Suit::new(CLUBS),
            "🃟" => Suit::new(TRUMP),
            "T" => Suit::new(TRUMP),
            _ => Suit::new(BLANK),
        }
    }

    pub fn generate_french_suits() -> Vec<Suit> {
        Suit::from_array(&[SPADES, HEARTS, DIAMONDS, CLUBS])
    }

    pub fn generate_arcana_suits() -> Vec<Suit> {
        Suit::from_array(&[MAJOR_ARCANA, WANDS, CUPS, SWORDS, PENTACLES])
    }

    pub fn generate_skat_suits() -> Vec<Suit> {
        Suit::from_array(&[EICHEL, LAUB, HERZ, SHELLEN])
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl Named for Suit {
    fn name(&self) -> &str {
        self.name.name()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_tests {
    use super::*;
    use crate::{GERMAN, US_ENGLISH};
    use rstest::rstest;

    #[test]
    fn display() {
        assert_eq!("Suit: ♥", format!("Suit: {}", Suit::new(HEARTS)));
    }

    #[test]
    fn new() {
        let expected = Suit {
            weight: 4,
            name: FluentName::new(SPADES),
        };

        assert_eq!(expected, Suit::new(SPADES));
    }

    #[test]
    fn new_with_value() {
        let expected = Suit {
            weight: 4,
            name: FluentName::new(SPADES),
        };

        assert_eq!(expected, Suit::new_with_weight(SPADES, 4));
    }

    #[rstest]
    #[case("♠", Suit::new(SPADES))]
    #[case("S", Suit::new(SPADES))]
    #[case("♥", Suit::new(HEARTS))]
    #[case("H", Suit::new(HEARTS))]
    #[case("♦", Suit::new(DIAMONDS))]
    #[case("D", Suit::new(DIAMONDS))]
    #[case("♣", Suit::new(CLUBS))]
    #[case("C", Suit::new(CLUBS))]
    #[case("🃟", Suit::new(TRUMP))]
    #[case("T", Suit::new(TRUMP))]
    #[case(" ", Suit::new(BLANK))]
    #[case("FOOBAR", Suit::new(BLANK))]
    #[case("", Suit::new(BLANK))]
    fn from_french_deck_index(#[case] input: &'static str, #[case] expected: Suit) {
        assert_eq!(expected, Suit::from_french_deck_index(input));
    }

    #[test]
    fn part1ial_eq() {
        assert_ne!(
            Suit::new_with_weight(SPADES, 3),
            Suit::new_with_weight(SPADES, 4)
        );
        assert_eq!(
            Suit::new_with_weight(SPADES, 4),
            Suit::new_with_weight(SPADES, 4)
        );
    }

    #[test]
    fn get_short() {
        let clubs = Suit::new(CLUBS);

        assert_eq!("C".to_string(), clubs.name.index_default());
        assert_eq!("K".to_string(), clubs.name.index(&GERMAN));
    }

    #[test]
    fn get_symbol() {
        let clubs = Suit::new(CLUBS);

        assert_eq!("♣".to_string(), clubs.symbol());
    }

    #[test]
    fn get_long() {
        let clubs = Suit::new("clubs");

        assert_eq!("Clubs".to_string(), clubs.name.long(&US_ENGLISH));
        assert_eq!("Klee".to_string(), clubs.name.long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Suit::new(CLUBS).to_string(), "♣".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Suit::new("clubs");

        assert_eq!(clubs.name.index(&GERMAN), "K".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_weight(CLUBS, 2));
        expected.push(Suit::new_with_weight(SPADES, 1));

        assert_eq!(expected, Suit::from_array(&[CLUBS, SPADES]));
    }

    #[test]
    fn generate_french_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_weight(SPADES, 4));
        expected.push(Suit::new_with_weight(HEARTS, 3));
        expected.push(Suit::new_with_weight(DIAMONDS, 2));
        expected.push(Suit::new_with_weight(CLUBS, 1));

        assert_eq!(expected, Suit::generate_french_suits());
    }

    #[test]
    fn generate_arcana_suits() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_weight(MAJOR_ARCANA, 5));
        expected.push(Suit::new_with_weight(WANDS, 4));
        expected.push(Suit::new_with_weight(CUPS, 3));
        expected.push(Suit::new_with_weight(SWORDS, 2));
        expected.push(Suit::new_with_weight(PENTACLES, 1));

        assert_eq!(expected, Suit::generate_arcana_suits());
    }

    #[test]
    fn revise_value() {
        let mut wands = Suit::new(WANDS);
        assert_eq!(4, wands.weight);

        wands.weight = 3;

        assert_eq!(3, wands.weight);
    }
}
