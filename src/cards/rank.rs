use std::fmt;

use crate::fluent_name::FluentName;
use crate::Named;
use crate::US_ENGLISH;

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

pub const BLANK_RANK: &str = "_";

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
///     prime: 19,
///     name: cardpack::fluent_name::FluentName::new(cardpack::ACE),
/// };
/// ```
/// This gives you maximum flexibility. Since the value of the Ace is 1, it will be sorted
/// at the end of a Suit (unless there are any Cards with negative weights).
///
/// # ``Rank::new()`` with a value string
/// ```
/// let king = cardpack::Rank::new(cardpack::KING);
/// ```
/// This sets the weight for the Rank based upon the default value as set in its fluent template
/// entry.
///
/// # ``Rank::new_with_weight()``
/// ```
/// let king = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
/// ```
/// Overrides the default weight for a Rank.
///
/// # ``Rank::from_array()``
/// ```
/// let ranks: Vec<cardpack::Rank> = cardpack::Rank::from_array(&[cardpack::ACE, cardpack::TEN,]);
/// ```
/// Returns a Vector of Ranks with their weights determined by the order they're passed in, high to
/// low. This facilitates the easy creation of custom decks, such as pinochle.
///
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank {
    /// Used by the Pile struct to sort Cards by their Suit and Rank.
    pub weight: u32,
    pub prime: u32,
    pub name: FluentName,
}

impl Rank {
    /// Returns a `Rank`, determining its weight by the default weight value for its name set in the
    /// fluent templates. For instance, if you look in `src/fluent/locales/core.ftl` you will see
    /// that the default weight for an Ace is 14. This will mean that when a pile of cards is sorted
    /// that it will be at the top of a standard French Deck where the Ace is high.
    ///
    /// ## Usage
    /// ```
    /// let king = cardpack::Rank::new(cardpack::HERMIT);
    /// ```
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        let name = FluentName::new(name);
        Self {
            weight: name.default_weight(),
            prime: name.default_prime(),
            name,
        }
    }

    /// Returns a Rank instance with the passed in name and weight, overriding the default value
    /// set in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// let king = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
    /// ```
    #[must_use]
    pub fn new_with_weight(name: &'static str, weight: u32) -> Self {
        let name = FluentName::new(name);
        Self {
            weight,
            prime: name.default_prime(),
            name,
        }
    }

    #[must_use]
    pub fn new_with_weight_and_prime(name: &'static str, weight: u32, prime: u32) -> Self {
        Self {
            weight,
            prime,
            name: FluentName::new(name),
        }
    }

    /// ## Usage
    /// ```
    /// let ranks: Vec<cardpack::Rank> = cardpack::Rank::from_array(&[
    ///     cardpack::ACE, cardpack::TEN, cardpack::KING,
    ///     cardpack::QUEEN, cardpack::JACK, cardpack::NINE]);
    /// ```
    /// Returns a Vector of Ranks with their weights determined by the order they're passed in, high to
    /// low. This facilitates the easy creation of custom decks, such as for pinochle.
    #[must_use]
    pub fn from_array(s: &[&'static str]) -> Vec<Rank> {
        let mut v: Vec<Rank> = Vec::new();

        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::into_iter_on_ref
        )]
        for (i, &elem) in s.into_iter().enumerate() {
            let weight = (s.len() - 1) - i;
            v.push(Rank::new_with_weight(elem, weight as u32));
        }
        v
    }

    /// Returns a Rank entity based on its index string.
    #[must_use]
    pub fn from_french_deck_index(index: &'static str) -> Self {
        match index {
            "JB" => Rank::new(BIG_JOKER),
            "JL" => Rank::new(LITTLE_JOKER),
            "A" | "a" => Rank::new(ACE),
            "K" | "k" => Rank::new(KING),
            "Q" | "q" => Rank::new(QUEEN),
            "J" | "j" => Rank::new(JACK),
            "T" | "t" | "0" | "10" => Rank::new(TEN),
            "9" => Rank::new(NINE),
            "8" => Rank::new(EIGHT),
            "7" => Rank::new(SEVEN),
            "6" => Rank::new(SIX),
            "5" => Rank::new(FIVE),
            "4" => Rank::new(FOUR),
            "3" => Rank::new(THREE),
            "2" => Rank::new(TWO),
            _ => Rank::new(BLANK_RANK),
        }
    }

    /// Returns a Rank entity based on its index string.
    #[must_use]
    pub fn from_french_deck_char(index: char) -> Rank {
        match index {
            'A' | 'a' => Rank::new(ACE),
            'K' | 'k' => Rank::new(KING),
            'Q' | 'q' => Rank::new(QUEEN),
            'J' | 'j' => Rank::new(JACK),
            'T' | 't' | '0' => Rank::new(TEN),
            '9' => Rank::new(NINE),
            '8' => Rank::new(EIGHT),
            '7' => Rank::new(SEVEN),
            '6' => Rank::new(SIX),
            '5' => Rank::new(FIVE),
            '4' => Rank::new(FOUR),
            '3' => Rank::new(THREE),
            '2' => Rank::new(TWO),
            _ => Rank::new(BLANK_RANK),
        }
    }

    #[must_use]
    pub fn generate_canasta_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            TWO, ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE,
        ])
    }

    #[must_use]
    pub fn generate_euchre_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, KING, QUEEN, JACK, TEN, NINE])
    }

    #[must_use]
    pub fn generate_french_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO,
        ])
    }

    #[must_use]
    pub fn generate_pinochle_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, TEN, KING, QUEEN, JACK, NINE])
    }

    #[must_use]
    pub fn generate_major_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            FOOL, MAGICIAN, PRIESTESS, EMPRESS, EMPEROR, HIEROPHANT, LOVERS, CHARIOT, STRENGTH,
            HERMIT, FORTUNE, JUSTICE, HANGED, DEATH, TEMPERANCE, DEVIL, TOWER, STAR, MOON, SUN,
            JUDGEMENT, WORLD,
        ])
    }

    #[must_use]
    pub fn generate_minor_arcana_ranks() -> Vec<Rank> {
        Rank::from_array(&[
            KING, QUEEN, KNIGHT, PAGE, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO, ACE,
        ])
    }

    #[must_use]
    pub fn generate_short_deck_ranks() -> Vec<Rank> {
        Rank::from_array(&[ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX])
    }

    #[must_use]
    pub fn generate_skat_ranks() -> Vec<Rank> {
        Rank::from_array(&[DAUS, KING, OBER, UNTER, TEN, NINE, EIGHT, SEVEN])
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.name.name() == BLANK_RANK
    }
}

