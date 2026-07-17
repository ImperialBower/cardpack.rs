# EPIC-01c: Vouchers (VOUCHER)

> **Follow-on to [`EPIC-01b_Shop.md`](./EPIC-01b_Shop.md), third in the Funky
> shop line ([`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 7).** EPIC-01b built
> the shop's two card slots and two pack slots and left the **1 voucher slot**
> explicitly out of scope ("the $10 slot stays unmodelled"). This EPIC fills it:
> a persistent, run-wide modifier the shop sells once, read live by the round
> configuration the engine already recomputes each blind.

**Date:** 2026-07-17 · **Branch:** `funky` · **Status:** Phases 1–4 complete
(2026-07-17); Phase 5 planned

---

## Context

The shop can sell jokers, consumables, rerolls, and booster packs, but its third
kind of stock — the **voucher** — does not exist beyond a bare tag:

- `BCardType::Voucher` (`src/funky/types/buffoon_card.rs:25`) is a display tag
  with no deck, no state, and no reader — the same "declared but inert" shape
  Credit Card was in before EPIC-01b.
- The seams a voucher would modify are all already built and, crucially, all
  **recompute-from-baseline or read-live**, which is exactly what a persistent
  modifier needs:
  - `recompute_draws` (`board.rs:1542`) rebuilds `draws` from `starting_draws`
    plus the board's live draw-modifier jokers every blind — so a hands/discards/
    hand-size voucher is a new reader of a pass that already exists;
  - `joker_slots` (`board.rs:167`) and `consumable_slots` (`board.rs:172`) are
    real, readable limits (base 5 / 2), already bumpable (a test does
    `board.joker_slots += 1`);
  - `reroll_cost` (`board.rs:1760`) computes `5 + (used − free)` live;
  - `stock_price` (`board.rs:1604`) and a pack's `cost` are read at buy time;
  - the interest cap `(self.money / 5).clamp(0, 5)` appears **twice**
    (`cash_out` `board.rs:2047`, `ExtraInterest` payout `board.rs:2224`) — a
    voucher that raises the cap wants those unified first.

**What this EPIC does NOT do.** No **editions** (foil/holo/poly/Negative still do
not exist), so the edition vouchers — Hone, Glow Up, Illusion — stay out, and so
does Omen Globe (spectral packs). No **ante progression**, so the ante vouchers —
Hieroglyph, Petroglyph, Director's Cut, Retcon — stay out. No **playing-card shop
stock** (Magic Trick), which the base game gates behind exactly these vouchers
anyway. No **pack-content shaping** (Telescope, Observatory). It wires the
vouchers that ride the draws / slots / economy / shop-weight seams above, at
exact wiki values, and states every deferral as a decision. It also **unblocks no
joker directly** — vouchers are their own reward: a complete, spendable shop.

---

## Status

| Component (phase) | Adds | Status |
|---|---|---|
| 1 — `Voucher` type, board state, shop slot, redeem | the voucher slot the shop has been missing | **Complete** (2026-07-17) |
| 2 — Draws vouchers (Grabber/Nacho Tong, Wasteful/Recyclomancy, Paint Brush/Palette) | hands / discards / hand-size, via `recompute_draws` | **Complete** (2026-07-17) |
| 3 — Slot vouchers (Overstock/Plus, Crystal Ball, Antimatter) | shop card slots, consumable slots, joker slots | **Complete** (2026-07-17) |
| 4 — Economy vouchers (Reroll Surplus/Glut, Clearance Sale/Liquidation, Seed Money/Money Tree) | reroll cost, buy discount, interest cap | **Complete** (2026-07-17) |
| 5 — Shop-weight vouchers (Tarot/Planet Merchant + Tycoon) | the 20/4/4 stock roll | Planned |

---

## Goals

- Fill the shop's **voucher slot** — the last unspent-money surface — closing the
  shop EPIC-01b left one slot short of complete.
- Model a voucher as what it is: a **redeemed-once, run-permanent** modifier, held
  in its own board state and read live by the configuration passes that already
  exist, never a scoring effect in `MPip`.
- Encode the **base → upgrade prerequisite** (Overstock Plus needs Overstock) as a
  redeem-time rule, so an upgrade cannot be taken without its base.
- Wire every in-scope voucher at its **exact wiki value**, each with a test that
  fails before its reader lands (Gold Standard, EPIC-00f).
- Unify the **duplicated interest cap** into one `interest_cap()` reader before
  Seed Money raises it — the refactor a second reader forces.

## Scope

Wiki-verified rules this EPIC must obey (balatrowiki.org, to be re-fetched at
implementation):

- **The voucher slot.** A shop shows **1 voucher**, priced **$10**. Redeeming it
  is permanent for the run; a redeemed voucher never returns to the pool. An
  **upgrade** voucher (the "Plus"/Tycoon/second-tier half) is only offered, and
  only redeemable, once its **base** is already redeemed.
- **Draws vouchers** (recompute-from-baseline, so they stack cleanly and a
  future removal would self-clean):
  - **Grabber** +1 hand / round; **Nacho Tong** +1 more (base must be held).
  - **Wasteful** +1 discard / round; **Recyclomancy** +1 more.
  - **Paint Brush** +1 hand size; **Palette** +1 more.
- **Slot vouchers** (permanent limit bumps):
  - **Overstock** shop card slots 2 → 3; **Overstock Plus** → 4.
  - **Crystal Ball** +1 consumable slot (2 → 3).
  - **Antimatter** +1 joker slot (5 → 6).
- **Economy vouchers** (read live at cost/cap time):
  - **Reroll Surplus** rerolls cost **$2 less**; **Reroll Glut** $2 more less
    ($4 total), floored at $0.
  - **Clearance Sale** all shop cards & packs **25% off**; **Liquidation** 50%
    off (floored, min $1 — never free).
  - **Seed Money** raises the interest cap **$5 → $10**; **Money Tree** → $20.
- **Shop-weight vouchers** (bias the card-slot roll):
  - **Tarot Merchant** tarots appear **2×** as often in the shop; **Tarot
    Tycoon** 4×. **Planet Merchant** / **Planet Tycoon** the same for planets.

---

## Domain map

| Balatro term (wiki) | What it needs | funky construct to add |
|---|---|---|
| voucher | a redeemed-once run modifier | `Voucher` enum |
| the redeemed set | run-permanent state | `BuffoonBoard.vouchers: Vec<Voucher>` |
| the $10 shop slot | one voucher on offer | `Shop.voucher: Option<Voucher>` |
| "redeem" | the buy path for a voucher | `redeem_shop_voucher()` |
| base → upgrade | a prerequisite check | `Voucher::requires()` |
| +hand / +discard / +hand size | a live read in the recompute | `recompute_draws` voucher arm |
| +slot | a permanent limit bump | `joker_slots` / `consumable_slots` / shop stock count |
| reroll discount / interest cap | a live cost read | `reroll_cost` / `interest_cap()` |
| shop weight bias | a re-weighted stock roll | `draw_stock_card` voucher-aware weights |

---

## Design

### Phase 1 — `Voucher` (new `src/funky/types/voucher.rs`)

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Voucher {
    Grabber, NachoTong,          // +1, +1 hand
    Wasteful, Recyclomancy,      // +1, +1 discard
    PaintBrush, Palette,         // +1, +1 hand size
    Overstock, OverstockPlus,    // shop card slot 2->3->4
    CrystalBall,                 // +1 consumable slot
    Antimatter,                  // +1 joker slot
    RerollSurplus, RerollGlut,   // reroll -$2, -$4
    ClearanceSale, Liquidation,  // buy 25%, 50% off
    SeedMoney, MoneyTree,        // interest cap $10, $20
    TarotMerchant, TarotTycoon,  // tarot weight x2, x4
    PlanetMerchant, PlanetTycoon,// planet weight x2, x4
}

impl Voucher {
    /// The base an upgrade voucher requires, or `None` for a base voucher.
    pub fn requires(self) -> Option<Voucher> { /* NachoTong -> Grabber, ... */ }
}

// BuffoonBoard gains:
pub vouchers: Vec<Voucher>,      // redeemed, run-permanent; defaults empty
// Shop gains:
pub voucher: Option<Voucher>,    // the $10 slot; None once redeemed or if none offered
```

A **dedicated enum, not a `BuffoonCard` + `MPip`.** Jokers and consumables are
`BuffoonCard`s because they live in piles, are held/sold, and (jokers) score
through `MPip`. A voucher does none of that: it is redeemed once, never held or
sold, and modifies *board configuration*, not hand score — so putting it in the
scoring vocabulary (`MPip`) would be a category error, the way `BossBlind`
(`src/funky/types/blind.rs`) is its own enum rather than a card. The `requires()`
prerequisite is data on the same enum, so "the upgrade needs the base" is one
match arm, not a lookup table.

`redeem_shop_voucher(&mut self) -> bool` is the voucher buy path: it charges $10
against the same [debt floor](./EPIC-01b_Shop.md) `buy_stock` uses, refuses if
the offered voucher's `requires()` is unmet, pushes onto `vouchers`, and clears
the slot. `open_shop_with_rng` draws the slot from the pool of vouchers not yet
redeemed and whose prerequisite is met.

### Phase 2 — Draws vouchers (one new arm in `recompute_draws`, `board.rs:1542`)

```rust
// after the joker loop, before the boss ability:
for voucher in &self.vouchers {
    match voucher {
        Voucher::Grabber      => draws.hands_to_play += 1,
        Voucher::NachoTong    => draws.hands_to_play += 1,
        Voucher::Wasteful     => draws.discards += 1,
        Voucher::Recyclomancy => draws.discards += 1,
        Voucher::PaintBrush   => draws.hand_size += 1,
        Voucher::Palette      => draws.hand_size += 1,
        _ => {}
    }
}
```

The **house pattern exactly**: `recompute_draws` already rebuilds `draws` from
`starting_draws` plus live jokers each blind, so a voucher read here stacks with
the jokers, never double-applies across blinds, and would self-clean if a voucher
were ever removed. The boss ability still lands **last** (a boss constrains the
round after every bonus), so The Needle still leaves exactly one hand whatever
Grabber said.

### Phase 3 — Slot vouchers

Board slots are **bumped once at redeem** rather than read live, because unlike
`draws` (rebuilt every blind) the slot fields are permanent limits with no
recompute pass — and a redeem happens once, guarded, so the bump cannot stack:

```rust
// inside redeem, after the voucher is accepted:
match voucher {
    Voucher::CrystalBall => self.consumable_slots += 1,
    Voucher::Antimatter  => self.joker_slots += 1,
    _ => {}
}
```

Overstock is the exception — it sizes the *shop's* card slots, not the board's —
so it is read **live** at `open_shop_with_rng` time:

```rust
let slots = 2 + self.overstock_bonus();   // 0 / 1 / 2 for none / Overstock / Plus
let stock = (0..slots).map(|_| Self::draw_stock_card(rng)).collect();
```

### Phase 4 — Economy vouchers

```rust
pub fn reroll_cost(&self) -> usize {
    let base = /* existing 5 + (used - free) */;
    base.saturating_sub(self.reroll_discount())   // $2 / $4, floored at 0
}

fn interest_cap(&self) -> isize {                 // 5 / 10 / 20
    match () {
        _ if self.vouchers.contains(&Voucher::MoneyTree) => 20,
        _ if self.vouchers.contains(&Voucher::SeedMoney) => 10,
        _ => 5,
    }
}
// stock_price / pack cost gain a clearance multiplier, floored at $1.
```

**The keystone refactor:** the interest cap `(money / 5).clamp(0, 5)` is
duplicated at `board.rs:2047` and `:2224`. Seed Money forces both to read one
`interest_cap()` — the second reader that turns a magic `5` into a named rule,
exactly the discipline EPIC-01b applied when To the Moon's interest and cash-out
interest had to agree. Both must move together or Seed Money is half-applied.

### Phase 5 — Shop-weight vouchers

`draw_stock_card` (`board.rs:1631`) rolls Joker 20 / Tarot 4 / Planet 4 out of
28. Tarot Merchant doubles the tarot band, Planet Merchant the planet band, the
Tycoons quadruple — so the roll's denominator and thresholds become voucher-aware
(e.g. Tarot Tycoon → Tarot 16, so 20/16/4 out of 40). The joker/consumable
*partition* stays exact; only the weights move.

---

## Work Items

### Phase 0 — Prerequisites — **Complete 2026-07-17**

- [x] **0a.** `vouchers__an_empty_set_is_inert_in_the_recompute`: a board with an
  empty `vouchers` set recomputes to its baseline draws and keeps the default
  slot counts after a blind select. The guard every later phase keeps green —
  the moment Phase 2's `recompute_draws` arm reads a voucher, this still holds
  for the empty set.

### Phase 1 — `Voucher`, board state, shop slot, redeem — **Complete 2026-07-17**

- [x] **1a.** New `src/funky/types/voucher.rs`: the 20-variant `Voucher` enum,
  `Voucher::ALL` (the draw pool), `requires()` (the base→upgrade prerequisite as
  data), and `Display` (wiki names). Registered `pub mod voucher;`
  (`src/funky/types/mod.rs`) and exported from `src/preludes/funky.rs`. 4 unit
  tests: `ALL` is 20 unique; each upgrade pairs to its base; every required base
  is itself a base (chains are one deep); Display. **Deviation:** Antimatter has
  no prerequisite here — Balatro gates it behind the Blank voucher, which does
  nothing and is out of scope, so the elision is invisible.
- [x] **1b.** `BuffoonBoard.vouchers: Vec<Voucher>` (init empty, `board.rs:266`)
  and `Shop.voucher: Option<Voucher>` (`shop.rs`), both defaulting so an
  un-shopped board is unchanged.
- [x] **1c.** `open_shop_with_rng` draws the voucher slot from `eligible_vouchers`
  (not redeemed, prerequisite met); drawn **after** stock and packs so the
  EPIC-01b RNG order is undisturbed. `redeem_shop_voucher()`: $10 vs the debt
  floor (`buy_stock`'s floor, so Credit Card composes), refusals for empty slot /
  already-held / unmet prerequisite, then push + clear the slot. 8 board tests:
  redeem adds and charges $10; refuses without money; an upgrade refuses without
  its base and succeeds with it; refuses an empty slot; a fresh shop offers an
  eligible voucher; a redeemed voucher never re-offers (64 seeds); an upgrade is
  offered once its base is held (256 seeds), and no upgrade is ever offered
  without its base.

### Phase 2 — Draws vouchers — **Complete 2026-07-17**

- [x] **2a.** One voucher arm in `recompute_draws` (`board.rs:1552`), read live
  like the jokers: Grabber/Nacho Tong → `hands_to_play`, Wasteful/Recyclomancy →
  `discards`, Paint Brush/Palette → `hand_size`. Placed **before** the
  discard-wipe and the boss ability, so the two ordering rules hold. 9 tests
  through a real `on_blind_selected`: each of the six at its value (5 hands / 6 /
  4 discards / 5 / hand size 9 / 10); The Needle still overrides Grabber to 1
  hand (proves the arm is before the boss); Burglar still zeroes a Wasteful
  discard (proves it is before the discard-wipe); and a permanent voucher adds
  its bonus **once**, not once per blind, across three `on_blind_selected`s
  (recompute-from-baseline, the Phase 0a inertness property holding for a
  non-empty set). The upgrade-needs-base rule is enforced at redeem (Phase 1c),
  so an unheld Nacho Tong never reaches the board to be read here.

### Phase 3 — Slot vouchers — **Complete 2026-07-17**

- [x] **3a.** Crystal Ball / Antimatter bump `consumable_slots` / `joker_slots`
  in `redeem_shop_voucher` (a permanent, once-guarded bump — the slot fields have
  no recompute pass, unlike the Draws vouchers). `has_consumable_room` /
  `has_joker_room` read those fields, so the new room appears immediately. Tests:
  each bumps only its own slot; and the end-to-end `crystal_ball__opens_room_for_a_third_consumable`
  — a full inventory refuses `buy_stock` of a third consumable, and redeeming
  Crystal Ball opens exactly the room for it.
- [x] **3b.** Overstock/Plus size the shop's card slots **live** at open via
  `overstock_bonus()` (there is no shop-slot field to bump — the count is
  computed each `open_shop_with_rng`): +1 for Overstock, +1 more for Overstock
  Plus (which requires Overstock, so holding it means holding both). Tests: a
  shop opened with Overstock offers 3 stock cards, with both offers 4.
- **Two mechanisms, by target.** Board slots (Crystal Ball/Antimatter) are
  persistent fields, so they bump once at redeem; the shop's card-slot count
  (Overstock) has no field and is read live at open — the same live-vs-bump split
  as Draws-vouchers-vs-board-slots, chosen by whether a persistent field exists
  to hold the effect.

### Phase 4 — Economy vouchers — **Complete 2026-07-17**

- [x] **4a.** `interest_cap()` — the keystone — unifies the two duplicated
  interest sites (`cash_out` and the `ExtraInterest` payout) onto one reader: $5
  base, $10 Seed Money, $20 Money Tree. Both `.clamp(0, 5)` became
  `.clamp(0, self.interest_cap())`, so Seed Money raises the cap in one place and
  base interest and To the Moon can never disagree. Tests: $60 held with Seed
  Money cashes out $10 interest (was $5); Money Tree caps $200 at $20; To the
  Moon reads the same raised cap ($10, not $5), the both-sites pin.
- [x] **4b.** `reroll_discount()` in `reroll_cost`: $2 per Reroll Surplus / Glut
  held (Glut requires Surplus, so both = $4), `saturating_sub` flooring at $0.
  Tests: Surplus makes a $5 reroll $3 (and $6 → $4); Glut makes it $1.
- [x] **4c.** `discounted(price)` — 25% off with Clearance Sale, 50% with
  Liquidation (supersedes, not stacks), `((price*(100-pct))/100).max(1)` (floor
  $1, never free) — applied in `buy_stock` and `open_pack_with_rng`, not to the
  $10 voucher price. Tests: Blue Joker $5 → $3 / $2; a $4 pack → $3. The `max(1)`
  floor is defensive — the cheapest in-scope item ($3) at 50% is already $1, so
  no in-scope card reaches it, but it keeps a future $1–$2 item from going free.

### Phase 5 — Shop-weight vouchers

- [ ] **5a.** Voucher-aware weights in `draw_stock_card`. Seeded distribution
  test: with Tarot Tycoon, tarots are ~4× their base share and the joker
  partition is still exact (only piled jokers appear).
- [ ] **5b.** Roadmap/doc: flip the EPIC-01 Story 3/7 voucher rows; note the
  deferred edition/ante/pack-content vouchers with their blockers.

---

## Test Plan

- One `voucher__` test per wired voucher at its exact wiki value, failing before
  its reader lands (Gold Standard).
- Prerequisite pins: an upgrade refuses without its base; redeeming the base then
  the upgrade stacks; a redeemed voucher never re-offers.
- Inertness: a board with no vouchers is byte-identical to a pre-EPIC-01c board
  across a full round + shop (Phase 0a, kept green throughout).
- Ordering pins: the boss ability still overrides Grabber; interest and To the
  Moon read one `interest_cap()`.
- Distribution: Tarot/Planet weight shifts move the band but keep the joker
  partition exact (the EPIC-01b sweep guarantee).

## Key Files

| File | Role |
|---|---|
| `src/funky/types/voucher.rs` | new — `Voucher` enum + `requires()` |
| `src/funky/types/board.rs` | `vouchers` field, `redeem_shop_voucher`, the `recompute_draws` / slot / `reroll_cost` / `interest_cap` / `draw_stock_card` readers |
| `src/funky/types/shop.rs` | `voucher: Option<Voucher>` slot |
| `src/funky/types/buffoon_card.rs` | `BCardType::Voucher` stays the display tag; no longer inert |
| `src/preludes/funky.rs` | export `Voucher` |

## Reuse (do NOT recreate)

- `recompute_draws` (`board.rs:1542`) — the Draws vouchers are new readers of the
  rebuild-from-baseline pass the draw-modifier jokers already use; do not add a
  parallel draws mutation.
- The debt floor & buy path (`buy_stock`, `debt_floor`, EPIC-01b) — redeeming is
  a third spender of that floor, not a new economy.
- `joker_slots` / `consumable_slots` (`board.rs:167,172`) — real limits already
  enforced by `has_*_room`; bump them, do not shadow them.
- The interest rule (`board.rs:2047,2224`) — unify the two existing readers
  rather than adding a third divergent one.
- `draw_stock_card` (`board.rs:1631`) & the rarity-pile partition — re-weight the
  bands; keep the partition.

## Compatibility

**Preserves** every existing score, draw, and shop behaviour — `vouchers`
defaults empty and `Shop.voucher` to `None`, so a board that redeems nothing is
byte-identical to a pre-EPIC-01c board (Phase 0a pins it). **Adds** the `Voucher`
type, the voucher slot, one `recompute_draws` arm, the slot bumps, the economy
readers, and the weight bias. **Breaks** nothing.

## Dependencies

- **Built on:** EPIC-01a (economy, round loop, slot limits, `recompute_draws`),
  EPIC-01b (the shop, debt floor, buy path, stock draw).
- **Blocks:** nothing hard — but Antimatter/Illusion/Hone want editions, and
  Hieroglyph/Director's Cut want ante progression, so this EPIC bounds where the
  edition and ante EPICs will pick up.
- **Related:** EPIC-01 Story 7 (this is its voucher item, executed); a future
  Editions EPIC and Ante-progression EPIC (the deferred vouchers' referents).

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features            # funky must not leak into no_std
cargo fmt --all -- --check
cargo doc --no-deps --all-features           # RUSTDOCFLAGS="-D warnings"
```

Exit criteria (per phase):

1. Every wired voucher has a test at its exact Balatro value that failed before
   its reader landed.
2. A board that redeems no voucher is byte-identical to before (Phase 0a green).
3. The base → upgrade prerequisite is enforced at redeem and at offer.
4. The Status table row flips to **Complete** only with cited, tested code.
