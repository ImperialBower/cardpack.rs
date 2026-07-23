# Ganjifa Decks (Mughal + Dashavatara) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Mughal Ganjifa (8 suits × 12 = 96 cards) and Dashavatara Ganjifa (10 suits × 12 = 120 cards) as first-class decks with per-suit inverted pip ranking, Fluent localization in all 5 locales, and `DeckKind` registry integration — per `docs/EPIC-02_Ganjifa.md`.

**Architecture:** No core-type changes. One shared vocabulary file (`src/basic/decks/cards/ganjifa.rs`) defines two rank ladders — strong (`K V T 9 … 2 A`, weights 11→0) and weak (`K V A 2 … 9 T`, same index chars, inverted pip weights) — plus both decks' suit `Pip`s and a `const fn` deck builder. Two thin deck files (`mughal.rs`, `dashavatara.rs`) clone the `skat.rs` shape. Because ladders and suit arrays are declared in descending weight, the builder's output is already in `sorted()` order, which `validate()` requires.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc discipline, fluent-templates i18n, clippy-pedantic CI.

## Global Constraints

- **Git:** The implementer must NOT run any state-changing git command (user's global CLAUDE.md). At each "Commit" step, STOP and tell the user the exact `git add … && git commit -m "…"` command to run themselves. Wait for confirmation before the next task.
- **Feature flags:** crate default is `default = []` (pure kernel). Run tests with `--features full`. i18n-dependent tests must be behind `#[cfg(feature = "i18n")]`; color code behind `#[cfg(feature = "colored-display")]`.
- **no_std discipline:** in `src/` use `alloc::string::{String, ToString}` and `alloc::vec::Vec` — never `std::` except behind the `colored-display` gate (`std::collections::HashMap`, `colored::Color`).
- **Clippy:** CI denies pedantic: `cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic`. Public fns that can panic need a `# Panics` doc section.
- **Weights:** rank weight max 11 (courts) — keeps the CKC shift at `16 + 11 = 27 < 32` (wasm32-safe). Never raise it.
- **Pips:** every `symbol` is a single Unicode scalar (`char`, no ZWJ sequences, no Unicode-16 emoji); every `index` is an uppercase ASCII letter or digit.
- **Copy (verbatim):** deck names `"Mughal Ganjifa"` / `"Dashavatara Ganjifa"`; fluent bases `"mughal"` / `"dashavatara"`; Dashavatara en-US courts `Raja` / `Pradhan`; Mughal en-US courts `King` / `Vizier`.
- **Ordering assertions in tests:** `BasicCard::Ord` is inverted (highest first). Always assert through `sorted()` output strings, never `<`/`>` intuition.
- **`ranks()` on a full Ganjifa deck returns 22 distinct `Pip`s** (2 courts + 10 strong + 10 weak), not 12. `suits()` returns 8 (Mughal) / 10 (Dashavatara).
- **OKF bundle:** `.okf/` documents the deck catalog; it must be updated in the same change (Task 6).

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `src/basic/decks/cards/ganjifa.rs` | Rewrite stub | Shared vocabulary: `GanjifaRank` (courts + strong/weak ladders), `MughalSuit`, `DashavataraSuit`, `GanjifaBasicCard` sampler consts, `ganjifa_deck` const builder, fluent key consts |
| `src/basic/decks/cards.rs` | Modify | Alphabetize the existing `pub mod ganjifa;` |
| `src/basic/decks/mughal.rs` | Create | `Mughal` marker struct + `DeckedBase`/`Decked` impls + test battery |
| `src/basic/decks/dashavatara.rs` | Create | `Dashavatara` twin |
| `src/basic/decks.rs` | Modify | Register `dashavatara`, `mughal` modules (alphabetical) |
| `src/prelude.rs` | Modify | Re-export `cards::ganjifa::*`, `mughal::*`, `dashavatara::*` |
| `src/localization/locales/{en-US,de,fr,la,tlh}/{mughal,dashavatara}.ftl` | Create ×10 | Fluent entries; non-English files carry DRAFT headers |
| `src/localization/locales/{fr,la,tlh}/README.md` | Modify | Add status-table rows |
| `src/basic/decks/registry.rs` | Modify | `DeckKind::Dashavatara` + `DeckKind::Mughal` variants, all match arms, renamed key test |
| `examples/demo.rs` | Modify | `--mughal` / `--dashavatara` long-only flags |
| `CHANGELOG.md`, `README.md` | Modify | Unreleased entry; deck list |
| `.okf/index.md`, `.okf/decks/index.md`, `.okf/decks/deck-catalog.md`, `.okf/architecture/localization.md`, `.okf/log.md` | Modify | 12 → 14 decks; catalog rows; fluent bases; log entry |

---

### Task 1: Ganjifa card vocabulary (`src/basic/decks/cards/ganjifa.rs`)

**Files:**
- Rewrite: `src/basic/decks/cards/ganjifa.rs` (current stub has a `GanjifaBasicCart` typo — replace the whole file)
- Modify: `src/basic/decks/cards.rs` (alphabetize the module list)
- Modify: `src/prelude.rs` (re-export `cards::ganjifa::*` so the doctest resolves)

**Interfaces:**
- Consumes: `Pip`, `PipType` (`src/basic/types/pips.rs`), `BasicCard` (`src/basic/types/basic_card.rs`) — both have `const fn new`.
- Produces (later tasks rely on these exact names):
  - `GanjifaRank::{KING, VIZIER, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, TREY, DEUCE, ACE, WEAK_ACE, WEAK_DEUCE, WEAK_TREY, WEAK_FOUR, WEAK_FIVE, WEAK_SIX, WEAK_SEVEN, WEAK_EIGHT, WEAK_NINE, WEAK_TEN}: Pip`
  - `GanjifaRank::STRONG: [Pip; 12]`, `GanjifaRank::WEAK: [Pip; 12]`
  - `MughalSuit::{SLAVES, CROWNS, SWORDS, RED_COINS, HARPS, BILLS, WHITE_COINS, CLOTH}: Pip`
  - `DashavataraSuit::{MATSYA, KURMA, VARAHA, NARASIMHA, VAMANA, PARASHURAMA, RAMA, KRISHNA, JAGANNATH, KALKI}: Pip`
  - `GanjifaBasicCard::{KING_SLAVES, WEAK_ACE_RED_COINS, KING_MATSYA, TEN_KALKI}: BasicCard`
  - `pub const fn ganjifa_deck<const S: usize, const N: usize>(suits: &[Pip; S], strong: &[bool; S]) -> [BasicCard; N]`
  - `pub const FLUENT_KEY_BASE_NAME_MUGHAL: &str = "mughal"`, `pub const FLUENT_KEY_BASE_NAME_DASHAVATARA: &str = "dashavatara"`

- [ ] **Step 1: Write the failing tests**

Replace `src/basic/decks/cards/ganjifa.rs` entirely with the imports, empty struct declarations, and the test module (no consts, no builder yet):

```rust
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::pips::{Pip, PipType};

/// Named [`BasicCard`] constants for tests and docs — a sampler, not all 216.
pub struct GanjifaBasicCard;
/// Rank [`Pip`]s shared by every Ganjifa deck: two courts plus a strong
/// (`10 > … > A`) and a weak (`A > … > 10`) pip ladder.
pub struct GanjifaRank;
/// The eight Mughal Ganjifa suit [`Pip`]s.
pub struct MughalSuit;
/// The ten Dashavatara Ganjifa suit [`Pip`]s (Vishnu's avatars).
pub struct DashavataraSuit;

pub const FLUENT_KEY_BASE_NAME_MUGHAL: &str = "mughal";
pub const FLUENT_KEY_BASE_NAME_DASHAVATARA: &str = "dashavatara";

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__decks__cards__ganjifa_tests {
    use super::*;
    use alloc::string::ToString;

    /// Guards `ganjifa_deck`'s `const fn` status: if the builder ever loses
    /// `const`, this declaration fails to compile.
    const _GUARD: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);

    const MUGHAL_SUITS: [Pip; 8] = [
        MughalSuit::SLAVES,
        MughalSuit::CROWNS,
        MughalSuit::SWORDS,
        MughalSuit::RED_COINS,
        MughalSuit::HARPS,
        MughalSuit::BILLS,
        MughalSuit::WHITE_COINS,
        MughalSuit::CLOTH,
    ];

    const DASHAVATARA_SUITS: [Pip; 10] = [
        DashavataraSuit::MATSYA,
        DashavataraSuit::KURMA,
        DashavataraSuit::VARAHA,
        DashavataraSuit::NARASIMHA,
        DashavataraSuit::VAMANA,
        DashavataraSuit::PARASHURAMA,
        DashavataraSuit::RAMA,
        DashavataraSuit::KRISHNA,
        DashavataraSuit::JAGANNATH,
        DashavataraSuit::KALKI,
    ];

    fn assert_weights_strictly_descending(pips: &[Pip]) {
        for w in pips.windows(2) {
            assert!(
                w[0].weight > w[1].weight,
                "weights not strictly descending: {:?} then {:?}",
                w[0],
                w[1]
            );
        }
    }

    fn assert_indexes_unique(pips: &[Pip]) {
        for (i, a) in pips.iter().enumerate() {
            for b in &pips[i + 1..] {
                assert_ne!(a.index, b.index, "duplicate index char in {a:?} / {b:?}");
            }
        }
    }

    #[test]
    fn ladders__strictly_descending() {
        assert_weights_strictly_descending(&GanjifaRank::STRONG);
        assert_weights_strictly_descending(&GanjifaRank::WEAK);
        assert_weights_strictly_descending(&MUGHAL_SUITS);
        assert_weights_strictly_descending(&DASHAVATARA_SUITS);
    }

    #[test]
    fn ladders__index_uniqueness() {
        assert_indexes_unique(&GanjifaRank::STRONG);
        assert_indexes_unique(&GanjifaRank::WEAK);
        assert_indexes_unique(&MUGHAL_SUITS);
        assert_indexes_unique(&DASHAVATARA_SUITS);
    }

    #[test]
    fn ladders__courts_first() {
        assert_eq!(GanjifaRank::STRONG[0], GanjifaRank::KING);
        assert_eq!(GanjifaRank::STRONG[1], GanjifaRank::VIZIER);
        assert_eq!(GanjifaRank::WEAK[0], GanjifaRank::KING);
        assert_eq!(GanjifaRank::WEAK[1], GanjifaRank::VIZIER);
    }

    #[test]
    fn ladders__weak_mirrors_strong() {
        // Every strong pip has a weak twin sharing index, symbol, and value;
        // pip weights (ladder positions 2..12) mirror, so each strong/weak
        // pair's weights sum to 9.
        for strong in &GanjifaRank::STRONG[2..] {
            let weak = GanjifaRank::WEAK[2..]
                .iter()
                .find(|w| w.index == strong.index)
                .expect("every strong pip needs a weak twin");
            assert_eq!(strong.symbol, weak.symbol);
            assert_eq!(strong.value, weak.value);
            assert_eq!(
                strong.weight + weak.weight,
                9,
                "weights must mirror for index {:?}",
                strong.index
            );
        }
    }

    #[test]
    fn pips__types_and_values() {
        for suit in MUGHAL_SUITS.iter().chain(DASHAVATARA_SUITS.iter()) {
            assert_eq!(suit.pip_type, PipType::Suit);
            assert_eq!(suit.value, suit.weight + 1);
        }
        for rank in GanjifaRank::STRONG.iter().chain(GanjifaRank::WEAK.iter()) {
            assert_eq!(rank.pip_type, PipType::Rank);
        }
    }

    #[test]
    fn ganjifa_deck__strong_suit_uses_strong_ladder() {
        let deck: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);
        for (card, rank) in deck.iter().zip(GanjifaRank::STRONG.iter()) {
            assert_eq!(card.suit, MughalSuit::SLAVES);
            assert_eq!(card.rank, *rank);
        }
    }

    #[test]
    fn ganjifa_deck__weak_suit_uses_weak_ladder() {
        let deck: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::RED_COINS], &[false]);
        for (card, rank) in deck.iter().zip(GanjifaRank::WEAK.iter()) {
            assert_eq!(card.suit, MughalSuit::RED_COINS);
            assert_eq!(card.rank, *rank);
        }
    }

    #[test]
    fn named_cards() {
        assert_eq!(GanjifaBasicCard::KING_SLAVES.index(), "KG");
        assert_eq!(GanjifaBasicCard::WEAK_ACE_RED_COINS.index(), "AR");
        assert_eq!(GanjifaBasicCard::KING_MATSYA.index(), "KM");
        assert_eq!(GanjifaBasicCard::TEN_KALKI.index(), "TC");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --features full ganjifa`
Expected: COMPILE ERROR — unresolved `ganjifa_deck`, `GanjifaRank::STRONG`, `MughalSuit::SLAVES`, etc.

- [ ] **Step 3: Write the implementation**

Insert between the `FLUENT_KEY_BASE_NAME_DASHAVATARA` const and the `#[cfg(test)]` module:

```rust
impl GanjifaRank {
    // Courts — shared by both ladders. Weight 11 max keeps the CKC shift at
    // 16 + 11 = 27 < 32, safe on wasm32.
    pub const KING: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 11,
        index: 'K',
        symbol: 'K',
        value: 12,
    };
    pub const VIZIER: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 10,
        index: 'V',
        symbol: 'V',
        value: 11,
    };

    // Strong-ladder pips: 10 > 9 > … > 2 > A.
    pub const TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'T',
        symbol: 'T',
        value: 10,
    };
    pub const NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '2',
        symbol: '2',
        value: 2,
    };
    pub const ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'A',
        symbol: 'A',
        value: 1,
    };

    // Weak-ladder pips: A > 2 > … > 9 > 10. Same index/symbol/value as their
    // strong twins; only the weights invert.
    pub const WEAK_ACE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 9,
        index: 'A',
        symbol: 'A',
        value: 1,
    };
    pub const WEAK_DEUCE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 8,
        index: '2',
        symbol: '2',
        value: 2,
    };
    pub const WEAK_TREY: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 7,
        index: '3',
        symbol: '3',
        value: 3,
    };
    pub const WEAK_FOUR: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 6,
        index: '4',
        symbol: '4',
        value: 4,
    };
    pub const WEAK_FIVE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 5,
        index: '5',
        symbol: '5',
        value: 5,
    };
    pub const WEAK_SIX: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 4,
        index: '6',
        symbol: '6',
        value: 6,
    };
    pub const WEAK_SEVEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 3,
        index: '7',
        symbol: '7',
        value: 7,
    };
    pub const WEAK_EIGHT: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 2,
        index: '8',
        symbol: '8',
        value: 8,
    };
    pub const WEAK_NINE: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 1,
        index: '9',
        symbol: '9',
        value: 9,
    };
    pub const WEAK_TEN: Pip = Pip {
        pip_type: PipType::Rank,
        weight: 0,
        index: 'T',
        symbol: 'T',
        value: 10,
    };

    /// Strong-suit rank ladder in descending-weight (sorted) order.
    pub const STRONG: [Pip; 12] = [
        Self::KING,
        Self::VIZIER,
        Self::TEN,
        Self::NINE,
        Self::EIGHT,
        Self::SEVEN,
        Self::SIX,
        Self::FIVE,
        Self::FOUR,
        Self::TREY,
        Self::DEUCE,
        Self::ACE,
    ];

    /// Weak-suit rank ladder in descending-weight (sorted) order — pips
    /// inverted: Ace high, Ten low.
    pub const WEAK: [Pip; 12] = [
        Self::KING,
        Self::VIZIER,
        Self::WEAK_ACE,
        Self::WEAK_DEUCE,
        Self::WEAK_TREY,
        Self::WEAK_FOUR,
        Self::WEAK_FIVE,
        Self::WEAK_SIX,
        Self::WEAK_SEVEN,
        Self::WEAK_EIGHT,
        Self::WEAK_NINE,
        Self::WEAK_TEN,
    ];
}

impl MughalSuit {
    pub const SLAVES: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 7,
        index: 'G',
        symbol: '👤',
        value: 8,
    };
    pub const CROWNS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 6,
        index: 'T',
        symbol: '👑',
        value: 7,
    };
    pub const SWORDS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 5,
        index: 'S',
        symbol: '⚔',
        value: 6,
    };
    pub const RED_COINS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 4,
        index: 'R',
        symbol: '🔴',
        value: 5,
    };
    pub const HARPS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'H',
        symbol: '🎵',
        value: 4,
    };
    pub const BILLS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'B',
        symbol: '📜',
        value: 3,
    };
    pub const WHITE_COINS: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'W',
        symbol: '⚪',
        value: 2,
    };
    pub const CLOTH: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'Q',
        symbol: '🧵',
        value: 1,
    };
}

impl DashavataraSuit {
    pub const MATSYA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 9,
        index: 'M',
        symbol: '🐟',
        value: 10,
    };
    pub const KURMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 8,
        index: 'U',
        symbol: '🐢',
        value: 9,
    };
    pub const VARAHA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 7,
        index: 'B',
        symbol: '🐗',
        value: 8,
    };
    pub const NARASIMHA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 6,
        index: 'N',
        symbol: '🦁',
        value: 7,
    };
    pub const VAMANA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 5,
        index: 'D',
        symbol: '☂',
        value: 6,
    };
    pub const PARASHURAMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 4,
        index: 'P',
        symbol: '🪓',
        value: 5,
    };
    pub const RAMA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 3,
        index: 'R',
        symbol: '🏹',
        value: 4,
    };
    pub const KRISHNA: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 2,
        index: 'K',
        symbol: '🐄',
        value: 3,
    };
    pub const JAGANNATH: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 1,
        index: 'J',
        symbol: '☸',
        value: 2,
    };
    pub const KALKI: Pip = Pip {
        pip_type: PipType::Suit,
        weight: 0,
        index: 'C',
        symbol: '🐎',
        value: 1,
    };
}

impl GanjifaBasicCard {
    pub const KING_SLAVES: BasicCard = BasicCard::new(MughalSuit::SLAVES, GanjifaRank::KING);
    pub const WEAK_ACE_RED_COINS: BasicCard =
        BasicCard::new(MughalSuit::RED_COINS, GanjifaRank::WEAK_ACE);
    pub const KING_MATSYA: BasicCard = BasicCard::new(DashavataraSuit::MATSYA, GanjifaRank::KING);
    pub const TEN_KALKI: BasicCard = BasicCard::new(DashavataraSuit::KALKI, GanjifaRank::TEN);
}

/// Builds a Ganjifa deck: for each suit, courts + 10 pips from the strong or
/// weak ladder, in descending-weight (sorted) order.
///
/// Because the ladders and any descending-weight `suits` array are already in
/// sorted order, the output equals the deck's `sorted()` order — which
/// `Decked::validate()` requires.
///
/// ```
/// use cardpack::prelude::*;
///
/// const SLAVES_ONLY: [BasicCard; 12] = ganjifa_deck(&[MughalSuit::SLAVES], &[true]);
/// assert_eq!(SLAVES_ONLY[0], GanjifaBasicCard::KING_SLAVES);
/// ```
///
/// # Panics
///
/// Panics (at compile time for const evaluation) if `N != S * 12`.
#[must_use]
pub const fn ganjifa_deck<const S: usize, const N: usize>(
    suits: &[Pip; S],
    strong: &[bool; S],
) -> [BasicCard; N] {
    assert!(N == S * 12, "N must equal S * 12");
    let mut deck = [BasicCard::new(
        Pip::new(PipType::Blank, 0, '_', '_'),
        Pip::new(PipType::Blank, 0, '_', '_'),
    ); N];
    let mut i = 0;
    while i < S {
        let ranks = if strong[i] {
            &GanjifaRank::STRONG
        } else {
            &GanjifaRank::WEAK
        };
        let mut j = 0;
        while j < 12 {
            deck[i * 12 + j] = BasicCard::new(suits[i], ranks[j]);
            j += 1;
        }
        i += 1;
    }
    deck
}
```

- [ ] **Step 4: Alphabetize the module registration**

`src/basic/decks/cards.rs` currently has `pub mod ganjifa;` appended last. Make the file exactly:

```rust
pub mod canasta;
pub mod french;
pub mod ganjifa;
pub mod pinochle;
pub mod skat;
pub mod tarot;
```

In `src/prelude.rs`, insert after the `pub use crate::basic::decks::cards::french::*;` line:

```rust
pub use crate::basic::decks::cards::ganjifa::*;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test --features full ganjifa`
Expected: PASS — 8 unit tests in `basic__decks__cards__ganjifa_tests`.

Run: `cargo test --doc --features full ganjifa_deck`
Expected: PASS — 1 doctest.

- [ ] **Step 6: Lint + no_std check**

Run: `cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: clean.
Run: `cargo test --no-default-features --lib ganjifa`
Expected: PASS (the vocabulary and its tests are alloc-only).

- [ ] **Step 7: Commit point — STOP, suggest to user (do not run)**

```bash
git add src/basic/decks/cards/ganjifa.rs src/basic/decks/cards.rs src/prelude.rs
git commit -m "feat: add Ganjifa card vocabulary with dual rank ladders and const deck builder"
```

---

### Task 2: Mughal deck (`src/basic/decks/mughal.rs`) + en-US `mughal.ftl`

**Files:**
- Create: `src/basic/decks/mughal.rs`
- Create: `src/localization/locales/en-US/mughal.ftl` (the deck's `fluent__name` test depends on it)
- Modify: `src/basic/decks.rs`, `src/prelude.rs`

**Interfaces:**
- Consumes (Task 1): `ganjifa_deck`, `MughalSuit::*`, `FLUENT_KEY_BASE_NAME_MUGHAL`, `GanjifaBasicCard::{KING_SLAVES, WEAK_ACE_RED_COINS}`.
- Produces: `pub struct Mughal {}` implementing `DeckedBase` + `Decked<Self>`; `pub type MughalDeck = Pile<Mughal>`; `pub type MughalCard = Card<Mughal>`; `Mughal::{DECK_SIZE: usize, SUITS: [Pip; 8], STRONG: [bool; 8], DECK: [BasicCard; 96]}`.

- [ ] **Step 1: Write the deck file with its failing test battery**

Create `src/basic/decks/mughal.rs`. NOTE the `Pip` import is NOT gated (unlike `skat.rs`) because `SUITS: [Pip; 8]` uses it unconditionally:

```rust
use crate::basic::decks::cards::ganjifa::{
    FLUENT_KEY_BASE_NAME_MUGHAL, MughalSuit, ganjifa_deck,
};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "colored-display")]
use colored::Color;
#[cfg(feature = "colored-display")]
use std::collections::HashMap;

/// [Mughal Ganjifa](https://en.wikipedia.org/wiki/Ganjifa) — 8 suits × 12 =
/// 96 cards. The four weak suits (Red Coins, Harps, Bills, Cloth) use the
/// inverted pip ladder: `A > 2 > … > 9 > 10`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mughal {}
#[allow(clippy::module_name_repetitions)]
pub type MughalDeck = Pile<Mughal>;
#[allow(clippy::module_name_repetitions)]
pub type MughalCard = Card<Mughal>;

impl Mughal {
    pub const DECK_SIZE: usize = 96;

    /// The eight suits in descending-weight (sorted) order.
    pub const SUITS: [Pip; 8] = [
        MughalSuit::SLAVES,
        MughalSuit::CROWNS,
        MughalSuit::SWORDS,
        MughalSuit::RED_COINS,
        MughalSuit::HARPS,
        MughalSuit::BILLS,
        MughalSuit::WHITE_COINS,
        MughalSuit::CLOTH,
    ];

    /// `true` = strong suit (pips 10 high), `false` = weak suit (Ace high).
    /// Parallel to [`Self::SUITS`].
    pub const STRONG: [bool; 8] = [true, true, true, false, false, false, true, false];

    pub const DECK: [BasicCard; Self::DECK_SIZE] = ganjifa_deck(&Self::SUITS, &Self::STRONG);
}

impl DeckedBase for Mughal {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(MughalSuit::CROWNS, Color::Yellow);
        mappie.insert(MughalSuit::SWORDS, Color::BrightBlue);
        mappie.insert(MughalSuit::RED_COINS, Color::Red);
        mappie.insert(MughalSuit::HARPS, Color::Magenta);
        mappie.insert(MughalSuit::BILLS, Color::Cyan);
        mappie.insert(MughalSuit::CLOTH, Color::Green);

        mappie
    }

    fn deck_name() -> String {
        "Mughal Ganjifa".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
    }
}

impl Decked<Self> for Mughal {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__mughal_tests {
    use super::*;
    use crate::basic::decks::cards::ganjifa::GanjifaBasicCard;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::{Decked, Ranged};
    #[cfg(feature = "i18n")]
    use crate::localization::{FluentName, Named};
    use core::str::FromStr;

    /// The load-bearing test: deck ↔ string round-trip plus
    /// seeded-shuffle → sort == original.
    #[test]
    fn decked__validate() {
        assert!(Mughal::validate());
    }

    #[test]
    fn decked__deck() {
        let deck = Mughal::deck();
        assert_eq!(deck.len(), 96);
        assert_eq!(
            deck.index(),
            "KG VG TG 9G 8G 7G 6G 5G 4G 3G 2G AG KT VT TT 9T 8T 7T 6T 5T 4T 3T 2T AT KS VS TS 9S 8S 7S 6S 5S 4S 3S 2S AS KR VR AR 2R 3R 4R 5R 6R 7R 8R 9R TR KH VH AH 2H 3H 4H 5H 6H 7H 8H 9H TH KB VB AB 2B 3B 4B 5B 6B 7B 8B 9B TB KW VW TW 9W 8W 7W 6W 5W 4W 3W 2W AW KQ VQ AQ 2Q 3Q 4Q 5Q 6Q 7Q 8Q 9Q TQ"
        );
        assert_eq!(
            deck.to_string(),
            "K👤 V👤 T👤 9👤 8👤 7👤 6👤 5👤 4👤 3👤 2👤 A👤 K👑 V👑 T👑 9👑 8👑 7👑 6👑 5👑 4👑 3👑 2👑 A👑 K⚔ V⚔ T⚔ 9⚔ 8⚔ 7⚔ 6⚔ 5⚔ 4⚔ 3⚔ 2⚔ A⚔ K🔴 V🔴 A🔴 2🔴 3🔴 4🔴 5🔴 6🔴 7🔴 8🔴 9🔴 T🔴 K🎵 V🎵 A🎵 2🎵 3🎵 4🎵 5🎵 6🎵 7🎵 8🎵 9🎵 T🎵 K📜 V📜 A📜 2📜 3📜 4📜 5📜 6📜 7📜 8📜 9📜 T📜 K⚪ V⚪ T⚪ 9⚪ 8⚪ 7⚪ 6⚪ 5⚪ 4⚪ 3⚪ 2⚪ A⚪ K🧵 V🧵 A🧵 2🧵 3🧵 4🧵 5🧵 6🧵 7🧵 8🧵 9🧵 T🧵"
        );
    }

    /// The Ganjifa signature rule, asserted through `sorted()` output
    /// (`BasicCard::Ord` is inverted — never assert with `<`/`>`).
    #[test]
    fn weak_suit__inversion() {
        // Red Coins is weak: Ace outranks Ten.
        let weak = MughalDeck::from_str("TR AR").unwrap().sorted();
        assert_eq!(weak.index(), "AR TR");

        // Slaves is strong: Ten outranks Ace.
        let strong = MughalDeck::from_str("AG TG").unwrap().sorted();
        assert_eq!(strong.index(), "TG AG");
    }

    /// 2 courts + 10 strong pips + 10 weak pips = 22 distinct rank `Pip`s
    /// (`Pip` equality covers all five fields) — NOT 12.
    #[test]
    fn ranks__distinct_pips() {
        assert_eq!(Mughal::deck().ranks().len(), 22);
        assert_eq!(Mughal::deck().suits().len(), 8);
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked__colors() {
        assert_eq!(Mughal::colors().len(), 6);
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Mughal::deck_name(), "Mughal Ganjifa");
    }

    #[test]
    fn decked__fluent_deck_key() {
        assert_eq!(
            Mughal::fluent_deck_key(),
            FLUENT_KEY_BASE_NAME_MUGHAL.to_string()
        );
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent__name() {
        let king_slaves = MughalCard::from_str("KG").unwrap();
        assert_eq!(king_slaves.fluent_rank_name(&FluentName::US_ENGLISH), "King");
        assert_eq!(king_slaves.fluent_suit_name(&FluentName::US_ENGLISH), "Slaves");
        assert_eq!(king_slaves.fluent_name_default(), "King of Slaves");

        // Weak-ladder cards resolve through the same shared rank keys.
        let weak_ace = MughalCard::from_str("AR").unwrap();
        assert_eq!(weak_ace.fluent_name_default(), "Ace of Red Coins");
    }
}
```

- [ ] **Step 2: Register the module and run tests to verify they fail**

In `src/basic/decks.rs`, insert `pub mod mughal;` between `pub mod french;` and `pub mod pinochle;`.
In `src/prelude.rs`, insert `pub use crate::basic::decks::mughal::*;` between `pub use crate::basic::decks::french::*;` (plus the CoPilot comment block that follows it) and `pub use crate::basic::decks::pinochle::*;`.

Run: `cargo test --features full mughal`
Expected: all non-i18n tests PASS; `fluent__name` FAILS (fluent keys unresolved — lookup returns the raw key or panics) because `en-US/mughal.ftl` doesn't exist yet.

- [ ] **Step 3: Write `src/localization/locales/en-US/mughal.ftl`**

```
# Mughal Ganjifa Deck
# Suits
name-suit-mughal-g = Slaves
name-suit-mughal-t = Crowns
name-suit-mughal-s = Swords
name-suit-mughal-r = Red Coins
name-suit-mughal-h = Harps
name-suit-mughal-b = Bills
name-suit-mughal-w = White Coins
name-suit-mughal-q = Cloth
name-suit-mughal-_ = Blank

## Ranks
name-rank-mughal-k = King
name-rank-mughal-v = Vizier
name-rank-mughal-t = Ten
name-rank-mughal-9 = Nine
name-rank-mughal-8 = Eight
name-rank-mughal-7 = Seven
name-rank-mughal-6 = Six
name-rank-mughal-5 = Five
name-rank-mughal-4 = Four
name-rank-mughal-3 = Three
name-rank-mughal-2 = Deuce
name-rank-mughal-a = Ace
name-rank-mughal-_ = Blank
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --features full mughal`
Expected: PASS — 8 tests in `basic__card__mughal_tests` (fluent-templates' `static_loader!` auto-discovers the new `.ftl`; a plain rebuild picks it up).

- [ ] **Step 5: Lint + no_std check**

Run: `cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: clean.
Run: `cargo build --no-default-features && cargo test --no-default-features --lib mughal`
Expected: builds and the non-gated tests pass (catches any ungated `std::`/`HashMap`/`colored` import).

- [ ] **Step 6: Commit point — STOP, suggest to user (do not run)**

```bash
git add src/basic/decks/mughal.rs src/basic/decks.rs src/prelude.rs src/localization/locales/en-US/mughal.ftl
git commit -m "feat: add Mughal Ganjifa deck (96 cards, weak-suit inverted ranking)"
```

---

### Task 3: Dashavatara deck (`src/basic/decks/dashavatara.rs`) + en-US `dashavatara.ftl`

**Files:**
- Create: `src/basic/decks/dashavatara.rs`
- Create: `src/localization/locales/en-US/dashavatara.ftl`
- Modify: `src/basic/decks.rs`, `src/prelude.rs`

**Interfaces:**
- Consumes (Task 1): `ganjifa_deck`, `DashavataraSuit::*`, `FLUENT_KEY_BASE_NAME_DASHAVATARA`, `GanjifaBasicCard::{KING_MATSYA, TEN_KALKI}`.
- Produces: `pub struct Dashavatara {}` implementing `DeckedBase` + `Decked<Self>`; `pub type DashavataraDeck = Pile<Dashavatara>`; `pub type DashavataraCard = Card<Dashavatara>`; `Dashavatara::{DECK_SIZE: usize, SUITS: [Pip; 10], STRONG: [bool; 10], DECK: [BasicCard; 120]}`.

- [ ] **Step 1: Write the deck file with its test battery**

Create `src/basic/decks/dashavatara.rs`:

```rust
use crate::basic::decks::cards::ganjifa::{
    DashavataraSuit, FLUENT_KEY_BASE_NAME_DASHAVATARA, ganjifa_deck,
};
use crate::basic::types::basic_card::BasicCard;
use crate::basic::types::card::Card;
use crate::basic::types::pile::Pile;
use crate::basic::types::pips::Pip;
use crate::basic::types::traits::{Decked, DeckedBase};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "colored-display")]
use colored::Color;
#[cfg(feature = "colored-display")]
use std::collections::HashMap;

/// [Dashavatara Ganjifa](https://en.wikipedia.org/wiki/Ganjifa) — 10 avatar
/// suits × 12 = 120 cards. The five weak suits (Matsya through Vamana) use
/// the inverted pip ladder: `A > 2 > … > 9 > 10`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dashavatara {}
#[allow(clippy::module_name_repetitions)]
pub type DashavataraDeck = Pile<Dashavatara>;
#[allow(clippy::module_name_repetitions)]
pub type DashavataraCard = Card<Dashavatara>;

impl Dashavatara {
    pub const DECK_SIZE: usize = 120;

    /// The ten avatar suits in descending-weight (sorted) order.
    pub const SUITS: [Pip; 10] = [
        DashavataraSuit::MATSYA,
        DashavataraSuit::KURMA,
        DashavataraSuit::VARAHA,
        DashavataraSuit::NARASIMHA,
        DashavataraSuit::VAMANA,
        DashavataraSuit::PARASHURAMA,
        DashavataraSuit::RAMA,
        DashavataraSuit::KRISHNA,
        DashavataraSuit::JAGANNATH,
        DashavataraSuit::KALKI,
    ];

    /// `true` = strong suit (pips 10 high), `false` = weak suit (Ace high).
    /// Parallel to [`Self::SUITS`].
    pub const STRONG: [bool; 10] =
        [false, false, false, false, false, true, true, true, true, true];

    pub const DECK: [BasicCard; Self::DECK_SIZE] = ganjifa_deck(&Self::SUITS, &Self::STRONG);
}

impl DeckedBase for Dashavatara {
    fn base_vec() -> Vec<BasicCard> {
        Self::DECK.to_vec()
    }

    #[cfg(feature = "colored-display")]
    fn colors() -> HashMap<Pip, Color> {
        let mut mappie = HashMap::new();

        mappie.insert(DashavataraSuit::MATSYA, Color::Blue);
        mappie.insert(DashavataraSuit::KURMA, Color::Green);
        mappie.insert(DashavataraSuit::NARASIMHA, Color::Yellow);
        mappie.insert(DashavataraSuit::PARASHURAMA, Color::Red);
        mappie.insert(DashavataraSuit::KRISHNA, Color::BrightBlue);
        mappie.insert(DashavataraSuit::KALKI, Color::BrightBlack);

        mappie
    }

    fn deck_name() -> String {
        "Dashavatara Ganjifa".to_string()
    }

    fn fluent_name_base() -> String {
        FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
    }

    fn fluent_deck_key() -> String {
        FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
    }
}

impl Decked<Self> for Dashavatara {}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__card__dashavatara_tests {
    use super::*;
    use crate::basic::decks::cards::ganjifa::GanjifaBasicCard;
    use crate::basic::types::pile::Pile;
    use crate::basic::types::traits::{Decked, Ranged};
    #[cfg(feature = "i18n")]
    use crate::localization::{FluentName, Named};
    use core::str::FromStr;

    /// The load-bearing test: deck ↔ string round-trip plus
    /// seeded-shuffle → sort == original.
    #[test]
    fn decked__validate() {
        assert!(Dashavatara::validate());
    }

    #[test]
    fn decked__deck() {
        let deck = Dashavatara::deck();
        assert_eq!(deck.len(), 120);
        assert_eq!(
            deck.index(),
            "KM VM AM 2M 3M 4M 5M 6M 7M 8M 9M TM KU VU AU 2U 3U 4U 5U 6U 7U 8U 9U TU KB VB AB 2B 3B 4B 5B 6B 7B 8B 9B TB KN VN AN 2N 3N 4N 5N 6N 7N 8N 9N TN KD VD AD 2D 3D 4D 5D 6D 7D 8D 9D TD KP VP TP 9P 8P 7P 6P 5P 4P 3P 2P AP KR VR TR 9R 8R 7R 6R 5R 4R 3R 2R AR KK VK TK 9K 8K 7K 6K 5K 4K 3K 2K AK KJ VJ TJ 9J 8J 7J 6J 5J 4J 3J 2J AJ KC VC TC 9C 8C 7C 6C 5C 4C 3C 2C AC"
        );
        assert_eq!(
            deck.to_string(),
            "K🐟 V🐟 A🐟 2🐟 3🐟 4🐟 5🐟 6🐟 7🐟 8🐟 9🐟 T🐟 K🐢 V🐢 A🐢 2🐢 3🐢 4🐢 5🐢 6🐢 7🐢 8🐢 9🐢 T🐢 K🐗 V🐗 A🐗 2🐗 3🐗 4🐗 5🐗 6🐗 7🐗 8🐗 9🐗 T🐗 K🦁 V🦁 A🦁 2🦁 3🦁 4🦁 5🦁 6🦁 7🦁 8🦁 9🦁 T🦁 K☂ V☂ A☂ 2☂ 3☂ 4☂ 5☂ 6☂ 7☂ 8☂ 9☂ T☂ K🪓 V🪓 T🪓 9🪓 8🪓 7🪓 6🪓 5🪓 4🪓 3🪓 2🪓 A🪓 K🏹 V🏹 T🏹 9🏹 8🏹 7🏹 6🏹 5🏹 4🏹 3🏹 2🏹 A🏹 K🐄 V🐄 T🐄 9🐄 8🐄 7🐄 6🐄 5🐄 4🐄 3🐄 2🐄 A🐄 K☸ V☸ T☸ 9☸ 8☸ 7☸ 6☸ 5☸ 4☸ 3☸ 2☸ A☸ K🐎 V🐎 T🐎 9🐎 8🐎 7🐎 6🐎 5🐎 4🐎 3🐎 2🐎 A🐎"
        );
    }

    /// Weak/strong boundary: Matsya (weak, Ace high) vs Kalki (strong, Ten high).
    #[test]
    fn weak_suit__inversion() {
        let weak = DashavataraDeck::from_str("TM AM").unwrap().sorted();
        assert_eq!(weak.index(), "AM TM");

        let strong = DashavataraDeck::from_str("AC TC").unwrap().sorted();
        assert_eq!(strong.index(), "TC AC");
    }

    /// 2 courts + 10 strong pips + 10 weak pips = 22 distinct rank `Pip`s.
    #[test]
    fn ranks__distinct_pips() {
        assert_eq!(Dashavatara::deck().ranks().len(), 22);
        assert_eq!(Dashavatara::deck().suits().len(), 10);
    }

    #[cfg(feature = "colored-display")]
    #[test]
    fn decked__colors() {
        assert_eq!(Dashavatara::colors().len(), 6);
    }

    #[test]
    fn decked__deck_name() {
        assert_eq!(Dashavatara::deck_name(), "Dashavatara Ganjifa");
    }

    #[test]
    fn decked__fluent_deck_key() {
        assert_eq!(
            Dashavatara::fluent_deck_key(),
            FLUENT_KEY_BASE_NAME_DASHAVATARA.to_string()
        );
    }

    #[cfg(feature = "i18n")]
    #[test]
    fn fluent__name() {
        let king_matsya = DashavataraCard::from_str("KM").unwrap();
        assert_eq!(king_matsya.fluent_rank_name(&FluentName::US_ENGLISH), "Raja");
        assert_eq!(king_matsya.fluent_suit_name(&FluentName::US_ENGLISH), "Matsya");
        assert_eq!(king_matsya.fluent_name_default(), "Raja of Matsya");

        let weak_ace = DashavataraCard::from_str("AM").unwrap();
        assert_eq!(weak_ace.fluent_name_default(), "Ace of Matsya");
    }
}
```

- [ ] **Step 2: Register the module and run tests to verify they fail**

In `src/basic/decks.rs`, insert `pub mod dashavatara;` between `pub mod cards;` and `pub mod euchre24;`.
In `src/prelude.rs`, insert `pub use crate::basic::decks::dashavatara::*;` between `pub use crate::basic::decks::cards::tarot::*;` and `pub use crate::basic::decks::euchre24::*;`.

Run: `cargo test --features full dashavatara`
Expected: non-i18n tests PASS; `fluent__name` FAILS (no `.ftl` yet).

- [ ] **Step 3: Write `src/localization/locales/en-US/dashavatara.ftl`**

Courts use Raja/Pradhan — the standard terms in English Ganjifa literature. Avatar suit names are Sanskrit proper names.

```
# Dashavatara Ganjifa Deck
# Suits — the ten avatars of Vishnu
name-suit-dashavatara-m = Matsya
name-suit-dashavatara-u = Kurma
name-suit-dashavatara-b = Varaha
name-suit-dashavatara-n = Narasimha
name-suit-dashavatara-d = Vamana
name-suit-dashavatara-p = Parashurama
name-suit-dashavatara-r = Rama
name-suit-dashavatara-k = Krishna
name-suit-dashavatara-j = Jagannath
name-suit-dashavatara-c = Kalki
name-suit-dashavatara-_ = Blank

## Ranks
name-rank-dashavatara-k = Raja
name-rank-dashavatara-v = Pradhan
name-rank-dashavatara-t = Ten
name-rank-dashavatara-9 = Nine
name-rank-dashavatara-8 = Eight
name-rank-dashavatara-7 = Seven
name-rank-dashavatara-6 = Six
name-rank-dashavatara-5 = Five
name-rank-dashavatara-4 = Four
name-rank-dashavatara-3 = Three
name-rank-dashavatara-2 = Deuce
name-rank-dashavatara-a = Ace
name-rank-dashavatara-_ = Blank
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --features full dashavatara`
Expected: PASS — 8 tests in `basic__card__dashavatara_tests`.

- [ ] **Step 5: Lint + no_std check**

Run: `cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic`
Expected: clean.
Run: `cargo build --no-default-features && cargo test --no-default-features --lib dashavatara`
Expected: builds; non-gated tests pass.

- [ ] **Step 6: Commit point — STOP, suggest to user (do not run)**

```bash
git add src/basic/decks/dashavatara.rs src/basic/decks.rs src/prelude.rs src/localization/locales/en-US/dashavatara.ftl
git commit -m "feat: add Dashavatara Ganjifa deck (120 cards)"
```

---

### Task 4: Non-English locales (de, fr, la, tlh) + locale READMEs

**Files:**
- Create: `src/localization/locales/de/mughal.ftl`, `de/dashavatara.ftl`, `fr/mughal.ftl`, `fr/dashavatara.ftl`, `la/mughal.ftl`, `la/dashavatara.ftl`, `tlh/mughal.ftl`, `tlh/dashavatara.ftl`
- Modify: `src/localization/locales/{fr,la,tlh}/README.md` (status tables), `src/basic/decks/mughal.rs` + `src/basic/decks/dashavatara.rs` (extend `fluent__name` with de assertions)

**Interfaces:**
- Consumes: fluent key schema from Tasks 2–3 (`name-suit-mughal-*`, `name-rank-mughal-*`, `name-suit-dashavatara-*`, `name-rank-dashavatara-*`).
- Produces: nothing consumed later — leaf task.

Conventions (from existing files): blank entries are de `Leer`, fr `Vide`, la `Vacuum`, tlh `pagh`. Avatar proper names stay untranslated in every locale. tlh entries each carry a preceding `# attested:` or `# coinage:` annotation (Fluent has no inline comments).

- [ ] **Step 1: Extend the fluent tests (failing first)**

In `src/basic/decks/mughal.rs`, add to the end of `fluent__name`:

```rust
        // German draft locale.
        assert_eq!(king_slaves.fluent_rank_name(&FluentName::DEUTSCH), "König");
        assert_eq!(king_slaves.fluent_suit_name(&FluentName::DEUTSCH), "Sklaven");
```

In `src/basic/decks/dashavatara.rs`, add to the end of `fluent__name`:

```rust
        // German draft locale — avatar proper names stay untranslated.
        assert_eq!(king_matsya.fluent_rank_name(&FluentName::DEUTSCH), "König");
        assert_eq!(king_matsya.fluent_suit_name(&FluentName::DEUTSCH), "Matsya");
```

Run: `cargo test --features full fluent__name`
Expected: both deck `fluent__name` tests FAIL (de lookups fall back to English "King"/"Slaves"/"Raja").

- [ ] **Step 2: Write the two `de/` files**

`src/localization/locales/de/mughal.ftl`:

```
# de/mughal.ftl — German (deutsch) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. Mughal Ganjifa has no German gaming
# tradition; suit names are direct semantic translations of the
# Persian/Mughal originals (Ghulam, Taj, Shamsher, Surkh, Chang, Barat,
# Safed, Qumash). Courts follow German court-card usage: König (King/Mir),
# Wesir (Vizier/Wazir).
#
# Needs: German speaker proofread.

name-suit-mughal-g = Sklaven
name-suit-mughal-t = Kronen
name-suit-mughal-s = Schwerter
name-suit-mughal-r = Rote Münzen
name-suit-mughal-h = Harfen
name-suit-mughal-b = Wechsel
name-suit-mughal-w = Weiße Münzen
name-suit-mughal-q = Stoffe
name-suit-mughal-_ = Leer

name-rank-mughal-k = König
name-rank-mughal-v = Wesir
name-rank-mughal-t = Zehn
name-rank-mughal-9 = Neun
name-rank-mughal-8 = Acht
name-rank-mughal-7 = Sieben
name-rank-mughal-6 = Sechs
name-rank-mughal-5 = Fünf
name-rank-mughal-4 = Vier
name-rank-mughal-3 = Drei
name-rank-mughal-2 = Zwei
name-rank-mughal-a = As
name-rank-mughal-_ = Leer
```

`src/localization/locales/de/dashavatara.ftl`:

```
# de/dashavatara.ftl — German (deutsch) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. Avatar suit names are Sanskrit proper
# names and stay untranslated (matching every other locale). Courts follow
# German usage: König (Raja), Wesir (Pradhan).
#
# Needs: German speaker proofread.

name-suit-dashavatara-m = Matsya
name-suit-dashavatara-u = Kurma
name-suit-dashavatara-b = Varaha
name-suit-dashavatara-n = Narasimha
name-suit-dashavatara-d = Vamana
name-suit-dashavatara-p = Parashurama
name-suit-dashavatara-r = Rama
name-suit-dashavatara-k = Krishna
name-suit-dashavatara-j = Jagannath
name-suit-dashavatara-c = Kalki
name-suit-dashavatara-_ = Leer

name-rank-dashavatara-k = König
name-rank-dashavatara-v = Wesir
name-rank-dashavatara-t = Zehn
name-rank-dashavatara-9 = Neun
name-rank-dashavatara-8 = Acht
name-rank-dashavatara-7 = Sieben
name-rank-dashavatara-6 = Sechs
name-rank-dashavatara-5 = Fünf
name-rank-dashavatara-4 = Vier
name-rank-dashavatara-3 = Drei
name-rank-dashavatara-2 = Zwei
name-rank-dashavatara-a = As
name-rank-dashavatara-_ = Leer
```

- [ ] **Step 3: Write the two `fr/` files**

`src/localization/locales/fr/mughal.ftl`:

```
# fr/mughal.ftl — French (français) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. Mughal Ganjifa has no French gaming
# tradition; suit names are semantic translations of the Persian originals.
# Courts: Roi (King/Mir), Vizir (Wazir) — "Vizir" is the standard French
# spelling for the Ottoman/Persian minister.
#
# Needs: French speaker proofread; Ganjifa literature cross-check.

name-suit-mughal-g = Esclaves
name-suit-mughal-t = Couronnes
name-suit-mughal-s = Épées
name-suit-mughal-r = Pièces Rouges
name-suit-mughal-h = Harpes
name-suit-mughal-b = Lettres de Change
name-suit-mughal-w = Pièces Blanches
name-suit-mughal-q = Étoffes
name-suit-mughal-_ = Vide

name-rank-mughal-k = Roi
name-rank-mughal-v = Vizir
name-rank-mughal-t = Dix
name-rank-mughal-9 = Neuf
name-rank-mughal-8 = Huit
name-rank-mughal-7 = Sept
name-rank-mughal-6 = Six
name-rank-mughal-5 = Cinq
name-rank-mughal-4 = Quatre
name-rank-mughal-3 = Trois
name-rank-mughal-2 = Deux
name-rank-mughal-a = As
name-rank-mughal-_ = Vide
```

`src/localization/locales/fr/dashavatara.ftl`:

```
# fr/dashavatara.ftl — French (français) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. Avatar suit names are Sanskrit proper
# names and stay untranslated. Courts: Roi (Raja), Vizir (Pradhan).
#
# Needs: French speaker proofread.

name-suit-dashavatara-m = Matsya
name-suit-dashavatara-u = Kurma
name-suit-dashavatara-b = Varaha
name-suit-dashavatara-n = Narasimha
name-suit-dashavatara-d = Vamana
name-suit-dashavatara-p = Parashurama
name-suit-dashavatara-r = Rama
name-suit-dashavatara-k = Krishna
name-suit-dashavatara-j = Jagannath
name-suit-dashavatara-c = Kalki
name-suit-dashavatara-_ = Vide

name-rank-dashavatara-k = Roi
name-rank-dashavatara-v = Vizir
name-rank-dashavatara-t = Dix
name-rank-dashavatara-9 = Neuf
name-rank-dashavatara-8 = Huit
name-rank-dashavatara-7 = Sept
name-rank-dashavatara-6 = Six
name-rank-dashavatara-5 = Cinq
name-rank-dashavatara-4 = Quatre
name-rank-dashavatara-3 = Trois
name-rank-dashavatara-2 = Deux
name-rank-dashavatara-a = As
name-rank-dashavatara-_ = Vide
```

- [ ] **Step 4: Write the two `la/` files**

`src/localization/locales/la/mughal.ftl`:

```
# la/mughal.ftl — Latin (Latina) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. There is no Latin Ganjifa tradition;
# suit names are semantic classical Latin (plural nouns, matching the
# medieval gaming-Latin convention of la/french.ftl and la/tarot.ftl).
# Courts: Rex (King/Mir), Praefectus (Vizier/Wazir — "governor/minister",
# positional semantics).
#
# Needs: Latinist proofread (Syngraphae "written bonds" for Bills is the
# most speculative choice).

name-suit-mughal-g = Servi
name-suit-mughal-t = Coronae
name-suit-mughal-s = Gladii
name-suit-mughal-r = Nummi Rubri
name-suit-mughal-h = Citharae
name-suit-mughal-b = Syngraphae
name-suit-mughal-w = Nummi Albi
name-suit-mughal-q = Panni
name-suit-mughal-_ = Vacuum

name-rank-mughal-k = Rex
name-rank-mughal-v = Praefectus
name-rank-mughal-t = Decem
name-rank-mughal-9 = Novem
name-rank-mughal-8 = Octo
name-rank-mughal-7 = Septem
name-rank-mughal-6 = Sex
name-rank-mughal-5 = Quinque
name-rank-mughal-4 = Quattuor
name-rank-mughal-3 = Tres
name-rank-mughal-2 = Duo
name-rank-mughal-a = As
name-rank-mughal-_ = Vacuum
```

`src/localization/locales/la/dashavatara.ftl`:

```
# la/dashavatara.ftl — Latin (Latina) translation
#
# Status: DRAFT — MEDIUM CONFIDENCE. Avatar suit names are Sanskrit proper
# names and stay untranslated. Courts: Rex (Raja), Praefectus (Pradhan).
#
# Needs: Latinist proofread.

name-suit-dashavatara-m = Matsya
name-suit-dashavatara-u = Kurma
name-suit-dashavatara-b = Varaha
name-suit-dashavatara-n = Narasimha
name-suit-dashavatara-d = Vamana
name-suit-dashavatara-p = Parashurama
name-suit-dashavatara-r = Rama
name-suit-dashavatara-k = Krishna
name-suit-dashavatara-j = Jagannath
name-suit-dashavatara-c = Kalki
name-suit-dashavatara-_ = Vacuum

name-rank-dashavatara-k = Rex
name-rank-dashavatara-v = Praefectus
name-rank-dashavatara-t = Decem
name-rank-dashavatara-9 = Novem
name-rank-dashavatara-8 = Octo
name-rank-dashavatara-7 = Septem
name-rank-dashavatara-6 = Sex
name-rank-dashavatara-5 = Quinque
name-rank-dashavatara-4 = Quattuor
name-rank-dashavatara-3 = Tres
name-rank-dashavatara-2 = Duo
name-rank-dashavatara-a = As
name-rank-dashavatara-_ = Vacuum
```

- [ ] **Step 5: Write the two `tlh/` files**

`src/localization/locales/tlh/mughal.ftl` (every entry annotated `# attested` / `# coinage`, matching tlh convention):

```
# tlh/mughal.ftl — Klingon (tlhIngan Hol) translation
#
# Status: DRAFT — LOW CONFIDENCE. Mughal Ganjifa's Persian court culture has
# no Klingon analog. A few suits land on attested vocabulary (toy'wI'
# "servant", yan "sword", Sut "clothing"); the rest are marked coinages
# following Klingon noun-noun compound order.
#
# Notation: same as tlh/french.ftl (# attested / # coinage).
#
# Needs: KLI-savvy reviewer.

# attested: "servant" (TKD); for Slaves (Ghulam)
name-suit-mughal-g = toy'wI'
# coinage: "great helmet" (mIv "helmet" TKD + -'a' augmentative); for Crowns (Taj)
name-suit-mughal-t = mIv'a'
# attested: "sword" (TKD); for Swords (Shamsher)
name-suit-mughal-s = yan
# coinage: "red money" (Huch "money" TKD + Doq "be red/orange" TKD); for Red Coins (Surkh)
name-suit-mughal-r = Huch Doq
# coinage: "music device" (QoQ "music" TKD + jan "device" TKD); for Harps (Chang)
name-suit-mughal-h = QoQ jan
# coinage: "paper money" (nav "paper" TKD + Huch "money" TKD); for Bills (Barat)
name-suit-mughal-b = nav Huch
# coinage: "white money" (Huch "money" TKD + chIS "be white" TKD); for White Coins (Safed)
name-suit-mughal-w = Huch chIS
# attested: "clothing" (TKD); for Cloth (Qumash)
name-suit-mughal-q = Sut
# attested: "zero, nothing" (TKD)
name-suit-mughal-_ = pagh

# attested: "emperor" (TKD); for King (Mir)
name-rank-mughal-k = ta'
# attested: "chancellor" (TKD); for Vizier (Wazir)
name-rank-mughal-v = Qang
# attested: "ten" (TKD)
name-rank-mughal-t = wa'maH
# attested: "nine" (TKD)
name-rank-mughal-9 = Hut
# attested: "eight" (TKD)
name-rank-mughal-8 = chorgh
# attested: "seven" (TKD)
name-rank-mughal-7 = Soch
# attested: "six" (TKD)
name-rank-mughal-6 = jav
# attested: "five" (TKD)
name-rank-mughal-5 = vagh
# attested: "four" (TKD)
name-rank-mughal-4 = loS
# attested: "three" (TKD)
name-rank-mughal-3 = wej
# attested: "two" (TKD)
name-rank-mughal-2 = cha'
# attested: "first" ordinal (TKD); for Ace
name-rank-mughal-a = wa'DIch
# attested: "zero, nothing" (TKD)
name-rank-mughal-_ = pagh
```

`src/localization/locales/tlh/dashavatara.ftl`:

```
# tlh/dashavatara.ftl — Klingon (tlhIngan Hol) translation
#
# Status: DRAFT — LOW CONFIDENCE on courts; suit names are Sanskrit proper
# names and stay untranslated in every locale (proper nouns are not
# transliterated into Klingon here).
#
# Notation: same as tlh/french.ftl (# attested / # coinage).
#
# Needs: KLI-savvy reviewer.

# proper name (Sanskrit): the fish avatar
name-suit-dashavatara-m = Matsya
# proper name (Sanskrit): the turtle avatar
name-suit-dashavatara-u = Kurma
# proper name (Sanskrit): the boar avatar
name-suit-dashavatara-b = Varaha
# proper name (Sanskrit): the lion-man avatar
name-suit-dashavatara-n = Narasimha
# proper name (Sanskrit): the dwarf avatar
name-suit-dashavatara-d = Vamana
# proper name (Sanskrit): the axe-bearer avatar
name-suit-dashavatara-p = Parashurama
# proper name (Sanskrit): the bow-bearer avatar
name-suit-dashavatara-r = Rama
# proper name (Sanskrit): the cowherd avatar
name-suit-dashavatara-k = Krishna
# proper name (Sanskrit): lord of the world
name-suit-dashavatara-j = Jagannath
# proper name (Sanskrit): the horse avatar
name-suit-dashavatara-c = Kalki
# attested: "zero, nothing" (TKD)
name-suit-dashavatara-_ = pagh

# attested: "emperor" (TKD); for Raja
name-rank-dashavatara-k = ta'
# attested: "chancellor" (TKD); for Pradhan
name-rank-dashavatara-v = Qang
# attested: "ten" (TKD)
name-rank-dashavatara-t = wa'maH
# attested: "nine" (TKD)
name-rank-dashavatara-9 = Hut
# attested: "eight" (TKD)
name-rank-dashavatara-8 = chorgh
# attested: "seven" (TKD)
name-rank-dashavatara-7 = Soch
# attested: "six" (TKD)
name-rank-dashavatara-6 = jav
# attested: "five" (TKD)
name-rank-dashavatara-5 = vagh
# attested: "four" (TKD)
name-rank-dashavatara-4 = loS
# attested: "three" (TKD)
name-rank-dashavatara-3 = wej
# attested: "two" (TKD)
name-rank-dashavatara-2 = cha'
# attested: "first" ordinal (TKD); for Ace
name-rank-dashavatara-a = wa'DIch
# attested: "zero, nothing" (TKD)
name-rank-dashavatara-_ = pagh
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test --features full fluent`
Expected: PASS — including both extended `fluent__name` tests.

- [ ] **Step 7: Update the three locale README status tables**

Add two rows to each table (keep each README's existing column format):

`src/localization/locales/fr/README.md` — after the `skat.ftl` row:

```markdown
| `mughal.ftl` | DRAFT | Medium (~70%) | French speaker; Ganjifa terminology cross-check |
| `dashavatara.ftl` | DRAFT | Medium (~75%) | French speaker (avatar names stay Sanskrit) |
```

`src/localization/locales/la/README.md` — after the `skat.ftl` row:

```markdown
| `mughal.ftl` | DRAFT | Medium (~65%) | Latinist proofread (Syngraphae for Bills is speculative) |
| `dashavatara.ftl` | DRAFT | Medium (~75%) | Latinist proofread (avatar names stay Sanskrit) |
```

`src/localization/locales/tlh/README.md` — after the `tarot.ftl` row:

```markdown
| `mughal.ftl` | DRAFT | Low (~35%) | KLI-savvy reviewer |
| `dashavatara.ftl` | DRAFT | Low (~40%) | KLI-savvy reviewer (avatar names stay Sanskrit) |
```

- [ ] **Step 8: Commit point — STOP, suggest to user (do not run)**

```bash
git add src/localization/locales src/basic/decks/mughal.rs src/basic/decks/dashavatara.rs
git commit -m "feat: localize Ganjifa decks in de, fr, la, and tlh"
```

---

### Task 5: Registry integration (`src/basic/decks/registry.rs`)

**Files:**
- Modify: `src/basic/decks/registry.rs`

**Interfaces:**
- Consumes (Tasks 2–3): `Mughal`, `Dashavatara` deck types.
- Produces: `DeckKind::Dashavatara`, `DeckKind::Mughal` variants dispatching to the typed decks.

- [ ] **Step 1: Rename + extend the key test and add registry assertions (failing first)**

In `src/basic/decks/registry.rs` tests, replace `fluent_deck_key__is_one_of_three` with:

```rust
    #[test]
    fn fluent_deck_key__is_one_of_five() {
        for kind in DeckKind::all() {
            let key = kind.fluent_deck_key();
            assert!(
                key == "dashavatara"
                    || key == "french"
                    || key == "mughal"
                    || key == "skat"
                    || key == "tarot",
                "{kind:?} returned unexpected fluent_deck_key {key:?}"
            );
        }
    }
```

And add to `base_vec__matches_typed_deck`:

```rust
        assert_eq!(DeckKind::Mughal.base_vec(), Mughal::base_vec());
        assert_eq!(DeckKind::Dashavatara.base_vec(), Dashavatara::base_vec());
```

Run: `cargo test --features full registry`
Expected: COMPILE ERROR — `DeckKind::Mughal` / `DeckKind::Dashavatara` don't exist yet.

- [ ] **Step 2: Add the variants and arms**

All additions alphabetical. In the imports block add:

```rust
use crate::basic::decks::dashavatara::Dashavatara;
use crate::basic::decks::mughal::Mughal;
```

(`dashavatara` sorts after `canasta`, `mughal` after `french`.)

Enum — insert `Dashavatara,` after `Canasta,` and `Mughal,` after `French,`:

```rust
pub enum DeckKind {
    Canasta,
    Dashavatara,
    Euchre24,
    Euchre32,
    French,
    Mughal,
    Pinochle,
    #[cfg(feature = "yaml")]
    Razz,
    Short,
    Skat,
    Spades,
    Standard52,
    Tarot,
    Tiny,
}
```

In `all()` insert `Self::Dashavatara,` after `Self::Canasta,` and `Self::Mughal,` after `Self::French,`. Update its doc comment line to:

```rust
    /// The slice length is 14 with `yaml` (the default) and 13 without.
```

In each of `deck_name()`, `base_vec()`, `fluent_deck_key()`, and `demo()`, insert matching arms in the same two positions, e.g. for `deck_name()`:

```rust
            Self::Dashavatara => Dashavatara::deck_name(),
```
after the `Canasta` arm, and
```rust
            Self::Mughal => Mughal::deck_name(),
```
after the `French` arm (same pattern with `base_vec()`, `fluent_deck_key()`, `demo(verbose)`).

Update the `fluent_deck_key()` doc comment:

```rust
    /// All decks share one of five keys: `dashavatara`, `french`, `mughal`,
    /// `skat`, or `tarot`.
```

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test --features full registry`
Expected: PASS — all `basic__decks__registry_tests`, including `fluent_deck_key__is_one_of_five` and the extended `base_vec__matches_typed_deck`.

Also run: `cargo test --no-default-features --lib registry`
Expected: PASS (registry is feature-clean; new arms compile without `yaml`/`i18n`).

- [ ] **Step 4: Commit point — STOP, suggest to user (do not run)**

```bash
git add src/basic/decks/registry.rs
git commit -m "feat: register Mughal and Dashavatara in DeckKind"
```

---

### Task 6: Docs, demo flags, and OKF bundle

**Files:**
- Modify: `CHANGELOG.md`, `README.md`, `examples/demo.rs`
- Modify: `.okf/index.md`, `.okf/decks/index.md`, `.okf/decks/deck-catalog.md`, `.okf/architecture/localization.md`, `.okf/log.md`

**Interfaces:**
- Consumes: `DeckKind::Mughal` / `DeckKind::Dashavatara` (Task 5).
- Produces: nothing consumed later — leaf task.

- [ ] **Step 1: `examples/demo.rs` — long-only flags**

Add to the `Args` struct after the `tarot` field (short `-m` is taken by `short`, `-d` would be ambiguous — long-only):

```rust
    /// Mughal Ganjifa (long-only; -m is taken by --short).
    #[clap(long)]
    mughal: bool,

    /// Dashavatara Ganjifa (long-only).
    #[clap(long)]
    dashavatara: bool,
```

Add to `main()` after the `args.standard` block:

```rust
    if args.mughal {
        DeckKind::Mughal.demo(args.verbose);
    }

    if args.dashavatara {
        DeckKind::Dashavatara.demo(args.verbose);
    }
```

Run: `cargo run --features full --example demo -- --mughal --dashavatara -v`
Expected: both decks print; weak suits read `K V A 2 3 4 5 6 7 8 9 T`; names resolve (no raw `name-rank-…` fallback keys visible).

- [ ] **Step 2: `CHANGELOG.md`**

Under `## [Unreleased]` add:

```markdown
### Added

- **Ganjifa decks** — two traditional Indian/Persian playing-card decks
  ([EPIC-02](docs/EPIC-02_Ganjifa.md)) with the signature per-suit inverted
  pip ranking (weak suits rank `A > 2 > … > 10`):
  - `Mughal` (8 suits × 12 = 96 cards) and `Dashavatara` (10 avatar suits
    × 12 = 120 cards), built from a shared `cards::ganjifa` vocabulary via
    a `const fn` deck builder — no core-type changes.
  - `DeckKind::Mughal` / `DeckKind::Dashavatara` registry variants.
  - Fluent localization in all 5 locales (en-US, de, fr, la, tlh); the
    non-English files follow the draft/confidence-header convention.
  - `--mughal` / `--dashavatara` flags on `examples/demo.rs`.
```

- [ ] **Step 3: `README.md`**

In the "Out of the box" deck list, insert after the `French Deck` sub-list (i.e., between the `Euchre` line and `Short Deck`):

```markdown
* [Ganjifa](https://en.wikipedia.org/wiki/Ganjifa) with per-suit inverted pip ranking
  * Mughal (8 suits × 12 = 96 cards)
  * Dashavatara (10 suits × 12 = 120 cards)
```

Change the "Other decks in the demo program" sentence to:

```markdown
Other decks in the demo program are `canasta`, `euchre`, `short`, `pinochle`, `skat`, `spades`,
`standard`, `tarot`, `mughal`, and `dashavatara`.
```

- [ ] **Step 4: OKF bundle**

1. `.okf/index.md`: change `the 12 shipped deck kinds` → `the 14 shipped deck kinds`.
2. `.okf/decks/index.md`: change `The 12 deck kinds the crate ships` → `The 14 deck kinds the crate ships`.
3. `.okf/decks/deck-catalog.md`:
   - frontmatter `description`: `The 12 deck kinds` → `The 14 deck kinds`; bump `timestamp` to the current UTC time.
   - heading `# The 12 deck kinds` → `# The 14 deck kinds`.
   - `runtime enum over all 12` → `runtime enum over all 14`.
   - add two table rows after `Tarot`:

```markdown
| `Mughal` | 96 | Mughal Ganjifa: 8 suits × 12; weak suits (Red Coins, Harps, Bills, Cloth) use **inverted pip ranking** (A > 2 > … > 10) via a second weight-inverted rank ladder |
| `Dashavatara` | 120 | Dashavatara Ganjifa: 10 avatar suits × 12; Matsya–Vamana weak (inverted pips), Parashurama–Kalki strong; shares the `cards::ganjifa` vocabulary with Mughal |
```

4. `.okf/architecture/localization.md`: change `` (`french`, `skat`, `tarot`) `` → `` (`french`, `skat`, `tarot`, `mughal`, `dashavatara`) ``; bump `timestamp`.
5. `.okf/log.md`: append under a dated heading for today:

```markdown
* **Decks**: Added Mughal (96) and Dashavatara (120) Ganjifa decks — [deck catalog](decks/deck-catalog.md) now covers 14 kinds; [localization](architecture/localization.md) gains the `mughal`/`dashavatara` fluent bases (EPIC-02).
```

6. Validate: run `/okf:validate .okf --strict` (okf skill). Expected: conformant.

- [ ] **Step 5: Format + docs build**

Run: `cargo fmt --all`
Run: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
Expected: both clean.

- [ ] **Step 6: Update the EPIC checkboxes**

Tick the completed `- [ ]` items in `docs/EPIC-02_Ganjifa.md` Stories 1–6 (leave the Verification matrix for Task 7).

- [ ] **Step 7: Commit point — STOP, suggest to user (do not run)**

```bash
git add CHANGELOG.md README.md examples/demo.rs .okf docs/EPIC-02_Ganjifa.md
git commit -m "docs: changelog, README, demo flags, and OKF bundle for Ganjifa decks"
```

---

### Task 7: Full verification matrix

**Files:** none created — this is the EPIC's release gate. Fix-forward anything red (and if a fix touches earlier tasks' files, re-run that task's tests before returning here).

- [ ] `cargo test --features full` — all unit + integration tests green
- [ ] `cargo test --doc --features full` — doctests green (includes the new `ganjifa_deck` doctest)
- [ ] `cargo clippy -- -Dclippy::all -Dclippy::pedantic` — clean (default features)
- [ ] `cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic` — clean
- [ ] `cargo fmt --all -- --check` — clean
- [ ] `cargo build --no-default-features` — clean
- [ ] `cargo build --no-default-features --features serde` — clean
- [ ] `cargo test --no-default-features --lib` — green (catches ungated `std::`/`HashMap`/`colored` in the new files)
- [ ] `cargo build --target wasm32-unknown-unknown --all-features` — clean (32-bit CKC shift path; max shift 27)
- [ ] `cargo build --target wasm32-unknown-unknown --no-default-features` — clean
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` — clean
- [ ] Manual smoke: `cargo run --features full --example demo -- --all -v` — visually confirm both Ganjifa decks appear, weak-suit inverted order, resolved fluent names in all locales
- [ ] Tick the Verification matrix checkboxes in `docs/EPIC-02_Ganjifa.md`
- [ ] **Final commit point — STOP, suggest to user (do not run):**

```bash
git add docs/EPIC-02_Ganjifa.md
git commit -m "docs: tick EPIC-02 verification matrix"
```
