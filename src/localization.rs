use crate::common::errors::CardError;
use crate::prelude::Pip;
use fluent_templates::{LanguageIdentifier, Loader, langid, static_loader};
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;

static_loader! {
    pub static LOCALES = {
        locales: "./src/localization/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/localization/locales/core.ftl",
    };
}

/// Trait used to enable localized names for card entities such as suits and ranks.
///
/// # NOTE
///
/// As of `version 0.6.0` of this library, most of the attributes are stored directly in the
/// [`BasicCard`](crate::basic::types::basic_card::BasicCard) struct. The only one that is
/// still actively used is the `name` attribute, which is called with the `Named.long()` method.
///
/// Fluent templates are intentionally kept as the localization mechanism to support i18n beyond
/// English and German. Adding a new locale requires only a new `.ftl` file under
/// `src/localization/locales/`.
///
/// The types of `Named` attributes are
///
/// * `index` - the default letter representation of a card identifier, such as `A` for Ace, or `S` for Spades.
/// * `long` - the long name of a card identifier, such as `Ace` or `Spades`.
/// * `symbol` - the symbol representation of a card identifier, such as `♠` for Spades.
/// * `weight` - the default weight of a card identifier. Used for sorting cards.
/// * `prime` - the prime number representation of a card identifier. Used for generating binary signatures.
///
/// **REREADME:** <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html/>
pub trait Named<'a> {
    const US_ENGLISH: LanguageIdentifier = langid!("en-US");
    const DEUTSCH: LanguageIdentifier = langid!("de");
    /// `fr` — French. Locale draft (high confidence on standard playing-card terminology).
    const FRANCAIS: LanguageIdentifier = langid!("fr");
    /// `la` — Latin. Locale draft (HIGH on french.ftl, MEDIUM-HIGH on tarot.ftl, LOW-MEDIUM on skat.ftl).
    const LATINA: LanguageIdentifier = langid!("la");
    /// `tlh` — Klingon. Locale draft (LOW confidence overall; canon-plus-coinages).
    const TLHINGAN: LanguageIdentifier = langid!("tlh");

    const FLUENT_INDEX_SECTION: &'a str = "index";
    const FLUENT_LONG_SECTION: &'a str = "long";
    const FLUENT_SYMBOL_SECTION: &'a str = "symbol";
    const FLUENT_WEIGHT_SECTION: &'a str = "weight";
    const FLUENT_PRIME_SECTION: &'a str = "prime";

    fn new_with_weight(name_str: &str, weight: u32) -> Self;

    /// Returns a Vector of Ranks with their weights determined by the order they're passed in, high to
    /// low. This facilitates the easy creation of custom decks, such as for pinochle.
    ///
    /// UPDATE: This was doing things in the wrong order.
    #[must_use]
    fn weighted_vector(names: &[&'static str]) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut weight = u32::try_from(names.len()).unwrap_or(0);
        names
            .iter()
            .map(|name| {
                weight -= 1;

                Self::new_with_weight(name, weight)
            })
            .collect()
    }

    fn fluent_name(&self) -> &FluentName;
    fn fluent_name_string(&self) -> &String;
    fn is_blank(&self) -> bool;

    /// This is the core method for getting fluent values. the index, long, and default weight
    /// methods are all just methods simplifying the call to this method.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// assert_eq!(
    ///   "♠",
    ///   FluentName::new("spades").fluent_value("symbol", &FluentName::US_ENGLISH)
    /// );
    /// ```
    fn fluent_value(&self, key_section: &str, lid: &LanguageIdentifier) -> String {
        let id = format!("{}-{}", self.fluent_name_string(), key_section);
        LOCALES.lookup(lid, id.as_str())
    }

    /// Returns the value of the `FluentName` index in the fluent templates. An index
    /// is defined as the default letter representation of a card identifier, such as
    /// `A` for Ace, or `S` for Spades.
    ///
    /// The index is defined as the identity indicator in the corner of a playing card.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let jack = FluentName::new("jack");
    /// assert_eq!("B", jack.index(&FluentName::DEUTSCH));
    /// ```
    fn index(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(Self::FLUENT_INDEX_SECTION, lid)
    }

    /// ```
    /// use cardpack::localization::*;
    ///
    /// let jack = FluentName::new("jack");
    /// assert_eq!('B', jack.index_char(&FluentName::DEUTSCH));
    /// ```
    fn index_char(&self, lid: &LanguageIdentifier) -> char {
        self.index(lid).chars().next().unwrap_or(Pip::BLANK_INDEX)
    }

    /// Returns the default, `US_ENGLISH` index value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let ten = FluentName::new("ten");
    /// assert_eq!("T", ten.index_default());
    /// ```
    fn index_default(&self) -> String {
        self.index(&Self::US_ENGLISH)
    }

    /// Returns the value of the `Named` long value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let big_joker = FluentName::new("big-joker");
    /// assert_eq!("Großer Joker", big_joker.long(&FluentName::DEUTSCH));
    /// ```
    fn long(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(Self::FLUENT_LONG_SECTION, lid)
    }

    /// Returns the default, `US_ENGLISH` value of the `Named` long value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let big_joker = FluentName::new("big-joker");
    /// assert_eq!("Full-Color", big_joker.long_default());
    /// ```
    fn long_default(&self) -> String {
        self.long(&Self::US_ENGLISH)
    }

    /// Returns the weight for `Named`, used to sort cards. There is no need for an alternative
    /// `LanguageIdentifier` to `US_ENGLISH`. Weights are stored in the `core.ftl` file.
    ///
    /// # Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let queen = FluentName::new("queen");
    /// assert_eq!(10, queen.weight());
    /// ```
    fn weight(&self) -> u32 {
        let weight = self.fluent_value(Self::FLUENT_WEIGHT_SECTION, &Self::US_ENGLISH);
        weight.parse().unwrap_or(0)
    }

    /// Returns the prime number for `Named`, used to generate binary signatures. There is no need
    /// for an alternative `LanguageIdentifier` to `US_ENGLISH`. Primes are stored in the `core.ftl`
    /// file.
    ///
    /// **ASIDE:** I'm not sure I like storing these as `FluentName`s.
    ///
    /// # Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// let queen = FluentName::new("queen");
    /// assert_eq!(31, queen.prime());
    /// ```
    fn prime(&self) -> u32 {
        let prime = self.fluent_value(Self::FLUENT_PRIME_SECTION, &Self::US_ENGLISH);
        prime.parse().unwrap_or(0)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(String);

impl FluentName {
    pub const BLANK: &'static str = "blank";

    ///  The difference between `new` and `from_str` is that `new` will default to
    /// `BLANK` if the passed in `&str` isn't  valid, whereas with `from_str` it
    ///  will return a `CardError`.
    ///
    /// One of the things that you want to consider when coding stuff like this
    /// is the user going "WHAT THE FUCK???" when things don't work as expected.
    ///
    /// Having a default value when passing shit in may be convenient at the moment,
    /// but can be a major pain in the ass when you're trying to debug things at the
    /// heat of the moment.
    ///
    /// Empathy of the users of your code is one of the traits that I have encountered
    /// in the wild as a software developer. **Remember, nine times out of ten the
    /// developer you will be cursing over they lack of empathy when their coded
    /// something will be you.**
    ///
    /// **NOTE:** there is no perfect way to do this. Empathy is an art form.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::localization::*;
    ///
    /// assert_eq!("spades", FluentName::new("spades").fluent_name_string());
    ///
    /// // Defaults to `BLANK` when an invalid name is passed in.
    /// assert_eq!(
    ///   FluentName::BLANK,
    ///   FluentName::new("+++").fluent_name_string()
    /// );
    /// ```
    #[must_use]
    pub fn new(name_str: &str) -> Self {
        if Self::is_alphanumeric_hyphen_dash(name_str) {
            Self(name_str.to_string())
        } else {
            log::warn!("Invalid name: {name_str} - Defaulting to 'blank'.");
            Self(Self::BLANK.to_string())
        }
    }

    fn is_alphanumeric_hyphen_dash(s: &str) -> bool {
        s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '–' || c == '—')
    }
}

impl Default for FluentName {
    fn default() -> Self {
        Self(Self::BLANK.to_string())
    }
}

impl Display for FluentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

/// USAGE:
/// ```
/// use std::str::FromStr;
/// use cardpack::localization::*;
///
/// assert_eq!(
///   "hierophant",
///    FluentName::from_str("hierophant").unwrap().fluent_name_string()
/// );
/// ```
///
/// Unlike `Fluent::new()`, `Fluent::from_str()` will return a `CardError` if the
/// passed in value is invalid.
///
/// ```
/// use std::str::FromStr;
/// use cardpack::common::errors::CardError;
/// use cardpack::localization::*;
///
/// let sut = FluentName::from_str("Only alphanumeric and hyphens please.");
///
/// assert_eq!(
///   CardError::InvalidFluentName("Only alphanumeric and hyphens please.".to_string()),
///   sut.unwrap_err()
/// );
/// ```
impl FromStr for FluentName {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_alphanumeric_hyphen_dash(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(CardError::InvalidFluentName(s.to_string()))
        }
    }
}

