pub use karten::*;

mod fluent;
mod karten;

use crate::fluent::{LOCALES, US_ENGLISH};
use fluent_templates::Loader;

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("♠", LOCALES.lookup(&US_ENGLISH, "spades-symbol"));
        assert_eq!("♤", LOCALES.lookup(&US_ENGLISH, "spades-light-symbol"));
    }
}
