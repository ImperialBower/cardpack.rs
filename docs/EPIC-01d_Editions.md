# EPIC-01d: Editions (EDITION)

> **Follow-on to [`EPIC-01c_Vouchers.md`](./EPIC-01c_Vouchers.md), fourth in the
> Funky line ([`EPIC-01_Funky.md`](./EPIC-01_Funky.md) domain map, "Editions"
> row: ❌ absent).** Editions are the orthogonal card/joker modifier Balatro
> layers on top of enhancements — a card can be Steel *and* Foil at once. Three
> of the four are pure scoring (`+chips` / `+mult` / `×mult`) and map onto the
> `ScoreOp` fold the engine already runs; the fourth (Negative) is a slot rule,
> the Antimatter-voucher shape. This EPIC also unblocks **Perkeo**, the last
> Legendary joker still `Blank`.

**Date:** 2026-07-17 · **Branch:** `funky` · **Status:** Phases 0–1 complete
(2026-07-17); Phases 2–4 planned

---

## Context

The engine scores chips × mult through a unified `ScoreOp` fold, but has no notion
of an **edition** — the foil/holographic/polychrome/negative overlay Balatro puts
on cards and jokers:

- `BuffoonCard` (`src/funky/types/buffoon_card.rs:78`) carries `suit`, `rank`,
  `card_type`, `enhancement: MPip`, `resell_value`, `debuffed` — **no edition
  field**. It is `#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq,
  Serialize, Deserialize)]` (`:77`), so any `Edition` type must be `Copy`/`const`/
  serde too, exactly the constraint that keeps effects plain data.
- `MPip` (`src/funky/types/mpip.rs:22`) has **no** Foil/Holo/Poly/Negative/Edition
  variant (confirmed across all 150+ variants). Editions are orthogonal to
  `enhancement`, so they want their own field, not a variant — a Steel Foil card
  is both at once.
- The scoring fold is ready for them: `ScoreOp` (`src/funky/types/effect.rs:25`)
  has `AddChips` (`:29`), `AddMult` (`:31`), `TimesMult(f32)` (`:35`), and `Seq`
  (`:37`). Foil = `AddChips(50)`, Holo = `AddMult(10)`, Polychrome =
  `TimesMult(1.5)` — all three already expressible. `TimesMult` routes to
  `Score::multi_mult` (`effect.rs:49`, `score.rs:43`, ceil-based), the same `×n/10`
  path `MultTimes1Dot` uses (`board.rs:768`), so ×1.5 is a solved sum.
