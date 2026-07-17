# Backlog

> Index of outstanding work, aggregated from EPIC docs, GitHub issues, and code
> comments by the `/backlog` skill (last refreshed 2026-07-15, branch `funky`;
> EPIC-01a section updated 2026-07-16 at its close-out).
> Detail lives in the linked sources; tech debt detail lives in
> [`docs/TECHNICAL_DEBT.md`](docs/TECHNICAL_DEBT.md). Items tagged 🤖 are
> machine-proposed suggestions, not commitments.

## EPICs / Features

### EPIC-01a — Joker wiring ([docs/EPIC-01a_Joker_Wiring_Backlog.md](docs/EPIC-01a_Joker_Wiring_Backlog.md)) — **complete** (closed out 2026-07-16, `e9ceeca`)

All eight phases landed, plus the round loop and the Stone card as follow-ons;
618 lib tests, all five gates green. 29 jokers wired across the phases; the 14
still `MPip::Blank` each carry a test-enforced reason (`BLANK_WITH_REASON`,
`src/funky/decks/joker.rs`) naming the subsystem they wait on — spectral cards,
booster packs, the shop, tags, a draw step, in-fold effects, per-hand boss
abilities. Carried forward, tracked outside the EPIC:

- [x] ~~The reconciling rarity/cost/pile sweep over the ~50 remaining
  defined-but-unpiled joker consts.~~ Swept 2026-07-16: 52 consts reconciled
  against the wiki, piles now partition `ALL_JOKERS`, two new guards pin it
  (see TECHNICAL_DEBT for the full note, incl. the Mystic Summit misfile).
- [ ] `BuffoonPile::draw(n)` loses cards when the deck can't supply the full
  count — sidestepped by `deal_to_hand_size`, tracked in TECHNICAL_DEBT.

### EPIC-01b — The Shop ([docs/EPIC-01b_Shop.md](docs/EPIC-01b_Shop.md)) — planned, the next EPIC

Cash-out (blind reward + $/hand + interest), shop stock drawn from the
now-reconciled rarity piles, buying (Credit Card debt floor), reroll (Flash
Card, Chaos the Clown), booster packs (Red Card, Hallucination). Four phases,
all open; unblocks 3 of the 14 `Blank` jokers and wires the 2 silently-inert
money consts.

### EPIC-01 — Funky/Balatro engine ([docs/EPIC-01_Funky.md](docs/EPIC-01_Funky.md)) — parent EPIC

Still open beyond EPIC-01a's scope:

- [ ] Remaining Balatro decks (Red, Blue, Yellow, Green, Black, Magic, Nebula,
  Ghost, Erratic, Painted, Anaglyph, Plasma, Zodiac).
- [ ] Score/apply tarot effects in the scoring engine (declared, mostly unhandled).
- [ ] Spectral cards (18) and Vouchers (32) — nothing beyond the type tags.
- [ ] Tests for `decks/planet.rs` (has 2) and `decks/tarot.rs` (has 0).
- [ ] Editions / seals contributions to scoring (red seal retrigger, foil, holo…).
- [ ] Ante progression (blind state + three boss effects landed via EPIC-01a
  Phase 8; ante supplies no `blind_target` yet); shop (`sell_joker` landed;
  buying, rerolls, packs open). The round loop itself landed with EPIC-01a's
  close-out.
- [ ] Serde on funky types; CHANGELOG entries for the funky feature.
- Note: the doc's checkboxes are frozen at 2026-07-05 (`cc1595d`) and several are
  stale — e.g. "Joker-modified hand detection (Smeared)" (line 83) and "Retrigger
  mechanics" (line 94) have since landed via EPIC-01a Phases 4/6. The doc wants a
  status refresh.

### EPIC-02 — Ganjifa decks ([docs/EPIC-02_Ganjifa.md](docs/EPIC-02_Ganjifa.md)) — fully planned, not started

Mughal (96-card) + Dashavatara (120-card) decks with inverted per-suit pip
ladders, 5-locale Fluent i18n, and registry integration. Five stories, all open;
no core-type changes needed. Good "next EPIC" candidate when funky pauses.

## GitHub issues ([tracker](https://github.com/ImperialBower/cardpack.rs/issues))

- [ ] **#54 — Add logging** (enhancement, 2025-03): `log` is already a dependency
  (`log/std` in the `std` feature); wiring is unstarted.
- [ ] **#33 — Add Georgia Skin example** (enhancement, 2022-01): oldest open item;
  an `examples/` game demo like the existing four-phase funky demo.
- #65 — "Balatro like features" — tracked by EPIC-01/EPIC-01a above (same work, two refs).

## Refactors & tech debt

Tracked in [`docs/TECHNICAL_DEBT.md`](docs/TECHNICAL_DEBT.md) — 16 code-comment
items (`TODO RF` / `TODO: HACK`) plus automated review findings. Highlights:

- [ ] `determine_hand_type` / `has_royal_flush` "HACKY" markers in
  `src/funky/types/buffoon_pile.rs` (also EPIC-01 §hand-detection).
- [ ] `Pile` → `VecDeque` consideration; treat vector end as top of deck
  (`pile.rs:647`, `basic_card.rs:31`).
- [ ] Closure-accepting common function for the `combos.rs` duplication (`combos.rs:99`).

## Docs / notes (de-emphasized)

- `docs/EPIC-01_Funky_Progress.md`, `docs/superpowers/{plans,specs}/2026-07-13-funky-retriggers*`
  are marked superseded — historical only.
- `docs/wasm.md` documents the verified wasm32 feature matrix; no open work.
