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
  (`src/funky/types/buffoon_pile.rs:185`)
- [ ] **`has_royal_flush` is HACKY** — same cluster as above.
  (`src/funky/types/buffoon_pile.rs:377`)
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

- [ ] **Blackboard / Abstract Joker share `weight: 895`** — weights should be
  unique. (`src/funky/decks/joker.rs`; EPIC-01a §Data fixes)
- [ ] **Baron mis-tagged Common/$5, should be Rare/$8** and belongs in
  `RARE_JOKERS`. (EPIC-01a §Data fixes)
- [ ] **~59 defined-but-unpiled joker consts** want one reconciling rarity/cost/
  pile sweep. (EPIC-01a §Data fixes)
- [ ] **Cavendish missing its 1-in-1000 destroy chance** — latent until a
  round-end hook exists. (EPIC-01a §Data fixes)
- [ ] **Stone card scores 0** — needs +50 chips *and* hand-type suppression
  together; tracked in `KNOWN_UNWIRED_CARD_ENHANCEMENTS`. (EPIC-01a §Data fixes)

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
