//! Negative-path tests for YAML deck serialization.
//!
//! Every assertion here pins a *specific* failure. Where the failure is
//! semantic, it downcasts to the exact `CardError` variant — `is_err()` alone
//! would pass for the wrong reason, which is precisely the failure mode this
//! file exists to prevent.
//!
//! # What `razz_bad.yml` actually demonstrates
//!
//! `src/basic/decks/yaml/razz_bad.yml` is a real artifact kept in the repo
//! because a Copilot-generated deck file got it wrong (see the doc comment at
//! `src/basic/decks/razz.rs:19-24`). It differs from the good `razz.yaml` in
//! exactly six places: the Tens carry `index: '10'` / `symbol: '10'`.
//!
//! It is **syntactically valid YAML** but **not a valid card list**:
//! `Pip::index` and `Pip::symbol` are `char`, so `'10'` fails to deserialize
//! with "invalid value: string \"10\", expected a character". The historical
//! danger was never that it produced a subtly-wrong deck — it is that the
//! parse error used to be swallowed, leaving `Razz::base_vec()` silently
//! returning an **empty** deck that `Decked::validate()` did not flag.
//!
//! So the guarantee worth enforcing is: the failure surfaces as an error, and
//! never as an empty deck.

#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]
#![allow(non_snake_case)]

use cardpack::prelude::*;

/// Downcast a boxed error to a concrete `CardError`, or fail loudly naming
/// what was actually returned.
fn card_error<'a>(err: &'a (dyn std::error::Error + 'static)) -> &'a CardError {
    err.downcast_ref::<CardError>()
        .unwrap_or_else(|| panic!("expected a CardError, got: {err}"))
}

fn french_envelope() -> String {
    DeckKind::French.to_yaml().expect("serialize")
}

// ---------------------------------------------------------------------
// Parse-level failures — these surface as boxed serializer errors, not
// `CardError`s, because nothing semantic has been evaluated yet.
// ---------------------------------------------------------------------

#[test]
fn malformed_yaml__errors() {
    assert!(DeckYaml::from_yaml("{{{ not yaml at all").is_err());
    assert!(BasicCard::cards_from_yaml_str("{{{ not yaml at all").is_err());
}

#[test]
fn scalar_document__errors_malformed() {
    let err = DeckYaml::from_yaml("just a plain string").unwrap_err();

    assert_eq!(*card_error(err.as_ref()), CardError::YamlMalformed);
}

#[test]
fn wrong_field_type__errors() {
    // `index` is a `char`; a multi-character string cannot deserialize into it.
    let bad = "- suit: {weight: 3, pip_type: Suit, index: '10', symbol: 'S', value: 4}\n  \
                 rank: {weight: 12, pip_type: Rank, index: 'A', symbol: 'A', value: 14}";

    assert!(BasicCard::cards_from_yaml_str(bad).is_err());
}

// ---------------------------------------------------------------------
// Semantic failures — each pins its exact `CardError` variant.
// ---------------------------------------------------------------------

/// The truncation guard. Without it, a half-written file would silently yield
/// a short deck.
#[test]
fn truncated_envelope__errors_count_mismatch() {
    // French is 54 cards. Claim 55; the body still holds 54.
    let tampered = french_envelope().replace("count: 54", "count: 55");
    assert!(
        tampered.contains("count: 55"),
        "French envelope should declare count: 54"
    );

    let err = DeckYaml::from_yaml(&tampered).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlCountMismatch {
            declared: 55,
            actual: 54
        }
    );
}

/// The asymmetry: an empty *deck* is fatal, an empty *pile* is a fully-drawn
/// deck and therefore fine. Do not "harmonize" these.
#[test]
fn empty_deck__errors_for_deck_but_not_pile() {
    let empty = "version: 1\nname: French\nfluent_deck_key: french\ncount: 0\ncards: []\n";

    let err = DeckKind::from_yaml(empty).unwrap_err();
    assert_eq!(*card_error(err.as_ref()), CardError::YamlEmptyDeck);

    let err = French::validate_yaml(empty).unwrap_err();
    assert_eq!(*card_error(err.as_ref()), CardError::YamlEmptyDeck);

    let pile = Pile::<French>::from_yaml(empty).expect("an empty pile is legitimate");
    assert!(pile.is_empty());
}

#[test]
fn unknown_deck_name__errors() {
    let renamed = french_envelope().replace("name: French", "name: Bicycle");
    let err = DeckKind::from_yaml(&renamed).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlUnknownDeck("Bicycle".to_string())
    );
}

