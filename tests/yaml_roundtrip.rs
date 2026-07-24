//! Round-trip guarantees for YAML deck serialization: every shipped deck, and
//! every kind of `Pile`, survives `deck -> YAML -> deck` unchanged.
//!
//! Driven off `DeckKind::all()` rather than a hardcoded list, so a new deck
//! inherits the guarantee automatically rather than needing a new test.
//!
//! These are integration tests, so they see only the crate's **public** API.
//! If something here fails to compile, an export is missing — that is a real
//! finding about the API surface, not a test bug.
//!
//! Skipped on `wasm32-unknown-unknown` because proptest's transitive
//! `wait-timeout` crate is unix-only, matching `tests/properties.rs`.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

// `Tiny` is the one deck not re-exported from the prelude; the full path works
// regardless of whether that is fixed.
use cardpack::basic::decks::tiny::Tiny;
use cardpack::prelude::*;
use proptest::prelude::*;

/// The EPIC's headline claim, for every shipped deck.
#[test]
fn every_deck_kind__roundtrips() {
    for kind in DeckKind::all() {
        let yml = kind.to_yaml().expect("serialize");
        let parsed = DeckKind::from_yaml(&yml).expect("deserialize");

        assert_eq!(parsed, *kind, "round-trip failed for {}", kind.deck_name());
    }
}

#[test]
fn every_deck_kind__preserves_cards() {
    for kind in DeckKind::all() {
        let yml = kind.to_yaml().expect("serialize");
        let cards = BasicCard::cards_from_yaml_str(&yml).expect("deserialize");

        assert_eq!(
            cards,
            kind.base_vec(),
            "cards differ for {}",
            kind.deck_name()
        );
        assert!(
            !cards.is_empty(),
            "{} serialized to an empty deck",
            kind.deck_name()
        );
    }
}

#[test]
fn every_deck_kind__preserves_metadata() {
    for kind in DeckKind::all() {
        let dy = DeckYaml::from_yaml(&kind.to_yaml().expect("serialize")).expect("deserialize");

        assert_eq!(dy.version, DeckYaml::VERSION);
        assert_eq!(dy.name, kind.deck_name());
        assert_eq!(dy.fluent_deck_key, kind.fluent_deck_key());
        assert_eq!(dy.count, kind.base_vec().len());
    }
}

/// The type-level twin of `every_deck_kind__roundtrips`. This is the path a
/// consumer's own deck takes through the blanket `YamlDecked` impl, so it
/// exercises a different code path than the registry version above.
#[test]
fn typed_decks__roundtrip() {
    macro_rules! assert_roundtrips {
        ($($deck:ty),+ $(,)?) => {
            $(
                let yml = <$deck>::to_yaml().expect("serialize");
                assert_eq!(
                    <$deck>::deck_from_yaml(&yml).expect("deserialize"),
                    <$deck>::base_vec(),
                    "round-trip failed for {}",
                    <$deck>::deck_name()
                );
                <$deck>::validate_yaml(&yml).expect("validate");
            )+
        };
    }

    assert_roundtrips!(
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

/// Order fidelity — the property that distinguishes the instance path from
/// the type path. The `assert_ne!` proves seed 42 actually produced a
/// non-canonical order, so the equality above it is not tautological.
#[test]
fn pile__shuffled_roundtrips_in_order() {
    let shuffled = Standard52::deck().shuffled_with_seed(42);
    let parsed =
        Pile::<Standard52>::from_yaml(&shuffled.to_yaml().expect("serialize")).expect("parse");

    assert_eq!(parsed, shuffled);
    assert_ne!(
        parsed,
        Standard52::deck(),
        "seed 42 should not reproduce canonical order"
    );
}

#[test]
fn pile__partial_roundtrips() {
    let mut deck = Standard52::deck();
    let hand = deck.draw(5).expect("draw 5");

    assert_eq!(
        Pile::<Standard52>::from_yaml(&hand.to_yaml().expect("serialize")).expect("parse"),
        hand
    );
}

/// 216 cards against a 54-card `base_vec()`: membership is the invariant,
/// not cardinality.
#[test]
fn pile__multideck_roundtrips() {
    let quad = French::decks(4);
    assert_eq!(quad.len(), 216);

    assert_eq!(
        Pile::<French>::from_yaml(&quad.to_yaml().expect("serialize")).expect("parse"),
        quad
    );
}

#[test]
fn pile__empty_roundtrips() {
    let empty = Pile::<French>::default();

    assert_eq!(
        Pile::<French>::from_yaml(&empty.to_yaml().expect("serialize")).expect("parse"),
        empty
    );
}

proptest! {
    /// Order fidelity across arbitrary permutations. `shuffled_with_seed`
    /// makes any failure reproducible from the printed seed.
    #[test]
    fn pile__roundtrips_for_any_seed(seed: u64) {
        let shuffled = Standard52::deck().shuffled_with_seed(seed);
        let yml = shuffled.to_yaml().expect("serialize");

        prop_assert_eq!(
            Pile::<Standard52>::from_yaml(&yml).expect("parse"),
            shuffled
        );
    }

    /// Any partial draw round-trips too, at every length from empty to a
    /// full deck.
    #[test]
    fn pile__partial_roundtrips_for_any_draw(seed: u64, n in 0usize..=52) {
        let mut deck = Standard52::deck().shuffled_with_seed(seed);
        let drawn = deck.draw(n).expect("draw n <= 52");
        let yml = drawn.to_yaml().expect("serialize");

        prop_assert_eq!(Pile::<Standard52>::from_yaml(&yml).expect("parse"), drawn);
    }
}
