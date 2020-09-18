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

// Constants representing the internal identifier for a Rank inside the Fluent template files.
// French Deck Ranks:
pub const ACE: &str = "ace";
pub const KING: &str = "king";
pub const QUEEN: &str = "queen";
pub const JACK: &str = "jack";
pub const TEN: &str = "ten";
pub const NINE: &str = "nine";
pub const EIGHT: &str = "eight";
pub const SEVEN: &str = "seven";
pub const SIX: &str = "six";
pub const FIVE: &str = "five";
pub const FOUR: &str = "four";
pub const THREE: &str = "three";
pub const TWO: &str = "two";
// Spades etc Ranks:
pub const BIG_JOKER: &str = "big-joker";
pub const LITTLE_JOKER: &str = "little-joker";
// Skat Deck Ranks:
pub const DAUS: &str = "daus";
pub const OBER: &str = "ober";
pub const UNTER: &str = "unter";
// Tarot Deck Ranks:
pub const FOOL: &str = "fool";
pub const MAGICIAN: &str = "magician";
pub const PRIESTESS: &str = "priestess";
pub const EMPRESS: &str = "empress";
pub const EMPEROR: &str = "emperor";
pub const HIEROPHANT: &str = "hierophant";
pub const LOVERS: &str = "lovers";
pub const CHARIOT: &str = "chariot";
pub const STRENGTH: &str = "strength";
pub const HERMIT: &str = "hermit";
pub const FORTUNE: &str = "fortune";
pub const JUSTICE: &str = "justice";
pub const HANGED: &str = "hanged";
pub const DEATH: &str = "death";
pub const TEMPERANCE: &str = "temperance";
pub const DEVIL: &str = "devil";
pub const TOWER: &str = "tower";
pub const STAR: &str = "star";
pub const MOON: &str = "moon";
pub const SUN: &str = "sun";
pub const JUDGEMENT: &str = "judgement";
pub const WORLD: &str = "world";
pub const KNIGHT: &str = "knight";
pub const PAGE: &str = "page";

/// Rank Struct for a Card. Examples of standard Card Ranks would include: Ace, Ten, and Deuce
/// Joker, Death (Tarot), and Ober (Skat). The weight of the Rank determines how a Card is sorted relative to
/// it's Suit.
///
/// There are four ways to instantiate a Rank, each of them having advantages:
///
/// # As an *instance* variable
/// ```
/// let ace = cardpack::Rank {
///     weight: 1,
///     raw: cardpack::ACE.to_string(),
/// };
/// ```
/// This gives you maximum flexibility. Since the value of the Ace is 1, it will be sorted
/// at the and of a Suit (unless there are any Cards with negative weights).
///
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

    pub fn get_default_index(&self) -> String {
        self.get_index(&US_ENGLISH)
    }

    /// "The number or letter printed in the corner of a playing card,
    /// so that it may be read when held in a fan." -- Wikipedia
    pub fn get_index(&self, lid: &LanguageIdentifier) -> String {
        let key = format!("{}-index", self.raw);
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
            ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO,
        ])
    }

    pub fn generate_pinochle_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, TEN, KING, QUEEN, JACK, NINE])
    }

    pub fn generate_major_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            FOOL, MAGICIAN, PRIESTESS, EMPRESS, EMPEROR, HIEROPHANT, LOVERS, CHARIOT, STRENGTH,
            HERMIT, FORTUNE, JUSTICE, HANGED, DEATH, TEMPERANCE, DEVIL, TOWER, STAR, MOON, SUN,
            JUDGEMENT, WORLD,
        ])
    }

    pub fn generate_minor_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            KING, QUEEN, KNIGHT, PAGE, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ACE,
        ])
    }

    pub fn generate_skat_ranks() -> Vec<Rank> {
        Rank::from_array(&[DAUS, KING, OBER, UNTER, TEN, NINE, EIGHT, SEVEN])
    }

    /// Returns the number of pips on the cards.
    pub fn pip(&self) -> u8 {
        self.get_default_index().parse().unwrap_or(0)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_index(&US_ENGLISH))
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
        assert_eq!("Rang: A", format!("Rang: {}", Rank::new(ACE)));
    }

    #[test]
    fn get_index() {
        let queen = Rank::new(QUEEN);

        assert_eq!("Q".to_string(), queen.get_default_index());
        assert_eq!("D".to_string(), queen.get_index(&GERMAN));
    }

    #[test]
    fn get_long() {
        let ace = Rank::new(ACE);

        assert_eq!("Ace".to_string(), ace.get_long(&US_ENGLISH));
        assert_eq!("Ass".to_string(), ace.get_long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::new(KING).to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rank {
            weight: 9,
            raw: "nine".to_string(),
        };

        assert_eq!(expected, Rank::new(NINE));
    }

    #[test]
    fn pip() {
        assert_eq!(Rank::new(KING).pip(), 0);
        assert_eq!(Rank::new(NINE).pip(), 9);
    }

    #[test]
    fn partial_eq() {
        assert_ne!(Rank::new_with_value(NINE, 3), Rank::new_with_value(NINE, 4));
        assert_ne!(Rank::new_with_value(TEN, 4), Rank::new_with_value(NINE, 4));
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_value(KING, 3));
        expected.push(Rank::new_with_value(QUEEN, 2));

        assert_eq!(expected, Rank::from_array(&[KING, QUEEN]));
    }

    #[test]
    fn generate_french_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new(ACE));
        expected.push(Rank::new(KING));
        expected.push(Rank::new(QUEEN));
        expected.push(Rank::new(JACK));
        expected.push(Rank::new(TEN));
        expected.push(Rank::new(NINE));
        expected.push(Rank::new(EIGHT));
        expected.push(Rank::new(SEVEN));
        expected.push(Rank::new(SIX));
        expected.push(Rank::new(FIVE));
        expected.push(Rank::new(FOUR));
        expected.push(Rank::new(THREE));
        expected.push(Rank::new(TWO));

        assert_eq!(expected, Rank::generate_french_ranks());
    }

    #[test]
    fn generate_pinochle_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_value(ACE, 7));
        expected.push(Rank::new_with_value(TEN, 6));
        expected.push(Rank::new_with_value(KING, 5));
        expected.push(Rank::new_with_value(QUEEN, 4));
        expected.push(Rank::new_with_value(JACK, 3));
        expected.push(Rank::new_with_value(NINE, 2));

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
            KING, QUEEN, KNIGHT, PAGE, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ACE,
        ]);

        assert_eq!(ex, Rank::generate_minor_arcana_ranks());
    }

    #[test]
    fn revise_value() {
        let mut ace = Rank::new(ACE);
        assert_eq!(14, ace.get_weight());

        ace.revise_weight(3);

        assert_eq!(3, ace.get_weight());
    }
}
