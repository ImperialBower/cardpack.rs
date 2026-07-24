//! # Features
//!
//! Uses `i18n` + `colored-display` (for `demo_cards`, fluent names and colored
//! output). cardpack is pure by default (`default = []`), so to use these APIs
//! in your own crate enable them explicitly:
//! `cardpack = { version = "0.9", features = ["i18n", "colored-display"] }`
//!
//! Running this example needs no `--features` flag — the self dev-dependency in
//! Cargo.toml turns them on for the repo's own examples.

use cardpack::prelude::*;

/// [WikiHow: Hand and Foot](https://www.wikihow.com/Play-Hand-and-Foot)
fn main() {
    let headfootdeck = French::decks(5);

    headfootdeck.demo_cards(true);
}
