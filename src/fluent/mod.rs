use fluent_templates::{static_loader, Loader};
use std::fmt;
use unic_langid::{langid, LanguageIdentifier};

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const GERMAN: LanguageIdentifier = langid!("de");

pub const FLUENT_INDEX_SECTION: &str = "index";
pub const FLUENT_LONG_SECTION: &str = "long";
pub const FLUENT_SYMBOL_SECTION: &str = "symbol";
pub const FLUENT_WEIGHT_SECTION: &str = "weight";

static_loader! {
    pub static LOCALES = {
        locales: "./src/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/fluent/locales/core.ftl",
    };
}

/// FluentName represents the fluent template key for a card entity such as a Suit or Rank,
/// which in turn determines its long name in any represented language, the short letter
/// used to display an index, and the default weight for the if it is instantiated via
/// `::new()`. A FluentName must have a corresponding entries in the fluent templates for
/// weight, long, and index.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(String);

impl FluentName {
    pub fn new<S: std::clone::Clone>(name: S) -> FluentName
    where
        S: Into<String>,
    {
        FluentName(name.into())
    }
}

impl fmt::Display for FluentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Named for FluentName {
    fn name(&self) -> &String {
        &self.0
    }
}

pub trait Named {
    fn name(&self) -> &String;

    /// This is the core method for getting fluent values. the index, long, and default weight
    /// methods are all just methods simplifying the call to this method.
    fn fluent_value(&self, key_section: &str, lid: &LanguageIdentifier) -> String {
        let id = format!("{}-{}", self.name(), key_section);
        LOCALES.lookup(lid, id.as_str())
    }

    /// Returns the value of the names' index in the fluent templates.
    ///
    /// The index is defined as the identity indicator in the corner of a playing card.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let jack = cardpack::FluentName::new("jack");
    /// println!("{}", jack.index(&GERMAN));
    /// ```
    /// Prints out `B` (for Bube).
    fn index(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(FLUENT_INDEX_SECTION, lid)
    }

    /// Returns the default, US_ENGLISH value of the names' index value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let ten = cardpack::FluentName::new("ten");
    /// println!("{}", ten.index_default());
    /// ```
    /// Prints out `T`.
    fn index_default(&self) -> String {
        self.index(&US_ENGLISH)
    }

    /// Returns the value of the names' long value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let queen = cardpack::FluentName::new("big-joker");
    /// println!("{}", queen.long(&GERMAN));
    /// ```
    /// Prints out `Großer Joker`.
    fn long(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(FLUENT_LONG_SECTION, lid)
    }

    /// Returns the default, US_ENGLISH value of the names' long value in the fluent templates.
    fn long_default(&self) -> String {
        self.long(&US_ENGLISH)
    }

    /// Returns the default weight for a name. Weight is used to sort cards.
    fn default_weight(&self) -> isize {
        let weight = self.fluent_value(FLUENT_WEIGHT_SECTION, &US_ENGLISH);
        weight.parse().unwrap_or(0)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;

    mod fluent_name_tests {
        use super::*;

        #[test]
        fn new() {
            let n = FluentName::new("boop");

            assert_eq!("boop".to_string(), n.0)
        }

        #[test]
        fn fluent_value() {
            let name = FluentName::new("swords");

            assert_eq!(
                "⚔".to_string(),
                name.fluent_value(FLUENT_SYMBOL_SECTION, &US_ENGLISH)
            )
        }

        #[test]
        fn index() {
            assert_eq!("0".to_string(), FluentName::new("fool").index(&US_ENGLISH))
        }

        #[test]
        fn index_default() {
            assert_eq!("0".to_string(), FluentName::new("fool").index_default())
        }

        #[test]
        fn long() {
            assert_eq!("Ober".to_string(), FluentName::new("ober").long(&GERMAN))
        }

        #[test]
        fn long_default() {
            assert_eq!("Deuce".to_string(), FluentName::new("daus").long_default())
        }

        #[test]
        fn name() {
            assert_eq!(&"foo".to_string(), FluentName::new("foo").name())
        }

        #[test]
        fn default_weight() {
            assert_eq!(11, FluentName::new("unter").default_weight())
        }

        #[test]
        fn default_weight__ne() {
            assert_eq!(0, FluentName::new("no-such-name").default_weight())
        }
    }
}
