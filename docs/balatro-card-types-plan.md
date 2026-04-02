# Plan: Complete Balatro Card Type Definitions — COMPLETED ✅

## Final State Summary

| Card Type | `BCardType` | Module | Constants | Deck/Array |
|---|---|---|---|---|
| Standard (Basic) | ✅ | `decks/basic.rs` | ✅ 52 cards | ✅ |
| Tarot (22) | ✅ | `decks/tarot.rs` | ✅ all 22 | ✅ `MajorArcana::DECK` |
| Planet (9 + 3 secret) | ✅ | `decks/planet.rs` | ✅ all 12 | ✅ `Planet::DECK` (9) + `Planet::SECRET_DECK` (3) |
| Jokers — Common (22) | ✅ | `decks/joker.rs` | ✅ 22 | ✅ `Joker::COMMON_JOKERS` |
| Jokers — Uncommon | ✅ | `decks/joker.rs` | ✅ 60 consts | ✅ `Joker::UNCOMMON_JOKERS` |
| Jokers — Rare | ✅ | `decks/joker.rs` | ✅ 18 consts | ✅ `Joker::RARE_JOKERS` |
| Jokers — Legendary (5) | ✅ | `decks/joker.rs` | ✅ 5 consts | ✅ `Joker::LEGENDARY_JOKERS` |
| Spectral (18) | ✅ | ✅ `decks/spectral.rs` | ✅ all 18 | ✅ |
| Voucher (32) | ✅ | ✅ `decks/voucher.rs` | ✅ all 32 | ✅ |

---

## Step 1 — Fix `Planet::DECK` (small bug) ✅

`src/funky/decks/planet.rs`

Mercury (`Pair`) is already defined as `card::MERCURY` but is absent from the `DECK` array. The deck should be 9 planets + a separate `SECRET_PLANETS` array for Planet X, Ceres, and Eris.

**Tasks:**
- Add `card::MERCURY` to `Planet::DECK` (bump `DECK_SIZE` to 9)
- Create `Planet::SECRET_DECK` with `[PLANET_X, CERES, ERIS]` for the 3 "hidden" planets that only appear with advanced hand types

---

## Step 2 — Complete Joker constants (101–150) ✅

`src/funky/decks/joker.rs`

The bottom ~70 lines are commented-out descriptions for jokers 101–150 that have no `const` definitions yet. These need to be converted to `BuffoonCard` constants following the existing pattern.

