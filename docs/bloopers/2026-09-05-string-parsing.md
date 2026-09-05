# Blooper Report: String Parsing / `FromStr`

Date: 2026-09-05
Focus: string parsing before `FromStr` was adopted

## The blooper

File: `src/cards/decks/standard52.rs` (present as of tag `v0.5.0`, first added 2022-01-29
in commit `29e8ef3`)

Two separate hand-rolled parsing functions did the same job two different (bad) ways:

```rust
pub fn card_from_index(index: &'static str) -> Card {
    let rank = Rank::from_french_deck_index(Standard52::rank_str_from_index(index));
    let suit = Suit::from_french_deck_index(Standard52::suit_char_from_index(index));
    // ...
}

fn suit_char_from_index(card_str: &'static str) -> char {
    if card_str.len() < 2 {
        return '_';
    }
    card_str.char_indices().nth(1).unwrap().1
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn card_from_string(index: String) -> Card {
    let char_vec: Vec<char> = index.chars().collect();

    let mut rank = Rank::default();
    let mut suit = Suit::default();

    if let Some(r) = char_vec.first() {
        rank = Rank::from_french_deck_char(*r);
    }

    if let Some(s) = char_vec.get(1) {
        suit = Suit::from_french_deck_index(*s);
    }
    // ...
}
```

### Why it's a blooper

- `card_from_index` only accepted `&'static str` — text baked into the binary at compile
  time. It could not parse a string typed by a user or read from a file at runtime.
- `card_from_string` existed as a parallel, differently-implemented version of the same
  logic, converting the string into a `Vec<char>` just to peek at index 0 and 1.
- `suit_char_from_index` called `.unwrap()` on `char_indices().nth(1)` — a crash waiting
  to happen on malformed input, worked around only by a manual length check beforehand.
- Neither function implemented the standard `std::str::FromStr` trait, so callers had no
  idiomatic `"AS".parse::<Card>()` or `Card::from_str("AS")` and no `Result`-based error
  handling — invalid input silently produced `Card::default()` instead of an error.

### The fix

Commit `2813a56` ("removed old", 2025-02-26) deleted this file (`src/old/...` tree)
entirely as part of a broader cleanup.

Current code implements `FromStr` properly in four places, all returning `Result`:

- `src/localization.rs` — `impl FromStr for FluentName`
- `src/funky/types/buffoon_card.rs` — `impl FromStr for BCardType` and
  `impl FromStr for BuffoonCard`
- `src/funky/types/buffoon_pile.rs` — `impl FromStr for BuffoonPile`

## Still outstanding

None — checked the current tree for the same patterns
(`char_indices().nth`, `chars().collect()` into a manual index lookup, stray `char_vec`
variables) and found no matches. All cleaned up 🎉
