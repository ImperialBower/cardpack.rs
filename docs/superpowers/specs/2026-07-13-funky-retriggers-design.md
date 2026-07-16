# EPIC-01a Phase 4a — Per-Card Retriggers (Design)

> ⚠️ **SUPERSEDED — this design lost the merge race, do not implement from it.**
> A parallel implementation of the same four jokers landed on `origin/funky`
> (`26db1db` "Phase 4" + `c5b7b0e` Mime) and won at merge `6d3ac11`. The shipped
> vocabulary differs from this spec: the `MPip` variants are
> `RetriggerPlayedRanks(n, ranks)`, `RetriggerPlayedFaces(n)`, and
> `RetriggerFirstPlayed(n)` — each carrying an explicit trigger count — not the
> count-less `RetriggerScoredRanks`/`RetriggerScoredFaces`/`RetriggerFirstScored`
> designed below. The count helpers shipped as `played_retriggers` /
> `held_retriggers` in `src/funky/types/board.rs`. The one piece of this effort
> that survived the merge is the stacking test
> (`score__stacked_retriggers_are_additive`). Read the below only as a record of
> the design explored on 2026-07-13; the code is the current truth.

**Date:** 2026-07-13 · **Branch:** `funky` · **EPIC:** [EPIC-01a Phase 4](../../EPIC-01a_Joker_Wiring_Backlog.md) (Retriggers)

## Summary

Add a per-card **retrigger** mechanism to the funky scoring engine: a played or
held card's scoring contribution is applied `1 + n` times, where `n` is granted by
retrigger jokers on the board. Wire four jokers end-to-end:

| Joker | # | Rarity / cost | Retrigger rule |
|---|---|---|---|
| **Hack** | 36 | Uncommon | each **played** card ranked 2, 3, 4, or 5 → +1 trigger |
| **Sock and Buskin** | 109 | Uncommon / $6 | each **played face** card (K/Q/J) → +1 trigger |
| **Hanging Chad** | 115 | Common / $4 | the **first played** card (scoring order) → +2 triggers |
| **Mime** | 19 | Uncommon | every **held** card → +1 trigger |

Retriggers must not change `score()` for boards without these jokers (byte-identical),
and each joker gets one test asserting its exact Balatro value that fails before its
arm lands (EPIC-00f Gold Standard).

## Context (current engine)

Scoring is a four-phase fold, each phase folding cards/jokers into one running
`Score` (`src/funky/types/board.rs`):

- `fold_played_cards` (phase 2, `board.rs:98`) — per card: `builtin_played_op`
  (base chips + `calculate_plus`) then a `special` op (Lucky RNG roll / `Custom`).
- `fold_held_cards` (phase 3, `board.rs:173`) — per held card: `builtin_held_op`
  (Steel `MultTimes1Dot(15)` = ×1.5, `MultTimes(n)` = ×n) or `Custom`.
- `fold_jokers` (phase 4, `board.rs:231`) — additive / ×mult / RNG / `Custom`.

`ScoreOp` (`effect.rs:25`) is the single application currency
(`Nothing`/`AddChips`/`AddMult`/`Add`/`TimesMult`/`Seq`).

Retrigger jokers are **not scoring effects** — they modify *other* cards' scoring —
so they plug into the played/held folds, not into `fold_jokers`.

## Decisions (from brainstorming)

1. **Scope:** unified played **and** held retriggers now, including **Mime** (held).
   Not just the three played jokers.
2. **Retrigger unit:** a retrigger re-runs the card's **entire** contribution — base
   chips + `calculate_plus` **and** a fresh independent Lucky roll via the threaded
   `rng`. Balatro-faithful. Pure `score()` has `rng = None`, so Lucky stays inert and
   deterministic; `score_with_seed`/`score_with_rng` re-roll once per trigger.