**Tasks:**
- Implement each remaining joker as a `pub const` in `pub mod card` (following the existing weight/index/symbol/MPip pattern)
- Some will require new `MPip` variants (see Step 5)
- Legendary jokers (Canio, Triboulet, Yorick, Chicot, Perkeo — #146–150) use `BCardType::LegendaryJoker`

---

## Step 3 — Organize Jokers into rarity arrays ✅

`src/funky/decks/joker.rs`

The `Joker` struct only has `COMMON_JOKERS`. The ~58 existing uncommon/rare constants and all new ones need to be grouped.

**Tasks:**
- Add `UNCOMMON_JOKERS: [BuffoonCard; N]` array (collecting all `BCardType::UncommonJoker` consts)
- Add `RARE_JOKERS: [BuffoonCard; N]` array
- Add `LEGENDARY_JOKERS: [BuffoonCard; 5]` array
- Add `pile_uncommon()`, `pile_rare()`, `pile_legendary()` methods mirroring the existing `pile_common()`

---

## Step 4 — Create `decks/spectral.rs` ✅

Balatro has **18 spectral cards**: Familiar, Grim, Incantation, Talisman, Aura, Wraith, Sigil, Ouija, Ectoplasm, Immolate, Ankh, Deja Vu, Hex, Trance, Medium, Cryptid, Soul, Black Hole.

**Tasks:**
- Create `src/funky/decks/spectral.rs` following the `tarot.rs` pattern
- Add a `Spectral` struct with `DECK_SIZE: usize = 18` and `DECK` array
- Define `pub mod card { pub const FAMILIAR: BuffoonCard = ...; ... }` for all 18
- Decide on rank/suit representation: reuse inline `Pip { pip_type: PipType::Special, ... }` (like planets) or add named `SpectralRank` constants
- Map each spectral effect to an `MPip` variant (new variants needed — see Step 5)
- Add `pub mod spectral;` to `decks/mod.rs`

---

## Step 5 — Add missing `MPip` variants ✅

`src/funky/types/mpip.rs`

Several joker and spectral/voucher effects don't map to any existing `MPip` variant. Before implementing Steps 2–4, audit which effects need new variants. Likely needed:

- Spectral-specific: `AddPolychromeToRandomJoker`, `AddNegativeToRandomJoker`, `DestroyRandomCards(usize)`, `CreateSpectral(usize)`, `UpgradeAllPokerHands`, `ConvertSuit(char, char)`
- Joker-specific (for remaining 101–150): `RetriggerOn10Or4`, `GainChipsOnDiscardedSuit`, `MultPlusOnFaceCards`, `GainMultEveryXCardsDiscarded`, etc.

**Tasks:**
- Audit effects for steps 2–4 against existing `MPip` variants
- Add only what's needed, following the naming convention already established

---

## Step 6 — Create `decks/voucher.rs` ✅

Balatro has **32 vouchers** — 16 base vouchers plus 16 upgraded versions (each upgrade is unlocked by purchasing the base).

**Base vouchers:** Overstock, Clearance Sale, Hone, Reroll Surplus, Crystal Ball, Telescope, Grabber, Wasteful, Tarot Merchant, Planet Merchant, Magic Trick, Hieroglyph, Director's Cut, Paint Brush, Blank, Antimatter

**Upgraded vouchers:** Overstock Plus, Liquidation, Glow Up, Reroll Glut, Omen Globe, Observatory, Nacho Tong, Recyclomancer, Tarot Tycoon, Planet Tycoon, Illusion, Petroglyph, Retcon, Palette, and their counterparts.

**Tasks:**
- Create `src/funky/decks/voucher.rs`
- Add a `Voucher` struct with `BASE_VOUCHERS` and `UPGRADED_VOUCHERS` arrays
- Define all 32 as `pub const` with `BCardType::Voucher`
- Map each voucher effect to an `MPip` variant (many will be new)
- Add `pub mod voucher;` to `decks/mod.rs`

---

---

## Step 7 — Implement `Decked` trait for Funky deck types

`src/funky/decks/` + `src/funky/types/`

The `basic` module exposes a `Decked<DeckType>` / `DeckedBase` trait pair that gives any deck type a uniform interface: `deck()`, `decks(n)`, `validate()`, `demo()`, etc., backed by `Pile<DeckType>`. The funky module has parallel deck structs (`Deck`, `MajorArcana`, `Planet`, `Joker`, `Spectral`, `Voucher`) but none of them implement these traits — each exposes ad-hoc methods like `basic_buffoon_pile()` / `pile_common()` / `pile_rare()` with no shared contract.

### Design decision: adapt vs. new trait

`DeckedBase` requires `base_vec() -> Vec<BasicCard>`, which is bridgeable because every `BuffoonCard` already exposes `.basic_card() -> BasicCard`. The harder constraint is that `Decked<DeckType>` requires `DeckType: Copy + Default + Ord + Hash`, and the funky deck structs currently satisfy none of those bounds. Two approaches:

**Option A — Fit the existing trait**
- Derive `Copy, Default, Ord, Hash` on `Deck`, `MajorArcana`, `Planet`, `Joker`, `Spectral`, `Voucher`
- Implement `DeckedBase` on each (providing `base_vec()` by calling `.basic_card()` on every `BuffoonCard` constant)
- Add a blanket `impl Decked<Self>` for each — same single line as `impl Decked<Self> for Standard52 {}`
- Gain `Pile<Deck>`, `Pile<Planet>`, etc. and the full `Decked` API "for free"
- **Downside:** `Pile<DeckType>` holds `Card<DeckType>`, not `BuffoonCard`, so you lose `MPip`, `BCardType`, and all funky metadata unless you add a separate conversion step

**Option B — New `FunkyDecked` trait**
- Define a `FunkyDecked` trait in `src/funky/types/` (or `src/funky/decks/mod.rs`) mirroring `Decked` but over `BuffoonCard` / `BuffoonPile`
- Core method: `fn buffoon_pile() -> BuffoonPile`; optional: `fn validate() -> bool`, `fn deck_name() -> &'static str`
- Implement on each funky deck struct
- **Upside:** preserves full `BuffoonCard` fidelity (MPip, BCardType, weight, etc.); each struct already has the raw arrays needed
- **Downside:** duplicates some infrastructure already in `Decked`

**Recommended: Option B**, because the funky decks' primary value is the `MPip`/`BCardType` metadata, which `Pile<DeckType>`/`BasicCard` would silently discard.

### Tasks

- Define `FunkyDecked` trait in `src/funky/types/mod.rs` (or a new `src/funky/types/traits.rs`):
  ```rust
  pub trait FunkyDecked {
      fn buffoon_pile() -> BuffoonPile;
      fn deck_name() -> &'static str;
      fn validate() -> bool { ... } // default impl: round-trip via Display/FromStr
  }
  ```
- Implement `FunkyDecked` for:
  - `funky::decks::basic::Deck` (52 basic cards)
  - `funky::decks::tarot::MajorArcana` (22 tarots)
  - `funky::decks::planet::Planet` (9 planets; `SECRET_DECK` handled separately)
  - `funky::decks::joker::Joker` — decide whether `buffoon_pile()` returns all jokers combined or only common; consider `pile_by_rarity()` returning a struct or tuple
  - `funky::decks::spectral::Spectral` (18 spectrals)
  - `funky::decks::voucher::Voucher` (32 vouchers)
- Add `pub use` for `FunkyDecked` to `src/funky/mod.rs` and `src/preludes/funky.rs`
- Add tests: for each impl assert `buffoon_pile().len() == DECK_SIZE` and that `deck_name()` returns a non-empty string

### Open questions

- Should `FunkyDecked::buffoon_pile()` for `Joker` return all ~105 jokers combined, or only common? Probably the combined pile is most useful; rarity-specific access stays on the `Joker` struct directly.
- Is `validate()` meaningful for funky cards? Round-tripping through `Display`/`FromStr` is only possible if those are already implemented on `BuffoonCard`. Check before adding the default impl.

---

## Execution Order (all completed ✅)

```
1. Fix Planet::DECK (Mercury bug)             ✅
2. Add MPip variants for remaining jokers     ✅
3. Complete joker consts (#101–150)           ✅
4. Organize jokers into rarity arrays         ✅
5. Add MPip variants for spectral effects     ✅
6. Create spectral.rs                         ✅
7. Add MPip variants for voucher effects      ✅
8. Create voucher.rs                          ✅
```