- The folds an edition contribution attaches to: `fold_played_cards`
  (`board.rs:333`, built-in op at `builtin_played_op` `:422`) and `fold_jokers`
  (`board.rs:541`, `builtin_joker_op` `:571`). Held cards (`fold_held_cards`
  `:463`) are **not** in scope — editions score when a card is *played* or on a
  *joker*, never from hand (that is enhancements' job, Steel).
- The Negative slot seam already exists in miniature: `has_joker_room`
  (`board.rs:1077`) is `self.jokers.len() < self.joker_slots`, and Antimatter
  bumps `joker_slots` in `redeem_shop_voucher` (`board.rs:1835`). Negative is the
  live-read version of that bump.
- **Perkeo** (`src/funky/decks/joker.rs:1830`, `MPip::Blank`) is in
  `BLANK_WITH_REASON` (`joker.rs:2712`) with reason *"needs the shop and Negative
  editions, neither of which exists"* — the shop now exists (EPIC-01b), so
  Negative editions are its last blocker.

**What this EPIC does NOT do.** It does not model **where editions come from** in
a run beyond a stamping seam: the shop's random edition rolls and the Hone / Glow
Up / Illusion frequency vouchers (deliberately deferred in EPIC-01c) stay out — a
follow-on owns edition *sourcing*. It does not add editions to **held-card**
scoring (they do not trigger from hand). It does not touch **seals** (a separate
overlay). It builds the edition *model*, its *scoring*, the *Negative slot rule*,
and the *Perkeo* unblock — the parts that are pure domain logic on seams that
already exist.

---

## Status

| Component (phase) | Adds | Status |
|---|---|---|
| 0 — `Edition` type + `BuffoonCard.edition` field | the overlay every card/joker can wear | **Complete** (2026-07-17) |
| 1 — Played-card editions (Foil/Holo/Poly) | `+50` chips / `+10` mult / `×1.5` when a card scores | **Complete** (2026-07-17) |
| 2 — Joker editions (Foil/Holo/Poly) | the same three on a joker's own contribution | Planned |
| 3 — Negative slots (jokers + consumables) | a Negative item takes no slot | Planned |
| 4 — Perkeo | the last Blank Legendary, via Negative consumables | Planned |

---

## Goals

- Add an **`Edition`** overlay to `BuffoonCard`, orthogonal to `enhancement`, so a
  card or joker can wear Foil / Holographic / Polychrome / Negative independent of
  its Steel/Glass/etc.
- Score the three **numeric editions** through the existing `ScoreOp` fold — no
  new scoring path, just new op sources on played cards and jokers.
- Model **Negative** as a live slot exemption (a Negative item does not count
  against its slot limit), mirroring the Antimatter bump but read live.
- **Unblock Perkeo** — the last Legendary joker still `Blank` — by giving Negative
  consumables a referent, shrinking `BLANK_WITH_REASON` by one.
- Every wired edition gets a test at its exact wiki value that fails before its
  arm lands (Gold Standard, EPIC-00f).

## Scope

Wiki-verified rules this EPIC must obey (balatrowiki.org, re-fetch at
implementation):

- **Foil** — `+50` chips when the bearer scores.
- **Holographic** — `+10` mult when the bearer scores.
- **Polychrome** — `×1.5` mult when the bearer scores (applied after the additive
  contributions, at the bearer's position in the fold — the Glass shape).
- **Negative** — a joker gains `+1` **joker slot** (equivalently: takes none); a
  consumable takes **no consumable slot**. **Not a scoring effect**, and **never**
  on a playing card (Balatro allows Negative only on jokers and consumables).
- Editions are **orthogonal** to enhancements: a card may be both (Steel + Foil),
  so `edition` is a separate field, not an `MPip` variant.
- Editions **do not trigger from hand** — a Foil card held and not played scores
  nothing (contrast Steel, which is the held-card mechanic).
- A card/joker carries **at most one** edition (`Edition::None` = the default,
  unedited state).

---

## Domain map

| Balatro term (wiki) | What it needs | funky construct to add |
|---|---|---|
| edition | a per-item overlay | `Edition` enum on `BuffoonCard` |
| Foil / Holographic | flat chips / mult | `Edition::score_op` → `AddChips(50)` / `AddMult(10)` |
| Polychrome | ×mult | `Edition::score_op` → `TimesMult(1.5)` |
| Negative (joker) | a free joker slot | live count in `has_joker_room` |
| Negative (consumable) | a free consumable slot | live count in `has_consumable_room` |
| "Foil this card" | stamp an edition | `BuffoonCard::with_edition` |
| Perkeo | a Negative consumable copy | `Edition::Negative` + a create seam |

---

## Design

### Phase 0 — `Edition` (new `src/funky/types/edition.rs`) + the field

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Edition {
    #[default]
    None,
    Foil,          // +50 chips
    Holographic,   // +10 mult
    Polychrome,    // ×1.5 mult
    Negative,      // +1 slot (jokers/consumables); no score
}

impl Edition {
    /// The scoring contribution of a *bearer that is scoring* (a played card or
    /// a joker). `None`/`Negative` contribute nothing.
    pub fn score_op(self) -> ScoreOp {
        match self {
            Self::Foil => ScoreOp::AddChips(50),
            Self::Holographic => ScoreOp::AddMult(10),
            Self::Polychrome => ScoreOp::TimesMult(1.5),
            Self::None | Self::Negative => ScoreOp::Nothing,
        }
    }
    pub fn is_negative(self) -> bool { matches!(self, Self::Negative) }
}
```

`BuffoonCard` (`buffoon_card.rs:78`) gains `pub edition: Edition` beside
`enhancement`. Because the struct is `Copy`/`const` and every card/joker is a
**full positional `const` literal** (e.g. Perkeo at `joker.rs:1830`), the field
addition is **mechanical but wide**: every literal in `decks/` gains `edition:
Edition::None,`, and the `bcard!` macro (`src/funky/macros.rs`) sets it once for
macro-built playing cards. `Edition::default() == None` keeps the field honest —
an unstamped card is unedited.

**Why a field, not a side-table:** a `BuffoonCard` is a `Copy` value with no
identity (two value-equal cards are interchangeable — the codebase relies on this
for deck mutation), so an edition cannot live in a board-side map keyed by card;
it must travel *in* the card value through deck → hand → played. That is the same
reason `enhancement` is a field.

### Phase 1 — Played-card editions

`fold_played_cards` (`board.rs:333`) already applies `builtin_played_op(card)`
(`:422`, additive chips+mult) then a per-`enhancement` special (Glass's `×mult` at
`:373`). Editions slot in the same two ways:

```rust
// after the card's built-in + enhancement ops, at the card's position:
score = card.edition.score_op().apply(score);
```

Foil/Holo are additive and compose in any order; **Polychrome is `×mult` and must
apply at the card's position** (after that card's chips/mult, before the next
card) — exactly how Glass is handled, so it reuses that ordering, not a global
post-multiply.

### Phase 2 — Joker editions

`fold_jokers` (`board.rs:541`) resolves each joker to an op
(`counter_joker_op(...).unwrap_or_else(|| builtin_joker_op(joker))`, `:559`) and
applies it (`:563`). A joker's own edition applies right after, at the joker's
position, so a Polychrome joker multiplies the running score after its effect —
Balatro's order:

```rust
score = op.apply(score);
score = joker.edition.score_op().apply(score);   // the joker's own edition
```

### Phase 3 — Negative slots

Negative is **not** a bump (that would need un-bumping on removal and could
desync); it is a **live count**, the EPIC-01c pattern — `has_joker_room` is the
natural recompute point:

```rust
pub fn has_joker_room(&self) -> bool {
    let taking_slots = self.jokers.iter().filter(|j| !j.edition.is_negative()).count();
    taking_slots < self.joker_slots
}
// has_consumable_room gains the identical filter on `consumables`.
```

So a board with 5 normal jokers + 1 Negative holds 6, and selling the Negative
one restores the limit automatically — no stored counter to drift. `joker_slots`
itself (and Antimatter's bump) is untouched; Negative composes with it.

### Phase 4 — Perkeo

Perkeo (`joker.rs:1830`): "at end of round, create a Negative copy of a random
consumable held." With Negative now real and consumables already creatable
(`create_consumable`, respecting `has_consumable_room` which now exempts Negatives),
Perkeo's effect becomes expressible: pick a held consumable, stamp it
`Edition::Negative` via `with_edition`, and create it (it takes no slot, so it
always fits). Flip `MPip::Blank` → a new `MPip::CreateNegativeConsumableCopy`
arm on the round-end hook, and remove Perkeo from `BLANK_WITH_REASON` (11 → 10).
*(If the copy-a-held-consumable seam proves larger than one arm, Perkeo stays
`Blank` with its reason updated to name only the remaining blocker — a decision
recorded, not skipped.)*

---

## Work Items

### Phase 0 — `Edition` type + the field — **Complete 2026-07-17**

- [x] **0a.** New `src/funky/types/edition.rs`: the `Edition` enum (`None`/Foil/
  Holographic/Polychrome/Negative, `#[default] None`) + `score_op()` (Foil →
  `AddChips(50)`, Holo → `AddMult(10)`, Poly → `TimesMult(1.5)`, None/Negative →
  `Nothing`) + `is_negative()` + `Display`. Registered `pub mod edition;` and
  exported from `src/preludes/funky.rs`. 6 unit tests including the exact
  `apply`-through-`Score` values (Poly ×1.5 ceils 10→15).
- [x] **0b.** `pub edition: Edition` added to `BuffoonCard` beside `enhancement`.
  The `bcard!` macro needed **no** change (it resolves to consts). The literal
  sweep hit the four deck files' `const` definitions (basic 53, joker 112, tarot
  22, planet 12 = 199) via a scripted insert + one `use` per `card` submodule;
  the fluent `..*self` literals and test-module cases inherit the field and were
  untouched (compile confirmed no stragglers). `with_edition(&self, edition)`
  added (mirrors the enhancement stamp); `with_edition__is_orthogonal_to_the_enhancement`
  pins that a Steel card stays Steel when foiled. All 699 lib tests green — the
  `Edition::None` default is inert, so every existing score is byte-identical.

