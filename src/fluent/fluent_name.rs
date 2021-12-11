use std::fmt;

use crate::Named;

const BLANK: &str = "blank";

/// FluentName is the primary implementation of the Named trait.
///
/// FluentName represents the fluent template key for a card entity such as a Suit or Rank,
/// which in turn determines its long name in any represented language, the short letter
/// used to display an index, and the default weight for the if it is instantiated via
/// `::new()`. A FluentName must have a corresponding entries in the fluent templates for
/// weight, long, and index.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(&'static str);

impl FluentName {
    pub fn new(name: &'static str) -> FluentName {
        if name.trim().is_empty() {
            FluentName(BLANK)
        } else {
            FluentName(name)
        }
    }
}

impl fmt::Display for FluentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Named for FluentName {
    fn name(&self) -> &str {
        self.0
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;

    mod fluent_name_tests {
        use super::*;
        use crate::{FLUENT_SYMBOL_SECTION, GERMAN, US_ENGLISH};

        #[test]
        fn new() {
            let n = FluentName::new("boop");

            assert_eq!("boop".to_string(), n.0)
        }

        #[test]
        fn new__empty_string() {
            let n = FluentName::new("");

            assert_eq!("_".to_string(), n.index_default())
        }

        #[test]
        fn new__blank_string() {
            let n = FluentName::new(" ");

            assert_eq!("_".to_string(), n.index_default())
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
