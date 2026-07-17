# EPIC-01b: The Shop (SHOP)

> **Follow-on to [`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 7, sibling of the
> closed [`EPIC-01a_Joker_Wiring_Backlog.md`](./EPIC-01a_Joker_Wiring_Backlog.md).**
> EPIC-01a built the economy (money, payouts, interest semantics), the round
> loop, and — in its final sweep — made every joker's rarity, cost, and sell
> value trustworthy. The shop is what spends that: the missing step between
> `on_round_end` and the next `on_blind_selected`, where money buys jokers and
> consumables drawn from the (now sound) rarity piles.

**Date:** 2026-07-16 · **Branch:** `funky` · **Status:** planned

---

## Context

The board can earn, hold, and pay out money, but nothing can *spend* it except
selling in reverse:

- `BuffoonBoard.money: isize` (`src/funky/types/board.rs:79`), written by the
  payout seam in `on_round_end` (`board.rs:1708`) / `on_round_end_with_rng`
  (`board.rs:1743`);
- `sell_joker` (`board.rs:1553`) pays out `resell_value` — now correct for all
  112 jokers after the 2026-07-16 sweep (sell = ⌊cost/2⌋ min $1, pinned by
  `all_jokers__resell_value_is_half_cost_floored_at_one`);
- real slot limits exist: `joker_slots` (`board.rs:152`), `consumable_slots`
  (`board.rs:157`), enforced by `create_consumable` (`board.rs:1055`);
- the round loop (EPIC-01a §The round loop) runs blind select → play/discard →
  round end, and `Blind::{Small, Big, Boss}` identity is on the board
  (`src/funky/types/blind.rs`).

Two joker consts are **declared and silently inert** — the assigned-but-unread
class EPIC-01a's guards exist for, invisible here because neither is a scoring
effect: **Credit Card** carries `MPip::Credit(20)` (`src/funky/decks/joker.rs:435`)
and **Chaos the Clown** carries `MPip::FreeReroll(1)` (`joker.rs:577`). Nothing
reads either; the shop is their referent.

**What this EPIC does NOT do.** No vouchers (the $10 slot stays unmodelled), no
editions (foil/holo/poly/**Negative** — so **Perkeo stays `Blank`**), no tags
(**Diet Cola stays `Blank`**), no spectral packs (spectral cards still do not
exist — EPIC-01 Story 3), no playing-card shop stock (base game gates that
behind vouchers anyway). It builds the smallest shop a run loop can spend money
in: cash-out, stock, buy, reroll, booster packs.

## Status

| Component (phase) | Unblocks | Status |
|---|---|---|
| 1 — Cash-out (blind reward + $/hand + interest) | the economy actually cycles; To the Moon gets its base to stack on | Planned |
| 2 — Shop state, stock draw, buying | Credit Card (`Credit(20)` debt floor) | Planned |
| 3 — Reroll | **Flash Card**; Chaos the Clown (`FreeReroll(1)`) | Planned |
| 4 — Booster packs (buy / skip / open) | **Red Card** (skip), **Hallucination** (open) | Planned |

## Goals

- Close the **economy cycle**: rounds pay a real cash-out, the shop spends it,
  and the next blind starts with the consequences.
- Give the two silently-inert money consts (**Credit Card**, **Chaos the
  Clown**) their referent, and wire the three shop-blocked jokers (**Flash
  Card**, **Red Card**, **Hallucination**) with exact wiki values.
- Draw shop stock from the **rarity piles** at Balatro's real odds — the sweep
  made the piles trustworthy precisely so this phase could trust them.
- Every wired behaviour gets a test that failed before its arm landed
  (Gold Standard, EPIC-00f), and every random draw rides the existing seeded-RNG
  seam — no hand-rolled randomness.

## Scope

Wiki-verified rules this EPIC must obey (balatrowiki.org, fetched 2026-07-16):

- **Cash-out** (end of won round): blind reward **$3 / $4 / $5**
  (Small/Big/Boss) + **$1 per unused hand** + interest **$1 per $5 held, capped
  at $5** (money above $25 earns nothing). All three components are computed
  from the balance *before* any of them is added — the same pre-event-board
  rule `apply_payouts` already follows, and the base To the Moon's
  `ExtraInterest(1)` (EPIC-01a 1c) was designed against.
- **Shop**: opens after every defeated blind; **2 card slots** and **2 booster
  pack slots** (the 1 voucher slot is out of scope). Card-slot contents by
  weight: **Joker 20 / Tarot 4 / Planet 4**; a joker slot rolls rarity at
  **70% Common / 25% Uncommon / 5% Rare** (Legendary never appears in the
  shop). Tarot/Planet stock costs **$3**; a joker costs its `rank.value`.
- **Buying**: refused without room (the `create_consumable` rule) or without
  money. The money floor is **$0 — or −$20 with Credit Card held**
  (`MPip::Credit(20)` read live, like Chicot's disable).
- **Reroll**: **$5 base, +$1 per reroll, resets each shop**; rerolls redraw the
  card slots only, never packs. **Chaos the Clown** grants 1 free reroll per
  shop.
- **Booster packs**: priced **$4** (the base tier; Jumbo $6 / Mega $8 are data,
  not new mechanics). Skipping fires an event (Red Card); opening fires an
  event (Hallucination: 1-in-2 to create a Tarot, "(Must have room)", rolled
  through `probability_numerator` so Oops! All 6s doubles it).

## Domain map

| Balatro term (wiki) | What it needs | funky construct to add |
|---|---|---|
| Cash Out screen | end-of-round income | `CashOut` computed in `on_round_end` |
| interest | pre-payout balance read | already the `apply_payouts` convention |
| the shop | between-rounds state | `BuffoonBoard.shop: Option<Shop>` |
| card slots | weighted stock draw | `Shop::stock`, drawn from the rarity piles |
| "Reroll $5" | escalating per-shop cost | `Shop::rerolls_used` + `reroll_cost()` |
| free reroll (Chaos) | joker-read discount | `FreeReroll` summed live from `jokers` |
| "go into debt" (Credit Card) | a buy floor | `debt_floor()`: 0 or −20, read live |
| booster pack | buy → skip/open → choose | `BoosterPack` + two `GrowthEvent`s |

## Design

### Phase 1 — Cash-out *(keystone)*

No new type; three components summed in `on_round_end`, all computed from the
pre-cash-out balance, applied as one delta beside `apply_payouts`' sum:

```rust
// board.rs — inside on_round_end, before joker payouts are applied
fn cash_out(&self) -> isize {
    let reward = match self.blind { /* Small 3, Big 4, Boss 5 */ };
    let per_hand = self.hands_remaining();            // board.rs:1265, $1 each
    let interest = (self.money / 5).clamp(0, 5);      // $1 per $5, cap $5
    reward + per_hand as isize + interest
}
```

**Ordering rule (the phase's one trap):** interest and To the Moon's
`ExtraInterest` read the **same** pre-cash-out balance — Balatro's cash-out
screen shows every line computed from the money you walked in with. Rocket's
grow-then-pay ordering (EPIC-01a item 8) is untouched: growth, then payouts
*and* cash-out from the pre-event board, then destruction.

Cash-out fires only when the round is won (`round_is_won`, `board.rs:1282`);
an untargeted round (`blind_target == 0`, the current caller mode) pays no
blind reward but still cashes out hands and interest — stated here so the
decision is a decision, not drift.

### Phase 2 — `Shop` (new `src/funky/types/shop.rs`)

```rust
pub struct Shop {
    pub stock: Vec<BuffoonCard>,     // the 2 card slots (jokers / tarots / planets)
    pub packs: Vec<BoosterPack>,     // the 2 pack slots
    pub rerolls_used: usize,
}
// BuffoonBoard gains:
pub shop: Option<Shop>,              // None = closed; opened by open_shop_with_rng
```

`open_shop_with_rng(&mut self, rng)` draws stock at the wiki weights
(20/4/4 → joker rarity 70/25/5 from `Joker::{COMMON,UNCOMMON,RARE}_JOKERS`,
`joker.rs:8,74,125`). There is deliberately **no pure `open_shop`** — a shop
without RNG has no stock to draw, exactly as `on_blind_selected_with_rng`
(`board.rs:1606`) exists for Riff-Raff. `buy_stock(index) -> bool` routes a
joker through `push_joker` (respecting `joker_slots`) and a consumable through
`create_consumable` (`board.rs:1055`); it refuses below the debt floor:

```rust
fn debt_floor(&self) -> isize {      // MPip::Credit read live, the Chicot pattern
    if self.jokers_carry_credit() { -20 } else { 0 }
}
```

Buying a joker does **not** fire `CardAdded` (`GrowthEvent`, `board.rs:28`) —
Hologram counts *playing cards added to the deck*, and a bought joker is
neither. The negative test is in the plan.

### Phase 3 — Reroll

```rust
pub fn reroll_cost(&self) -> usize {
    let free = self.free_rerolls();          // Σ MPip::FreeReroll across jokers
    let used = self.shop.as_ref().map_or(0, |s| s.rerolls_used);
    if used < free { 0 } else { 5 + (used - free) }
    // the first `free` rerolls cost $0; paid ones start at $5 and climb $1 each
}
```

`reroll_with_rng` pays, redraws `stock` (never `packs`), increments
`rerolls_used`, and fires a new `GrowthEvent::ShopRerolled` — **Flash Card**
(`MultPlusPerReroll(2)`) is a plain `joker_state` counter on it, the Green
Joker shape, not retroactive.

### Phase 4 — Booster packs

```rust
pub struct BoosterPack { pub kind: PackKind, pub cost: usize }   // Buffoon | Arcana | Celestial
```

`skip_pack(index)` fires `GrowthEvent::PackSkipped` → **Red Card**
(`MultPlusPerPackSkipped(3)`), a counter. `open_pack_with_rng(index)` draws the
pack's choices from the existing piles/decks, fires `PackOpened` →
**Hallucination** rolls its 1-in-2 through `probability_numerator`
(`board.rs:706`) and respects consumable room; the player's pick lands through
the same seams as buying. Contents stay minimal: a Buffoon pack draws jokers
(rarity-rolled), Arcana draws tarots, Celestial draws planets — spectral packs
need cards that do not exist and stay out.

## Work Items

### Phase 0 — Prerequisites

- [ ] **0a.** Extend the `GrowthEvent` seam (`board.rs:20`) with `ShopRerolled`,
  `PackSkipped`, `PackOpened`. The exhaustive matches in `growth_delta`
  (`board.rs:1142`) / `payout_delta` (`board.rs:1809`) will not compile until
  every arm decides, which is the point.
- [ ] **0b.** Triage test for the two inert consts: pin that `Credit(20)` and
  `FreeReroll(1)` currently change nothing (the characterization that fails
  the day they wire, EPIC-01a's Splash discipline).

### Phase 1 — Cash-out *(keystone)*

- [ ] **1a.** `cash_out()` + its application in `on_round_end`; blind reward
  read from `self.blind`. Tests per component and one for the pre-balance rule:
  interest on $23 with To the Moon held pays $4 + $4, never compounding.
- [ ] **1b.** Round-loop integration: `round_loop__a_won_round_cashes_out`
  extends the EPIC-01a composition test through a full earn→spend cycle.

### Phase 2 — Shop & buying

- [ ] **2a.** `Shop` type, `BuffoonBoard.shop`, `open_shop_with_rng` at the
  20/4/4 and 70/25/5 weights (seeded tests pin the distribution shape, not
  exact draws).
- [ ] **2b.** `buy_stock`: money check (with `debt_floor`), slot check, routing.
  **Credit Card** wired — buying at −$19 succeeds with it, refuses without it.
- [ ] **2c.** Negative guard: buying a joker fires no `CardAdded` (Hologram
  stays still).

### Phase 3 — Reroll

- [ ] **3a.** `reroll_cost` / `reroll_with_rng`: $5, $6, $7…; resets on the
  next `open_shop_with_rng`; packs untouched by redraw.
- [ ] **3b.** **Chaos the Clown** wired: first reroll $0, second $5. Two Chaos
  = two free (the Stencil per-copy rule, verified against source before
  wiring).
- [ ] **3c.** **Flash Card** wired: `Blank` → `MultPlusPerReroll(2)`, +2 mult
  per reroll, exact-value test.

### Phase 4 — Booster packs

- [ ] **4a.** `BoosterPack` + buy/skip: **Red Card** `Blank` →
  `MultPlusPerPackSkipped(3)`.
- [ ] **4b.** `open_pack_with_rng` + choose: **Hallucination** `Blank` → its
  1-in-2 tarot, seeded; three Oops! All 6s make it certain on every seed (the
  Gros Michel pin, EPIC-01a 1c).
- [ ] **4c.** `BLANK_WITH_REASON` shrinks by three; the guard
  (`all_jokers__every_blank_joker_has_a_stated_reason`) fails if a wired joker
  stays listed, which is the close-out check.

## Test Plan

- One `score__`/`shop__` test per wired joker at its exact wiki value, failing
  before the arm lands (Gold Standard).
- Ordering pins: interest pre-balance; reroll reset; free-before-paid rerolls.
- Inertness: a board that never opens a shop scores byte-identically to today.
- Distribution: seeded stock draws only ever produce piled jokers — the sweep's
  partition guard makes "piled" and "exists" the same set.

## Key Files

| File | Role |
|---|---|
| `src/funky/types/shop.rs` | new — `Shop`, `BoosterPack`, `PackKind` |
| `src/funky/types/board.rs` | `shop` field, cash-out, buy/reroll/pack methods, new `GrowthEvent` arms |
| `src/funky/types/mpip.rs` | `MultPlusPerReroll`, `MultPlusPerPackSkipped`, Hallucination's variant (+ `Display`) |
| `src/funky/decks/joker.rs` | flip Flash Card / Red Card / Hallucination from `Blank`; shrink `BLANK_WITH_REASON` |
| `src/funky/types/blind.rs` | blind reward values |

## Reuse (do NOT recreate)

- The payout convention — one delta from the pre-event board (`apply_payouts`,
  EPIC-01a 1c). Cash-out is a third reader of that rule, not a new one.
- `create_consumable` / `use_consumable` (`board.rs:1055,1090`) — buying and
  pack-choosing route through them; "(Must have room)" is already enforced.
- The rarity piles (`joker.rs:8,74,125`) — stock draws trust them because the
  2026-07-16 sweep reconciled them; do not build a parallel catalog.
- `probability_numerator` (`board.rs:706`) — Hallucination's roll; Oops! All 6s
  composes free.
- The `_with_rng` split (`score_with_rng`, `on_blind_selected_with_rng`) — the
  shop's RNG methods follow it; pure paths stay deterministic.
- `sell_joker` (`board.rs:1553`) — selling already works; the shop adds no
  second sell path.

## Compatibility

**Preserves** every existing score and hook — `shop` defaults to `None` and no
new event fires on a board that never opens one. **Adds** the shop surface,
three `GrowthEvent`s, and four `MPip` variants. **Breaks** nothing.

## Dependencies

- **Built on:** EPIC-01a Phases 1 (money/payouts), 5c (consumable seam & slot
  limits), 8 (blinds), the round loop, and the rarity/cost/pile sweep.
- **Blocks:** vouchers and editions (EPIC-01 Story 3/7 remainders); Perkeo
  (Negative editions) and Diet Cola (tags) stay `Blank` past this EPIC.
- **Related:** EPIC-01 Story 7 (this is its shop item, executed).

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features            # funky must not leak into no_std
cargo fmt --all -- --check
```

Exit criteria (per phase):

1. Every wired joker/const has a test at its exact Balatro value that failed
   before the arm landed.
2. A board that never opens a shop is byte-identical to before (new state inert
   by default).
3. The Status table row flips to **Complete** only with cited, tested code.
