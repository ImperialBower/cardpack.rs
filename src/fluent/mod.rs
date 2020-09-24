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

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(String);

impl FluentName {
    pub fn new<S: std::clone::Clone>(name: S) -> FluentName
    where
        S: Into<String>,
    {
        FluentName(name.into())
    }

    pub fn fluent_value(&self, key_section: &str, lid: &LanguageIdentifier) -> String {
        let id = format!("{}-{}", self.name(), key_section);
        LOCALES.lookup(lid, id.as_str())
    }

    pub fn name(&self) -> &String {
        &self.0
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
    pub fn index(&self, lid: &LanguageIdentifier) -> String {
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
    pub fn index_default(&self) -> String {
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
    pub fn long(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(FLUENT_LONG_SECTION, lid)
    }

    /// Returns the default, US_ENGLISH value of the names' long value in the fluent templates.
    pub fn long_default(&self) -> String {
        self.long(&US_ENGLISH)
    }

    pub fn default_weight(&self) -> isize {
        let weight = self.fluent_value(FLUENT_WEIGHT_SECTION, &US_ENGLISH);
        weight.parse().unwrap_or(0)
    }
}

impl fmt::Display for FluentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// region FluentCard

// pub trait FluentCard {
//     /// Returns the default, US_ENGLISH value of the implementer's index as set in the fluent
//     /// templates.
//     fn get_default_index(&self) -> String {
//         self.index(&US_ENGLISH)
//     }
//
//     /// "The number or letter printed in the corner of a playing card,
//     /// so that it may be read when held in a fan." -- Wikipedia
//     fn get_index(&self, lid: &LanguageIdentifier) -> String {
//         get_fluent_value(self.get_name(), FLUENT_INDEX_SECTION, lid)
//     }
//
//     /// Returns the default, US_ENGLISH long name for the Rank, as set in the fluent templates.
//     fn get_default_long(&self) -> String {
//         self.get_long(&US_ENGLISH)
//     }
//
//     /// Returns the long name value for the passed in LanguageIdentifier, as set in the fluent
//     /// templates for that language.
//     ///
//     /// ## Usage
//     /// ```
//     /// use cardpack::{GERMAN, FluentCard};
//     /// let queen = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
//     /// println!("{}", queen.get_long(&GERMAN));
//     /// ```
//     /// Prints out `Dame`.
//     fn get_long(&self, lid: &LanguageIdentifier) -> String {
//         get_fluent_value(self.get_name(), FLUENT_LONG_SECTION, lid)
//     }
//
//     fn get_name(&self) -> &String;
//
//     fn revise_weight(&mut self, new_value: isize);
//
//     fn get_weight(&self) -> isize;
// }

fn get_weight(name: &str) -> String {
    get_fluent_value(name, FLUENT_WEIGHT_SECTION, &US_ENGLISH)
}

fn get_fluent_value(key_name: &str, key_section: &str, lid: &LanguageIdentifier) -> String {
    let id = format!("{}-{}", key_name, key_section);
    LOCALES.lookup(lid, id.as_str())
}

pub fn get_weight_isize(name: &str) -> isize {
    let s = get_weight(name);
    s.parse().unwrap_or(0)
}

// endregion

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
