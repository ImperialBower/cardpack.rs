//! Dumps golden vectors for the /deconstruct regeneration pack.
//! Public API only — this program is a consumer of the crate.
//!
//! # Features
//!
//! Uses `std` + `i18n` + `yaml`. cardpack is pure by default (`default = []`),
//! so to use these APIs in your own crate enable them explicitly:
//! `cardpack = { version = "0.9", features = ["std", "i18n", "yaml"] }`
//! (`yaml` implies `serde`). Note this dumper writes files, but it does so with
//! its own `std::fs` — reading decks from a YAML *file* via cardpack would need
//! the separate `std-io` feature.
//!
//! Running this example needs no `--features` flag — the self dev-dependency in
//! Cargo.toml turns them on for the repo's own examples.

// This example is a *consumer* of the crate (a golden-vector dumper), not part
// of the pure kernel, so it deliberately performs filesystem I/O. The
// kernel-purity lints (clippy.toml) exist to keep the *library* pure; allow
// them for this binary only. See docs/audit-2026-07-18-domain-kernel.md.
#![allow(clippy::disallowed_types, clippy::disallowed_methods)]

use cardpack::prelude::*;
use core::hash::Hash;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

fn write_vector(epic: &str, slug_dir: &str, behavior: &str, data: &Value) {
    let root = Path::new("docs/deconstruct/vectors").join(slug_dir);
    fs::create_dir_all(&root).expect("mkdir vectors dir");
    let envelope = json!({ "epic": epic, "behavior": behavior, "data": data });
    let path = root.join(format!("{behavior}.json"));
    fs::write(
        &path,
        serde_json::to_string_pretty(&envelope).expect("serialize") + "\n",
    )
    .expect("write vector");
    println!("wrote {}", path.display());
}

/// Full facet breakdown for every card in a deck's canonical (unshuffled)
/// order: position, index string, localized rank/suit names, and the
/// display/symbol string.
fn deck_entries<D>(deck: &Pile<D>) -> Vec<Value>
where
    D: DeckedBase + Default + Copy + Ord + Hash,
{
    deck.iter()
        .enumerate()
        .map(|(i, c)| {
            json!({
                "position": i,
                "index": c.index(),
                "rank": c.fluent_rank_name(&FluentName::US_ENGLISH),
                "suit": c.fluent_suit_name(&FluentName::US_ENGLISH),
                "symbol": c.to_string(),
            })
        })
        .collect()
}

/// The full rank/suit facet breakdown for a single illustrative card.
fn card_anatomy<D>(card: Card<D>) -> Value
where
    D: DeckedBase + Default + Copy + Ord + Hash,
{
    let base = card.base();
    json!({
        "index": card.index(),
        "symbol": card.to_string(),
        "full_name": card.fluent_name_default(),
        "rank": {
            "name": card.fluent_rank_name(&FluentName::US_ENGLISH),
            "weight": base.rank.weight,
            "index": base.rank.index.to_string(),
            "symbol": base.rank.symbol.to_string(),
        },
        "suit": {
            "name": card.fluent_suit_name(&FluentName::US_ENGLISH),
            "weight": base.suit.weight,
            "index": base.suit.index.to_string(),
            "symbol": base.suit.symbol.to_string(),
        },
    })
}

/// A deck's name, card count, and canonical-order index-string list.
fn composition<D>() -> (String, usize, Vec<String>)
where
    D: Decked<D> + Default + Copy + Ord + Hash,
{
    let deck = D::deck();
    let indices = deck.iter().map(Card::index).collect();
    (D::deck_name(), deck.len(), indices)
}

/// Just the variant name of a `CardError`, discarding the payload — the
/// payload is the input string we already recorded separately.
fn error_variant_name(e: &CardError) -> &'static str {
    match e {
        CardError::Fubar => "Fubar",
        CardError::InvalidCard(_) => "InvalidCard",
        CardError::InvalidCardCount(_) => "InvalidCardCount",
        CardError::InvalidFluentName(_) => "InvalidFluentName",
        CardError::InvalidIndex(_) => "InvalidIndex",
        CardError::NotEnoughCards(_) => "NotEnoughCards",
        CardError::TooManyCards(_) => "TooManyCards",
    }
}