/// Defaults to a blank `Rank`.
impl Default for Rank {
    fn default() -> Rank {
        Rank::new(BLANK_RANK)
    }
}

/// Allows for the Rank to be displayed as a binary value based upon it's prime field.
/// This will be used for Cactus Kev style hand evaluation.
/// ```
/// let king = cardpack::Rank::new(cardpack::KING);
/// assert_eq!(format!("King as binary is: {:06b}", king), "King as binary is: 100101");
/// ```
impl fmt::Binary for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.prime;

        fmt::Binary::fmt(&val, f)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.index(&US_ENGLISH))
    }
}

impl Named for Rank {
    fn name(&self) -> &str {
        self.name.name()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use crate::GERMAN;
    use rstest::rstest;

    #[test]
    fn display() {
        assert_eq!("Rang: A", format!("Rang: {}", Rank::new(ACE)));
    }

    #[test]
    fn get_index() {
        let queen = Rank::new(QUEEN);

        assert_eq!("Q".to_string(), queen.name.index_default());
        assert_eq!("D".to_string(), queen.name.index(&GERMAN));
    }

    #[test]
    fn get_long() {
        let ace = Rank::new(ACE);

        assert_eq!("Ace".to_string(), ace.name.long(&US_ENGLISH));
        assert_eq!("Ass".to_string(), ace.name.long(&GERMAN));
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::new(KING).to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let expected = Rank {
            weight: 7,
            prime: 19,
            name: FluentName::new(NINE),
        };

        assert_eq!(expected, Rank::new(NINE));
    }

    #[test]
    fn new__tarot() {
        let hermit = Rank::new(HERMIT);

        assert_eq!(Rank::new(HERMIT), hermit)
    }

    #[test]
    fn partial_eq() {
        assert_ne!(
            Rank::new_with_weight(NINE, 3),
            Rank::new_with_weight(NINE, 4)
        );
        assert_ne!(
            Rank::new_with_weight(TEN, 4),
            Rank::new_with_weight(NINE, 4)
        );
    }

    #[test]
    fn to_vec() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(KING, 1));
        expected.push(Rank::new_with_weight(QUEEN, 0));

