# Demo: EPIC-01 Funky — Balatro-Style Cards

> A terminal-only walkthrough of the Balatro engine that EPIC-01 closed out:
> the four-phase scoring pipeline, editions, the shop and vouchers, spectral
> cards, and the test battery that makes the numbers trustworthy.

## Audience & framing

Engineering audience (teammate or code-curious stakeholder). The angle:
"a solver-grade, data-driven Balatro engine, built as a feature-gated layer on
a no_std card library — and every number you'll see matches the Balatro wiki."
Adjust the talking points down if the audience doesn't know Balatro; the
chips × mult idea carries the demo either way.

## Prerequisites

- Repo checked out at `ImperialBower/cardpack.rs`, branch `funky` (tip
  `e50fdd0` or later — the tour example must exist).
- Rust toolchain installed (edition 2024; any recent stable).
- One terminal pane, full-screen, font large enough to read emoji suit glyphs.
- No services, containers, or credentials — everything is `cargo run`.

## Setup (~2 minutes)

1. **Pre-warm the build so demo commands run instantly**
   ```bash
   cargo build --features funky --examples
   ```
   _Expected:_ `Finished` in under a minute on a warm cache.
   _Talking point:_ funky is feature-gated — off by default, the no_std core stays pure.

## The demo (~8 minutes)

### 1. The four-phase scoring pipeline (~2 min)

1. **Run the scoring example**
   ```bash
   cargo run --example buffoon --features funky
   ```
   _Expected:_ four phase lines ending `Final score = 11254`.
   _Talking point:_ base hand → played cards → held Steel ×1.5 → jokers left-to-right, exactly Balatro's order.

2. **Point at the phase lines** — each one folds into a running score; joker
   order matters and is tested both ways.

### 2. The grand tour — round, editions, shop, spectrals (~4 min)

1. **Run the tour**
   ```bash
   cargo run --example funky_tour --features funky
   ```
   _Expected:_ four banner'd acts, same output every run (seeded RNG).

2. **Act 1 — the round loop.** Point at the dealt hand, the played score, and
   the cash-out line (`$0 -> $6`).
   _Talking point:_ blind reward + $1 per unused hand + capped interest — the economy EPIC-01a built.

3. **Act 2 — editions.** Point at the four score lines: plain Droll 2718,
   Foil 3618 (+50 chips), Holographic 4228 (+10 mult), Polychrome 4077 (×1.5).
   _Talking point:_ an edition is orthogonal to an enhancement — same joker, one field changed.

4. **Act 3 — the shop.** Point at the stock slots, pack offers, the $10
   voucher, and money ticking $25 → $20 → $15 → $5.
   _Talking point:_ rerolls climb $5, $6, $7…; vouchers are run-wide modifiers read live.

5. **Act 4 — spectrals.** Point at Black Hole leveling Flush 1→2, then Hex
   leaving one survivor stamped `Polychrome`.
   _Talking point:_ spectrals are Balatro's high-risk consumables — Hex destroyed the other two jokers.

### 3. Why you can trust the numbers (~2 min)

1. **Run the battery**
   ```bash
   cargo test --features funky 2>&1 | grep "test result"
   ```
   _Expected:_ `769 passed` (lib), plus integration and doctest lines, 0 failed.
   _Talking point:_ every wired joker has an exact-wiki-value test.

2. **Show the honesty guard** (optional, if the audience is engineers)
   ```bash
   grep -n -A3 "BLANK_WITH_REASON" src/funky/decks/joker.rs | head -12
   ```
   _Expected:_ the 8-entry list, each joker with a stated reason.
   _Talking point:_ a test fails if a joker is Blank without a reason — or wired but still listed.

## What to highlight verbally

- Effects are **data, not code**: a 69-variant `MPip` enum on each card const,
  interpreted at scoring time — which is what makes a solver and a mod system
  possible on the same engine.
- The whole thing is **deterministic on demand**: `score()` never rolls;
  `score_with_seed(n)` rolls Lucky cards and Misprint reproducibly. Same seed,
  same run — that's the solver contract.
- The engine **reuses the mature core**: hand detection delegates to the
  `basic` engine's `Ranged` trait; funky only adds the Balatro layer.
- Deferred work is **named, not hidden**: 8 jokers and 4 spectrals stay
  `Blank` with test-enforced reasons, waiting on Seals/Tags/Antes EPICs.
- Mods plug in without touching a match arm: `MPip::Custom(id)` + an
  `EffectRegistry` resolve custom effects on played cards, held cards, and
  jokers.

## Likely questions & answers

- **Q: Can it play a full Balatro run?** A: Not yet — ante progression, boss
  roster, and seals are deferred to named future EPICs; the round loop, shop,
  and scoring within a round are complete.
- **Q: Why do those jokers print as glyphs, not names?** A: Cards are consts
  with `Pip` identity (index chars); a display-name layer is cosmetic and
  hasn't been needed by the solver goal.
- **Q: How do I know the values are right?** A: Each wired joker's test
  asserts the exact wiki value; the reachability guard proves every wired
  scoring variant actually fires on at least one probe board.
- **Q: Could I add my own joker without forking?** A: Yes — `MPip::Custom(id)`
  plus an `Effect` impl in an `EffectRegistry`, scored via
  `score_with_registry`; no core match arm changes.
- **Q: Why is the crate no_std but this needs std?** A: funky is deliberately
  a std-only feature (`funky = ["std"]`) layered above the 0.7.0 no_std core —
  the core stays embeddable.

## Cleanup

Nothing to tear down — no processes, containers, or state were started. If
you pre-built, `cargo clean` reclaims disk; otherwise close the terminal.
