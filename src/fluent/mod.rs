use fluent_templates::static_loader;
use unic_langid::{LanguageIdentifier, langid};

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

pub trait ToLocaleString {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String;
}