### Phase 1 — Played-card editions — **Complete 2026-07-17**

- [x] **1a.** `card.edition.score_op().apply(score)` folded into
  `fold_played_cards` right after each card's built-in + special ops, **inside
  the retrigger loop** — so a Polychrome ×1.5 multiplies the running score at the
  card's position (the Glass shape) and a retriggered card re-applies its edition
  each pass. Tests through `score()`, each failing before the arm: a Foil card
  adds +50 chips (no mult), a Holo card +10 mult (no chips), a Polychrome card
  on a pair (base mult 2) → mult 3 (`ceil(2 × 1.5)`), chips unchanged. Plus the
  inertness anchor: `Edition::None` on a played card scores byte-identical.

### Phase 2 — Joker editions

- [ ] **2a.** Fold `joker.edition.score_op()` into `fold_jokers` after the
  joker's op. Tests: a Foil joker +50 chips, Holo +10 mult, Polychrome ×1.5 on
  the running score after its effect (composition-order test both ways).

### Phase 3 — Negative slots

- [ ] **3a.** Live Negative exemption in `has_joker_room` and
  `has_consumable_room`. Tests: a 6th joker fits iff one held is Negative; buying
  past the limit is refused without a Negative and allowed with one; selling the
  Negative restores the limit. A Negative joker scores nothing (it is not a
  numeric edition).

