# EPIC-02: Ganjifa Decks (Mughal + Dashavatara)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Work story-by-story; "default features green" (`cargo test --all`) is a precondition for every story — if it goes red mid-story, stop and diagnose before moving on.

**Goal:** Add two Ganjifa deck types — [traditional Indian/Persian playing cards](https://en.wikipedia.org/wiki/Ganjifa) — as first-class decks: **Mughal Ganjifa** (8 suits × 12 = 96 cards) and **Dashavatara Ganjifa** (10 suits × 12 = 120 cards), including the signature per-suit inverted pip ranking, full Fluent localization in all 5 locales, and registry integration.

**Architecture:** No core-type changes. Each `BasicCard` carries its own rank `Pip` with its own `weight` (`src/basic/types/pips.rs:79`, `src/basic/types/basic_card.rs:36`), so "weak" suits simply pair with a second, inverted-weight rank ladder — the same dual-ladder trick Tarot uses for Major/Minor Arcana (`src/basic/decks/cards/tarot.rs`). One shared vocabulary file (`cards/ganjifa.rs`) with a `const fn` deck builder serves two thin deck files (`mughal.rs`, `dashavatara.rs`), mirroring how `cards/french.rs` serves Standard52/Short/etc.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc discipline (default-on `std` feature), fluent-templates i18n, GitHub Actions CI with clippy-pedantic, no_std, and wasm32 matrix jobs.

---

## Domain

Each Ganjifa suit has 12 cards: 2 courts — **King** (Mir/Raja) and **Vizier** (Wazir/Pradhan) — plus pips 10 → 1 (Ace). The defining rule:

- **Strong suits:** pips rank `10 > 9 > … > 2 > A`.
- **Weak suits:** pips rank **inverted**: `A > 2 > … > 9 > 10`.
- Courts always outrank pips in every suit: `K > V > (pips)`.

| Deck | Strong suits | Weak suits |
|---|---|---|
| Mughal | Slaves, Crowns, Swords, White Coins | Red Coins, Harps, Bills, Cloth |
| Dashavatara | Parashurama, Rama, Krishna, Jagannath, Kalki | Matsya, Kurma, Varaha, Narasimha, Vamana |

---

## Design decisions (settled)

1. **One shared vocabulary file, two deck files.** `src/basic/decks/cards/ganjifa.rs` + `src/basic/decks/mughal.rs` + `src/basic/decks/dashavatara.rs`.
2. **Separate fluent bases `"mughal"` and `"dashavatara"`** (not a shared `"ganjifa"`). Suit fluent keys are `name-suit-{base}-{suit.index}` (`src/basic/types/card.rs:264-305`); cross-deck suit index-char collisions (R, B, K…) make a shared base ambiguous. Matches the one-base-per-family precedent (french/skat/tarot).
3. **Rank index chars shared between strong and weak ladders:** both use `K V T 9 8 7 6 5 4 3 2 A`. Card parsing identity is the full `(rank.index, suit.index)` pair matched against the deck's own `base_vec()` (`card.rs:388-401`), so duplicate rank chars across suits are unambiguous — and one set of rank fluent keys serves both ladders.
4. **Const-fn deck builder** instead of 216 hand-written card consts. `Pip::new` (`pips.rs:118`) and `BasicCard::new` (`basic_card.rs:58`) are both `const fn`; while-loops and array indexing are const-legal. Pips needing non-zero `value` use const struct literals (precedent: `french.rs:268`).
5. **Symbols follow the Tarot convention:** suits get single-scalar emoji `symbol`s with uppercase ASCII `index` chars; ranks stay ASCII (index == symbol).

---

## Story 1: Card vocabulary (`src/basic/decks/cards/ganjifa.rs`)

**Acceptance:** vocabulary compiles in const context; ladder ordering and index-uniqueness tests pass.

**Files:**
- Create: `src/basic/decks/cards/ganjifa.rs`
- Modify: `src/basic/decks/cards.rs` (add `pub mod ganjifa;`)

### Rank ladders (`GanjifaRank`) — weights 11 → 0

Courts (shared consts across both ladders):

| Const | weight | index/symbol | value |
|---|---|---|---|
| `KING` | 11 | `K` | 12 |
| `VIZIER` | 10 | `V` | 11 |

Strong pips: `TEN` {w:9, `T`, value:10} down to `DEUCE` {w:1, `2`, value:2}, `ACE` {w:0, `A`, value:1}.

Weak pips (same index/symbol/value, inverted weights): `WEAK_ACE` {w:9, `A`, value:1} … `WEAK_NINE` {w:1, `9`, value:9}, `WEAK_TEN` {w:0, `T`, value:10}.

Ladder arrays in descending-weight order:

```rust
pub const STRONG: [Pip; 12] = [KING, VIZIER, TEN, NINE, /* … */ DEUCE, ACE];
pub const WEAK:   [Pip; 12] = [KING, VIZIER, WEAK_ACE, WEAK_DEUCE, /* … */ WEAK_NINE, WEAK_TEN];
```

Weight 11 max keeps the CKC shift at `16 + 11 = 27 < 32` — safe on wasm32.

### Mughal suits (`MughalSuit`) — all `PipType::Suit`, `value = weight + 1`

| Const | weight | index | symbol | ladder |
|---|---|---|---|---|
| `SLAVES` (Ghulam) | 7 | `G` | 👤 | strong |
| `CROWNS` (Taj) | 6 | `T` | 👑 | strong |
| `SWORDS` (Shamsher) | 5 | `S` | ⚔ | strong |
| `RED_COINS` (Surkh) | 4 | `R` | 🔴 | weak |
| `HARPS` (Chang) | 3 | `H` | 🎵 | weak |
| `BILLS` (Barat) | 2 | `B` | 📜 | weak |
| `WHITE_COINS` (Safed) | 1 | `W` | ⚪ | strong |
| `CLOTH` (Qumash) | 0 | `Q` | 🧵 | weak |

### Dashavatara suits (`DashavataraSuit`)

| Const | weight | index | symbol | ladder |
|---|---|---|---|---|
| `MATSYA` (fish) | 9 | `M` | 🐟 | weak |
| `KURMA` (turtle) | 8 | `U` | 🐢 | weak |
| `VARAHA` (boar) | 7 | `B` | 🐗 | weak |
| `NARASIMHA` (lion-man) | 6 | `N` | 🦁 | weak |
| `VAMANA` (dwarf) | 5 | `D` | ☂ | weak |
| `PARASHURAMA` (axe) | 4 | `P` | 🪓 | strong |
| `RAMA` (bow) | 3 | `R` | 🏹 | strong |
| `KRISHNA` (cowherd) | 2 | `K` | 🐄 | strong |
| `JAGANNATH` (Buddha) | 1 | `J` | ☸ | strong |
| `KALKI` (horse) | 0 | `C` | 🐎 | strong |

Constraints: every `symbol` must be a **single Unicode scalar** (`Pip.symbol: char` — no ZWJ sequences; avoid Unicode-16 additions like 🪉). Every `index` must be uppercase/digit (`from_str` uppercases input). Suit `T`/`K`/`V` coexisting with rank `T`/`K`/`V` is pair-safe (e.g. `TT` = Ten of Crowns, `KK` = King of Krishna).

### Const deck builder

```rust
/// Builds a Ganjifa deck: for each suit, courts + 10 pips from the
/// strong or weak ladder, in descending-weight (sorted) order.
#[must_use]
pub const fn ganjifa_deck<const S: usize, const N: usize>(
    suits: &[Pip; S],
    strong: &[bool; S],
) -> [BasicCard; N] {
    assert!(N == S * 12);
    let mut deck = [BasicCard::new(Pip::new(PipType::Blank, 0, '_', '_'),
                                   Pip::new(PipType::Blank, 0, '_', '_')); N];
    let mut i = 0;
    while i < S {
        let ranks = if strong[i] { &GanjifaRank::STRONG } else { &GanjifaRank::WEAK };
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

Because ladders and suit arrays are in descending-weight order, the output equals `sorted()` order — which `validate()` requires.

Also export: `pub const FLUENT_KEY_BASE_NAME_MUGHAL: &str = "mughal";`, `pub const FLUENT_KEY_BASE_NAME_DASHAVATARA: &str = "dashavatara";`. Optionally a handful of named `BasicCard` consts for tests/docs (e.g. `KING_SLAVES`, `WEAK_ACE_RED_COINS`) — not all 216.

### Tasks

- [ ] Write `GanjifaRank` courts + strong + weak pip consts and `STRONG`/`WEAK` ladder arrays
- [ ] Write `MughalSuit` and `DashavataraSuit` consts per the tables above
- [ ] Write `ganjifa_deck` const builder + fluent key base consts
- [ ] Tests: const-guard (`const _GUARD: [BasicCard; 96] = ganjifa_deck(...)`), ladder weights strictly descending, suit/rank index-char uniqueness sweeps
- [ ] Register `pub mod ganjifa;` in `src/basic/decks/cards.rs`; `cargo check`

---

## Story 2: Mughal deck (`src/basic/decks/mughal.rs`)

**Acceptance:** `Mughal::validate()` passes; full test battery green.

**Files:**
- Create: `src/basic/decks/mughal.rs` (clone the `skat.rs` shape)
- Modify: `src/basic/decks.rs`, `src/prelude.rs`

```rust
pub struct Mughal {}   // derives: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
pub type MughalDeck = Pile<Mughal>;
pub type MughalCard = Card<Mughal>;

impl Mughal {
    pub const DECK_SIZE: usize = 96;
    pub const SUITS: [Pip; 8] = [SLAVES, CROWNS, SWORDS, RED_COINS, HARPS, BILLS, WHITE_COINS, CLOTH];
    pub const STRONG: [bool; 8] = [true, true, true, false, false, false, true, false];
    pub const DECK: [BasicCard; Self::DECK_SIZE] = ganjifa_deck(&Self::SUITS, &Self::STRONG);
}
```

`impl DeckedBase for Mughal`:
- `base_vec()` → `Self::DECK.to_vec()`
- `deck_name()` → `"Mughal Ganjifa"`
- `fluent_name_base()` and `fluent_deck_key()` → `FLUENT_KEY_BASE_NAME_MUGHAL` (keep identical, skat/tarot precedent)
- `colors()` behind `#[cfg(feature = "colored-display")]`: RED_COINS→Red, CROWNS→Yellow, SWORDS→BrightBlue, HARPS→Magenta, BILLS→Cyan, CLOTH→Green (rest default white)

`impl Decked<Self> for Mughal {}`

**no_std discipline** (copy `skat.rs:1-15` imports exactly): `alloc::string::{String, ToString}`, `alloc::vec::Vec`; gate `std::collections::HashMap` / `colored::Color` / color-related `Pip` imports behind `#[cfg(feature = "colored-display")]`.

### Tasks

- [ ] Marker struct, type aliases, `DECK_SIZE`/`SUITS`/`STRONG`/`DECK` consts
- [ ] `DeckedBase` + `Decked` impls with no_std-clean imports
- [ ] `pub mod mughal;` in `src/basic/decks.rs` (alphabetical); prelude re-exports for `cards::ganjifa::*` and `mughal::*`
- [ ] Test battery (`mod basic__card__mughal_tests`, mirroring `skat.rs:94-156`):
  - `decked__validate` — **the load-bearing test** (round-trips deck↔string; seeded-shuffle→sort == original)
  - `decked__deck` — full `deck.index()` string (strong suits read `K V T 9 8 7 6 5 4 3 2 A`, weak suits `K V A 2 3 4 5 6 7 8 9 T`), `to_string()` with emoji, `len() == 96`
  - `weak_suit__inversion` — domain test via `sorted()` output: `MughalDeck::from_str("TR AR")` sorts Ace first (weak suit), `"AG TG"` sorts Ten first (strong suit). NB: `BasicCard::Ord` is inverted (highest first, `basic_card.rs:179`) — assert through `sorted()` strings or `rank.weight`, never `<`/`>` intuition
  - `decked__colors` (cfg colored-display), `decked__deck_name`, `decked__fluent_deck_key`

---

## Story 3: Dashavatara deck (`src/basic/decks/dashavatara.rs`)

**Acceptance:** `Dashavatara::validate()` passes; full test battery green.

Twin of Story 2: `DECK_SIZE: 120`, `SUITS: [Pip; 10]` (Matsya → Kalki), `STRONG: [bool; 10]` = `[false, false, false, false, false, true, true, true, true, true]`, `deck_name()` → `"Dashavatara Ganjifa"`, fluent base `"dashavatara"`. Colors: MATSYA→Blue, KURMA→Green, NARASIMHA→Yellow, PARASHURAMA→Red, KRISHNA→BrightBlue, KALKI→BrightBlack (rest default).

### Tasks

- [ ] Deck file with consts, impls, no_std-clean imports
- [ ] `pub mod dashavatara;` in `src/basic/decks.rs`; prelude re-export
- [ ] Test battery as Story 2, with 120-card index string and weak/strong boundary check (Matsya weak, Kalki strong)

---

## Story 4: Localization (10 new `.ftl` files)

**Acceptance:** `fluent__name` tests pass in both deck files; `cargo run --example demo` shows resolved names, no fallback keys.

**Files:**
- Create: `mughal.ftl` + `dashavatara.ftl` under each of `src/localization/locales/{en-US,de,fr,la,tlh}/`
- Modify: locale `README.md` status tables where they exist (fr, la, tlh)

The `static_loader!` (`src/localization.rs:8`) auto-discovers `.ftl` files — no code registration. Format per `en-US/tarot.ftl`; every base needs `-_` Blank entries for both suit and rank. Ganjifa uses only `PipType::Suit`, so the `-special-` key path never fires.

en-US `mughal.ftl` keys: suits `name-suit-mughal-{g,t,s,r,h,b,w,q}` = Slaves/Crowns/Swords/Red Coins/Harps/Bills/White Coins/Cloth; ranks `name-rank-mughal-{k,v,t,9…2,a}` = King/Vizier/Ten…Deuce/Ace.

en-US `dashavatara.ftl`: suits = avatar proper names keyed `{m,u,b,n,d,p,r,k,j,c}`; courts **Raja/Pradhan** (standard terms in English ganjifa literature).

Non-English locales follow the DRAFT-header convention (see `tlh/tarot.ftl`); avatar proper names stay untranslated everywhere. Courts: de König/Wesir, fr Roi/Vizir, la Rex/Praefectus, tlh coinages with `# coinage` annotations.

### Tasks

- [ ] en-US `mughal.ftl` + `dashavatara.ftl` (write these first — Story 2/3 `fluent__name` tests depend on them)
- [ ] de, fr, la, tlh files with DRAFT headers
- [ ] `fluent__name` tests (cfg i18n): `"KG"` → "King of Slaves"; `"AR"` → "Ace of Red Coins" (proves the weak ladder resolves shared rank keys); `"KM"` → "Raja of Matsya"
- [ ] Update fr/la/tlh locale README status tables

---

## Story 5: Registry integration (`src/basic/decks/registry.rs`)

**Acceptance:** registry tests green, including the renamed key test.

- [ ] Add `DeckKind::Dashavatara` and `DeckKind::Mughal` variants (alphabetical order among existing variants)
- [ ] Add arms in `all()`, `deck_name()`, `base_vec()`, `fluent_deck_key()`, `demo()`
- [ ] Update `all()` doc comment slice counts (12 with yaml / 11 without → 14/13)
- [ ] Rename test `fluent_deck_key__is_one_of_three` (`registry.rs:238-246`) → `fluent_deck_key__is_one_of_five`, accepting `"mughal"` and `"dashavatara"` — **same commit as the variants** (the old test fails the moment they land)

---

## Story 6: Docs + polish

- [ ] `CHANGELOG.md` — `[Unreleased] ### Added` entry (Keep-a-Changelog format; purely additive → minor version, `cargo-semver-checks` unaffected)
- [ ] README deck list additions
- [ ] Optional: `examples/demo.rs` flags for the new decks (long-only flags; short `-m` is taken by `short`)
- [ ] `cargo fmt`

---

## Verification matrix (from Makefile / CI)

- [ ] `cargo test --all` (or `cargo nextest run`)
- [ ] `cargo clippy -- -Dclippy::all -Dclippy::pedantic` (CI denies pedantic)
- [ ] `cargo fmt --all -- --check`
- [ ] no_std: `cargo build --no-default-features` · `cargo build --no-default-features --features serde` · `cargo test --no-default-features --lib` (catches ungated `std::`/`HashMap`/`colored` in new files)
- [ ] wasm: `cargo build --target wasm32-unknown-unknown --all-features` and `--no-default-features` (exercises the 32-bit CKC shift path; max shift 27)
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
- [ ] Manual smoke: `cargo run --example demo -- -v` — visually confirms weak-suit inverted order and fluent resolution

---

## Gotchas

- **`DECK` order vs `validate()`:** the const builder emits sorted order by construction; reordering `SUITS`/ladders without adjusting weights is the most likely failure and `decked__validate` catches it.
- **`ranks()` returns 22 distinct rank Pips** on a full Ganjifa deck (2 courts + 10 strong + 10 weak; `Pip` equality covers all 5 fields) — don't write tests assuming 12.
- **Inverted `Ord`:** `BasicCard` sorts highest-first (suit weight desc, then rank weight desc). Write ordering assertions through `sorted()` output.
- **Emoji:** single Unicode scalar per `symbol` only; index chars uppercase/digits only.
- **No suit-count assumptions exist** in `Ranged`/`combos`/`Pile`; CKC suit flags only fire for `suit.value 1..=4` (harmless); `demo()` prints 96/120-card lines (Canasta already prints 108).
