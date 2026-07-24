# EPIC-03: YAML Deck Serialization (YDS)

> **For agentic workers:** Steps use checkbox (`- [ ]`) syntax for tracking. Work story-by-story; "default features green" (`cargo test --all`) is a precondition for every story — if it goes red mid-story, stop and diagnose before moving on.

**Goal:** Make **every deck** round-trip through YAML — `deck → YAML → deck` — as a first-class, tested capability. Introduce a self-describing **deck envelope** format that carries deck identity (not just a bare card list), a blanket `YamlDecked` trait so shipped *and* consumer-authored decks get serialization for free, and a three-layer integration test suite under `tests/` that pins round-trip fidelity, byte-level format stability, and error behavior.

**Architecture:** No core-type changes. `Pip`, `BasicCard`, `Card<T>`, and `Pile<T>` already derive `Serialize`/`Deserialize` under the `serde` feature (`src/basic/types/pips.rs:25`, `:78`; `basic_card.rs:44`; `card.rs:33`; `pile.rs:64`), and the conversions the deserialize side needs already exist (`From<BasicCard> for Card<DeckType>` at `card.rs:346`, `From<Vec<BasicCard>> for Pile<DeckType>` at `pile.rs:994`). This EPIC adds one new type (`DeckYaml`), one blanket trait (`YamlDecked`), and symmetric `to_yaml`/`from_yaml` pairs on `DeckKind` and `Pile<T>` — all behind the existing `yaml` feature.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), no_std + alloc discipline, `serde_norway` 0.9 behind the opt-in `yaml` feature, `proptest` for seeded round-trip properties, GitHub Actions CI with clippy-pedantic, no_std, and wasm32 matrix jobs.

---

## Context

The crate today has a YAML **read** path and no **write** path.

- `BasicCard::cards_from_yaml_str` (`src/basic/types/basic_card.rs:109`) deserializes a bare YAML sequence into `Vec<BasicCard>`. It is the only YAML entry point in the public API.
- `BasicCard::cards_from_yaml_file` (`basic_card.rs:94`) is the crate's single filesystem seam, quarantined behind the opt-in `std-io` feature and deliberately excluded from `full` (`Cargo.toml:32-37`, [`.okf/decisions/std-io-outside-full.md`](../.okf/decisions/std-io-outside-full.md)).
- `Razz` is the one deck built from YAML, embedded at compile time with `include_str!("yaml/razz.yaml")` (`src/basic/decks/razz.rs:36`) so deck construction stays pure. It is gated on `yaml` throughout — the deck module (`src/basic/decks.rs:9`), the prelude re-export (`src/prelude.rs:17`), and the `DeckKind::Razz` variant (`src/basic/decks/registry.rs:58`).
- There is **no serialize-side API at all**. The two existing round-trip tests reach for `serde_norway::to_string` directly (`src/basic/decks/cards/french.rs:386`, `:396`), which means the format crate is visible at every call site that wants to write YAML.

Three shipped YAML files exist under `src/basic/decks/yaml/`: `french.yaml` (54 cards), `razz.yaml` (52), and `razz_bad.yml` — a Copilot-generated file kept deliberately as a cautionary artifact. `Razz`'s own doc comment records the lesson (`razz.rs:19-24`): the bad file *parsed cleanly*, `Pile::<Razz>::validate()` did not catch it, and only a `from_str` test surfaced the problem after debugging. That near-miss is the single strongest argument shaping this EPIC's test plan.

`DeckKind` (`src/basic/decks/registry.rs:50`) is the non-generic registry over all 14 shipped decks — `all()` at `:80`, `deck_name()` at `:109`, `base_vec()` at `:142`, `fluent_deck_key()` at `:167`. It is the only construct in the crate that can express "for each shipped deck" without generic gymnastics, so it is the natural spine of an "every deck" guarantee.

**What this EPIC does NOT do:**

- **No funky/Balatro decks.** The five `BuffoonCard`/`BuffoonPile` decks (`src/funky/decks/`) are a separate type family with `MPip` effects and no registry enum. They already derive serde (`src/funky/types/buffoon_card.rs:13`) but are out of scope here.
- **No new filesystem I/O in the library.** Nothing in this EPIC touches `std::fs` inside `src/`. Kernel purity Invariant 1 holds: the only filesystem seam remains `cards_from_yaml_file` behind `std-io`. The fixture generator is an *example* binary — a crate consumer — exactly like `examples/deconstruct_vectors.rs`.
- **No published YAML data files.** The 14 golden fixtures live under `tests/fixtures/yaml/` and are never packaged: `Cargo.toml:13`'s `include` list covers `src/**` only. Package size is unchanged.
- **No migration of `razz.yaml`.** It stays in its legacy bare-sequence form on purpose — see Design decision 3.
- **No JSON, TOML, or other formats.** `serde_norway` only.

---

## Status

| Component | Status |
|---|---|
| `DeckYaml` envelope type + shape-sniffing reader | Planned |
| `CardError` YAML variants | Planned |
| `YamlDecked` blanket trait | Planned |
| `DeckKind::to_yaml` / `from_yaml` | Planned |
| `Pile<T>::to_yaml` / `from_yaml` | Planned |
| `examples/yaml_decks.rs` fixture generator | Planned |
| 14 golden fixtures under `tests/fixtures/yaml/` | Planned |
| `tests/yaml_roundtrip.rs` | Planned |
| `tests/yaml_golden.rs` | Planned |
| `tests/yaml_errors.rs` | Planned |
| Docs / CHANGELOG / `.okf/` bundle | Planned |

