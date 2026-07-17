# Technical Debt

> Maintained by the `/backlog` skill (created 2026-07-15). Items tagged 🤖 were
> proposed by automated review — review and edit them; they are suggestions,
> not facts. Everything under "Tracked debt" is sourced from code comments the
> authors wrote (`TODO` / `TODO RF` / `TODO: HACK`) or from EPIC docs.

## Tracked debt

### Refactors (`TODO RF`)

- [ ] **Common closure-accepting combinator in `Combos`** — `of_same_rank` and
  siblings duplicate their fold shape; one function taking a closure would
  collapse them. (`src/basic/types/combos.rs:99`)
- [ ] **Treat the end of the vector as the top of the deck** — restructure so
  deal-from-top is `pop()` instead of `remove(0)`. (`src/basic/types/basic_card.rs:31`)
- [ ] **Consider `VecDeque` for `Pile`** — related to the item above; front
  removals are O(n) today. (`src/basic/types/pile.rs:647`)
- [ ] **`jokers()`-style accessors: return a plain (possibly empty) vector**
  instead of the current shape. (`src/basic/types/traits.rs:424`)
- [ ] **More configurable `fluent_connector`** — per-locale connector is
  hard-coded. (`src/basic/types/card.rs:239`)

### Hacks (`TODO: HACK`)

- [ ] **`determine_hand_type` is HACKY** — sequential if-chain hand detection;
  also tracked in EPIC-01 ("Resolve the HACKY markers").
  (`src/funky/types/buffoon_pile.rs:235`)
- [ ] **`has_royal_flush` is HACKY** — same cluster as above.
  (`src/funky/types/buffoon_pile.rs:430`)
- [ ] **`Pip::PRIMES` const marked HACK** — a hard-coded 60-prime table used for
  weighting. (`src/basic/types/pips.rs:98`)
- [ ] **i18n `fluent_*` path marked HACK** — author notes fluent-templates may
  be outliving its usefulness; "deck from yaml" floated as the successor
  direction. (`src/basic/types/card.rs:217`, `src/basic/types/card.rs:260`)

### Design notes / open questions (de-emphasized)

- **Unlimited pip-types-in-a-vector idea** deferred until after the current
  version. (`src/basic/types/pips.rs:39`)
- **Truncated TODO** — comment ends mid-sentence ("…my friend Jim Prior. TODO:");
  intent unrecoverable, ask the author or delete. (`src/basic/types/combos.rs:66`)
- **"Why are these in reverse order?"** — question about `unsuited()` combo
  ordering, in a test. (`src/basic/types/combos.rs:342`)
- **Slices note** (`src/basic/types/traits.rs:116`) and **abstraction-limits
  note** (`src/basic/types/traits.rs:287`) — reflections, not actionable work.
- `src/basic/types/card.rs:96` carries a `TODO RF` that is immediately followed
  by "DONE!!!" — the comment can simply be cleaned up.

### From EPIC docs (single source of truth is the EPIC; listed here for the debt view)

> **Refreshed 2026-07-16, when EPIC-01a closed out.** Four of the five items
> previously listed here were already fixed and had gone stale — the risk of a
> debt view that mirrors an EPIC rather than deriving from it. Resolved:
> Blackboard/Abstract's `weight: 895` clash (0a — one of **14** collisions, not
> the 1 the EPIC flagged), Baron's rarity (0a), Cavendish's missing 1-in-1000
> destroy chance (1c), and Erosion/Stone Joker's rarity (7c).

- [x] ~~**~50 defined-but-unpiled joker consts** want one reconciling rarity/cost/
  pile sweep.~~ — **Swept 2026-07-16**, against balatrowiki.org. 52 consts
  reconciled (47 adrift + Mystic Summit + 4 resell-only stragglers): every joker
  now carries its wiki rarity, cost, and sell value, and sits in its rarity's
  pile (COMMON 26→56, UNCOMMON 26→41, RARE 8→10; the piles now partition
  `ALL_JOKERS`). Two new guards keep it fixed:
  `all_jokers__every_joker_is_piled_by_its_rarity` (the inverse of the superset
  guard — an adrift const can no longer hide) and
  `all_jokers__resell_value_is_half_cost_floored_at_one` (Balatro's sell rule as
  an invariant; `sell_joker` pays `resell_value` out for real, so a stale 0 was
  silently lost money). *Finding:* **Mystic Summit was piled and wrong** —
  Uncommon/$6 in `UNCOMMON_JOKERS`, but Common/$5 per the wiki infobox. A
  consistently misfiled joker is invisible to both guards; only reconciling
  against the source catches it. (EPIC-01a §Data fixes)

