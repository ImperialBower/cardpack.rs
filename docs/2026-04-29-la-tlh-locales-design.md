# Design: Latin (`la`) and Klingon (`tlh`) Locales

**Date:** 2026-04-29
**Branch:** gapped
**Author:** Claude Code (claude-opus-4-7) + folkengine
**Predecessor work:** commit `5547839` (fr/ drafted; de/tarot.ftl schema fix)
**Audit reference:** [`docs/audit-2026-04-29.md`](./audit-2026-04-29.md) §7, §16 row 6e/6f

---

## 1. Purpose

Extend cardpack's localization breadth from 3 locales (en-US, de, fr) to 5 by
adding Latin (`la`) and Klingon (`tlh`). Both ship as drafts under the same
filesystem-auto-discovery, per-file-confidence-marker, per-locale-README
pattern established by fr/.

The two languages serve different purposes — Latin grounds in Renaissance
gaming/scholarly Latin; Klingon follows pragmatic Marc Okrand canon (TKD,
KGT, Power Klingon) with marked coinages — but ship under the same
architectural pattern.

## 2. Scope

| Locale | `french.ftl` | `skat.ftl` | `tarot.ftl` | `README.md` |
|---|---|---|---|---|
| `la/` | ✅ HIGH conf | ✅ LOW-MEDIUM conf | ✅ MEDIUM-HIGH conf | ✅ |
| `tlh/` | ✅ LOW conf | ✅ LOW conf | ✅ LOW conf | ✅ |

8 new files. Plus 2 new tests in `src/localization.rs`. No Rust source
changes for locale registration — `static_loader!` discovers locale
subdirectories at compile time.

## 3. Translation policy

### 3.1 Latin (`la/`)

**Renaissance/medieval gaming Latin where attested; classical Latin where
unattested but unambiguous.**

- `french.ftl` — court ranks: `Rex / Regina / Famulus` (King/Queen/Jack);
  numbers: classical (`Decem / Novem / Octo / ... / Duo`); suits:
  medieval European gaming convention (`Spathae / Corda / Rhombi /
  Trifolia`). The Greek-derived `Rhombi` (diamonds) is medieval Latin.
- `tarot.ftl` — Major Arcana via attested Renaissance Italian-Latin
  tradition (`Stultus / Praestigiator / Papissa / Imperatrix / Imperator
  / Pontifex / Amantes / Currus / Fortitudo / Eremita / Rota Fortunae /
  Iustitia / Suspensus / Mors / Temperantia / Diabolus / Turris / Stella
  / Luna / Sol / Iudicium / Mundus`); Minor suits use the classical
  Italian-Latin `Calices / Gladii / Baculi / Denarii`; Knight rank is
  `Eques`.
- `skat.ftl` — direct semantic translation of German suit names
  (`Glandes / Folia / Corda / Tintinnabula`); `Superior / Inferior` for
  Ober/Unter (positional semantics, not court-rank mapping); `As` for
  Daus (semantic Ace, matching fr/ and standard Skat semantics).

### 3.2 Klingon (`tlh/`)

**Pragmatic Okrand canon + marked coinages.**

- Use TKD/KGT/Power Klingon attested roots verbatim where they exist.
- Form compounds following Klingon morphology rules (`-Hom` diminutive,
  `-'a'` augmentative, `-wI'` agentive, `-pu'` plural, `be'` female).
- Mark every coinage on the line preceding its FTL entry with
  `# coinage: <back-translation>`.
- Mark every attested term on the line preceding with
  `# attested: <gloss> (<source>)`.
- Standard Latin transcription with Okrand case distinctions
  (`q/Q/H/'`, `tlh/ng`). No pIqaD.

#### High-confidence attested terms used:

| Concept | Klingon | Source |
|---|---|---|
| heart | `tIq` | TKD |
| sword | `yan` | TKD |
| stone | `nagh` | TKD |
| staff | `naQ` | TKD |
| fool | `qoH` | TKD |
| zero/nothing | `pagh` | TKD |
| emperor | `ta'` | TKD |
| servant | `toy'wI'` | TKD |
| warrior class | `vaj` | TKD |
| star | `Hov` | TKD |
| moon | `maS` | TKD |
| sun | `jul` | TKD |
| world | `qo'` | TKD |
| death/die | `Hegh` | TKD |
| strength | `HoS` | TKD |
| justice | `ruv` | TKD |
| Fek'lhr (devil) | `veqlargh` | TKD |
| cup | `HIvje'` | TKD |
| money | `Huch` | TKD |
| numbers (1–10) | `wa', cha', wej, ..., wa'maH` | TKD |
| first (ordinal) | `wa'DIch` | TKD |
| lover | `parmaqqay` | KGT |
| above (place) | `Dung` | TKD |
| below (place) | `bIng` | TKD |
| be alone (verb) | `mob` | TKD |
| be calm (verb) | `jot` | TKD |

#### Notable coinages:

| Coinage | Back-translation | Used for |
|---|---|---|
| `qoH'a'` | great fool | big-joker / Full-Color |
| `qoHHom` | small fool | small-joker / One-Color |
| `ta'be'` | emperor's wife | Queen |
| `ta'be''a'` | great emperor's wife | Empress (Major Arcana 3) |
| `lalDanwI'` | religious-person | (root for priestess/hierophant) |
| `lalDanwI''a'` | great religious-person | Hierophant (Major Arcana 5) |
| `lalDanwI''a' be'` | great female religious-person | High Priestess (Major Arcana 2) |
| `mobwI'` | lone-one | Hermit (Major Arcana 9) |
| `San jIrwI'` | fate-spinner | Wheel of Fortune (Major Arcana a) |
| `loD lIS` | suspended man | Hanged Man (Major Arcana c) — speculative |
| `qach 'aD` | tall building | Tower (Major Arcana g) |
| `ruv jaj` | justice day | Judgement (Major Arcana k) |
| `paQDI'norgh tIQ` | ancient teaching | Major Arcana suit name |
| `jajlo'wI'` | mystic-one | Magician (Major Arcana 1) |
| `Sor naH` | tree fruit | Acorns (Skat) |
| `Sor Hap` | tree material | Leaves (Skat) |
| `wabHom` | small noise | Bells (Skat) |

## 4. Confidence marker convention

### 4.1 Per-file FTL header

```
# {locale-dir}/{file}.ftl — {Language} ({native name}) translation
#
# Status: DRAFT — {HIGH|MEDIUM-HIGH|MEDIUM|LOW-MEDIUM|LOW} CONFIDENCE.
# {one-paragraph confidence rationale}
#
# {optional specific concerns or schema notes}
#
# Needs: {specific reviewer profile}
```

### 4.2 Per-locale README.md

Four-column markdown table:

```markdown
| File | Status | Confidence | Needs |
|---|---|---|---|
| `<file>.ftl` | DRAFT | <Level> (~<%>) | <Reviewer profile> |
```

Plus a "Translation policy" section, an "Inline marker convention" section
(tlh/ only — la/ uses file-header markers exclusively), and a "Before
merging" section listing per-file review criteria.

### 4.3 Inline coinage markers (tlh/ only)

```
# coinage: "great fool" (qoH + -'a' augmentative); for big-joker
name-rank-french-b = qoH'a'
# attested: "first" ordinal (TKD); coinage in card context (Ace)
name-rank-french-a = wa'DIch
```

Comments precede the entry rather than trailing it because Fluent (FTL)
syntax treats anything after `=` as the value (no inline trailing
comments). This was a non-obvious finding during implementation — an
earlier draft of the plan proposed inline trailing comments, which would
have parsed as part of the value text.

## 5. Tests

Two new wired-tests in `src/localization.rs`, modeled on
`french_locale_is_wired`. Each tests two attested-vocabulary lookups
(immune to coinage revision):

