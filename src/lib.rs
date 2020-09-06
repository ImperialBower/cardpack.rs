pub use karten::*;

mod fluent;
mod karten;

use fluent_templates::{Loader};
use crate::fluent::{LOCALES, US_ENGLISH};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", LOCALES.lookup(&US_ENGLISH, "spades-symbol"));
        println!("{}", LOCALES.lookup(&US_ENGLISH, "spades-light-symbol"));
        assert_eq!(2 + 2, 4);
    }
}