- [x] ~~**Stone card scores 0**~~ — **Fixed.** It needed +50 chips *and* hand-type
  suppression together, and both are in: `BuffoonCard::is_stone` masks the chips
  flat (a Stone Ace is 50, not 61) and `BuffoonPile::detectable` drops Stones from
  classification. `KNOWN_UNWIRED_CARD_ENHANCEMENTS` is now **empty** — as are its
  two siblings.

  *The recorded plan here was wrong.* "Blank the pips" models no-rank-no-suit as
  absent data; Balatro **masks** at the accessor layer over a preserved base
  (Vampire strips the enhancement and the rank returns). Filtering on the
  enhancement is load-bearing: blanked pips make every Stone identical, so two of
  them would pair with each other. See EPIC-01a §The Stone card.

- [x] ~~**Three jokers are `Blank` by omission**~~ — Card Sharp, Diet Cola and
  Ancient Joker had no recorded reason. Resolved: **two were never blocked at
  all** and are now wired (Card Sharp, Ancient Joker); Diet Cola has a real
  reason (Tags). Every `Blank` joker's reason now lives as **data** in
  `BLANK_WITH_REASON` (`src/funky/decks/joker.rs`), enforced by
  `all_jokers__every_blank_joker_has_a_stated_reason` and
  `blank_jokers__every_reason_names_a_blocker`. (EPIC-01a §The untriaged three)

- [ ] **`BuffoonPile::draw(n)` loses cards when the deck is short.** It pops one
  at a time and returns `None` if it cannot supply the full `n` — but the cards
  it already popped go with the dropped return value. Ask a 3-card deck for 5 and
  the deck ends up **empty with those 3 gone**. Either drain nothing on failure
  (check `len()` first) or return what it has; the second is what callers want,
  and is what `BuffoonBoard::deal_to_hand_size` does instead of using this.
  Found while building the round loop. (`src/funky/types/buffoon_pile.rs:277`)

- [ ] **`draw` and `draw_first` deal from opposite ends** — `draw(n)` uses `pop`
  (the end), `draw_first` uses `remove(0)` (the front). Related to the "treat the
  end of the vector as the top of the deck" refactor above; the round loop deals
  with `pop`. (`src/funky/types/buffoon_pile.rs:277,289`)

## 🤖 Automated review findings

<!-- Machine-proposed. Promote good ones up to "Tracked debt", delete the rest. -->
<!-- Deep review run 2026-07-15 (branch `funky`, post-merge 6d3ac11). -->

- [ ] 🤖 **`Pile::draw_random` panics on an empty pile, contradicting its own
  doc** — the doc says "If the `Pile` is empty, `None` is returned," but
  `rng.random_range(0..self.len())` panics on an empty range; only the
  non-empty case is tested. Suggested: early-return `None` when empty + an
  empty-pile test. (`src/basic/types/pile.rs:286`)
- [ ] 🤖 **`BasicPile::remove` / `BasicPileCell::remove` panic out-of-bounds,
  unlike the deliberately hardened `Pile::remove`** — `Pile::remove`
  (`pile.rs:648`) documents returning a blank card to avoid the `Vec::remove`
  panic; the guard was never ported to the other two, and neither has a unit
  test. Suggested: apply the same bounds check (or document the panic) + tests.
  (`src/basic/types/basic_pile.rs:182`, `src/basic/types/basic.rs:174`)
- [ ] 🤖 **`BuffoonPile::remove` — third copy of the same unguarded panic** —
  public API; current internal callers bounds-check first, so it's latent.
  Three copy-pasted `remove`s, one hardened and two not, is basic↔funky drift.
  Suggested: harden to match `Pile::remove` + test. (`src/funky/types/buffoon_pile.rs:454`)
- [ ] 🤖 **`BuffoonPile::forgiving_from_str` swallows parse errors silently,
  unlike its `basic` counterpart** — `Pile::forgiving_from_str` logs a
  `log::warn!` on invalid input; the funky version returns an empty pile with
  no diagnostic, and it backs the `bcards!` macro (`src/funky/macros.rs:545`)
  used across the joker test suite — a typo'd card string silently truncates a
  hand. Suggested: add the same `log::warn!` for parity. (`src/funky/types/buffoon_pile.rs:264`)

Checked and ruled out by the review: no `unwrap`/`panic` in funky library code
(all in `#[cfg(test)]`), sign-loss casts in counter scoring are `.max(0)`-guarded
and explicitly allowed, no `std` leakage into `basic`, and the known
`ChanceDestroyed`/joker-wiring gaps are already tracked in EPIC-01a.