// ---------------------------------------------------------------------
// DECON-01 — card-model
// ---------------------------------------------------------------------

fn decon01_canonical_order() {
    let deck = Standard52::deck();
    let cards = deck_entries(&deck);

    write_vector(
        "DECON-01",
        "card-model",
        "canonical-order",
        &json!({
            "deck": "Standard52",
            "count": deck.len(),
            "cards": cards,
        }),
    );
}

fn decon01_card_anatomy() {
    let cards = vec![
        card_anatomy(Card::<Standard52>::new(FrenchBasicCard::ACE_SPADES)),
        card_anatomy(Card::<Standard52>::new(FrenchBasicCard::TEN_DIAMONDS)),
        card_anatomy(Card::<Standard52>::new(FrenchBasicCard::DEUCE_CLUBS)),
        card_anatomy(Card::<French>::new(FrenchBasicCard::BIG_JOKER)),
    ];

    write_vector(
        "DECON-01",
        "card-model",
        "card-anatomy",
        &json!({ "cards": cards }),
    );
}

// ---------------------------------------------------------------------
// DECON-02 — pile-ops
// ---------------------------------------------------------------------

fn decon02_draw_semantics() {
    let mut deck = Standard52::deck();
    let initial_size = deck.len();

    let draw_first = deck.draw_first().map(|c| c.index());
    let size_after_first = deck.len();

    let draw_last = deck.draw_last().map(|c| c.index());
    let size_after_last = deck.len();

    let draw_three = deck
        .draw(3)
        .map(|p| p.iter().map(Card::index).collect::<Vec<_>>());
    let size_after_three = deck.len();

    let draw_zero = deck
        .draw(0)
        .map(|p| p.iter().map(Card::index).collect::<Vec<_>>());
    let size_after_zero = deck.len();

    let draw_too_many = deck
        .draw(1000)
        .map(|p| p.iter().map(Card::index).collect::<Vec<_>>());
    let size_after_too_many = deck.len();

    write_vector(
        "DECON-02",
        "pile-ops",
        "draw-semantics",
        &json!({
            "initial_size": initial_size,
            "steps": [
                { "op": "draw_first", "result": draw_first, "size_after": size_after_first },
                { "op": "draw_last", "result": draw_last, "size_after": size_after_last },
                { "op": "draw(3)", "result": draw_three, "size_after": size_after_three },
                { "op": "draw(0)", "result": draw_zero, "size_after": size_after_zero },
                { "op": "draw(1000)", "result": draw_too_many, "size_after": size_after_too_many },
            ],
        }),
    );
}

fn decon02_sort_variants() {
    let deck = Standard52::deck();
    let shuffled = deck.shuffled_with_seed(42);
    let sorted_default = shuffled.sorted();
    let sorted_by_rank = shuffled.sorted_by_rank();

    write_vector(
        "DECON-02",
        "pile-ops",
        "sort-variants",
        &json!({
            "seed": 42,
            "shuffled_index": shuffled.index(),
            "sorted_default_index": sorted_default.index(),
            "sorted_by_rank_index": sorted_by_rank.index(),
        }),
    );
}

fn decon02_extraction() {
    let deck = Standard52::deck();
    let ranks: Vec<String> = deck.ranks().iter().map(|p| p.index.to_string()).collect();
    let suits: Vec<String> = deck.suits().iter().map(|p| p.index.to_string()).collect();

    write_vector(
        "DECON-02",
        "pile-ops",
        "extraction",
        &json!({
            "ranks_index": deck.ranks_index(" "),
            "ranks": ranks,
            "suits_index": deck.suits_index(" "),
            "suits": suits,
            "combos_2_count": deck.combos(2).len(),
        }),
    );
}

