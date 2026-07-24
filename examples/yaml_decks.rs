//! Dumps one golden YAML fixture per shipped deck into `tests/fixtures/yaml/`.
//! Public API only — this program is a consumer of the crate.
//!
//! # Features
//!
//! Uses `std` + `yaml`. cardpack is pure by default (`default = []`), so to use
//! these APIs in your own crate enable them explicitly:
//! `cardpack = { version = "0.9", features = ["std", "yaml"] }`
//! (`yaml` implies `serde`). Note this dumper writes files, but it does so with
//! its own `std::fs` — reading decks from a YAML *file* via cardpack would need
//! the separate `std-io` feature.
//!
//! Run it from the repo root with `cargo ex yaml_decks` — the alias in
//! `.cargo/config.toml` supplies the features, so no `--features` flag is
//! needed. Or `make yaml-fixtures`.
//!
//! The fixtures it writes are compared **byte for byte** by
//! `tests/yaml_golden.rs`. Regenerate whenever a deck's card data legitimately
//! changes, then review the diff — an unexpected diff means deck data drifted.

// This example is a *consumer* of the crate (a golden-fixture dumper), not part
// of the pure kernel, so it deliberately performs filesystem I/O. The
// kernel-purity lints (clippy.toml) exist to keep the *library* pure; allow
// them for this binary only. See docs/audit-2026-07-18-domain-kernel.md.
#![allow(clippy::disallowed_types, clippy::disallowed_methods)]

use cardpack::prelude::*;
use std::fs;
use std::path::Path;

/// `"Standard 52"` -> `"standard_52"`, `"Dashavatara Ganjifa"` ->
/// `"dashavatara_ganjifa"`.
///
/// Kept in sync with the identical helper in `tests/yaml_golden.rs`; the
/// fixture-count test there fails if the two ever disagree about how many
/// files should exist.
fn slug(deck_name: &str) -> String {
    deck_name.to_lowercase().replace(' ', "_")
}

fn main() {
    let root = Path::new("tests/fixtures/yaml");
    fs::create_dir_all(root).expect("mkdir tests/fixtures/yaml");

    for kind in DeckKind::all() {
        let yaml = kind.to_yaml().expect("serialize deck");
        let path = root.join(format!("{}.yaml", slug(&kind.deck_name())));
        fs::write(&path, &yaml).expect("write fixture");
        println!(
            "wrote {} ({} cards, {} bytes)",
            path.display(),
            kind.base_vec().len(),
            yaml.len()
        );
    }

    println!("\n{} fixtures written", DeckKind::all().len());
}
