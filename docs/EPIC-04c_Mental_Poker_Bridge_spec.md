# EPIC-04c: Mental Poker Bridge (MPB) — cross-repo contract

> **For agentic workers:** This is a **`_spec`** — a cross-repo contract, not an implementation plan. It names what `cardpack` promises, what `pkmental` would implement against it, and how `pkcore`'s `EPIC-79b` branch (`CardSeal`, and now EPIC-82's `TableCrypt`) lines up with it. **No code lands in cardpack from this document** beyond what [EPIC-04](./EPIC-04_Sealed_Decks.md) already ships. Adoption on the other side is blocked on `pkcore`'s `cardpack = "0.6.9"` pin. Status as of `main` @ `1c14440`, 2026-08-24.

**Goal:** Make cardpack the **card-side substrate for mental poker** without shipping any of the protocol. `pkmental` today re-derives the one thing it needs from a card library — a total `Card ↔ 0..52` bijection — at runtime with hash maps and a panicking `expect`. After EPIC-04, that becomes `Codebook<D>`. The referee's state in `pkcore` EPIC-82 — a `Vec<SlotId>` deck and `HoleSlot { slot, revealed: Option<Card> }` — is exactly cardpack's `SlotPile` + `Revealed<D>`, generic over the deck instead of hard-wired to 52. And the `Seal<D>` adapter is shaped so `pkmental`'s Barnett–Smart masking is one `impl`, used only where a reveal must be *verified*.

**Architecture:** Three tables and a divergence register. (1) **cardpack promises** — the stability contract on `Codebook`, canonical bytes, `Permutation`, `SlotId`/`SlotPile`/`Revealed`, the `Seal<D>` shape, and the `commit-reveal` preimage format. (2) **pkmental implements** — how `CardCrypto` (`pkmental/src/crypto/mod.rs:52`) maps onto `Seal<D>` for verified reveals, including the rule that token verification happens *inside* `unseal`, and how the slot-based referee state maps to what `pkmental`'s `Coordinator` already emits. (3) **pkcore lines up** — `TableCrypt`'s fields against cardpack's types, the `CardSeal` ↔ `Seal<D>` shim, and the recommendation to adopt the slot+RNG signature. The register lists every place the designs disagree, so none of them "discovers" it later.

**Tech Stack:** This document only. `pkmental` is `std` (arkworks over Pallas, `ed25519-dalek`, `sha2 0.10`, `rand 0.8`); the bridge lives *there*, never in cardpack.

---

## Context

**`pkmental`** (`../pkmental`, `main` @ `ac72bc1`) implements Barnett–Smart threshold ElGamal over Pallas with Sako–Kilian cut-and-choose shuffle proofs, DLEQ reveal tokens, and an Ed25519-signed hash-chained event log. Its crypto seam is:

- `trait CardCrypto` (`pkmental/src/crypto/mod.rs:52`) with associated types `SecretKey`, `PublicKey`, `AggregateKey`, `MaskedCard: Clone + Eq` (`:62`), `RevealToken` (untrusted, the wire form), `VerifiedToken` (only mintable via `verify_reveal_token`), `KeyProof`, `MaskProof`, `ShuffleProof`.
- `MpError` (`pkmental/src/crypto/mod.rs:33`) — `Clone + Debug + Eq + thiserror::Error`, `&'static str`/`usize` payloads; includes `StillMasked`, `BadProof`, `DeckLength`.
- The card surface it consumes is exactly two items: `pkcore::card::Card` and `pkcore::deck::DECK_ARRAY` (`pkmental/docs/EPIC-79_Mental_Poker_Progress.md:74`). It does **not** depend on cardpack; cardpack reaches it transitively via `pkcore`.
- The bijection it builds for itself: `point_for_index(i) = G · (i + 1)` (`pkmental/src/encode.rs:33-35`), and two `OnceLock<HashMap>`s — bytes → `Card` and `Card` → `DECK_ARRAY` index — populated by scanning the array (`encode.rs:38-60`), with `expect("every Card is in DECK_ARRAY")` on the encode path (`encode.rs:73-81`). Index 0 is skipped so no encoding is the identity.
- **The players hold the masked deck.** Masking, remasking, the verifiable shuffle, and share collection all happen between players over the `Coordinator` log. Cards travel only as curve points; `Card` never crosses the wire (`pkmental/src/wire.rs:11`). A referee — if there is one — sees positions and, at reveal, values.