// ---------------------------------------------------------------------
// DECON-03 — shuffling
// ---------------------------------------------------------------------

fn decon03_seeded_shuffle() {
    let deck = Standard52::deck();
    let seeds: [u64; 3] = [42, 1337, 2026];
    let shuffles: Vec<Value> = seeds
        .iter()
        .map(|&seed| {
            json!({
                "seed": seed,
                "index": deck.shuffled_with_seed(seed).index(),
            })
        })
        .collect();

    let first = deck.shuffled_with_seed(42).index();
    let second = deck.shuffled_with_seed(42).index();

    write_vector(
        "DECON-03",
        "shuffling",
        "seeded-shuffle",
        &json!({
            "deck": "Standard52",
            "shuffles": shuffles,
            "seed_42_repeatable": {
                "first": first,
                "second": second,
                "equal": first == second,
            },
        }),
    );
}

// ---------------------------------------------------------------------
// DECON-04 — formats
// ---------------------------------------------------------------------

fn decon04_roundtrip() {
    let deck = French::deck();
    let cards = deck_entries(&deck);

    write_vector(
        "DECON-04",
        "formats",
        "roundtrip",
        &json!({
            "deck": "French",
            "count": deck.len(),
            "cards": cards,
        }),
    );
}

fn decon04_parse_cases() {
    let inputs = ["as", "__", "ZZ"];
    let cases: Vec<Value> = inputs
        .iter()
        .map(|&input| match Card::<Standard52>::from_str(input) {
            Ok(card) => json!({ "input": input, "status": "ok", "index": card.index() }),
            Err(e) => json!({
                "input": input,
                "status": "error",
                "error": error_variant_name(&e),
            }),
        })
        .collect();

    write_vector(
        "DECON-04",
        "formats",
        "parse-cases",
        &json!({ "cases": cases }),
    );
}

fn decon04_ckc_encoding() {
    let deck = Standard52::deck();
    let cards: Vec<Value> = deck
        .iter()
        .map(|c| {
            let base: BasicCard = (*c).into();
            json!({ "index": c.index(), "ckc": base.get_ckc_number() })
        })
        .collect();

    write_vector(
        "DECON-04",
        "formats",
        "ckc-encoding",
        &json!({
            "deck": "Standard52",
            "count": deck.len(),
            "cards": cards,
        }),
    );
}

// ---------------------------------------------------------------------
// DECON-05 — french-family
// ---------------------------------------------------------------------

fn decon05_compositions() {
    let mut decks: Vec<Value> = Vec::new();

    for (name, count, cards) in [
        composition::<French>(),
        composition::<Standard52>(),
        composition::<Short>(),
        composition::<Spades>(),
        composition::<Euchre24>(),
        composition::<Euchre32>(),
        composition::<Pinochle>(),
        composition::<Canasta>(),
    ] {
        decks.push(json!({ "name": name, "count": count, "cards": cards }));
    }

    #[cfg(feature = "yaml")]
    {
        let (name, count, cards) = composition::<Razz>();
        decks.push(json!({ "name": name, "count": count, "cards": cards }));
    }
    #[cfg(not(feature = "yaml"))]
    {
        eprintln!(
            "SKIP: Razz composition omitted — `yaml` feature not enabled for this dumper run."
        );
    }

    write_vector(
        "DECON-05",
        "french-family",
        "compositions",
        &json!({ "decks": decks }),
    );
}

// ---------------------------------------------------------------------
// DECON-06 — tarot-skat
// ---------------------------------------------------------------------