---

## Goals

- Every one of the **14 shipped decks** serializes to YAML and deserializes back to an identical deck — asserted by iterating `DeckKind::all()`, so a new deck cannot be added without the guarantee extending to it.
- **Deck identity survives the round trip.** A serialized deck says which deck it is; `DeckKind::from_yaml` can recover the variant without content-matching against all 14 `base_vec()`s.
- **Custom decks get it free.** A blanket impl over `DeckedBase` means any consumer marker struct that implements the trait is YAML-serializable with no extra work — the same "implement one trait, get everything" property `Decked` and `Ranged` already have.
- **Ordered instances round-trip too.** A shuffled or partially-drawn `Pile<T>` serializes its actual cards in order, so YAML is usable for saved hands and game state, not just deck definitions.
- **Well-formed-but-wrong is caught.** The `razz_bad.yml` failure mode — a file that parses fine and yields a wrong deck — becomes an enforced test, not an anecdote in a doc comment.
- **Nothing breaks.** `cards_from_yaml_str` keeps accepting bare sequences; `razz.yaml` keeps working untouched; the pure-kernel and no_std/wasm gates stay green.

## Scope

The concrete rules the feature must obey:

1. All new API is `#[cfg(feature = "yaml")]`. A default (`default = []`) build gains nothing and loses nothing.
2. No `serde_norway` type appears in any public signature. Parse failures are `Box<dyn Error>`; semantic failures are `CardError` variants, boxed (kernel purity Invariant 2).
3. The envelope is **versioned**, so a future format change is detected rather than mis-parsed.
4. Reading accepts **both** the envelope and the legacy bare sequence. Writing always produces the envelope.
5. `count` is validated against `cards.len()` on read — a truncated file is an error, not a short deck.
6. An empty card list is rejected where a *deck* is expected (`DeckKind::from_yaml`, `YamlDecked::validate_yaml`) and accepted where a *pile* is expected (a fully-drawn deck is legitimate). This is the direct fix for the "silently produce an empty deck" failure mode `razz.rs:36-39` already guards against with `log::error!`.
7. `Pile<T>::from_yaml` rejects an envelope naming a different deck, and rejects any card absent from `T::base_vec()`. It does **not** check length: partial hands and multi-deck piles (`French::decks(4)`) are both legal.
8. Golden fixtures are compared **byte-for-byte**, so format drift fails loudly rather than silently.

---

## Domain

The kata's three layers for this slice:

**Things.** The `DeckYaml` *document* — a deck rendered as data, distinct from both the deck *type* (`French`, a compile-time fact) and the deck *instance* (`Pile<French>`, an ordered runtime value). Serialization has to serve all three, and they are not the same Thing:

| Thing | Identity lives in | What YAML must preserve |
|---|---|---|
| Deck type (`French`) | The Rust type | `base_vec()` — the canonical card list |
| Deck kind (`DeckKind::French`) | An enum variant | Enough to recover the variant — i.e. the name |
| Pile instance (`Pile<French>`) | Runtime value | The actual cards **in their actual order** |

**Business Requirements.** A serialized deck must be (a) *identifiable* — you can tell what it is without guessing, (b) *complete* — a truncated or empty file is an error, never a quietly smaller deck, and (c) *faithful* — what comes back equals what went out, including order for piles.

**Business Logic.** The envelope's `name` field satisfies identifiability; the `count` field satisfies completeness; `Eq`-based round-trip assertions across `DeckKind::all()` satisfy faithfulness. Each requirement is driven out by the test that would fail without it.

---

## Design decisions (settled)

1. **A versioned envelope, not a bare sequence.** The current on-disk form is a bare YAML sequence of `BasicCard` — it carries no deck identity, so YAML → *deck* is impossible without content-matching against every `base_vec()`. The envelope adds `version`, `name`, `fluent_deck_key`, and `count` around the card list.

2. **`colors()` is excluded from the envelope.** `DeckedBase::colors()` is gated on `colored-display` and returns `HashMap<Pip, Color>` — presentation data whose inclusion would make the YAML format vary by feature flag. The envelope must be identical whether or not `colored-display` is on.

3. **`src/basic/decks/yaml/razz.yaml` is NOT migrated.** Leaving it in the legacy bare-sequence form makes it live, in-production proof that the back-compat reader works — `Razz::base_vec()` (`razz.rs:36`) exercises that path on every single build. A migrated file would only prove the new path, and the old one would rot untested.

4. **`Tiny` is not in the prelude.** `src/basic/decks.rs:17` declares `pub mod tiny;` but `src/prelude.rs` has no `tiny::*` re-export, unlike every other deck. `DeckKind::Tiny` works fine, but type-level tests must import `cardpack::basic::decks::tiny::Tiny` by full path. Adding the missing re-export is a one-line fix and is folded into Story 7.

5. **Shape-sniff on a parsed `serde_norway::Value`, not on raw text.** Parse once to `Value`, then branch: `Mapping` → envelope, `Sequence` → legacy list. Text sniffing (does the first line start with `-`?) breaks on flow style (`[{...}]`) and on leading comments; try-envelope-then-fall-back-to-sequence produces useless "both parses failed" errors. Sniffing the parsed value gives one precise error from the branch that was actually taken.

6. **`cards_from_yaml_str` is rerouted, not replaced.** It becomes a thin wrapper over `DeckYaml::from_yaml(...).map(|d| d.cards)`, so it gains envelope support as a strict superset. Existing callers — including any downstream consumer feeding it bare lists — are unaffected. This is deliberately additive; the documented public API keeps its contract.

