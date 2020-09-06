mod anzug;
mod anzug_buchstabe;
mod anzug_name;
mod anzug_symbol;
mod rang;
mod rang_kurz;
mod rang_name;

use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, US_ENGLISH};
use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;

#[derive(Clone, Debug, PartialEq)]
pub struct Karte {
    pub rang: Rang,
    pub anzug: Anzug,
}

impl Karte {
    pub fn new<S: std::clone::Clone>(rang: S, anzug: S) -> Karte
    where
        S: Into<String>,
    {
        Karte {
            rang: Rang::new(rang),
            anzug: Anzug::new(anzug),
        }
    }
}

impl ToLocaleString for Karte {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rang.to_locale_string(&lid);
        let anzug = self.anzug.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

impl fmt::Display for Karte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn new() {
        let expected = Karte {
            rang: Rang::new("ace"),
            anzug: Anzug::new("spades"),
        };

        assert_eq!(expected, Karte::new("ace", "spades"));
    }

    #[test]
    fn to_string_by_locale() {
        let karte = Karte::new("queen", "clubs");

        assert_eq!(karte.to_locale_string(&GERMAN), "D♣".to_string());
    }
}