impl Named<'_> for FluentName {
    /// `FluentName` is a thin wrapper around a name string and has no place to
    /// store a weight, so `weight` is intentionally ignored. Use a separate
    /// weight-bearing type (see the `WeightedName` test helper for the
    /// pattern) when you need both fields.
    fn new_with_weight(name_str: &str, _weight: u32) -> Self {
        Self::new(name_str)
    }

    fn fluent_name(&self) -> &FluentName {
        self
    }

    fn fluent_name_string(&self) -> &String {
        &self.0
    }

    fn is_blank(&self) -> bool {
        self.fluent_name_string() == Self::BLANK
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(FluentName("queen".to_string()), FluentName::new("queen"));
    }

    #[test]
    fn is_alphanumeric_hyphen_dash() {
        assert!(FluentName::is_alphanumeric_hyphen_dash("Hello-World"));
        assert!(FluentName::is_alphanumeric_hyphen_dash("HelloWorld"));
        assert!(!FluentName::is_alphanumeric_hyphen_dash("🁚"));
        assert!(!FluentName::is_alphanumeric_hyphen_dash("  "));
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "hierophant",
            FluentName::from_str("hierophant")
                .unwrap()
                .fluent_name_string()
        );
    }

    #[test]
    fn from_str__error() {
        let sut = FluentName::from_str("I'm a bad bad fluent string name.");

        let my_err = sut.unwrap_err();

        assert_eq!(
            CardError::InvalidFluentName("I'm a bad bad fluent string name.".to_string()),
            my_err
        );
        assert_eq!(
            "Invalid FluentName: `I'm a bad bad fluent string name.`. Must be alphanumeric with hyphens, en-dashes, or em-dashes.",
            my_err.to_string()
        );
    }

    #[test]
    fn named__fluent_value() {
        assert_eq!(
            "Daus",
            FluentName::new("daus").fluent_value("long", &FluentName::DEUTSCH)
        );
        assert_eq!(
            "_",
            FluentName::new("+++").fluent_value("symbol", &FluentName::US_ENGLISH)
        );
    }

    #[test]
    fn named__is_blank() {
        assert!(FluentName::new("blank").is_blank());
        assert!(!FluentName::new("long").is_blank());
    }

    #[test]
    fn named__index() {
        assert_eq!(
            "S",
            FluentName::new("spades").index(&FluentName::US_ENGLISH)
        );
        assert_eq!(
            "P",
            FluentName::new("pentacles").index(&FluentName::US_ENGLISH)
        );
        assert_eq!("K", FluentName::new("clubs").index(&FluentName::DEUTSCH));
    }

    #[test]
    fn named__index_default() {
        assert_eq!("S", FluentName::new("spades").index_default());
        assert_eq!("P", FluentName::new("pentacles").index_default());
    }

    /// Guards the no-panic behavior of `FluentName::new_with_weight`. Previously the
    /// impl was `todo!()` and panicked on call; now it discards the weight and
    /// delegates to `FluentName::new`, since `FluentName` has no weight field.
    #[test]
    fn fluent_name__new_with_weight__does_not_panic() {
        let name = FluentName::new_with_weight("spades", 13);
        assert_eq!(name, FluentName::new("spades"));
    }

    /// Test helper to exercise `Named::weighted_vector` — uses a minimal concrete implementation
    /// of `Named` because `FluentName::new_with_weight` discards the weight (`FluentName`
    /// has nowhere to store it).
    #[allow(dead_code)]
    struct WeightedName {
        name: FluentName,
        weight: u32,
    }

    impl Named<'_> for WeightedName {
        fn new_with_weight(name_str: &str, weight: u32) -> Self {
            WeightedName {
                name: FluentName::new(name_str),
                weight,
            }
        }

        fn fluent_name(&self) -> &FluentName {
            &self.name
        }

        fn fluent_name_string(&self) -> &String {
            self.name.fluent_name_string()
        }

        fn is_blank(&self) -> bool {
            self.name.is_blank()
        }
    }

    #[test]
    fn named__weighted_vector__not_empty() {
        let names = &["ace", "king", "queen", "jack"];
        let result = WeightedName::weighted_vector(names);
        assert!(!result.is_empty());
        assert_eq!(result.len(), names.len());
    }

    #[test]
    fn named__weighted_vector__weights_decrease() {
        // Weights should go from high to low as we iterate through names
        // This catches -= -> += and -= -> /= mutations
        let names = &["ace", "king", "queen", "jack", "ten"];
        let result = WeightedName::weighted_vector(names);
        for i in 0..(result.len() - 1) {
            assert!(
                result[i].weight > result[i + 1].weight,
                "Expected weight[{i}] ({}) > weight[{}] ({})",
                result[i].weight,
                i + 1,
                result[i + 1].weight
            );
        }
    }

    #[test]
    fn fluent_name__fmt__not_empty() {
        let name = FluentName::new("spades");
        let s = format!("{name}");
        assert!(!s.is_empty());
        assert_eq!(s, "spades");
    }

    #[test]
    fn fluent_name__accessor__returns_self() {
        let name = FluentName::new("hearts");
        // fluent_name() on a FluentName should return itself
        assert_eq!(name.fluent_name(), &name);
    }

    #[test]
    fn is_alphanumeric_hyphen_dash__en_dash() {
        // en-dash (–) should be valid; catches || -> && mutation at col 66
        assert!(FluentName::is_alphanumeric_hyphen_dash(
            "hello\u{2013}world"
        ));
    }

    #[test]
    fn is_alphanumeric_hyphen_dash__em_dash() {
        // em-dash (—) should be valid; catches || -> && mutation at col 66
        assert!(FluentName::is_alphanumeric_hyphen_dash(
            "hello\u{2014}world"
        ));
    }

    #[test]
    fn is_alphanumeric_hyphen_dash__invalid() {
        assert!(!FluentName::is_alphanumeric_hyphen_dash("not valid!"));
        assert!(!FluentName::is_alphanumeric_hyphen_dash(" "));
    }

    /// Confirms the `fr` locale (added 2026-04-29) is wired into the static loader.
    /// A regression in `src/localization/locales/fr/` discovery would fail this test.
    /// The `french.ftl` Queen entry is high-confidence terminology and unlikely to
    /// shift if the file is later reviewed.
    #[test]
    fn french_locale_is_wired() {
        let fr = langid!("fr");
        assert_eq!("Dame", LOCALES.lookup(&fr, "name-rank-french-q"));
        assert_eq!("As", LOCALES.lookup(&fr, "name-rank-french-a"));
    }

    /// Confirms the `la` (Latin) locale is wired into the static loader.
    /// Both assertions hit high-confidence classical Latin entries — Regina
    /// (queen) and Rex (king) — that are unlikely to shift on review.
    #[test]
    fn latin_locale_is_wired() {
        let la = langid!("la");
        assert_eq!("Regina", LOCALES.lookup(&la, "name-rank-french-q"));
        assert_eq!("Rex", LOCALES.lookup(&la, "name-rank-french-k"));
    }

    /// Confirms the `tlh` (Klingon) locale is wired into the static loader.
    /// Both assertions hit attested Okrand-canon entries (TKD): tIq "heart"
    /// and ta' "emperor". Coinages are deliberately not asserted here — they
    /// may be revised on KLI review without breaking this wired-test.
    #[test]
    fn klingon_locale_is_wired() {
        let tlh = langid!("tlh");
        assert_eq!("tIq", LOCALES.lookup(&tlh, "name-suit-french-h"));
        assert_eq!("ta'", LOCALES.lookup(&tlh, "name-rank-french-k"));
    }

    /// Guards the de/tarot.ftl schema fix (2026-04-29). Major Arcana keys were
    /// renamed to use the `name-rank-tarot-special-` prefix and Minor Arcana
    /// entries were added; previously both silently fell back to English.
    #[test]
    fn german_tarot_resolves_correctly() {
        let de = &FluentName::DEUTSCH;
        // Major Arcana — uses the corrected `-special-` prefix
        assert_eq!("Der Narr", LOCALES.lookup(de, "name-rank-tarot-special-0"));
        assert_eq!("Die Welt", LOCALES.lookup(de, "name-rank-tarot-special-l"));
        // Minor Arcana — newly added entries
        assert_eq!("Ass", LOCALES.lookup(de, "name-rank-tarot-a"));
        assert_eq!("Königin", LOCALES.lookup(de, "name-rank-tarot-q"));
    }
}
