# Backlog

> Index of outstanding work, aggregated from EPIC docs, GitHub issues, and code
> comments by the `/backlog` skill (last refreshed 2026-07-15, branch `funky`;
> EPIC-01 family sections updated 2026-07-18 at EPIC-01's close-out, `e50fdd0`).
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

### EPIC-01b — The Shop ([docs/EPIC-01b_Shop.md](docs/EPIC-01b_Shop.md)) — **complete** (closed out 2026-07-17)

All four phases landed: cash-out, stock/buying (Credit Card debt floor),
rerolls (Flash Card, Chaos the Clown), booster packs (Red Card, Hallucination).

### EPIC-01c — Vouchers ([docs/EPIC-01c_Vouchers.md](docs/EPIC-01c_Vouchers.md)) — **complete** (closed out 2026-07-17)

`Voucher` enum + $10 shop slot; 20 in-scope vouchers wired at exact wiki values
across draws, slots, economy, and shop weights. The edition/ante/pack-content
subset (12) deferred onto its subsystems.

### EPIC-01d — Editions ([docs/EPIC-01d_Editions.md](docs/EPIC-01d_Editions.md)) — **complete** (closed out 2026-07-17)

Foil/Holo/Polychrome fold into played-card and joker scoring; Negative is a
live slot exemption; Perkeo (the last `Blank` Legendary) wired. Edition
*sourcing* (shop rolls, frequency vouchers) deferred.

### EPIC-01e — Spectral cards ([docs/EPIC-01e_Spectral_Cards.md](docs/EPIC-01e_Spectral_Cards.md)) — **complete** (Phases 0–3, closed out 2026-07-18)

18-card deck, Sixth Sense/Séance create-path, 14 of 18 effects wired. The four
seal spectrals stay `Blank`, deferred to a future Seals EPIC.

### EPIC-01 — Funky/Balatro engine ([docs/EPIC-01_Funky.md](docs/EPIC-01_Funky.md)) — **✅ closed out** (2026-07-18, `e50fdd0`)

The parent EPIC is **Complete with named deferrals** — see its Implementation
corrigendum for the full deferral register. 769 lib + 10 integration tests +
101 doctests green; all five gates pass. What it deferred (each a future EPIC,
none in-flight):

- [ ] **Seals** — 4 seal spectrals, red-seal retriggers, seal scoring; the last
  ❌ subsystem.
- [ ] **Antes & boss blinds** — ante progression, ~17 more bosses, per-hand
  boss triggers (unblocks Matador).
- [ ] **Decks** — 13 remaining Balatro decks (Red, Blue, Yellow, Green, Black,
  Magic, Nebula, Ghost, Erratic, Painted, Anaglyph, Plasma, Zodiac).
- [ ] **Tags** (unblocks Diet Cola); **draw step / mutation hooks** (unblocks
  DNA, Trading Card, Lucky Cat, To Do List, Mail-In Rebate).
- [ ] **Serde on funky types** + serde-stable string ids for mod effects.
- [ ] **Edition sourcing** + the 12 deferred vouchers; a Spectral `PackKind`.
- [ ] **Supernova & Loyalty Card** — the two carried-but-unscored (silent-zero)
  `MPip` variants (`MultPlusOnHandPlays`, `MultTimesEveryXHands`).

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

- [x] ~~`determine_hand_type` / `has_royal_flush` "HACKY" markers in
  `src/funky/types/buffoon_pile.rs`~~ — **resolved 2026-07-18** (cascade
  documented; royal-flush check made order-independent via Ace+King anchors).
- [ ] `Pile` → `VecDeque` consideration; treat vector end as top of deck
  (`pile.rs:647`, `basic_card.rs:31`).
- [ ] Closure-accepting common function for the `combos.rs` duplication (`combos.rs:99`).

## Docs / notes (de-emphasized)

- `docs/EPIC-01_Funky_Progress.md`, `docs/superpowers/{plans,specs}/2026-07-13-funky-retriggers*`
  are marked superseded — historical only.
- `docs/wasm.md` documents the verified wasm32 feature matrix; no open work.