fn decon06_compositions() {
    let tarot_deck = Tarot::deck();
    let tarot_cards: Vec<Value> = tarot_deck
        .iter()
        .enumerate()
        .map(|(i, c)| {
            json!({
                "position": i,
                "index": c.index(),
                "symbol": c.to_string(),
                "rank": c.fluent_rank_name(&FluentName::US_ENGLISH),
                "suit": c.fluent_suit_name(&FluentName::US_ENGLISH),
            })
        })
        .collect();

    let skat_deck = Skat::deck();
    let skat_cards: Vec<Value> = skat_deck
        .iter()
        .enumerate()
        .map(|(i, c)| {
            json!({
                "position": i,
                "index": c.index(),
                "symbol": c.to_string(),
                "rank": c.fluent_rank_name(&FluentName::DEUTSCH),
                "suit": c.fluent_suit_name(&FluentName::DEUTSCH),
            })
        })
        .collect();

    write_vector(
        "DECON-06",
        "tarot-skat",
        "compositions",
        &json!({
            "tarot": { "count": tarot_deck.len(), "cards": tarot_cards },
            "skat": { "count": skat_deck.len(), "cards": skat_cards },
        }),
    );
}

// ---------------------------------------------------------------------
// DECON-07 — localization
// ---------------------------------------------------------------------

fn decon07_locales() {
    let deck = Standard52::deck();
    // Canonical Standard52 order (see DECK const): Spades A..2 occupy
    // positions 0..13, then Hearts, Diamonds, Clubs each occupy the next
    // 13. One Ace per suit sits at 0, 13, 26, 39.
    let suit_positions = [0usize, 13, 26, 39];
    let rank_positions: Vec<usize> = (0..13).collect();
    // Sample cards: first (Ace Spades), a middle card (King Hearts), and
    // the last (Deuce Clubs) — fixed positions, not locale-dependent.
    let sample_positions = [0usize, 14, 51];

    let locale_defs = [
        ("en-US", &FluentName::US_ENGLISH),
        ("de", &FluentName::DEUTSCH),
        ("fr", &FluentName::FRANCAIS),
        ("la", &FluentName::LATINA),
        ("tlh", &FluentName::TLHINGAN),
    ];

    let locales: Vec<Value> = locale_defs
        .iter()
        .map(|&(name, lid)| {
            let suits: Vec<Value> = suit_positions
                .iter()
                .map(|&i| {
                    let c = deck.get(i).expect("suit sample card in range");
                    json!({
                        "index": c.base().suit.index.to_string(),
                        "name": c.fluent_suit_name(lid),
                    })
                })
                .collect();

            let ranks: Vec<Value> = rank_positions
                .iter()
                .map(|&i| {
                    let c = deck.get(i).expect("rank sample card in range");
                    json!({
                        "index": c.base().rank.index.to_string(),
                        "name": c.fluent_rank_name(lid),
                    })
                })
                .collect();

            let sample_cards: Vec<Value> = sample_positions
                .iter()
                .map(|&i| {
                    let c = deck.get(i).expect("sample card in range");
                    json!({ "index": c.index(), "name": c.fluent_name(lid) })
                })
                .collect();

            json!({
                "locale": name,
                "suits": suits,
                "ranks": ranks,
                "sample_cards": sample_cards,
            })
        })
        .collect();

    write_vector(
        "DECON-07",
        "localization",
        "locales",
        &json!({ "locales": locales }),
    );
}

// ---------------------------------------------------------------------
// DECON-08 — extension-registry
// ---------------------------------------------------------------------

fn decon08_registry() {
    let decks: Vec<Value> = DeckKind::all()
        .iter()
        .map(|k| {
            json!({
                "name": k.deck_name(),
                "count": k.base_vec().len(),
            })
        })
        .collect();

    write_vector(
        "DECON-08",
        "extension-registry",
        "registry",
        &json!({ "decks": decks }),
    );
}

fn main() {
    decon01_canonical_order();
    decon01_card_anatomy();

    decon02_draw_semantics();
    decon02_sort_variants();
    decon02_extraction();

    decon03_seeded_shuffle();

    decon04_roundtrip();
    decon04_parse_cases();
    decon04_ckc_encoding();

    decon05_compositions();

    decon06_compositions();

    decon07_locales();

    decon08_registry();
}
