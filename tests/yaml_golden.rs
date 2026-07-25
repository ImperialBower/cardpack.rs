//! Golden-fixture tests: the serialized form of every shipped deck must match
//! its committed fixture **byte for byte**.
//!
//! This is deliberately stricter than a round-trip check. A round-trip passes
//! happily even if a deck's card data silently changed — it only proves the
//! code is self-consistent. A byte comparison against a reviewed fixture also
//! catches deck-data drift, and catches `serde_norway` reformatting across
//! dependency bumps.
//!
//! When a diff appears: regenerate with `make yaml-fixtures`, review the diff,
//! and only accept it if the change is explainable. Do **not** loosen these to
//! a parse-and-compare — the byte check is the entire point.
//!
//! Needs `std::fs` to read fixtures, so it is skipped on wasm.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]
// This is a test harness reading golden fixtures from disk, not kernel code.
// The kernel-purity lints (clippy.toml `disallowed_types`/`disallowed_methods`,
// whose stated reasons are all "no filesystem I/O in a pure *kernel*") exist to
// keep `src/` pure; allow them for this test binary only, exactly as
// `examples/deconstruct_vectors.rs` does. See
// docs/audit-2026-07-18-domain-kernel.md.
#![allow(clippy::disallowed_types, clippy::disallowed_methods)]

// `Tiny` is the one deck not re-exported from the prelude; the full path works
// regardless of whether that is fixed.
use cardpack::basic::decks::tiny::Tiny;
use cardpack::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/yaml")
}

/// `"Standard 52"` -> `"standard_52"`.
///
/// Duplicated from `examples/yaml_decks.rs` on purpose — examples and
/// integration tests are separate crates that share only the public API, so
/// there is nowhere to put a shared helper. `fixture_count__matches_registry`
/// is what catches the two drifting apart.
fn slug(deck_name: &str) -> String {
    deck_name.to_lowercase().replace(' ', "_")
}

fn fixture_path(kind: DeckKind) -> PathBuf {
    fixture_dir().join(format!("{}.yaml", slug(&kind.deck_name())))
}

fn read_fixture(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "missing fixture {}: {e}\nrun `make yaml-fixtures`",
            path.display()
        )
    })
}

/// Adding deck 15 without a fixture must fail here rather than silently
/// leaving it unguarded. Reads the registry, never a hardcoded 14.
#[test]
fn fixture_count__matches_registry() {
    let count = fs::read_dir(fixture_dir())
        .expect("tests/fixtures/yaml must exist — run `make yaml-fixtures`")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
        .count();

    assert_eq!(
        count,
        DeckKind::all().len(),
        "fixture count != DeckKind count; run `make yaml-fixtures`"
    );
}

/// The core golden assertion. Byte-for-byte, not parsed-and-compared.
#[test]
fn every_deck_kind__matches_golden_bytes() {
    for kind in DeckKind::all() {
        let golden = read_fixture(&fixture_path(*kind));
        let generated = kind.to_yaml().expect("serialize");

        assert_eq!(
            generated,
            golden,
            "{} drifted from its fixture; run `make yaml-fixtures` and review the diff",
            kind.deck_name()
        );
    }
}

#[test]
fn every_golden__deserializes_to_its_deck() {
    for kind in DeckKind::all() {
        let golden = read_fixture(&fixture_path(*kind));

        assert_eq!(DeckKind::from_yaml(&golden).expect("deserialize"), *kind);
        assert_eq!(
            BasicCard::cards_from_yaml_str(&golden).expect("cards"),
            kind.base_vec()
        );
    }
}

/// Fixtures must be not merely parseable but *correct* — the distinction
/// `razz_bad.yml` exists to teach.
#[test]
fn every_golden__passes_validate_yaml() {
    macro_rules! assert_validates {
        ($($deck:ty),+ $(,)?) => {
            $(
                let path = fixture_dir()
                    .join(format!("{}.yaml", slug(&<$deck>::deck_name())));
                let golden = read_fixture(&path);

                <$deck>::validate_yaml(&golden).unwrap_or_else(|e| {
                    panic!("{} fixture failed validate_yaml: {e}", <$deck>::deck_name())
                });
            )+
        };
    }

    assert_validates!(
        Canasta,
        Dashavatara,
        Euchre24,
        Euchre32,
        French,
        Mughal,
        Pinochle,
        Razz,
        Short,
        Skat,
        Spades,
        Standard52,
        Tarot,
        Tiny,
    );
}

/// Every fixture carries the current format version, so a future bump is a
/// visible, reviewable change rather than a silent one.
#[test]
fn every_golden__declares_current_version() {
    for kind in DeckKind::all() {
        let golden = read_fixture(&fixture_path(*kind));
        let dy = DeckYaml::from_yaml(&golden).expect("parse");

        assert_eq!(dy.version, DeckYaml::VERSION, "{}", kind.deck_name());
    }
}

/// Guards the coupling between this file's `slug` and the generator's: if
/// they disagree, `fixture_path` silently points at a file that does not
/// exist, and every test above fails with a confusing "missing fixture".
/// This names the real problem instead.
#[test]
fn every_deck_kind__has_a_fixture_file() {
    for kind in DeckKind::all() {
        let path = fixture_path(*kind);

        assert!(
            path.is_file(),
            "no fixture for {} at {} — slug rule may have drifted from examples/yaml_decks.rs",
            kind.deck_name(),
            path.display()
        );
    }
}