7. **Blanket impl over `DeckedBase`, with no override hook.** `impl<T: DeckedBase> YamlDecked for T {}` with default method bodies means consumers cannot customize the format. That is the point: one format for all decks is what makes the golden fixtures and the `DeckKind` round-trip meaningful.

8. **Trait methods return `Vec<BasicCard>`, not `Pile<T>`.** `DeckedBase` alone does not imply `Default + Ord + Copy + Hash`, which `Pile<T>` requires (`pile.rs:65-67`). Keeping the trait at the `BasicCard` layer means it applies to *every* `DeckedBase` implementor; the richer pile conversion lives on `Pile<T>` where the bounds already exist.

9. **Semantic errors are `CardError` variants, boxed.** `CardError` (`src/common/errors.rs:5`) is `Eq + PartialEq` and alloc-only — a `serde_norway::Error` cannot be embedded without breaking both properties, which is precisely why `cards_from_yaml_str` already returns `Box<dyn Error>`. New variants carry `String`/`usize` payloads and get boxed, so callers can still `downcast_ref::<CardError>()` and match.

10. **Fixture filenames derive from `deck_name()`** via `to_lowercase().replace(' ', "_")`. All 14 names are distinct, so the mapping is injective — asserted by a test. `"Standard 52"` → `standard_52.yaml`, `"Dashavatara Ganjifa"` → `dashavatara_ganjifa.yaml`.

---

## Design

### `DeckYaml` — the envelope

`src/basic/types/deck_yaml.rs` (new), `#[cfg(feature = "yaml")]`:

```rust
/// A deck rendered as a self-describing YAML document.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeckYaml {
    /// Format version. `1` for everything this EPIC ships.
    pub version: u8,
    /// `DeckedBase::deck_name()` — what makes the deck identifiable.
    pub name: String,
    /// `DeckedBase::fluent_deck_key()` — the i18n base. Lowercase; Razz's is
    /// `"french"` (`src/basic/decks/cards/french.rs:8`), not `"Razz"`.
    pub fluent_deck_key: String,
    /// Redundant with `cards.len()` on purpose: a truncation guard.
    pub count: usize,
    pub cards: Vec<BasicCard>,
}

impl DeckYaml {
    /// Build an envelope from a deck type's canonical card list.
    #[must_use]
    pub fn from_decked<T: DeckedBase>() -> Self;

    /// The only format version this crate writes.
    pub const VERSION: u8 = 1;

    /// Build an envelope from an explicit, ordered card list. `count` is
    /// derived from `cards`, never passed in — the truncation guard is only
    /// meaningful if writers cannot get it wrong.
    #[must_use]
    pub fn new(name: String, fluent_deck_key: String, cards: Vec<BasicCard>) -> Self;

    /// Serialize to YAML.
    ///
    /// # Errors
    /// Propagates the underlying serializer's error, boxed.
    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>>;

    /// Parse YAML in **either** the envelope or the legacy bare-sequence form.
    ///
    /// # Errors
    /// Malformed YAML, a scalar document, or a `count` that disagrees with
    /// `cards.len()`.
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>>;
}
```

Serialized shape (`Razz`, abridged). This is **verified real `serde_norway` output**, not an illustration — note that `Pip` fields emit in declaration order (`weight` before `pip_type`, `src/basic/types/pips.rs:79-85`), which is *not* the order the hand-authored `razz.yaml` uses, and that block sequences under a mapping key are not extra-indented:

```yaml
version: 1
name: Razz
fluent_deck_key: french
count: 52
cards:
- suit:
    weight: 3
    pip_type: Suit
    index: 'S'
    symbol: '♠'
    value: 4
  rank:
    weight: 12
    pip_type: Rank
    index: 'A'
    symbol: 'A'
    value: 14
```

`from_yaml` branches on the parsed `serde_norway::Value`:

| Document shape | Interpretation |
|---|---|
| `Mapping` | Envelope. Deserialize into `DeckYaml`, then verify `count == cards.len()`. |
| `Sequence` | Legacy bare list. Deserialize into `Vec<BasicCard>`; synthesize a `DeckYaml` with `version: 1`, empty `name`/`fluent_deck_key`, `count = cards.len()`. |
| anything else | `CardError::YamlMalformed`. |

An empty `name` is the marker for "came from a legacy file" and is what `DeckKind::from_yaml` rejects.

### `CardError` — new variants

`src/common/errors.rs`, appended, `#[cfg(feature = "yaml")]` so a pure build carries no dead variants:

```rust
#[error("YAML deck count mismatch: header says `{declared}`, found `{actual}` cards")]
YamlCountMismatch { declared: usize, actual: usize },

#[error("YAML deck mismatch: document is `{found}`, expected `{expected}`")]
YamlDeckMismatch { expected: String, found: String },

#[error("Unknown deck in YAML: `{0}`")]
YamlUnknownDeck(String),

#[error("YAML document has no cards")]
YamlEmptyDeck,

#[error("Card `{card}` is not part of the `{deck}` deck")]
YamlForeignCard { deck: String, card: String },

#[error("YAML document is neither a deck envelope nor a card sequence")]
YamlMalformed,
```

These stay `Eq + PartialEq` and alloc-only, preserving `CardError`'s existing derives and no_std compatibility.

### `YamlDecked` — the blanket trait

`src/basic/types/traits.rs`, alongside `DeckedBase` (`:22`) and `Decked` (`:50`):

