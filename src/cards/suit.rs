/*  CardPack - A generic pack of cards library written in Rust.
Copyright (C) <2020>  Christoph Baker

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::*;

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

/// Suit struct for a playing card. Made up of the suit's name, letter, and symbol.
/// Supports internationalization through fluent template files.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Suit {
    pub weight: isize,
    pub raw: String,
}

impl Suit {
    pub fn new<S: std::clone::Clone>(name: S) -> Suit
    where
        S: Into<String>,
    {
        let n = name.into();
        let weight = get_weight_isize(n.as_str());
        Suit::new_with_weight(n, weight)
    }

    pub fn new_with_weight<S: std::clone::Clone>(name: S, value: isize) -> Suit
    where
        S: Into<String>,
    {
        Suit {
            weight: value,
            raw: name.into(),
        }
    }

    pub fn get_default_short(&self) -> String {
        self.get_short(&US_ENGLISH)
    }

    pub fn get_short(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-letter", self.raw);
        get_value_by_key(key.as_str(), lid)
    }

    pub fn get_default_long(&self) -> String {
        self.get_long(&US_ENGLISH)
    }

    pub fn get_long(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-name", self.raw);
        get_value_by_key(key.as_str(), lid)
    }

    pub fn get_symbol(&self) -> String {
        let key = format!("{}-symbol", self.raw);
        get_value_by_key(key.as_str(), &US_ENGLISH)
    }

    fn top_down_value(len: usize, i: usize) -> isize {
        (len - i) as isize
    }

    fn from_array_gen(s: &[&str], f: impl Fn(usize, usize) -> isize) -> Vec<Suit> {
        let mut v: Vec<Suit> = Vec::new();

        #[allow(clippy::into_iter_on_ref)]
        for (i, &elem) in s.into_iter().enumerate() {
            let value = f(s.len(), i);
            v.push(Suit::new_with_weight(elem, value));
        }
        v
    }

    pub fn from_array(s: &[&str]) -> Vec<Suit> {
        Suit::from_array_gen(s, Suit::top_down_value)
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
        write!(f, "{}", self.get_symbol())
    }
}

impl Weighty for Suit {
    fn revise_weight(&mut self, new_value: isize) {
        self.weight = new_value
    }

    fn get_weight(&self) -> isize {
        self.weight
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suit_tests {
    use super::*;
    use crate::fluent::GERMAN;

    #[test]
    fn display() {
        assert_eq!("Suit: ♥", format!("Suit: {}", Suit::new(HEARTS)));
    }

    #[test]
    fn new() {
        let expected = Suit {
            weight: 4,
            raw: SPADES.to_string(),
        };

        assert_eq!(expected, Suit::new(SPADES));
    }

    #[test]
    fn new_with_value() {
        let expected = Suit {
            weight: 4,
            raw: SPADES.to_string(),
        };

        assert_eq!(expected, Suit::new_with_weight(SPADES, 4));
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Suit::new_with_weight(SPADES, 3),
            Suit::new_with_weight(SPADES, 4)
        );
        assert_ne!(
            Suit::new_with_weight(HEARTS, 4),
            Suit::new_with_weight(SPADES, 4)
        );
    }

    #[test]
    fn get_short() {
        let clubs = Suit::new(CLUBS);

        assert_eq!("C".to_string(), clubs.get_default_short());
        assert_eq!("K".to_string(), clubs.get_short(&GERMAN));
    }

    #[test]
    fn get_symbol() {
        let clubs = Suit::new(CLUBS);

        assert_eq!("♣".to_string(), clubs.get_symbol());
    }

    #[test]
    fn get_long() {
        let clubs = Suit::new("clubs");

        assert_eq!("Clubs".to_string(), clubs.get_long(&US_ENGLISH));
        assert_eq!("Klee".to_string(), clubs.get_long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Suit::new(CLUBS).to_string(), "♣".to_string());
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = Suit::new("clubs");

        assert_eq!(clubs.get_short(&GERMAN), "K".to_string());
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Suit> = Vec::new();
        expected.push(Suit::new_with_weight("clubs", 2));
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
        assert_eq!(4, wands.get_weight());

        wands.revise_weight(3);

        assert_eq!(3, wands.get_weight());
    }
}