### Phase 4 — Perkeo

- [ ] **4a.** `MPip::CreateNegativeConsumableCopy` on the round-end hook: copy a
  held consumable, stamp `Edition::Negative`, create it (always fits). Flip
  Perkeo's const; remove from `BLANK_WITH_REASON` (11 → 10); the guard
  `all_jokers__every_blank_joker_has_a_stated_reason` is the close-out check.
  Test: Perkeo at round end yields a Negative consumable that occupies no slot.

---

## Test Plan

- One `score__`/`edition__` test per numeric edition at its exact wiki value,
  failing before its arm (Gold Standard).
- Orthogonality: a Steel Foil card scores *both* the held-Steel ×mult (when held)
  and the Foil +50 (when played) — the two fields do not interfere.
- Negative: slot exemption is live (fits/refuses/restores on sell); a Negative
  bearer contributes no chips/mult.
- Inertness: a board whose every card/joker is `Edition::None` scores
  byte-identical to today (the field defaults inert).
- Perkeo: end-of-round Negative consumable copy, occupying no slot.

## Key Files

| File | Role |
|---|---|
| `src/funky/types/edition.rs` | new — `Edition` enum + `score_op` / `is_negative` |
| `src/funky/types/buffoon_card.rs` | `edition` field, `with_edition` stamp |
| `src/funky/types/board.rs` | edition ops in `fold_played_cards` / `fold_jokers`; Negative live-count in `has_*_room`; Perkeo's round-end arm |
| `src/funky/types/mpip.rs` | `CreateNegativeConsumableCopy` (Perkeo) + `Display` |
| `src/funky/decks/*.rs` + `macros.rs` | the mechanical `edition: Edition::None` on every literal |
| `src/funky/decks/joker.rs` | flip Perkeo; shrink `BLANK_WITH_REASON` |
| `src/preludes/funky.rs` | export `Edition` |

## Reuse (do NOT recreate)

- `ScoreOp` + its `apply` fold (`effect.rs:25,43`) — editions are new *op sources*,
  not a new scoring path. `TimesMult` → `multi_mult` (`score.rs:43`) already does
  Polychrome's ×1.5.
- The Glass `×mult`-at-card-position handling (`board.rs:373`) — Polychrome reuses
  its ordering exactly.
- `has_joker_room` / `has_consumable_room` (`board.rs:1077`) — Negative adds a
  filter; do not add a parallel capacity field.
- `enhance_swap`'s fluent stamp (`buffoon_card.rs:227`) — `with_edition` mirrors it.
- `create_consumable` (`board.rs`) — Perkeo's copy routes through it; the room
  clause now exempts Negatives for free.

## Compatibility

**Preserves** every existing score — `edition` defaults to `Edition::None`, whose
`score_op` is `Nothing`, so an unstamped board is byte-identical. **Adds** the
`Edition` type, one field (with a wide but mechanical literal sweep), edition ops
in two folds, the Negative slot exemption, and one `MPip` variant for Perkeo.
**Breaks** nothing at runtime; the field addition is a source-level sweep, not an
API break for callers using the `bcard!` macro.

## Dependencies

- **Built on:** the `ScoreOp` fold (EPIC-01 Story 8), the slot limits (EPIC-01a
  5c), `create_consumable` (EPIC-01a).
- **Blocks:** spectral cards (several need editions — Ectoplasm/Ankh/Hex), and
  the deferred edition vouchers (Hone/Glow Up/Illusion, EPIC-01c) — this EPIC is
  their prerequisite.
- **Related:** EPIC-01c (Negative mirrors the Antimatter slot bump); a future
  edition-*sourcing* EPIC (shop rolls + frequency vouchers).

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features            # funky must not leak into no_std
cargo fmt --all -- --check
cargo doc --no-deps --all-features           # RUSTDOCFLAGS="-D warnings"
```

Exit criteria (per phase):

1. Every numeric edition has a test at its exact Balatro value that failed before
   its arm landed.
2. A board with no editions (`Edition::None` everywhere) is byte-identical to
   before.
3. Negative's slot exemption is live — fits, refuses, and restores on sell.
4. Perkeo leaves `BLANK_WITH_REASON`, or its reason names only a genuinely
   remaining blocker.
5. A Status row flips to **Complete** only with cited, tested code.