```rust
#[cfg(feature = "yaml")]
pub trait YamlDecked: DeckedBase {
    /// This deck's canonical card list as an envelope YAML document.
    ///
    /// # Errors
    /// Propagates serialization failure, boxed.
    fn to_yaml() -> Result<String, Box<dyn Error>> {
        DeckYaml::from_decked::<Self>().to_yaml()
    }

    /// Parse a YAML document (envelope or legacy sequence) into cards.
    ///
    /// # Errors
    /// Malformed YAML or a `count` mismatch.
    fn deck_from_yaml(yaml_str: &str) -> Result<Vec<BasicCard>, Box<dyn Error>> {
        Ok(DeckYaml::from_yaml(yaml_str)?.cards)
    }

    /// Verify a YAML document describes *this* deck, exactly.
    ///
    /// # Errors
    /// Empty card list, a `name` naming a different deck, or cards that
    /// differ from `Self::base_vec()`.
    fn validate_yaml(yaml_str: &str) -> Result<(), Box<dyn Error>>;
}

#[cfg(feature = "yaml")]
impl<T: DeckedBase> YamlDecked for T {}
```

The blanket impl is what makes the EPIC's title claim literally true: every deck — the 14 shipped ones and every consumer-authored marker struct following [`.okf/decks/extending-decks.md`](../.okf/decks/extending-decks.md) — is YAML-serializable the moment it implements `DeckedBase`. No per-deck wiring, nothing to forget when deck 15 lands.

`validate_yaml` is the YAML analogue of `Decked::validate()` (`traits.rs:135`), which the extending-decks playbook already calls "the fundamental correctness test." It is the check that `razz_bad.yml` fails.

### `DeckKind` — the non-generic path

`src/basic/decks/registry.rs`, joining `all()` (`:80`), `deck_name()` (`:109`), `base_vec()` (`:142`), `fluent_deck_key()` (`:167`):

```rust
#[cfg(feature = "yaml")]
impl DeckKind {
    /// This deck as an envelope YAML document.
    ///
    /// # Errors
    /// Propagates serialization failure, boxed.
    pub fn to_yaml(self) -> Result<String, Box<dyn Error>>;

    /// Recover the `DeckKind` a YAML document describes.
    ///
    /// # Errors
    /// A legacy bare sequence (no name to match on), an unrecognized deck
    /// name, or an empty card list.
    pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>>;
}
```

`from_yaml` matches the envelope's `name` against `DeckKind::all()`'s `deck_name()`. All 14 names are distinct (`canasta.rs:152`, `dashavatara.rs:72`, `euchre24.rs:71`, `euchre32.rs:79`, `french.rs:106`, `mughal.rs:66`, `pinochle.rs:93`, `razz.rs:48`, `short.rs:83`, `skat.rs:80`, `spades.rs:96`, `standard52.rs:105`, `tarot.rs:120`, `tiny.rs:36`), so the match is unambiguous — pinned by a test rather than assumed.

A legacy sequence is rejected rather than content-matched. Guessing a deck from its cards is a different feature with different failure modes; if it is ever wanted it belongs in its own EPIC.

### `Pile<T>` — the instance path

`src/basic/types/pile.rs`, inside the existing `impl<DeckType: DeckedBase + Default + Ord + Copy + Hash> Pile<DeckType>` block:

```rust
/// This pile's **actual** cards, in their **actual** order, as an envelope.
///
/// # Errors
/// Propagates serialization failure, boxed.
#[cfg(feature = "yaml")]
pub fn to_yaml(&self) -> Result<String, Box<dyn Error>>;

/// Rebuild a pile from a YAML document.
///
/// # Errors
/// A `name` naming a different deck, or any card absent from
/// `DeckType::base_vec()`.
#[cfg(feature = "yaml")]
pub fn from_yaml(yaml_str: &str) -> Result<Self, Box<dyn Error>>;
```

`to_yaml` uses `self.into_basic_cards()` (`pile.rs:387`), **not** `base_vec()` — that is the whole point of the instance path, and it is what makes a shuffled pile round-trip. `from_yaml` validates each card against `DeckType::base_vec()` and then goes through the existing `From<Vec<BasicCard>> for Pile<DeckType>` (`pile.rs:994`).

Deliberately **not** length-checked: `French::decks(4)` produces a 216-card pile of duplicates and a dealt hand produces five cards. Both must round-trip. Membership is the invariant; cardinality is not.

---

## Story 1: `DeckYaml` envelope + back-compat reader (`src/basic/types/deck_yaml.rs`)

**Acceptance:** envelope round-trips through `serde_norway`; legacy bare sequences still parse; `count` mismatch and malformed input both error; `cards_from_yaml_str` behavior unchanged for existing inputs.

**Files:**
- Create: `src/basic/types/deck_yaml.rs`
- Modify: `src/basic/types.rs` (add `#[cfg(feature = "yaml")] pub mod deck_yaml;`)
- Modify: `src/common/errors.rs` (new gated `CardError` variants)
- Modify: `src/basic/types/basic_card.rs:109` (reroute `cards_from_yaml_str`)
- Modify: `src/prelude.rs` (gated `DeckYaml` re-export)

### Tasks

