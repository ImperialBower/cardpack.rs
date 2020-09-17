use fluent_templates::{static_loader, Loader};
use unic_langid::{langid, LanguageIdentifier};

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const GERMAN: LanguageIdentifier = langid!("de");

static_loader! {
    pub static LOCALES = {
        locales: "./src/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/fluent/locales/core.ftl",
    };
}

pub trait Weighty {
    fn revise_weight(&mut self, new_value: isize);

    fn get_weight(&self) -> isize;
}

pub fn get_value_by_key(key: &str, lid: &LanguageIdentifier) -> String {
    LOCALES.lookup(lid, key)
}

fn get_weight(name: &str) -> String {
    let var = "-weight";
    let id = format!("{}{}", name, var);
    LOCALES.lookup(&US_ENGLISH, id.as_str())
}

pub fn get_weight_isize(name: &str) -> isize {
    let s = get_weight(name);
    s.parse().unwrap_or(0)
}

#[cfg(test)]
mod fluent_tests {
    use super::*;

    #[test]
    fn doit() {
        let s = LOCALES.lookup(&US_ENGLISH, "spades-letter");

        assert_eq!("S", s);
    }
}