        assert_eq!(expected, Rank::from_array(&[KING, QUEEN]));
    }

    #[rstest]
    #[case('A', Rank::new(ACE))]
    #[case('a', Rank::new(ACE))]
    #[case('K', Rank::new(KING))]
    #[case('k', Rank::new(KING))]
    #[case('Q', Rank::new(QUEEN))]
    #[case('q', Rank::new(QUEEN))]
    #[case('J', Rank::new(JACK))]
    #[case('j', Rank::new(JACK))]
    #[case('T', Rank::new(TEN))]
    #[case('t', Rank::new(TEN))]
    #[case('0', Rank::new(TEN))]
    #[case('9', Rank::new(NINE))]
    #[case('8', Rank::new(EIGHT))]
    #[case('7', Rank::new(SEVEN))]
    #[case('6', Rank::new(SIX))]
    #[case('5', Rank::new(FIVE))]
    #[case('4', Rank::new(FOUR))]
    #[case('3', Rank::new(THREE))]
    #[case('2', Rank::new(TWO))]
    #[case('_', Rank::new(BLANK_RANK))]
    #[case(' ', Rank::new(BLANK_RANK))]
    fn from_french_deck_char(#[case] input: char, #[case] expected: Rank) {
        assert_eq!(expected, Rank::from_french_deck_char(input));
    }

    #[rstest]
    #[case("JB", Rank::new(BIG_JOKER))]
    #[case("JL", Rank::new(LITTLE_JOKER))]
    #[case("A", Rank::new(ACE))]
    #[case("a", Rank::new(ACE))]
    #[case("K", Rank::new(KING))]
    #[case("k", Rank::new(KING))]
    #[case("Q", Rank::new(QUEEN))]
    #[case("q", Rank::new(QUEEN))]
    #[case("J", Rank::new(JACK))]
    #[case("j", Rank::new(JACK))]
    #[case("T", Rank::new(TEN))]
    #[case("t", Rank::new(TEN))]
    #[case("10", Rank::new(TEN))]
    #[case("0", Rank::new(TEN))]
    #[case("9", Rank::new(NINE))]
    #[case("8", Rank::new(EIGHT))]
    #[case("7", Rank::new(SEVEN))]
    #[case("6", Rank::new(SIX))]
    #[case("5", Rank::new(FIVE))]
    #[case("4", Rank::new(FOUR))]
    #[case("3", Rank::new(THREE))]
    #[case("2", Rank::new(TWO))]
    #[case("_", Rank::new(BLANK_RANK))]
    #[case("", Rank::new(BLANK_RANK))]
    #[case(" ", Rank::new(BLANK_RANK))]
    fn from_french_deck_index(#[case] input: &'static str, #[case] expected: Rank) {
        assert_eq!(expected, Rank::from_french_deck_index(input));
    }

    #[test]
    fn generate_canasta_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(TWO, 12));
        expected.push(Rank::new_with_weight(ACE, 11));
        expected.push(Rank::new_with_weight(KING, 10));
        expected.push(Rank::new_with_weight(QUEEN, 9));
        expected.push(Rank::new_with_weight(JACK, 8));
        expected.push(Rank::new_with_weight(TEN, 7));
        expected.push(Rank::new_with_weight(NINE, 6));
        expected.push(Rank::new_with_weight(EIGHT, 5));
        expected.push(Rank::new_with_weight(SEVEN, 4));
        expected.push(Rank::new_with_weight(SIX, 3));
        expected.push(Rank::new_with_weight(FIVE, 2));
        expected.push(Rank::new_with_weight(FOUR, 1));
        expected.push(Rank::new_with_weight(THREE, 0));

        assert_eq!(expected, Rank::generate_canasta_ranks());
    }

    #[test]
    fn generate_euchre_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(ACE, 5));
        expected.push(Rank::new_with_weight(KING, 4));
        expected.push(Rank::new_with_weight(QUEEN, 3));
        expected.push(Rank::new_with_weight(JACK, 2));
        expected.push(Rank::new_with_weight(TEN, 1));
        expected.push(Rank::new_with_weight(NINE, 0));

        assert_eq!(expected, Rank::generate_euchre_ranks());
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
        expected.push(Rank::new_with_weight(ACE, 5));
        expected.push(Rank::new_with_weight(TEN, 4));
        expected.push(Rank::new_with_weight(KING, 3));
        expected.push(Rank::new_with_weight(QUEEN, 2));
        expected.push(Rank::new_with_weight(JACK, 1));
        expected.push(Rank::new_with_weight(NINE, 0));

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
    fn generate_short_deck_ranks() {
        let mut expected: Vec<Rank> = Vec::new();
        expected.push(Rank::new_with_weight(ACE, 8));
        expected.push(Rank::new_with_weight(KING, 7));
        expected.push(Rank::new_with_weight(QUEEN, 6));
        expected.push(Rank::new_with_weight(JACK, 5));
        expected.push(Rank::new_with_weight(TEN, 4));
        expected.push(Rank::new_with_weight(NINE, 3));
        expected.push(Rank::new_with_weight(EIGHT, 2));
        expected.push(Rank::new_with_weight(SEVEN, 1));
        expected.push(Rank::new_with_weight(SIX, 0));

        assert_eq!(expected, Rank::generate_short_deck_ranks());
    }

    #[test]
    fn revise_value() {
        let mut ace = Rank::new(ACE);
        assert_eq!(12, ace.weight);

        ace.weight = 3;

        assert_eq!(3, ace.weight);
    }

    #[test]
    fn fmt_binary() {
        let king = Rank::new(KING);
        let jack = Rank::new(JACK);
        let five = Rank::new(FIVE);

        assert_eq!(
            format!("King as binary is: {:08b}", king),
            "King as binary is: 00100101"
        );
        assert_eq!(
            format!("Jack as binary is: {:08b}", jack),
            "Jack as binary is: 00011101"
        );
        assert_eq!(
            format!("Five as binary is: {:08b}", five),
            "Five as binary is: 00000111"
        );
    }

    #[test]
    fn default() {
        assert_eq!(Rank::default(), Rank::new(BLANK_RANK));
    }
}
