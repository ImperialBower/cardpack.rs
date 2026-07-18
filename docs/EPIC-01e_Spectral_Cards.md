# EPIC-01e: Spectral Cards (SPECTRAL)

> **Follow-on to [`EPIC-01d_Editions.md`](./EPIC-01d_Editions.md), fifth in the
> Funky line ([`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 3: "Spectral cards —
> nothing beyond the tag").** Spectral cards are Balatro's high-risk consumables.
> Three of them stamp editions on cards/jokers — which is exactly why editions
> had to land first (EPIC-01d): Aura, Ectoplasm, and Hex are wireable *now* and
> were not before. This EPIC gives `BCardType::Spectral` a real deck, dispatches
> the effects that ride existing seams, and **unblocks Sixth Sense and Séance** —
> the two jokers whose whole purpose is to *create* a spectral.

**Date:** 2026-07-17 · **Branch:** `funky` · **Status:** Phase 0 complete
(2026-07-17); Phases 1–3 planned

---

## Context

`BCardType::Spectral` (`src/funky/types/buffoon_card.rs:24`) is a **bare tag**:
no deck file exists (`src/funky/decks/` holds only `basic`, `joker`, `planet`,
`tarot`, per `decks/mod.rs:1-4`), and nothing dispatches a spectral effect. The
seams a spectral would use are, however, all present:

- **Consumable dispatch** — `use_consumable` (`board.rs:1153`) removes the card
  and `match`es its `card_type` (`:1159`): `Planet → poker_hands.increment`,
  `Tarot → replace_deck_card` per target, `_ => {}`. A `Spectral` arm slots in
  here, and `GrowthEvent::ConsumableUsed` already fires for every consumable
  (`:1173`) — though `growth_delta` only rewards Constellation on a *Planet*
  (`:1253`), so a spectral correctly feeds nothing.
- **Editions** (EPIC-01d) — `with_edition` (`buffoon_card.rs:243`) is the stamp
  Aura / Ectoplasm / Hex need; it did not exist when spectrals were first
  deferred.
- **Joker creation** — `joker_pool(rarity)` (`board.rs:2115`) + the Riff-Raff
  loop (`board.rs:2150-2164`) are the "create N jokers of rarity R" pattern for
  Wraith (Rare) and The Soul (Legendary); `push_joker`/`remove_joker`
  (`board.rs:1002,1008`) manipulate the pile (Ankh, Hex).
- **Hand leveling** — `PokerHands::increment` (`hands.rs:106`) is Planet-gated,
  so Black Hole (level *every* hand) needs a sibling that bumps each
  `PokerHand` via `get_mut` (`hands.rs:102`).
- **Money / Draws** — `money: isize` (`board.rs:96`) for Wraith ($0) and Immolate
  (+$20); `recompute_draws` (`board.rs:1583`) reads persistent draw modifiers for
  the −1 hand size Ectoplasm/Ouija apply.

**What this EPIC does NOT do.** No **seals** — a grep confirms no `Seal` type,
field, or variant anywhere (`joker.rs:1975` is a lone comment), so the four
seal-adding spectrals — **Talisman, Deja Vu, Trance, Medium** — stay `Blank` with
a stated reason; a seals EPIC is their prerequisite. No **spectral booster pack**
(EPIC-01b deferred it; packs draw jokers/tarots/planets only). No **playing-card
targeting UI** beyond the `targets: &[usize]` slice `use_consumable` already
takes. It ships the deck, the dispatch, the effects on seams that exist, and the
Sixth Sense / Séance unblock.

---

## Status

| Component (phase) | Adds | Status |
|---|---|---|
| 0 — `spectral.rs` deck (18 cards) | the 18 cards behind the tag | **Complete** (2026-07-17) |
| 1 — Sixth Sense & Séance | the two jokers that *create* a spectral | Planned |
| 2 — Run-level spectrals (Black Hole, The Soul, Wraith, Ectoplasm, Hex, Ankh) | leveling / joker creation / joker editions / money | Planned |
| 3 — In-hand seam + hand spectrals (Aura, Sigil, Ouija, Immolate, Familiar, Grim, Incantation, Cryptid) | the new `in_hand` mutation seam | Planned |
| — Seal spectrals (Talisman, Deja Vu, Trance, Medium) | — | **Deferred** (no seals) |

---

## Goals

- Give **`BCardType::Spectral`** a real 18-card deck (`src/funky/decks/spectral.rs`)
  mirroring `tarot.rs`, behind the tag that has been inert since Story 3.
- **Unblock Sixth Sense and Séance** — both create a random spectral, so they need
  the deck plus a create-spectral path; shrink `BLANK_WITH_REASON` by two.
- Dispatch spectral effects through a **`use_consumable_with_rng`** twin (the
  `score`/`score_with_rng` split), since most spectrals roll a random joker, card,
  suit, or rank.
- Wire every spectral whose effect **rides an existing seam** (editions, joker
  pool, hand leveling, money, a new in-hand seam) at its exact wiki value, each
  with a test that fails before its arm.
- Name the **seal-blocked four** as deferred decisions, not omissions.

## Scope

Wiki-verified effects (balatrowiki.org, re-fetch at implementation). The 18
spectrals, grouped by the seam each needs:

- **Edition stamp** (EPIC-01d): **Aura** — Foil/Holo/Poly (random) onto 1 selected
  hand card; **Ectoplasm** — Negative onto a random joker, −1 hand size;
  **Hex** — Polychrome onto a random joker, destroy the others.
- **Joker creation / manipulation**: **The Soul** — create a random Legendary
  joker; **Wraith** — create a random Rare joker, set money to $0; **Ankh** —
  copy a random joker, then destroy the others (original + copy remain).
- **Hand leveling**: **Black Hole** — every poker hand +1 level.
- **Money / hand**: **Immolate** — destroy 5 random hand cards, gain $20.
- **Hand conversion**: **Sigil** — all hand cards to a single random suit;
  **Ouija** — all hand cards to a single random rank, −1 hand size.
- **Hand destroy + create**: **Familiar** — destroy 1 random hand card, add 3
  random Enhanced face cards; **Grim** — destroy 1, add 2 Enhanced Aces;
  **Incantation** — destroy 1, add 4 Enhanced numbered cards; **Cryptid** —
  create 2 copies of 1 selected hand card.
- **Deferred (seals)**: **Talisman** (Gold), **Deja Vu** (Red), **Trance**
  (Blue), **Medium** (Purple).

---

## Domain map

| Balatro spectral | Effect | Seam | Status |
|---|---|---|---|
| The Soul | create Legendary joker | `joker_pool` + `push_joker` | Phase 2 |
| Black Hole | +1 level all hands | new `PokerHands::increment_all` | Phase 2 |
| Wraith | Rare joker, money → $0 | `joker_pool` + `money` | Phase 2 |
| Ectoplasm | Negative on a joker, −1 hand | `with_edition` + a hand-size delta | Phase 2 |
| Hex | Polychrome on a joker, destroy rest | `with_edition` + `remove_joker` | Phase 2 |
| Ankh | copy a joker, destroy rest | `push_joker` + `remove_joker` | Phase 2 |
| Aura | random edition on a hand card | `with_edition` + new in-hand seam | Phase 3 |
| Sigil / Ouija | convert hand suit / rank | new in-hand seam | Phase 3 |
| Immolate | destroy 5 hand cards, +$20 | in-hand seam + `money` | Phase 3 |
| Familiar / Grim / Incantation | destroy 1, add N enhanced | in-hand seam | Phase 3 |
| Cryptid | 2 copies of a card | in-hand seam | Phase 3 |
| Talisman / Deja Vu / Trance / Medium | add a seal | **none — seals absent** | Deferred |

---

## Design

### Phase 0 — the deck + dispatch

`src/funky/decks/spectral.rs` mirrors `tarot.rs` exactly: a `Spectral` marker
struct with `DECK_SIZE = 18` and a `DECK` const array, then `pub mod card` of 18
`const NAME: BuffoonCard` literals (`card_type: BCardType::Spectral`, `edition:
Edition::None`). **Each spectral carries an `MPip` describing its effect** — the
house model (tarots carry `MPip`, planets carry `ChipsMultPlusOnHand`) — with the
four seal spectrals carrying `MPip::Blank`.

Dispatch is a new `Spectral` arm in a **`use_consumable_with_rng`** twin, because
most effects roll:

```rust
// board.rs — the RNG twin of use_consumable (the score/score_with_rng split)
pub fn use_consumable_with_rng<R: Rng + ?Sized>(
    &mut self, index: usize, targets: &[usize], rng: &mut R,
) -> Option<BuffoonCard> {
    // …bounds-check + remove as use_consumable does…
    match card.card_type {
        BCardType::Spectral => self.apply_spectral(card.enhancement, targets, rng),
        _ => { /* delegate the deterministic Planet/Tarot arms */ }
    }
    self.apply_growth(&GrowthEvent::ConsumableUsed(card));
    Some(card)
}
```

The pure `use_consumable` stays the deterministic entry point (Planet, Tarot, and
Black Hole — the one spectral needing no RNG); a spectral that rolls is inert
without RNG, exactly as a Lucky card is in the pure `score`.

New `MPip` variants (grouped by shape, so siblings share): `CreateJokerOfRarity(BCardType)`
(The Soul, Wraith), `EditionRandomJoker(Edition)` (Ectoplasm, Hex), plus one-offs
(`LevelAllHands`, `MoneyToZero`, `CopyRandomJokerDestroyRest`, …). Each is
non-scoring — added to `scores_hand`'s false branch, like the other creators.

### Phase 1 — Sixth Sense & Séance

Both create a random spectral into a consumable slot. Once the deck exists, this
is `create_consumable(Spectral::DECK[rng.random_range(0..18)])` on the joker's
trigger — Sixth Sense on a first-hand single 6, Séance on a Straight Flush. Flip
both consts from `Blank`, remove from `BLANK_WITH_REASON` (10 → 8). *(Doing this
in Phase 1, right after the deck, delivers the EPIC's headline unblock before the
long tail of effects.)*

### Phase 2 — run-level spectrals

Effects that write `jokers` / `money` / `poker_hands` / `draws` directly, no
in-hand seam:

- **Black Hole** — a new `PokerHands::increment_all` bumping every `PokerHand`'s
  level/chips/mult (the Planet-gated `increment` cannot loop, `hands.rs:106`).
- **The Soul / Wraith** — the Riff-Raff creation pattern at a fixed rarity;
  Wraith then `self.money = 0` (or the debt floor).
- **Ectoplasm / Hex** — pick a random joker, `remove_joker` + `push_joker(joker.with_edition(…))`
  (Negative / Polychrome); Hex then destroys the others; Ectoplasm applies −1 via
  a persistent `hand_size_delta` field `recompute_draws` reads (the vouchers-loop
  shape, `board.rs:1602`).
- **Ankh** — copy a random joker, destroy the rest, leaving original + copy.

### Phase 3 — the in-hand seam + hand spectrals

**No in-hand mutation seam exists** — `add_card_to_deck` / `destroy_deck_card` /
`replace_deck_card` all act on `full_deck`/`deck` (`board.rs:1027-1076`). Phase 3
adds the mirror for `in_hand`:

```rust
fn replace_in_hand(&mut self, index: usize, replacement: BuffoonCard) -> bool;
fn destroy_in_hand(&mut self, index: usize) -> Option<BuffoonCard>;
fn add_to_hand(&mut self, card: BuffoonCard);   // Familiar/Grim/Incantation/Cryptid
```

On it ride Aura (stamp a hand card), Sigil/Ouija (rewrite suit/rank across the
hand), Immolate (destroy 5 + $20), and the create-enhanced trio + Cryptid. This is
the largest phase and the one furthest from existing seams, so it lands last.

### Deferred — seal spectrals

Talisman / Deja Vu / Trance / Medium each add a seal, and **no seal concept
exists**. They carry `MPip::Blank` and a `BLANK_WITH_REASON` entry naming seals as
the blocker — a decision, not an omission, exactly as spectral cards themselves
were before this EPIC.

---

## Work Items

### Phase 0 — the deck — **Complete 2026-07-17**

- [x] **0a.** `src/funky/decks/spectral.rs`: the `Spectral` marker struct,
  `DECK_SIZE = 18`, `DECK`, and 18 `card::` consts via a local `spectral!` macro
  (all `MPip::Blank` for now; a shared `SPECTRAL_SUIT` Pip + a distinct-`weight`
  `rank()` per card, which is what makes the 18 unique while every effect is still
  `Blank`). Registered `pub mod spectral;` (`decks/mod.rs`) and exported from the
  prelude. 3 data-invariant tests: deck is 18, all `BCardType::Spectral`, and
  distinct (via a `HashSet`).
- **0b deferred to where it's used, not built dead.** The design put
  `use_consumable_with_rng` + the dispatch arm in Phase 0, but nothing dispatches
  a spectral effect until Phase 2, and Sixth Sense / Séance (Phase 1) *create*
  spectrals through `create_consumable`, not *use* them — so the dispatch twin
  would be dead code here. It lands in Phase 2 with the first effect, the same
  "don't build half a subsystem" discipline this branch has followed since
  EPIC-01b's `PackOpened`.

### Phase 1 — Sixth Sense & Séance

- [ ] **1a.** Wire both to create a random spectral on their trigger (Sixth Sense
  first-hand single-6; Séance Straight Flush). Flip the consts; remove from
  `BLANK_WITH_REASON` (10 → 8). Tests: each yields a spectral in a consumable
  slot on its condition and nothing off it.

### Phase 2 — run-level spectrals

- [ ] **2a.** `PokerHands::increment_all` + **Black Hole**: every hand +1 level.
- [ ] **2b.** **The Soul** (Legendary) / **Wraith** (Rare + money $0) via the
  creation pattern.
- [ ] **2c.** **Ectoplasm** (Negative joker + persistent −1 hand size) / **Hex**
  (Polychrome joker + destroy rest) / **Ankh** (copy + destroy rest). Each at its
  exact effect, seeded.

### Phase 3 — in-hand seam + hand spectrals

- [ ] **3a.** The `replace_in_hand` / `destroy_in_hand` / `add_to_hand` seam +
  **Aura** (random edition on a selected hand card).
- [ ] **3b.** **Sigil** / **Ouija** (hand → single suit / rank, Ouija −1 hand).
- [ ] **3c.** **Immolate** (destroy 5 + $20), **Familiar** / **Grim** /
  **Incantation** (destroy 1, add N enhanced), **Cryptid** (2 copies).

### Deferred

- [ ] **Seals.** Talisman / Deja Vu / Trance / Medium stay `Blank`, each with a
  stated reason naming seals as the blocker. `all_jokers__every_blank_joker_has_a_stated_reason`
  guards the *jokers*, so a **spectral** equivalent — a plain data test over the
  spectral deck, mirroring that guard — pins these four as intentionally `Blank`
  rather than forgotten.

## Test Plan

- One `spectral__` test per wired card at its exact wiki value, failing before
  its arm (Gold Standard).
- Deck invariants: 18 cards, all Spectral, unique.
- Unblock: Sixth Sense / Séance each create a spectral on their trigger.
- Seeded determinism: every random spectral (Aura's edition, Wraith's joker,
  Sigil's suit) is deterministic per seed, like `score_with_seed`.
- Inertness: a spectral used through the pure (no-RNG) `use_consumable` changes
  nothing; a board that uses no spectral is byte-identical.

## Key Files

| File | Role |
|---|---|
| `src/funky/decks/spectral.rs` | new — the 18-card deck |
| `src/funky/types/board.rs` | `use_consumable_with_rng` + the `Spectral` arm + `apply_spectral`; the in-hand seam; `hand_size_delta` |
| `src/funky/types/mpip.rs` | the spectral-effect variants (+ `Display`) |
| `src/funky/types/hands.rs` | `increment_all` (Black Hole) |
| `src/funky/decks/joker.rs` | flip Sixth Sense / Séance; shrink `BLANK_WITH_REASON` |
| `src/funky/decks/mod.rs`, `src/preludes/funky.rs` | register + export the deck |

## Reuse (do NOT recreate)

- `use_consumable`'s dispatch match (`board.rs:1159`) — add an arm; the RNG twin
  mirrors `score`/`score_with_rng`, not a new pattern.
- `with_edition` (`buffoon_card.rs:243`) — Aura / Ectoplasm / Hex stamp through it.
- `joker_pool` + the Riff-Raff loop (`board.rs:2115,2150`) — Wraith / The Soul
  reuse it; do not build a parallel creator.
- `recompute_draws`' persistent-modifier loop (`board.rs:1602`) — Ectoplasm /
  Ouija's −1 hand size is a new reader, not a new pass.
- `tarot.rs` structure (`:3-35`) — `spectral.rs` copies it verbatim in shape.

## Compatibility

**Preserves** every existing consumable and score — the `Spectral` arm is new,
`use_consumable` keeps its exact behaviour, and a board that uses no spectral is
unchanged. **Adds** the deck, the RNG twin, the spectral `MPip` variants, the
in-hand seam, and `increment_all`. **Breaks** nothing.

## Dependencies

- **Built on:** EPIC-01d (editions — Aura/Ectoplasm/Hex), EPIC-01a (joker pool,
  deck mutation, `money`, `recompute_draws`), the consumable slots (EPIC-01a 5c).
- **Blocks:** a spectral booster pack (EPIC-01b's deferred fourth pack kind now
  has cards to draw); the seal spectrals wait on a future Seals EPIC.
- **Related:** EPIC-01 Story 3 (this is its spectral item, executed).

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features            # funky must not leak into no_std
cargo fmt --all -- --check
cargo doc --no-deps --all-features           # RUSTDOCFLAGS="-D warnings"
```

Exit criteria (per phase):

1. Every wired spectral has a test at its exact Balatro value that failed before
   its arm landed.
2. Sixth Sense and Séance leave `BLANK_WITH_REASON`; the seal four remain, with a
   stated reason.
3. A board that uses no spectral is byte-identical to before.
4. A Status row flips to **Complete** only with cited, tested code.
