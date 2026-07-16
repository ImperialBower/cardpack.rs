# Backlog

> Index of outstanding work, aggregated from EPIC docs, GitHub issues, and code
> comments by the `/backlog` skill (last refreshed 2026-07-15, branch `funky`).
> Detail lives in the linked sources; tech debt detail lives in
> [`docs/TECHNICAL_DEBT.md`](docs/TECHNICAL_DEBT.md). Items tagged 🤖 are
> machine-proposed suggestions, not commitments.

## EPICs / Features

### EPIC-01a — Joker wiring ([docs/EPIC-01a_Joker_Wiring_Backlog.md](docs/EPIC-01a_Joker_Wiring_Backlog.md)) — in progress, the active EPIC

Phases 0, 6, 7 complete; 1–5 partially done. Open work items, in the EPIC's own
leverage order:

- [ ] **1c — Lifecycle hooks** (`on_round_end`, `on_discard`, `on_scored` payouts):
  the economy keystone; pays out Golden Joker et al. and unblocks the round loop.
- [ ] **2c — `on_blind_selected()` hook + `Draws` mutators** (Burglar, Juggler, Drunkard).
- [ ] **3b/3c — Remaining counter jokers** (growth hooks + ~7 of 13 counter jokers:
  Vampire, Constellation, Hologram, Lucky Cat, Popcorn, Red Card, Fortune Teller,
  Flash Card + Canio/Yorick).
- [ ] **4c — Round-state retriggers**: Dusk (needs final-hand state), Seltzer
  (needs the 10-hand counter).
- [ ] **5c/5d/5e — Consumables, packs, blind-select creators** (DNA, Séance,
  Riff-Raff, Vagabond, …) — blocked on shop/consumable subsystems.
- [ ] **Phase 8 — Boss blinds** (Madness, Luchador, Matador, Chicot) — self-contained sub-EPIC.
- [ ] **Data fixes**: Baron rarity/cost (Rare/$8, currently Common/$5), Blackboard/
  Abstract Joker duplicate `weight: 895`, Stone card scoring (needs chips + hand-type
  suppression together), Cavendish's missing 1-in-1000 destroy chance (latent, wants
  the round-end hook first), and the reconciling rarity/cost/pile sweep over ~59
  defined-but-unpiled joker consts.

### EPIC-01 — Funky/Balatro engine ([docs/EPIC-01_Funky.md](docs/EPIC-01_Funky.md)) — parent EPIC

Still open beyond EPIC-01a's scope:

- [ ] Remaining Balatro decks (Red, Blue, Yellow, Green, Black, Magic, Nebula,
  Ghost, Erratic, Painted, Anaglyph, Plasma, Zodiac).
- [ ] Score/apply tarot effects in the scoring engine (declared, mostly unhandled).
- [ ] Spectral cards (18) and Vouchers (32) — nothing beyond the type tags.
- [ ] Tests for `decks/planet.rs` (has 2) and `decks/tarot.rs` (has 0).
- [ ] Editions / seals contributions to scoring (red seal retrigger, foil, holo…).
- [ ] Blind/ante progression; shop (buy/sell/reroll/packs); round loop consuming `Draws`.
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
