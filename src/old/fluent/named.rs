use fluent_templates::{static_loader, Loader};
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
    pub static LOCALES = {
        locales: "./src/old/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/old/fluent/locales/core.ftl",
    };
}

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const GERMAN: LanguageIdentifier = langid!("de");

pub const FLUENT_INDEX_SECTION: &str = "index";
pub const FLUENT_LONG_SECTION: &str = "long";
pub const FLUENT_SYMBOL_SECTION: &str = "symbol";
pub const FLUENT_WEIGHT_SECTION: &str = "weight";
pub const FLUENT_PRIME_SECTION: &str = "prime";

pub trait Named {
    fn name(&self) -> &str;

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
    /// use cardpack::old::fluent::named::*;
    /// use cardpack::old::fluent::fluent_name::*;
    ///
    /// let jack = FluentName::new("jack");
    /// println!("{}", jack.index(&GERMAN));
    /// ```
    /// Prints out `B` (for Bube).
    fn index(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(FLUENT_INDEX_SECTION, lid)
    }

    /// Returns the default, `US_ENGLISH` value of the names' index value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::old::fluent::named::*;
    /// use cardpack::old::fluent::fluent_name::*;
    ///
    /// let ten = FluentName::new("ten");
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
    /// use cardpack::old::fluent::named::*;
    /// use cardpack::old::fluent::fluent_name::*;
    ///
    /// let queen = FluentName::new("big-joker");
    /// println!("{}", queen.long(&GERMAN));
    /// ```
    /// Prints out `Großer Joker`.
    fn long(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(FLUENT_LONG_SECTION, lid)
    }

    /// Returns the default, `US_ENGLISH` value of the names' long value in the fluent templates.
    fn long_default(&self) -> String {
        self.long(&US_ENGLISH)
    }

    /// Returns the default weight for a name. Weight is used to sort cards.
    fn default_weight(&self) -> u32 {
        let weight = self.fluent_value(FLUENT_WEIGHT_SECTION, &US_ENGLISH);
        weight.parse().unwrap_or(0)
    }

    fn default_prime(&self) -> u32 {
        let prime = self.fluent_value(FLUENT_PRIME_SECTION, &US_ENGLISH);
        prime.parse().unwrap_or(0)
    }
}