/// A legacy bare sequence is unidentifiable as a *deck* by design, but is
/// still perfectly readable as *cards*.
#[test]
fn legacy_sequence__errors_in_deck_kind_but_reads_as_cards() {
    let legacy = serde_norway::to_string(&DeckKind::French.base_vec()).expect("serialize");

    let err = DeckKind::from_yaml(&legacy).unwrap_err();
    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlUnknownDeck(String::new())
    );

    assert_eq!(
        BasicCard::cards_from_yaml_str(&legacy).expect("cards still parse"),
        DeckKind::French.base_vec()
    );
}

#[test]
fn wrong_deck_name__errors_in_pile() {
    let err = Pile::<Tarot>::from_yaml(&french_envelope()).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlDeckMismatch {
            expected: "Tarot".to_string(),
            found: "French".to_string(),
        }
    );
}

/// Right header, wrong cards — only the membership check catches this.
#[test]
fn foreign_card__errors_in_pile() {
    let disguised = french_envelope().replace("name: French", "name: Tarot");
    let err = Pile::<Tarot>::from_yaml(&disguised).unwrap_err();

    assert!(
        matches!(
            card_error(err.as_ref()),
            CardError::YamlForeignCard { deck, .. } if deck == "Tarot"
        ),
        "expected YamlForeignCard, got {:?}",
        card_error(err.as_ref())
    );
}

#[test]
fn wrong_deck__errors_in_validate_yaml() {
    let err = Tarot::validate_yaml(&french_envelope()).unwrap_err();

    assert_eq!(
        *card_error(err.as_ref()),
        CardError::YamlDeckMismatch {
            expected: "Tarot".to_string(),
            found: "French".to_string(),
        }
    );
}

/// `validate_yaml` has **two** ways to reject a wrong deck: the `name` header
/// check, and a card-by-card comparison against `base_vec()`. A legacy bare
/// sequence has no header, so only the second can catch it — without this
/// test, that branch is unexercised at the integration level.
#[test]
fn legacy_sequence_of_wrong_cards__errors_in_validate_yaml() {
    let tarot_cards = serde_norway::to_string(&DeckKind::Tarot.base_vec()).expect("serialize");
    let err = French::validate_yaml(&tarot_cards).unwrap_err();

    assert!(
        matches!(
            card_error(err.as_ref()),
            CardError::YamlDeckMismatch { expected, .. } if expected == "French"
        ),
        "expected YamlDeckMismatch for French, got {:?}",
        card_error(err.as_ref())
    );
}

// ---------------------------------------------------------------------
// The razz_bad regression — see the module docs above.
// ---------------------------------------------------------------------

/// `razz_bad.yml` must be **rejected**, loudly.
///
/// It is valid YAML syntax, so a naive "does it parse as YAML" check passes;
/// it is the *typed* deserialization into `Vec<BasicCard>` that rejects it.
#[test]
fn razz_bad__is_rejected() {
    let bad = include_str!("../src/basic/decks/yaml/razz_bad.yml");

    let err = BasicCard::cards_from_yaml_str(bad)
        .expect_err("razz_bad.yml must not deserialize into cards");

    assert!(
        err.to_string().contains("expected a character"),
        "expected a char-conversion failure on the '10' Tens, got: {err}"
    );

    assert!(
        Razz::validate_yaml(bad).is_err(),
        "razz_bad.yml must not validate as Razz"
    );
}

/// The historical bug this file guards against: the parse failure used to be
/// swallowed, leaving `Razz` with a silently **empty** deck. An empty deck
/// must never be the result of a bad file.
#[test]
fn razz_bad__never_yields_an_empty_deck() {
    let bad = include_str!("../src/basic/decks/yaml/razz_bad.yml");

    match BasicCard::cards_from_yaml_str(bad) {
        Err(_) => { /* correct: the failure surfaces */ }
        Ok(cards) => panic!(
            "razz_bad.yml must error, not yield {} cards (an empty or partial \
             deck here is the exact bug this guards)",
            cards.len()
        ),
    }

    // And the real Razz deck, built from the *good* embedded YAML, is intact.
    assert_eq!(DeckKind::Razz.base_vec().len(), 52);
}

/// The good file, for contrast — same code path, opposite verdict.
#[test]
fn razz_good__validates() {
    let good = include_str!("../src/basic/decks/yaml/razz.yaml");

    Razz::validate_yaml(good).expect("the shipped razz.yaml must validate as Razz");
}