- [ ] Add the six `#[cfg(feature = "yaml")] CardError` variants; confirm `CardError` keeps `Eq + PartialEq` and that `cargo build --no-default-features` is still green
- [ ] Write the `DeckYaml` struct with `version`/`name`/`fluent_deck_key`/`count`/`cards`, deriving `Serialize`/`Deserialize`
- [ ] `from_decked::<T>()` and `new(...)` constructors; `to_yaml()`
- [ ] `from_yaml()` with `serde_norway::Value` shape-sniffing (Mapping → envelope, Sequence → legacy, else `YamlMalformed`) and the `count == cards.len()` guard
- [ ] Reroute `BasicCard::cards_from_yaml_str` (`basic_card.rs:109`) through `DeckYaml::from_yaml`; keep its signature and doc comment contract intact
- [ ] Unit tests (`mod basic__types__deck_yaml_tests`): envelope round-trip; legacy sequence parse yields empty `name`; `count` mismatch errors with `YamlCountMismatch`; scalar document errors with `YamlMalformed`; flow-style sequence (`[{...}]`) parses as legacy
- [ ] Regression test: `BasicCard::cards_from_yaml_str(include_str!("../decks/yaml/razz.yaml"))` still yields 52 cards (path is relative to `src/basic/types/deck_yaml.rs`)
- [ ] `cargo test --features yaml --lib` green

---

## Story 2: `YamlDecked` blanket trait (`src/basic/types/traits.rs`)

**Acceptance:** all 14 shipped decks gain `to_yaml`/`deck_from_yaml`/`validate_yaml` with zero per-deck code; a throwaway custom deck in a test gets them too.

**Files:**
- Modify: `src/basic/types/traits.rs` (trait + blanket impl, after `Decked` at `:50`)
- Modify: `src/prelude.rs` (gated `YamlDecked` re-export alongside `Decked`/`DeckedBase`)

### Tasks

- [ ] Define `YamlDecked: DeckedBase` with default bodies for `to_yaml` and `deck_from_yaml`
- [ ] Implement `validate_yaml`: reject empty `cards` (`YamlEmptyDeck`); if `name` is non-empty and `!= Self::deck_name()`, reject (`YamlDeckMismatch`); compare `cards` against `Self::base_vec()` and reject on difference
- [ ] `impl<T: DeckedBase> YamlDecked for T {}`
- [ ] Doc examples on each method (they become doctests under `--features yaml`)
- [ ] Unit tests: `French::to_yaml()` → `French::deck_from_yaml()` equals `French::base_vec()`; `Tarot::validate_yaml(&French::to_yaml()?)` errors; a local `struct FakeDeck` implementing only `DeckedBase` gets all three methods (proves the blanket impl reaches consumer types)
- [ ] `cargo test --features yaml --lib` and `cargo test --doc --features full` green

---

## Story 3: `DeckKind::to_yaml` / `from_yaml` (`src/basic/decks/registry.rs`)

**Acceptance:** `DeckKind::from_yaml(&kind.to_yaml()?)? == kind` for every variant in `all()`.

**Files:**
- Modify: `src/basic/decks/registry.rs`

### Tasks

- [ ] Add a `#[cfg(feature = "yaml")] impl DeckKind` block with `to_yaml`/`from_yaml`
- [ ] `to_yaml` builds the envelope from `self.deck_name()` (`:109`), `self.fluent_deck_key()` (`:167`), `self.base_vec()` (`:142`)
- [ ] `from_yaml` parses, rejects empty `cards` (`YamlEmptyDeck`) and empty `name` (`YamlUnknownDeck("")` — a legacy sequence is unidentifiable), then matches `name` against `all()`
- [ ] Module doc example showing the `for kind in DeckKind::all()` round-trip, mirroring the existing example at `registry.rs:9-16`
- [ ] Unit tests: `deck_name__all_distinct` (pins the assumption `from_yaml` rests on); round-trip over `all()`; unknown name errors with `YamlUnknownDeck`; legacy sequence errors
- [ ] `cargo test --features yaml --lib` green

---

## Story 4: `Pile<T>::to_yaml` / `from_yaml` (`src/basic/types/pile.rs`)

**Acceptance:** shuffled, sorted, partially-drawn, multi-deck, and empty piles all round-trip; foreign cards and wrong deck names are rejected.

**Files:**
- Modify: `src/basic/types/pile.rs`

### Tasks

- [ ] `to_yaml(&self)` — envelope from `DeckType::deck_name()`, `DeckType::fluent_deck_key()`, and `self.into_basic_cards()` (`:387`), preserving order
- [ ] `from_yaml(yaml_str)` — parse; if `name` is non-empty and `!= DeckType::deck_name()`, error with `YamlDeckMismatch`; validate each card is present in `DeckType::base_vec()` (else `YamlForeignCard`); build via `From<Vec<BasicCard>>` (`:994`)
- [ ] Explicitly allow an empty `cards` list (a fully-drawn deck), and explicitly do **not** compare lengths against `base_vec()`
- [ ] Doc examples: shuffled round-trip with `shuffled_with_seed` (`:775`); a five-card dealt hand round-trip
- [ ] Unit tests: `Standard52::deck().shuffled_with_seed(42)` round-trips with order intact; `French::decks(4)` (216 cards) round-trips; empty pile round-trips; `Pile::<Tarot>::from_yaml(&French::deck().to_yaml()?)` errors with `YamlDeckMismatch`; an envelope carrying the right name but a foreign card errors with `YamlForeignCard`
- [ ] `cargo test --features yaml --lib` green

---

## Story 5: Fixture generator + 14 golden fixtures (`examples/yaml_decks.rs`)

**Acceptance:** `cargo ex yaml_decks` writes 14 files under `tests/fixtures/yaml/`; re-running produces no diff.

**Files:**
- Create: `examples/yaml_decks.rs`
- Create: `tests/fixtures/yaml/*.yaml` (14 files)
- Modify: `Cargo.toml` (`[[example]] name = "yaml_decks"`, `required-features = ["std", "yaml"]`)
- Modify: `Makefile` (a `yaml-fixtures` target)