**`pkcore`** (`../pkcore`, branch `EPIC-79b`, tip `bde2353f`; `main` @ `677e0d15` has none of this):

- **EPIC-79b is complete on the branch** (closed at `93673808`): `src/seal/{card_seal,slot,sealed_card,sealed_deck,plaintext,null}.rs` landed; `Table` became `TableOf<S: CardSeal>` with `pub type Table = TableOf<NullSeal>`; the reveal ledger (`TableAction::SealedDealt` / `Revealed`, `revealed_hole_cards`) landed. Its `CardSeal` is `seal(&self, Card)` / `unseal(&self, &Sealed, &Token)`, `SlotId(u8)`.
- **Its own handoff section** (branch `EPIC-79b_Sealed_Deck.md:507-600`, "Implementing `CardSeal` in `pkmental`") already records the three facts this spec relies on: `Token` binds to **`Vec<RevealToken>`** (l-out-of-l); `seal` needs an `AggregateKey` and an RNG the trait does not pass, so the scheme instance must carry `RefCell<ChaCha20Rng>`; and `unseal` must verify every token's DLEQ proof, fold the shares, then `decode` (`StillMasked` if one is missing).
- **EPIC-82 *The Betting Kernel*** (`docs/epics/EPIC-82_The_Betting_Kernel.md`, drafted 2026-08-23, Phase 0 spike passing) supersedes EPIC-79c and **demotes** the generic containers: "pkcore must not hold the hidden cards." Decision 5: `TableCrypt` is a plain struct — `deck: Vec<SlotId>`, per-seat `HoleSlot { slot: SlotId, revealed: Option<Card> }`, `board: Cards`, `muck_slots: Vec<SlotId>`. Decision 7: reveals arrive as `(SlotId, Card)`, "optionally with a `(scheme, token)` pair for verified unseal via the 79b seam." §4: `CardSeal`/`SealedDeck<S>` stay valid for dealer-custody only.
- `pkcore` pins `cardpack = "0.6.9"` (`pkcore/Cargo.toml:65`) and uses its own `u32 Card` (`pkcore/src/card.rs:30`); its `DECK_ARRAY` order is **not** cardpack's `Standard52::base_vec()` order.

**cardpack** after EPIC-04 ships `Codebook<D>`, `CANON_V1`, `Permutation`, `SlotId`, `SlotPile`, `Revealed<D>` with `reveal` / `reveal_with`, `Seal<D>`, and the `seal_roundtrip` helper. None of it has landed at `1c14440`. Its design follows EPIC-82's rule (EPIC-04 decision 2).

**What this spec does NOT do:**

- No `impl Seal<D>` for anything ElGamal lands in cardpack. cardpack has no arkworks, no curves, no `std` requirement, and will not gain them.
- No change to `pkmental` or `pkcore` is made from this repo. Their follow-ups are *named* here so the three docs agree; the edits happen there.
- No shuffle proof, no threshold decryption, no transport. Those remain `pkmental`'s.

---

## Status

Status as of `main` @ `1c14440`, **2026-08-24**.

| Component | Status |
|---|---|
| cardpack promises table (this doc) | Planned |
| pkmental implementation table (this doc) | Planned |
| pkcore alignment table + shim + divergence register (this doc) | Planned |
| `seal_roundtrip` exported under `seal-test-double` (EPIC-04 Story 5) | Planned — cardpack |
| `pkmental`: `impl<D> Seal<D> for ElGamalSeal<D>` behind a `cardpack` feature | Planned — pkmental, not started |
| `pkmental`: replace `encode.rs:33-60` tables with `Codebook<Standard52>` | Planned — pkmental, not started |
| `pkcore`: EPIC-82 `TableCrypt` over `SlotId` (its own type today) | Proposed there — Phase 0 stop |
| `pkcore`: adopt slot + RNG in `CardSeal` (EPIC-79b corrigendum) | Proposed — pkcore, not started |
| `pkcore`: bump `cardpack` 0.6.9 → 0.11 | **Blocked** — pkcore's own release cadence |