3. **Mechanism:** a per-card retrigger-count pre-pass, then apply the card op
   `1 + count` times (Approach A below). Not pile-expansion (loses position) and not
   `ScoreOp::Seq` (can't express a fresh roll per pass).

## Approach A — retrigger-count + repeat-the-op loop

Add a per-card count helper to each fold and wrap the existing per-card body in a
`0..=extra` loop:

```rust
// fold_played_cards (phase 2)
for (i, card) in self.played.iter().enumerate() {
    let extra = self.played_retrigger_count(card, i);   // 0 when no retrigger jokers
    for _ in 0..=extra {                                // 1 + extra passes
        score = Self::builtin_played_op(card).apply(score);
        let special = /* Lucky roll (fresh each pass) / Custom, as today */;
        score = special.apply(score);
    }
}

// fold_held_cards (phase 3)
for (i, card) in self.in_hand.iter().enumerate() {
    let extra = self.held_retrigger_count(card, i);
    let op = /* Custom | builtin_held_op, as today */;
    for _ in 0..=extra {
        score = op.apply(score);
    }
}
```

`*_retrigger_count` sums across **all** jokers, so retriggers **stack additively** —
two Hacks double-count a low card; Hanging Chad's +2 adds on top of Hack's +1 for a
first-played `2` (1 base + 1 + 2 = 4 triggers). This matches Balatro (each joker,
and each copy, contributes independently).

**Rejected alternatives.** *Pile-expansion* (duplicate retriggered cards into a temp
pile, fold once): loses card position, so Hanging Chad's "first card" is undefined,
and "duplicate a card" is meaningless for Mime's held-ability retrigger. *`ScoreOp::Seq`*
(`Seq(vec![op; 1+n])`): the op is computed once up front, so it cannot re-roll Lucky
per pass (decision 2).

## Data — `MPip` variants

Retrigger jokers carry a variant read only by the count helpers; they **never**
produce a `ScoreOp` directly. Ranks use fixed `[char; N]` arrays, matching the
existing `*Ranks` variants (`mpip.rs`).

Three **new** variants for the played retriggers:

```rust
RetriggerScoredRanks([char; 4]),  // Hack: ['2','3','4','5']
RetriggerScoredFaces,             // Sock and Buskin: K/Q/J
RetriggerFirstScored(usize),      // Hanging Chad: 2
```

Mime **reuses the existing** `RetriggerCardsInHand(usize)` variant (`mpip.rs:121`),
which `MIME` already carries (`RetriggerCardsInHand(1)`) but which no scoring path
handled — so Mime silently scored 0 today. No new held-retrigger variant is added
(the earlier `RetriggerHeld` idea is dropped); Mime needs a scoring-path change only,
no `joker.rs` data edit.

Each gets a `Display` arm (crate convention). Count helpers:

```rust
fn played_retrigger_count(&self, card: &BuffoonCard, index: usize) -> usize {
    self.jokers.iter().map(|j| match j.enhancement {
        MPip::RetriggerScoredRanks(rs) if rs.contains(&card.rank.index) => 1,
        MPip::RetriggerScoredFaces if matches!(card.rank.index, 'K' | 'Q' | 'J') => 1,
        MPip::RetriggerFirstScored(n) if index == 0 => n,
        _ => 0,
    }).sum()
}

fn held_retrigger_count(&self, _card: &BuffoonCard, _index: usize) -> usize {
    self.jokers.iter().map(|j| match j.enhancement {
        MPip::RetriggerCardsInHand(n) => n,
        _ => 0,
    }).sum()
}
```

The face predicate reuses the exact form already at `board.rs:270`
(`matches!(card.rank.index, 'K' | 'Q' | 'J')`).

## Joker consts (`decks/joker.rs`)

- **Hack** (`HACK`, exists, currently `Blank`) → `RetriggerScoredRanks(['2','3','4','5'])`.
- **Mime** (`MIME`, exists) already carries `RetriggerCardsInHand(1)` — no data edit;
  Task wires the scoring path only.
- **Sock and Buskin** — **new const** (Uncommon, $6). Add to `UNCOMMON_JOKERS`
  (12 → 13, bump `UNCOMMON_JOKERS_SIZE`).
- **Hanging Chad** — **new const** (Common, $4). Add to `COMMON_JOKERS`
  (22 → 23, bump `COMMON_JOKERS_SIZE`).
- `ALL_JOKERS` grows `105 → 107` (add both new consts); the `_SIZE` array bound and
  the pile-invariant tests (`assert_rarity_pile`, `all_jokers__is_superset_of_every_pile`,
  weight-uniqueness) update accordingly. New consts get unique weights.

## Silent-zero guard

`all_jokers__intended_hand_scorers_are_reachable` uses a `scores_hand(MPip)` oracle
that classifies jokers by variant. All four retrigger variants (the three new ones plus
`RetriggerCardsInHand`, already in the oracle's non-scoring arm) add nothing on a probe
board with no matching cards, so they must be classified **not a direct scorer** —
otherwise the existing guard false-positives them as silently-inert.

That would remove the retrigger class from silent-zero protection (the hole that hid
Banner/Mystic Summit). So **add a parallel guard**: a test proving each retrigger
joker actually *increases* a matching card's contribution on a probe board — Hack over
a played `2`, Sock and Buskin over a King, Hanging Chad over the first card, Mime over
a held Steel. The `scores_hand` match stays exhaustive, so a future retrigger variant
will not compile until classified.

## Tests (Gold Standard: each fails before its arm lands)

Following the `board.rs` `score__*` pattern (exact Balatro value):

- `score__hack_retriggers_low_cards` — a played `2` scores its chips twice.
- `score__sock_and_buskin_retriggers_faces` — a played King scores twice.
- `score__hanging_chad_retriggers_first_card_thrice` — the first played card scores
  3×, a later card 1×.
- `score__mime_retriggers_held_steel` — a held Steel card applies its ×1.5 twice.
- `score__stacked_retriggers_are_additive` — Hack + Hanging Chad on a first-played
  `2` yields 4 total triggers.
- **Retrigger guard** — each retrigger joker multiplies a matching card's
  contribution (the new silent-inert protection for this class).
- **Compatibility** — a board with none of these jokers scores identically to before
  (covered by inert-by-default helpers + the existing `score__*` suite).

## Compatibility

- `played_retrigger_count`/`held_retrigger_count` return 0 with no retrigger jokers →
  each card applied exactly once → `score()` byte-identical to today.
- New `MPip` variants are inert unless a joker carries one **and** a matching card is
  present. Adds board-free behaviour; breaks nothing.

## Key files

| File | Change |
|---|---|
| `src/funky/types/board.rs` | `played_retrigger_count`/`held_retrigger_count`; `0..=extra` loop in `fold_played_cards` + `fold_held_cards`; retrigger tests |
| `src/funky/types/mpip.rs` | 3 new variants + `Display` arms (Mime reuses existing `RetriggerCardsInHand`) |
| `src/funky/decks/joker.rs` | wire Hack/Mime; new Sock and Buskin + Hanging Chad consts; pile + `ALL_JOKERS` + `_SIZE` updates; `scores_hand` oracle classification; retrigger guard |

## Reuse (do NOT recreate)

- The fold pattern and `ScoreOp::apply` — wrap the existing per-card body, do not add
  a new loop shape.
- The face predicate at `board.rs:270`.
- The seeded RNG path (`score_with_rng`/`score_with_seed`) for the fresh-roll-per-pass
  behaviour — the loop threads the existing `rng`, no new randomness.

## Out of scope (later Phase 4 items)

Dusk (retrigger in final hand — needs Phase 2 "final hand" state), Seltzer (retrigger
all for 10 hands — needs a Phase 3 counter), Hanging Chad's blind-defeat unlock and
other lifecycle concerns. This slice is the pure per-card/​per-held retrigger only.

## Verification

```bash
cargo test --features funky
cargo clippy --features funky --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo build --no-default-features    # funky must not leak into no_std
cargo fmt --all -- --check
```

Exit criteria: all four jokers wired with an exact-value test that failed before its
arm; `score()` for a retrigger-free board unchanged; the retrigger guard green; the
EPIC-01a Phase 4 status row updated with cited, tested code.