### Tasks

- [ ] Write the generator following the `examples/deconstruct_vectors.rs` template exactly: `# Features` doc section naming `std` + `yaml`, and the `#![allow(clippy::disallowed_types, clippy::disallowed_methods)]` header with the same "this is a consumer, not the kernel" rationale (`deconstruct_vectors.rs:16-20`)
- [ ] Iterate `DeckKind::all()`; slug each `deck_name()` via `to_lowercase().replace(' ', "_")`; write `tests/fixtures/yaml/<slug>.yaml`
- [ ] Add the `[[example]]` entry to `Cargo.toml` with `required-features` — do **not** add a self dev-dependency (`Cargo.toml:85-92` documents why that breaks `cargo deny check bans`)
- [ ] Add `make yaml-fixtures` running `cargo ex yaml_decks`, and mention it in `make help`
- [ ] Generate the 14 fixtures; eyeball `tiny.yaml` (4 cards) and `standard_52.yaml` for shape
- [ ] Verify idempotence: run twice, `git status` clean the second time

---

## Story 6: Integration test suite (`tests/`)

**Acceptance:** three new integration test files, all green; deleting any single production guard from Stories 1–4 turns at least one of them red.

**Files:**
- Create: `tests/yaml_roundtrip.rs`
- Create: `tests/yaml_golden.rs`
- Create: `tests/yaml_errors.rs`

Each carries `#![cfg(all(feature = "yaml", not(target_arch = "wasm32")))]` and `#![allow(non_snake_case)]`, matching the header convention in `tests/properties.rs:16-17`. The wasm exclusion is for the same reason properties.rs carries one: `proptest`'s transitive `wait-timeout` is unix-only, and the golden tests need `std::fs` to read fixtures.

### Tasks — `tests/yaml_roundtrip.rs`