---

## Goals

- **One bijection, owned by the card library.** `pkmental` stops building lookup tables; `Codebook::ordinal` is total, `Option`-returning, and pinned by a golden test.
- **One referee shape, two crates.** `pkcore` EPIC-82's `TableCrypt` fields and cardpack's `SlotPile` + `Revealed<D>` are the same design; if `pkcore` ever bumps its pin, the deck-and-reveal half of `TableCrypt` is a re-export.
- **One adapter shape, three crates.** A backend written against cardpack's `Seal<D>` serves `pkcore`'s `CardSeal` through a shim, not a rewrite — and neither crate has a container generic over it.
- **Verification inside `unseal`.** The rule `pkmental` and the branch 79b doc both already state is written into the contract so nobody re-discovers it.
- **Named bijections in transcripts.** Because cardpack's `Codebook` order and `pkcore`'s `DECK_ARRAY` order differ, every protocol names which one it uses in its domain tag.

## Scope

1. Everything in the "cardpack promises" table is a **stability contract** from 0.11.0. Breaking any row is semver-major.
2. Everything in the "pkmental implements" table is **advisory** to `pkmental` — the shape cardpack expects, not code cardpack ships.
3. The pkcore alignment and shim are **recommendations**; `pkcore` decides at its EPIC-82 Phase 0.
4. Any future divergence between the designs is added to the register here **and** cross-referenced from the other docs.

---

## Domain

**Things.** The bijection (`Codebook<D>`); canonical bytes (`CANON_V1`); the shuffle as data (`Permutation`); the referee state (`SlotPile`, `Revealed<D>`); the adapter (`Seal<D>`); the `MaskedCard` / `RevealToken` mapping; the shim.

**Business Requirements.** (a) cardpack exposes a stable `Card ↔ 0..V` map per deck and a version-tagged byte encoding. (b) cardpack's referee state holds no ciphertext and matches EPIC-82's. (c) `Seal<D>` is satisfiable by `CardCrypto` with verification inside `unseal`. (d) `CardSeal` and `Seal<D>` are bridgeable by a shim.

**Business Logic.** (a) is EPIC-04 Stories 1–3 plus the golden tests; (b) is EPIC-04 Stories 4–5 and the alignment table; (c) is the pkmental table; (d) is the shim below.

---

## The contract

### 1. cardpack promises

| Promise | Where | Stability |
|---|---|---|
| `Codebook<D>::ordinal` / `::card` is a total bijection over the deck **vocabulary** (`base_vec()` deduplicated, first-occurrence order) | EPIC-04 Story 1 | Reordering a shipped `base_vec()` is **semver-major** from 0.11.0. `codebook__standard52_golden` pins Standard52. |
| `Card::default()` and foreign cards have no ordinal (`None`) | EPIC-04 Story 1 | Invariant |
| Canonical pile bytes `CANON_V1` — `[0x01][u16 BE name_len][deck_name][u16 BE count][u16 BE ordinal]*`, iteration order | EPIC-04 Story 2 | `v1` never changes; new layouts get a new version byte |
| `Permutation` with `out[i] = in[p[i]]`, `canonical_bytes` = `[u16 BE len][u16 BE]*` | EPIC-04 Story 3 | Frozen; inverse/compose laws tested |
| `Permutation::from_rng` ≡ `Pile::shuffle_with_rng` ≡ `SlotPile::shuffle_with_rng` for the same RNG state | EPIC-04 Stories 3–4 | Pinned by the agreement tests |
| `SlotId(u16)`; `SlotPile` holds **only** slots and has no method mentioning `Card` | EPIC-04 Story 4 | Invariant — the EPIC-82 rule |
| `Revealed<D>` is the **only** kernel `SlotId → Card<D>` map; `reveal` (unverified) and `reveal_with` (verified via `Seal<D>`) both end there | EPIC-04 Story 5 | Invariant |
| `Seal<D>` shape: `Sealed: Clone + Eq + Debug`, `Token`, `Error: Error + Send + Sync + 'static`, `seal(&self, Card<D>, SlotId, &mut dyn RngCore)`, `unseal(&self, &Sealed, SlotId, &Token)`; **no cardpack struct is generic over `S`** | EPIC-04 Story 5 | Additions only via **new** traits; never a new required method |
| `seal_roundtrip<D, S>` conformance helper under `seal-test-double` | EPIC-04 Story 5 | Runs unchanged against any backend |
| `commit-reveal` preimage formats `v1` (contribution, seed, permutation, pile) | EPIC-04a | Frozen |

