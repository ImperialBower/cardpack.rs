/*  CardPack - A generic pack of cards library written in Rust.
Copyright (C) <2020>  Christoph Baker

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */
use fluent_templates::{static_loader, Loader};
use unic_langid::{langid, LanguageIdentifier};

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const GERMAN: LanguageIdentifier = langid!("de");
pub const FLUENT_INDEX_SECTION: &str = "index";
pub const FLUENT_LONG_SECTION: &str = "long";
pub const FLUENT_WEIGHT_SECTION: &str = "weight";

static_loader! {
    pub static LOCALES = {
        locales: "./src/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/fluent/locales/core.ftl",
    };
}

pub trait FluentCard {
    /// Returns the default, US_ENGLISH value of the implementer's index as set in the fluent
    /// templates.
    fn get_default_index(&self) -> String {
        self.get_index(&US_ENGLISH)
    }

    /// "The number or letter printed in the corner of a playing card,
    /// so that it may be read when held in a fan." -- Wikipedia
    fn get_index(&self, lid: &LanguageIdentifier) -> String {
        get_fluent_value(self.get_name(), FLUENT_INDEX_SECTION, lid)
    }

    /// Returns the default, US_ENGLISH long name for the Rank, as set in the fluent templates.
    fn get_default_long(&self) -> String {
        self.get_long(&US_ENGLISH)
    }

    /// Returns the long name value for the passed in LanguageIdentifier, as set in the fluent
    /// templates for that language.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::{GERMAN, FluentCard};
    /// let queen = cardpack::Rank::new_with_weight(cardpack::QUEEN, 12);
    /// println!("{}", queen.get_long(&GERMAN));
    /// ```
    /// Prints out `Dame`.
    fn get_long(&self, lid: &LanguageIdentifier) -> String {
        get_fluent_value(self.get_name(), FLUENT_LONG_SECTION, lid)
    }

    fn get_name(&self) -> &String;

    fn revise_weight(&mut self, new_value: isize);

    fn get_weight(&self) -> isize;
}

pub fn get_value_by_key(key: &str, lid: &LanguageIdentifier) -> String {
    LOCALES.lookup(lid, key)
}

fn get_weight(name: &str) -> String {
    get_fluent_value(name, FLUENT_WEIGHT_SECTION, &US_ENGLISH)
}

pub fn get_fluent_value(key_name: &str, key_section: &str, lid: &LanguageIdentifier) -> String {
    let id = format!("{}-{}", key_name, key_section);
    LOCALES.lookup(lid, id.as_str())
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
        let s = LOCALES.lookup(&US_ENGLISH, "spades-index");

        assert_eq!("S", s);
    }
}