- [ ] `every_deck_kind__roundtrips` — for each `DeckKind::all()`: `DeckKind::from_yaml(&kind.to_yaml()?)? == *kind`
- [ ] `every_deck_kind__preserves_cards` — the parsed envelope's `cards` equals `kind.base_vec()`
- [ ] `every_deck_kind__preserves_metadata` — `name` and `fluent_deck_key` survive
- [ ] `typed_decks__roundtrip` — `T::deck_from_yaml(&T::to_yaml()?)? == T::base_vec()` for all 14 types (this is the type-level twin of the registry test, and it is what a consumer's custom deck would run)
- [ ] `pile__shuffled_roundtrips_in_order` — `Pile::<French>::from_yaml(&shuffled.to_yaml()?)? == shuffled` for a seeded shuffle
- [ ] `pile__partial_and_multideck_roundtrip` — a five-card hand and `French::decks(4)`
- [ ] `proptest`: for any `seed: u64`, `Pile::<Standard52>::deck().shuffled_with_seed(seed)` round-trips — the reproducible-from-seed property `tests/properties.rs:1-13` is built on

### Tasks — `tests/yaml_golden.rs`

- [ ] `fixture_count__matches_registry` — the number of `.yaml` files in `tests/fixtures/yaml/` equals `DeckKind::all().len()`, so adding deck 15 without a fixture fails
- [ ] `every_deck_kind__matches_golden_bytes` — `kind.to_yaml()?` is byte-identical to its fixture
- [ ] `every_golden__deserializes_to_its_deck` — each fixture round-trips back to the right `DeckKind` and the right `base_vec()`
- [ ] `every_golden__passes_validate_yaml` — each fixture passes the corresponding `T::validate_yaml`

### Tasks — `tests/yaml_errors.rs`

- [ ] `malformed_yaml__errors` — a syntactically invalid document
- [ ] `scalar_document__errors_malformed` — a bare scalar yields `YamlMalformed`
- [ ] `truncated_envelope__errors_count_mismatch` — `count: 52` with 51 cards yields `YamlCountMismatch`
- [ ] `empty_deck__errors` — an envelope with `cards: []` is rejected by `DeckKind::from_yaml` and `validate_yaml`, and **accepted** by `Pile::from_yaml`
- [ ] `unknown_deck_name__errors` — `name: Bicycle` yields `YamlUnknownDeck`
- [ ] `legacy_sequence__errors_in_deck_kind` — a bare list is unidentifiable, but the same input succeeds through `cards_from_yaml_str`
- [ ] `foreign_card__errors` — a French card in a Tarot-named envelope yields `YamlForeignCard`
- [ ] `wrong_deck_name__errors` — `Pile::<Tarot>::from_yaml(french_yaml)` yields `YamlDeckMismatch`
- [ ] **`razz_bad__is_caught`** — `include_str!("../src/basic/decks/yaml/razz_bad.yml")` parses successfully (it is well-formed YAML) but fails `Razz::validate_yaml`. The headline test: it promotes the near-miss documented at `razz.rs:19-24` from a doc-comment anecdote to an enforced guarantee
- [ ] All error assertions downcast to `CardError` and match the specific variant — not just `is_err()`, which would pass for the wrong reason

---

## Story 7: Docs, prelude, and knowledge bundle

**Acceptance:** `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` clean; `/okf:validate .okf --strict` clean.

### Tasks

- [ ] `src/prelude.rs` — gated re-exports for `DeckYaml` and `YamlDecked`
- [ ] `src/prelude.rs` — add the missing `pub use crate::basic::decks::tiny::*;` (design decision 4: every other deck is re-exported, `Tiny` is not)
- [ ] `src/lib.rs` — a short "Decks as YAML" section in the crate docs, next to the existing custom-deck example
- [ ] `CHANGELOG.md` — `[Unreleased] ### Added` entry (Keep-a-Changelog). Purely additive: new gated API + new `CardError` variants. **Check whether the `CardError` variant additions are semver-major** — EPIC-02 Story 6 hit exactly this with `DeckKind` and resolved it by marking the enum `#[non_exhaustive]`; consider the same for `CardError`, and run `cargo semver-checks` to confirm
- [ ] `README.md` — mention YAML serialization in the feature list
- [ ] `.okf/decks/extending-decks.md` — expand the "Alternative: decks from YAML" section with the envelope format and `YamlDecked`; refresh `timestamp`
- [ ] `.okf/architecture/feature-flags.md` — note what `yaml` now buys; refresh `timestamp`
- [ ] Create `.okf/decisions/yaml-envelope-format.md` — the load-bearing decision: **the back-compat sequence reader must not be deleted**, because `razz.yaml` (`razz.rs:36`) depends on it at build time. A future "clean up the two code paths" refactor would pass CI on the new format and break `Razz`. Add it to `.okf/decisions/index.md`
- [ ] Append a dated entry to `.okf/log.md`
- [ ] `/okf:validate .okf --strict`

---

## Test Plan

| Test | Asserts |
|---|---|
| `every_deck_kind__roundtrips` | The EPIC's headline claim, for all 14 decks, driven off the registry so it extends automatically |
| `typed_decks__roundtrip` | The same claim at the type level — the path a consumer's custom deck takes |
| `deck_name__all_distinct` | The uniqueness assumption `DeckKind::from_yaml` rests on |
| `pile__shuffled_roundtrips_in_order` | Order fidelity, which distinguishes the instance path from the type path |
| `pile__partial_and_multideck_roundtrip` | That membership, not cardinality, is the pile invariant |
| `proptest` seeded round-trip | Order fidelity across arbitrary permutations, reproducible from a failing seed |
| `fixture_count__matches_registry` | That deck 15 cannot land without a fixture |
| `every_deck_kind__matches_golden_bytes` | Format stability — silent deck-data drift and serializer reformatting both fail |
| `every_golden__deserializes_to_its_deck` | Fixtures are not merely well-formed but correct |
| `truncated_envelope__errors_count_mismatch` | The `count` guard actually guards |
| `empty_deck__errors` | The asymmetry: empty is fatal for a deck, legal for a pile |
| `foreign_card__errors` / `wrong_deck_name__errors` | Cross-deck contamination is rejected at both the card and the header level |
| **`razz_bad__is_caught`** | Well-formed-but-wrong is caught — the failure mode that `validate()` missed in production |

**Gold Standard check:** every one of these must be able to fail. Before closing the EPIC, delete each production guard in turn (the `count` check, the `name` check, the membership check, the empty check) and confirm a named test goes red. A guard no test can kill is a guard that is not doing anything.

---

## Key Files

| File | Role |
|---|---|
| `src/basic/types/deck_yaml.rs` | **New.** `DeckYaml` envelope, shape-sniffing reader |
| `src/basic/types/traits.rs` | `YamlDecked` trait + blanket impl, beside `DeckedBase` (`:22`) / `Decked` (`:50`) |
| `src/basic/types/basic_card.rs` | `cards_from_yaml_str` (`:109`) rerouted through `DeckYaml` |
| `src/basic/types/pile.rs` | `Pile<T>::to_yaml` / `from_yaml` |
| `src/basic/decks/registry.rs` | `DeckKind::to_yaml` / `from_yaml` |
| `src/common/errors.rs` | Six gated `CardError` variants |
| `src/prelude.rs` | Gated `DeckYaml` / `YamlDecked` re-exports |
| `examples/yaml_decks.rs` | **New.** Fixture generator (crate consumer, own `std::fs`) |
| `tests/fixtures/yaml/*.yaml` | **New.** 14 golden fixtures, unpublished |
| `tests/yaml_roundtrip.rs` | **New.** Round-trip layer |
| `tests/yaml_golden.rs` | **New.** Byte-stability layer |
| `tests/yaml_errors.rs` | **New.** Negative layer |
| `.okf/decisions/yaml-envelope-format.md` | **New.** Why the legacy reader must survive |

## Reuse (do NOT recreate)

- `src/basic/types/basic_card.rs:44`, `pips.rs:25`, `pips.rs:78`, `card.rs:33`, `pile.rs:64` — the `serde` derives already exist. **No new derives on core types.**
- `src/basic/types/card.rs:346` — `From<BasicCard> for Card<DeckType>`.
- `src/basic/types/pile.rs:994` — `From<Vec<BasicCard>> for Pile<DeckType>`.
- `src/basic/types/pile.rs:387` — `into_basic_cards()` is the serialize-side accessor for the instance path.
- `src/basic/decks/registry.rs:80`/`:109`/`:142`/`:167` — `all()`, `deck_name()`, `base_vec()`, `fluent_deck_key()` supply every envelope field. **Do not add a parallel deck list.**
- `src/basic/types/traits.rs:135` — `Decked::validate()` is the model `validate_yaml` follows.
- `src/common/errors.rs:5` — extend `CardError`; do not introduce a second error enum.
- `examples/deconstruct_vectors.rs` — the golden-dumper template, including the clippy-allow rationale.
- `tests/properties.rs` — the integration-test header conventions and the `proptest` idiom.
- `.cargo/config.toml` `ex` alias — the flag-free example runner. **Do not add a self dev-dependency** to make examples flag-free (`Cargo.toml:78-86`, [`.okf/decisions/examples-flag-free-alias.md`](../.okf/decisions/examples-flag-free-alias.md)).

## Compatibility

- **Preserves:** `BasicCard::cards_from_yaml_str` keeps its signature and keeps accepting bare sequences — envelope support is a strict superset. `cards_from_yaml_file` and the `std-io` seam are untouched. `src/basic/decks/yaml/razz.yaml` and `french.yaml` are unmodified. A `default = []` build gains no API and no dependencies.
- **Adds:** `DeckYaml`, `YamlDecked`, `DeckKind::to_yaml`/`from_yaml`, `Pile<T>::to_yaml`/`from_yaml`, six `CardError` variants — all behind `yaml`.
- **Breaks:** nothing intended. The one semver risk is the `CardError` variant additions on a non-`#[non_exhaustive]` enum; Story 7 resolves it, following the `DeckKind` precedent from EPIC-02.
- **Package size:** unchanged. Fixtures live under `tests/`, outside `Cargo.toml:13`'s `include` list.

## Dependencies

- **Blocks:** a future funky-deck serialization EPIC (`BuffoonPile` YAML) would reuse this envelope shape; a future "identify a deck from its cards" feature would build on `DeckKind::from_yaml`.
- **Built on:** the existing `yaml` feature and `serde_norway` dependency; `DeckKind` from EPIC-02 Story 5; the domain-kernel purity invariants from `docs/audit-2026-07-18-domain-kernel.md`.
- **Related:** EPIC-02 (Ganjifa — added the two decks that make the registry sweep 14 wide, and set the `#[non_exhaustive]` semver precedent).

## Verification

```bash
# Feature-gated build and unit tests
cargo build --features yaml
cargo test --features yaml --lib

# Full matrix
cargo test --all-features
cargo test --doc --features full
cargo clippy --all-features -- -Dclippy::all -Dclippy::pedantic
cargo fmt --all -- --check

# Purity gates — the new API must be invisible without `yaml`
cargo build --no-default-features
cargo build --no-default-features --features serde
cargo test --no-default-features --lib
cargo deny check bans

# Portability
cargo build --target wasm32-unknown-unknown --all-features
cargo build --no-default-features --target thumbv7em-none-eabihf

# Fixtures are reproducible
make yaml-fixtures && git diff --exit-code tests/fixtures/yaml/

# Docs and knowledge bundle
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

Exit criteria:

1. `DeckKind::from_yaml(&kind.to_yaml()?)? == kind` holds for all 14 variants, and `T::deck_from_yaml(&T::to_yaml()?)? == T::base_vec()` holds for all 14 types.
2. `tests/fixtures/yaml/` holds exactly `DeckKind::all().len()` files, each byte-identical to freshly generated output.
3. `razz_bad.yml` parses without error and is rejected by `Razz::validate_yaml` — asserted, not assumed.
4. Every negative test matches a specific `CardError` variant via downcast, not bare `is_err()`.
5. `cargo build --no-default-features` and the bare-metal/wasm targets stay green: nothing added here leaks into the pure kernel.
6. `cargo deny check bans` stays green — no new normal-edge dependency on `serde_norway`.
7. `src/basic/decks/yaml/razz.yaml` is byte-unchanged and `Razz::deck()` still yields 52 cards.
8. `.okf/` updated (extending-decks, feature-flags, the new decision, `index.md`, `log.md`) and `/okf:validate .okf --strict` is clean.

---

## Gotchas

1. **`serde_norway` output formatting is part of the golden contract.** A minor bump could reformat quoting or indentation and redden `yaml_golden.rs`. That is working as intended — regenerate with `make yaml-fixtures`, review the diff, and note it in the CHANGELOG. Do not "fix" it by loosening the comparison to a parse-and-compare; the byte check is what catches format drift.

2. **`Razz` is `yaml`-gated, so `DeckKind::all()` is 13 without the feature** (`registry.rs:71`). The test suite is `#[cfg(feature = "yaml")]`-gated throughout, so it always sees 14 — but the `fixture_count__matches_registry` assertion must read `DeckKind::all().len()` rather than a hardcoded `14`, or it becomes a landmine for whoever adds deck 15.

3. **Don't check `cards.len()` against `base_vec().len()` in `Pile::from_yaml`.** `French::decks(4)` is 216 cards and a dealt hand is five. Membership is the invariant. Getting this wrong makes multi-deck games unserializable, and the failure looks like a YAML bug rather than a validation bug.

4. **The empty-list asymmetry is deliberate and needs a comment in the code.** `Pile` accepts empty (a fully-drawn deck), `DeckKind`/`validate_yaml` reject it. Without a comment someone will "harmonize" them and reintroduce exactly the silent-empty-deck bug `razz.rs:36-39` exists to prevent.

5. **`CardError` must keep `Eq + PartialEq` and stay alloc-only.** The new variants carry `String`/`usize` only. A `#[from] serde_norway::Error` would break both derives *and* kernel purity Invariant 2 — parse errors stay boxed, as `cards_from_yaml_str` already does.

6. **Blanket impls and the orphan rule.** `impl<T: DeckedBase> YamlDecked for T {}` is legal because `YamlDecked` is local, but it forecloses any future per-deck override. That is intended (decision 7); if a deck ever needs a custom format, this trait is the wrong place for it.

7. **`tests/fixtures/` must not end up in the package.** `Cargo.toml:13`'s `include` list is an allowlist covering `src/**`, so fixtures are excluded by construction — but verify with `cargo package --list` before release rather than trusting it.
