# Design: Open Effect Interpretation (Mod / Effect Registry)

**Date:** 2026-07-11
**Branch:** funky
**EPIC reference:** [`EPIC-01_Funky.md`](./EPIC-01_Funky.md) Story 8 — "Open effect interpretation"

---

## 1. Purpose

Reach the branch's stated end-goal of **dynamic creation of custom Balatro
mods**. Today every effect is a hard-coded `match` arm on the closed `MPip`
enum, spread across `buffoon_card.rs`, `buffoon_pile.rs`, and `board.rs`. A mod
cannot add a scoring effect without editing funky source. This design adds an
**extension seam** so a mod crate can register new effects at runtime.

## 2. The binding constraint

`BuffoonCard` is `#[derive(Clone, Copy, …, Serialize, Deserialize)]` and ~200
cards are declared as `const`s, each embedding `enhancement: MPip`. Therefore
the on-card effect representation **must** be:

- `Copy` and `const`-constructible (no heap, no `Box`), and
- `Serialize`/`Deserialize`/`Eq`/`Hash` (cards round-trip through serde).

This **rules out `Box<dyn Effect>` on the card** — the usual trait-object plugin
answer. Function pointers (`fn(…) -> …`, cf. the abandoned `fpips.rs::FIntPip`)
are `Copy`/const but are **not** serializable and make `Eq`/`Hash` address-based
— also unfit for the card.

**Conclusion:** the card carries only *plain, serializable data*; the
indirection (the `dyn` handler) lives off the card, in a **registry keyed by a
`u32` id**.

## 3. Design (implemented)

Four pieces, in `src/funky/types/effect.rs`:

```rust
pub enum ScoreOp {                    // declarative contribution to a Score
    Nothing,
    AddChips(usize),
    AddMult(usize),
    TimesMult(f32),
    Seq(Vec<Self>),
}
impl ScoreOp { pub fn apply(&self, score: Score) -> Score; }

pub struct ScoringContext<'a> {       // what an effect may read while scoring
    pub board: &'a BuffoonBoard,      //   played/held/joker piles, leveled hands
    pub source: BuffoonCard,          //   the card/joker carrying the effect
}

pub trait Effect {                    // object-safe; a mod implements this
    fn score(&self, ctx: &ScoringContext<'_>) -> ScoreOp;
}

pub struct EffectRegistry { /* HashMap<u32, Box<dyn Effect>> */ }
impl EffectRegistry { pub fn new(); pub fn register(id, impl Effect); pub fn get(id); }
```

On the card side, one new variant — data only:

```rust
enum MPip { /* … built-ins … */ Custom(u32) }   // stays Copy/const/Serde/Eq/Hash
```

Scoring integration (`board.rs`, additive / non-breaking):

```rust
pub fn score_with_registry(&self, registry: &EffectRegistry) -> Score;
pub fn scoring_phase4_joker_scoring_with_registry(&self, running: Score, registry: &EffectRegistry) -> Score;
```

A `MPip::Custom(id)` joker is resolved by `registry.get(id)`, called with a
`ScoringContext`, and its `ScoreOp` folded into the running score. **Built-in
effects are untouched** — they still go through the existing match arms. The
pure `score()` treats `Custom` as the zero floor (same philosophy as
probabilistic effects), so no registry is required for built-in play.

### Why `ScoreOp` (not "mutate a `Score`")

Effects return a *description* of their contribution, so the pipeline keeps
control of ordering (additive vs multiplicative) and effects are unit-testable
without a board. `Seq` composes; `apply` is the single fold point.

## 4. What a mod writes (no core edits)

```rust
struct FlushDoubler;                                  // in the mod crate
impl Effect for FlushDoubler {
    fn score(&self, ctx: &ScoringContext<'_>) -> ScoreOp {
        if ctx.board.played.has_flush() { ScoreOp::TimesMult(2.0) } else { ScoreOp::Nothing }
    }
}

let mut reg = EffectRegistry::new();
reg.register(9001, FlushDoubler);
let card = BuffoonCard { enhancement: MPip::Custom(9001), ..some_joker };
board.score_with_registry(&reg);                      // the ×2 now applies
```

Proven by tests in `board.rs` (`score_with_registry__*`) and `effect.rs`.

## 5. Alternatives rejected

- **`Box<dyn Effect>` on the card** — breaks `Copy`/`const`/serde. Non-starter.
- **Function-pointer variant (`MPip::Custom(fn…)`, cf. `FIntPip`)** — `Copy`/const
  but not serializable; forces a hand-written `MPip` serde impl and custom cards
  can't round-trip. Rejected in favour of the id (see AskUserQuestion 2026-07-11).
- **Full trait-boundary refactor now** — replace *all* built-in match arms with
  `Effect` impls immediately. Correct long-term shape but a large, higher-risk
  rewrite of working, tested scoring. Deferred to an incremental migration.

## 6. Migration path (future, incremental)

1. Extend registry resolution to **played-card (phase 2)** and **held-card
   (phase 3)** effects — same `ScoringContext`/`ScoreOp`, mirror the phase-4 wiring.
2. ~~**Unify** the phase-4 variants (`_`, `_with_rng`, `_with_registry`).~~
   **Done** — all three now delegate to one private `fold_jokers(running,
   Option<&mut Rng>, Option<&EffectRegistry>)`; the joker fold lives once.
3. Migrate **built-in** `MPip` variants to `Effect` impls behind a **`phf`**
   static map (`phf` is declared but unused today), so built-ins and mods share
   one dispatch and the big `match`es shrink.
4. Combine registry + seeded RNG (a custom effect that needs randomness).
5. **Retire `fpips.rs`** — `FIntPip` is the superseded prototype of this idea.
6. Consider stable string ids (`Custom([u8; N])` name hash) for serde stability
   across mods, versus today's caller-assigned `u32`.

## 7. Status

Implemented: `ScoreOp`, `Effect`, `ScoringContext`, `EffectRegistry`,
`MPip::Custom(u32)`, `score_with_registry` (jokers). Non-breaking; all built-in
scoring unchanged. Steps in §6 remain open.