```rust
#[test]
fn latin_locale_is_wired() {
    let la = langid!("la");
    assert_eq!("Regina", LOCALES.lookup(&la, "name-rank-french-q"));
    assert_eq!("Rex", LOCALES.lookup(&la, "name-rank-french-k"));
}

#[test]
fn klingon_locale_is_wired() {
    let tlh = langid!("tlh");
    assert_eq!("tIq", LOCALES.lookup(&tlh, "name-suit-french-h"));
    assert_eq!("ta'", LOCALES.lookup(&tlh, "name-rank-french-k"));
}
```

The tests guard *loader registration*, not translation correctness. A
failure means the static loader stopped picking up the locale dir; a
translation revision (e.g., reviewer changes a coinage) does not break
them.

## 6. Deferred

- `LATINUM` / `KLINGON` consts on the `Named` trait (`src/localization.rs:39–40`).
  Same semver reasoning as deferred `FRANÇAIS`: cargo-semver-checks may flag
  new associated consts as a public-API addition. A future minor-bump can
  add `FRANÇAIS / LATINUM / KLINGON` in one batch.
- Schema-correctness tests beyond "wired" (e.g., `latin_tarot_resolves_correctly`,
  `klingon_skat_resolves_correctly`). Wired-only matches fr/. Full schema
  tests can come once each locale is reviewed and promoted from draft.
- pIqaD (Klingon native script) support. Out of scope for terminal display.
- `clubs-index` override for la/ (Trifolia/T collides with Tres/T and
  Decem/T) — left to the symbol fallback (♣) per fr/'s precedent.

## 7. Verification

```sh
cargo build                         # FTL parser validates syntax at build time
cargo test --lib localization       # 19 tests including 2 new wired-tests
cargo test --lib                    # 285 tests, all passing
make ayce                           # umbrella: build + test + fmt + clippy + doc
```

End-to-end success criteria, all verified at landing time:

1. ✅ `cargo build` succeeds (FTL syntax valid in 6 new files).
2. ✅ `cargo test` shows all 285 tests pass; both new wired-tests included.
3. ✅ `latin_locale_is_wired` resolves `Regina` and `Rex`.
4. ✅ `klingon_locale_is_wired` resolves `tIq` and `ta'`.
5. ✅ `docs/audit-2026-04-29.md` reflects la/ and tlh/ as drafted with
   per-file confidence markers (§2 closed-items, §7 i18n breadth, §16
   rows 6e/6f, §17 file reference).
6. ✅ Per-locale `README.md` tables list each file's status, confidence,
   and reviewer profile.

## 8. Out of scope

- Promoting fr/, la/, or tlh/ from DRAFT to REVIEWED. Each requires the
  reviewer profile listed in its README.
- Additional locales (es, it, ja, etc.). Pattern is now clear: copy
  en-US/, translate, add wired-test, update audit doc.
- Adding `--lang` flag to `examples/demo.rs` (audit doc §18 cited
  `cargo run --example demo -- --pinochle -v --lang de` as a verification
  step, but `examples/demo.rs` does not currently expose a `--lang` flag —
  the demo defaults to en-US display. The audit's §18 example was
  aspirational, not actual; a follow-up should either implement the flag
  or correct the audit text. Out of scope for this change.)
- Klingon canon vocabulary expansion via KLI-extended sources. Current
  scope is strict Okrand canon + morphology-respecting coinages.

## 9. Trajectory

After this lands:

- Three drafted locales (fr, la, tlh) accumulate review needs. A future
  PR could add an `i18n-review-queue.md` doc to consolidate.
- One semver-bounded follow-up: when `FRANÇAIS / LATINUM / KLINGON` consts
  are added to `Named`, batch them in a single minor-bump release.
- The audit doc's §7 framing ("language breadth is the credibility gap")
  is now substantively addressed at the locale-count level. Promotion
  from DRAFT to REVIEWED is the remaining gap and shifts the burden
  from translation-work to review-work.