### 2. pkmental implements

**2a. The referee state is already what `pkmental` emits.** Its event log carries per-slot masked cards and per-slot reveal shares; a referee or spectator process folding that log needs exactly: the deck as an ordered list of slots (`SlotPile`), the permutation each verified shuffle applied (`Permutation`, whose `canonical_bytes` should be what the Sako–Kilian transcript hashes), and a map from slot to value once *l* shares have been verified (`Revealed<D>::reveal` — unverified from cardpack's point of view, because `pkmental` already verified). No ciphertext enters cardpack on this path.

**2b. `impl<D: DeckedBase> Seal<D> for ElGamalSeal<D>`** — for the case where a referee wants to *check* a reveal itself through `Revealed::reveal_with`. Lives in `pkmental` behind a `cardpack` feature. `ElGamalSeal<D>` holds **public** material only — `AggregateKey`, `Vec<PublicKey>`, the binding context, a `Codebook<D>` — so verifiers need no secret.

| `Seal<D>` item | pkmental construct | Notes |
|---|---|---|
| `Sealed` | `MaskedCard` (`pkmental/src/crypto/mod.rs:62`, `elgamal.rs:83`) | Already `Clone + Copy + Debug + Eq` per branch 79b's table; satisfies the bound as written |
| `Token` | `Vec<RevealToken>` — **unverified**, l-out-of-l | Matches branch 79b §"The token is plural" and `Progress.md:742`. Not `VerifiedToken`: callers of `reveal_with` cannot mint one |
| `Error` | `MpError` (`mod.rs:33`) | `Clone + Debug + Eq + Error`; `Send + Sync + 'static` holds (`&'static str`, `usize` payloads) |
| `seal(card, slot, rng)` | `codebook.ordinal(&card)` → `point_for_index(i) = G · (i + 1)` → `mask(agg_key, point, rng)` | Replaces `encode.rs:33-60` tables with `Codebook<D>`. `slot` ignored. `rng` is passed in — no `RefCell<ChaCha20Rng>` needed, unlike the `pkcore` `CardSeal` impl (branch 79b §"Seal needs state pkcore does not pass"). Adapt `rand 0.8` ↔ `0.10` at the boundary |
| `unseal(sealed, slot, token)` | ∀ `(pk_i, t_i)`: `verify_reveal_token(pk_i, sealed, t_i)` → `VerifiedToken`; fold every `d_i`; `unmask` → point → `codebook.card(ordinal)` | **Verification inside.** Fewer than *l* tokens → `Err(StillMasked)`. A bad proof → `Err(BadProof)`. `slot` ignored |
| Bijection naming | Domain tag `b"cardpack/<deck_name>/v1"` in `pkmental`'s `binding.rs` context | So a transcript can never be replayed against `pkcore`'s `DECK_ARRAY` order |

Future `pkmental` tests (named here so they exist by the time the impl does): `elgamal_seal__roundtrip_law` (via `seal_roundtrip`), `elgamal_seal__unverified_token_rejected`, `elgamal_seal__fewer_than_l_tokens_still_masked`, `elgamal_seal__codebook_replaces_encode_tables` (same point for the same card, old path vs new, **after** the domain tag names the cardpack bijection).

### 3. pkcore lines up

**3a. `TableCrypt` (EPIC-82 Decision 5) against cardpack.** When `pkcore` adopts cardpack ≥ 0.11, the deck-and-reveal half of its crypt table is already written:

| `TableCrypt` field (EPIC-82) | cardpack type | Note |
|---|---|---|
| `deck: Vec<SlotId>` | `SlotPile` | plus blind `shuffle`/`cut`/`draw`/`permute`/`audit` for free |
| `HoleSlot { slot, revealed: Option<Card> }` per seat | `SlotId` + `Revealed<Standard52>::get(slot)` | the seat keeps the slot; the value lives in one map |
| `muck_slots: Vec<SlotId>` | `SlotPile` | same type, second instance |
| reveal `(SlotId, Card)` | `Revealed::reveal` | unverified path |
| reveal with `(scheme, token)` | `Revealed::reveal_with` | verified path, generic at the method — EPIC-82 Decision 7 exactly |
| `SlotId(u8)` | `SlotId(u16)` | `pkcore` widens or wraps |
| `board: Cards` | `Pile<Standard52>` via `Revealed::pile_for` | public anyway |

**3b. The `CardSeal` shim** — for `pkcore`'s dealer-custody shape (`SealedDeck<S>`, EPIC-82 §4), any cardpack `Seal<Standard52>` satisfies `CardSeal`:

```rust
// in pkcore, behind a `cardpack-seal` feature
pub struct Bridge<S> { scheme: S, rng: RefCell<StdRng>, next_slot: Cell<u16> }

impl<S: cardpack::seal::Seal<Standard52>> pkcore::seal::CardSeal for Bridge<S> {
    type Sealed = S::Sealed;
    type Token  = (cardpack::seal::SlotId, S::Token);   // the slot must ride along
    type Error  = S::Error;
    fn seal(&self, card: Card) -> Result<Self::Sealed, Self::Error> {
        let slot = SlotId::new(self.next_slot.replace(self.next_slot.get() + 1));
        self.scheme.seal(to_cardpack(card), slot, &mut *self.rng.borrow_mut())
    }
    fn unseal(&self, s: &Self::Sealed, (slot, t): &Self::Token) -> Result<Card, Self::Error> {
        self.scheme.unseal(s, *slot, t).map(from_cardpack)
    }
}
```

The shim shows the cost of the 79b signature: a `RefCell` RNG and a token that smuggles the slot — the same `RefCell<ChaCha20Rng>` the branch's own handoff section had to plan. **Recommendation to `pkcore`:** adopt `seal(&self, Card, SlotId, &mut dyn RngCore)` and `unseal(&self, &Sealed, SlotId, &Token)` via an EPIC-79b corrigendum, and widen `SlotId` to `u16`. Then the shim is a type alias.

`to_cardpack` / `from_cardpack` map `pkcore::Card` (Cactus-Kev `u32`) ↔ `cardpack::Card<Standard52>`; cardpack's `CKCRevised::get_ckc_number` (`src/basic/types/traits.rs:253`) already produces the same bit layout, so the mapping is a lookup over 52 entries.

### 4. Divergence register

| Topic | cardpack (EPIC-04) | pkcore 79b (branch, landed) | pkcore EPIC-82 (proposed) | pkmental | Resolution |
|---|---|---|---|---|---|
| Who holds ciphertext | nobody in the kernel; dealer custody = plain `Vec<(SlotId, Bytes)>` (04b) | `SealedDeck<S>` generic container | nobody — `TableCrypt` holds slots | the players | cardpack and EPIC-82 agree; 79b's container is the dealer-custody exception |
| Referee state | `SlotPile` + `Revealed<D>` | `SealedDeck<S>` + seats with `Card` | `Vec<SlotId>` + `HoleSlot` | log fold | same design in cardpack and EPIC-82; §3a maps fields |
| Slot in `seal`/`unseal` | yes | no | inherits 79b | n/a | shim smuggles slot in `Token`; recommend pkcore adopt |
| RNG in `seal` | `&mut dyn RngCore` param | none (`&self` only) | inherits 79b | `mask` takes an RNG | shim uses `RefCell<StdRng>`; recommend pkcore adopt |
| `SlotId` width | `u16` | `u8` | `u8` | n/a | pkcore widen; 216-card shoes exist |
| `draw(n)` | `Option<Self>` (parity with `Pile::draw`) | `Result<Vec<_>, PKError>` | n/a | n/a | each crate keeps its own idiom |
| Bijection order | `Codebook` = `base_vec()` first-occurrence | `DECK_ARRAY` | `DECK_ARRAY` | builds its own from `DECK_ARRAY` | **different orders**; protocols name theirs in the domain tag |
| Token plurality | whatever `S` says | whatever `S` says | same | `Vec<RevealToken>` | no conflict; both traits are generic |
| Token verification | inside `unseal` (contract) | unspecified in the trait; documented in §5a | inherits | must be inside (`Progress.md:742`) | write into 79b's corrigendum |
| `Sealed: Eq` meaning | ciphertext equality, not card equality | same | same | `MaskedCard: Eq` compares points | all agree; none may use it for distinctness |
| `rand` version | 0.10 | 0.10 (via cardpack) | same | 0.8 | pkmental bumps or adapts at its boundary |

---

## Work Items (spec-only)

### Story S: Write and cross-reference the contract

- [ ] **S1.** The three tables above, reviewed against `pkmental/src/crypto/mod.rs:52-130`, `pkcore` branch `EPIC-79b` `src/seal/card_seal.rs`, and `EPIC-82_The_Betting_Kernel.md` §3 at write time
- [ ] **S2.** Add a "cardpack `Seal<D>` conformance" follow-up to `pkmental/docs/EPIC-79_Mental_Poker_Progress.md` §follow-ups, pointing here (edit happens in `pkmental`)
- [ ] **S3.** Open a `pkcore` issue against branch `EPIC-79b`: "EPIC-79b corrigendum — adopt slot + RNG in `CardSeal`, widen `SlotId` to `u16`, specify verification inside `unseal`; EPIC-82 Phase 0 — note cardpack's `SlotPile`/`Revealed` as the deck-and-reveal half of `TableCrypt` once the pin moves" (edit happens in `pkcore`)
- [ ] **S4.** Record the `cardpack 0.6.9` pin as the adoption blocker in this Status table and in `BACKLOG.md`
- [ ] **S5.** Confirm EPIC-04 Story 5 exports `seal_roundtrip` under `seal-test-double` with bounds `pkmental` can satisfy (`D: Decked<D> + Default + Ord + Copy + Hash`)
- [ ] **S6.** When `pkmental`'s impl lands, add its commit hash to the Status table here and flip the row

---

## Test Plan

None in cardpack beyond EPIC-04's. The tests this contract *causes* live in `pkmental`:

| Test (pkmental) | Asserts |
|---|---|
| `elgamal_seal__roundtrip_law` | `seal_roundtrip::<Standard52, ElGamalSeal<_>>` passes unchanged |
| `elgamal_seal__unverified_token_rejected` | A token with a bad DLEQ proof is `Err(BadProof)` from `unseal` |
| `elgamal_seal__fewer_than_l_tokens_still_masked` | `l − 1` tokens → `Err(StillMasked)` |
| `elgamal_seal__codebook_replaces_encode_tables` | `Codebook` ordinal + 1 == the old `DECK_ARRAY`-index + 1 for every card **after** the bijection and domain tag switch |
| `referee__fold_log_into_slot_pile_and_revealed` | Folding a `Coordinator` log yields a `SlotPile` in the proven order and a `Revealed` with exactly the opened slots — no ciphertext touched cardpack |

## Key Files

| File | Role |
|---|---|
| `pkmental/src/crypto/mod.rs:52` | `CardCrypto` — the trait being adapted |
| `pkmental/src/encode.rs:33-60` | The tables `Codebook` replaces |
| `pkmental/docs/EPIC-79_Mental_Poker_Progress.md:742-750` | The "verify inside `unseal`" finding |
| `pkcore` branch `EPIC-79b`: `src/seal/card_seal.rs`, `src/seal/slot.rs` | The landed trait and `SlotId`; shim target |
| `pkcore` branch `EPIC-79b`: `docs/epics/EPIC-79b_Sealed_Deck.md:507-600` | The handoff section this spec agrees with |
| `pkcore` branch `EPIC-79b`: `docs/epics/EPIC-82_The_Betting_Kernel.md` §3 | Decisions 5 and 7 — the rule and the reveal shape |
| `pkcore/Cargo.toml:65` | The `cardpack = "0.6.9"` pin |
| `src/seal/{slot,slot_pile,revealed,seal}.rs` (EPIC-04) | `SlotId`, `SlotPile`, `Revealed<D>`, `Seal<D>` |
| `src/basic/types/ordinal.rs` (EPIC-04) | `Codebook<D>`, `CANON_V1` |
| `src/basic/types/permutation.rs` (EPIC-04) | `Permutation` |
| `src/basic/types/traits.rs:253` | `CKCRevised` — the `pkcore::Card` mapping |

## Reuse (do NOT recreate)

- Everything in the "cardpack promises" table — the point of this spec is that `pkmental` **stops** owning a bijection and `pkcore` **stops** owning a slot deck.
- `pkmental`'s `VerifiedToken` newtype (`crypto/mod.rs:66-76`) — the "forgot to verify" compile error stays; it just moves inside `unseal`.
- `pkmental`'s `PlaintextCrypto` (`crypto/plaintext.rs:22`) — do not port it; cardpack's `PlaintextSeal` is the kernel-side double and `seal_roundtrip` is the shared law.
- `pkcore` EPIC-82's spike (`docs/epics/EPIC-82_spike-kernel/`) — the three-shells-one-kernel proof is the model for "plain value, derive everything"; cardpack's `SlotPile` follows it.

## Compatibility

- **cardpack:** nothing beyond EPIC-04. This document adds no code.
- **pkmental:** additive, behind a `cardpack` feature. Its existing `pkcore`-only path keeps working.
- **pkcore:** the shim is additive and feature-gated; the corrigendum is a design change to a trait that is landed on a branch but untagged (`0.8.0` unreleased).

## Dependencies

- **Built on:** [EPIC-04](./EPIC-04_Sealed_Decks.md) (all stories).
- **Cites:** [EPIC-04a](./EPIC-04a_Commit_Reveal_Shuffle.md) (transcript preimage format), [EPIC-04b](./EPIC-04b_Holder_Key_Seal.md) (the reference `Seal<D>` impl and the dealer-custody shape).
- **Blocked by:** `pkcore`'s cardpack pin; `pkmental`'s `rand 0.8`; `pkcore` EPIC-82 Phase 0 (its own decide-and-stop).

## Verification

```bash
# cardpack side: the contract's public surface documents and links resolve
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features crypto,seal-test-double
# the helper pkmental will call is exported
cargo test --features seal-test-double --doc -- seal_roundtrip
# the referee state is scheme-free
! grep -rnE 'struct \w+<[^>]*S: Seal' src/seal/
```

Exit criteria:

1. The three tables agree with the cited source lines at write time, and every divergence in the register is cross-referenced from EPIC-04 decisions 2–3.
2. `seal_roundtrip` is exported with bounds a `std` crate over `Standard52` can satisfy.
3. `pkmental`'s progress doc and a `pkcore` issue point back here (S2, S3).

---

## Gotchas

1. **`Token = Vec<RevealToken>` is a `Vec`, and `reveal_with` takes `&Token`.** Fine — but a reveal for a *seat* (not "to all") needs a different share set per recipient; that is `pkmental`'s `ToSeat` path and is unaffected by the trait.
2. **`MaskedCard: Eq` compares ciphertexts.** Same caveat as EPIC-04 gotcha 2 — remasking makes equal cards unequal; no crate may use it for distinctness.
3. **The two bijections are not the same.** A transcript built on `Codebook<Standard52>` order replayed against `DECK_ARRAY` order decodes to the wrong cards *silently* unless the domain tag names the bijection. This is the single most important row of the register.
4. **`pkmental` is `std`; cardpack is not.** The bridge and the arkworks types live in `pkmental`. Nothing in this spec licenses a `std` requirement on cardpack.
5. **Two `pkcore` states exist.** `main` @ `677e0d15` has no `src/seal/`; branch `EPIC-79b` @ `bde2353f` has 79b complete *and* EPIC-82 proposing to demote parts of it. Cite the branch and the commit, not "pkcore," and re-run `git -C ../pkcore log --oneline main..EPIC-79b` before editing the register.
6. **Do not let `Revealed::reveal` become a back door.** It exists because `pkmental` verifies before cardpack ever sees a value. A referee that accepts `reveal` from an untrusted peer has skipped the protocol; the doc comment says "the caller vouches" for a reason.
