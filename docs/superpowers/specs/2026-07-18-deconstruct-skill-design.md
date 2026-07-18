# /deconstruct skill — design

**Date:** 2026-07-18
**Status:** Approved design, pre-implementation
**Lab rat:** cardpack.rs (this repo)

## Purpose

`/deconstruct` takes an existing codebase and distills it into a set of EPICs
(in the `/epic` house style) plus machine-extracted golden test vectors, so the
codebase's functionality can be cleanly regenerated — by people or by agents —
without being constrained by the original's programmatic design choices.

It is the inverse of `/epic`: `/epic` goes design → code; `/deconstruct` goes
code → design. It is distinct from `/codebase-kata` (codebase → teaching
exercise) and `/domain-kernel` (purity assessment/enforcement).

## Settled decisions

1. **Regeneration target: language-agnostic domain spec.** Epics capture the
   domain model, business rules, and observable behavior in language-neutral
   terms. A rebuild in Rust, Python, or TypeScript is equally valid; API
   compatibility with the original is a non-goal.
2. **Verification: golden test vectors.** Language-neutral fixtures (JSON)
   extracted by *running the original code*, never hand-transcribed. Any
   implementation that reproduces them passes.
3. **Output: self-contained pack directory** in the source repo
   (`docs/deconstruct/`), with its own `DECON-NN` numbering namespace so it
   never collides with the repo's live `EPIC-NN` docs. Portable: copy the
   folder out and hand it to any agent.
4. **Scope line: observable behavior only.** Anything a library user can
   observe (compositions, orderings, parsing/formatting, localized names,
   seeded-shuffle results) is spec material. Platform/engineering traits
   (`no_std`, wasm, MSRV, bit-packing internals, CI matrix) go in a single
   engineering-constraints appendix as context, never as requirements.
5. **Process: one checkpoint.** The skill surveys, proposes the epic map, and
   pauses once for user approval of the manifest. Then it generates everything
   without further interaction.
6. **Perspectives are first-class spec material.** The pack analyzes which
   actor perspectives the code supports and how well. Canonical taxonomy
   (extended per-repo as discovered):
   - **God-mode** — central control over what the domain *is*: definitions,
     vocabularies, rank ladders.
   - **Administrative** — operates and supports the domain without redefining
     it.
   - **User/client** — consumes functionality with bounded access, unable to
     corrupt the underlying domain.
   - **Observer/operator** — read-only insight into domain activity
     (OTel-style telemetry: traces, metrics, logs) without the ability to
     affect it.
   Plus two **quality lenses**, rated the same way but carrying measured
   characteristics instead of actor boundaries, informative by default
   (binding one requires an explicit spec-decision flag):
   - **Performant** — how performant is it: complexity of core operations,
     allocation behavior, benchmarks.
   - **Flexibility** — where and how can it run: runtime environments,
     optional-capability layering, configurability.
   Each perspective gets a support rating (**Full / Partial / Absent**) with
   evidence. Boundary invariants are expressed as domain rules ("deck
   vocabulary is immutable to consumers", "observation must not mutate domain
   state"), never as the original's mechanism (`const` items, the `log`
   crate). Absent perspectives are recorded explicitly, so a rebuilder can
   distinguish "not supported by design" from "spec omission."

## Skill identity

- **Location:** `~/.claude/skills/deconstruct/SKILL.md` plus `references/`
  (methodology + pack templates), mirroring `/epic`'s layout.
- **Invocation:** `/deconstruct` (whole repo, default) or
  `/deconstruct <subsystem>` (scoped run, e.g. one deck family).
- **Frontmatter description:** trigger-rich house style — fires on
  "deconstruct this codebase", "distill this repo into epics", "make a
  regeneration spec", and similar phrasings even without the word
  "deconstruct". Explicitly does **not** fire for forward-looking design work
  (`/epic`) or teaching material (`/codebase-kata`).

## Output artifact — the pack

```
docs/deconstruct/
  MANIFEST.md                       ← pack index & entry point
  DECON-01_Name.md                  ← epics, own numbering namespace
  DECON-02_Name.md
  ...
  APPENDIX_Engineering_Constraints.md
  vectors/
    README.md                       ← vector format contract
    <epic-slug>/*.json              ← golden vectors, grouped per epic
```

`MANIFEST.md` carries:

- the pack's goal statement,
- the epic list with build order and dependency graph,
- the **coverage table**: every observable behavior → its epic, or an explicit
  out-of-scope row,
- the **perspectives taxonomy**: each perspective (god-mode, administrative,
  user/client, observer/operator, plus any repo-specific ones) with its
  support rating (Full / Partial / Absent) and evidence — defined once here,
  applied per-epic,
- provenance: source repo, commit hash, extraction date.

Nothing in the pack depends on access to the source repo.

## Epic format — the "deconstruct profile"

Each `DECON-NN` doc follows `/epic`'s canonical section order (the skill loads
`/epic`'s `references/methodology.md` and template to stay in-voice), with
these deliberate deltas:

| /epic convention | deconstruct profile |
|---|---|
| Status rows reflect landed work | All rows `Planned` — the spec describes work not yet done |
| Design sections carry ```rust API sketches | Language-neutral prose, tables, pseudocode; rationale states the *domain constraint*, never the original mechanism |
| Domain map: concept → code construct | Concept → required behavior → vector file |
| Verification: ```bash of exact commands | Contract clause: "any implementation must reproduce `vectors/<slug>/*.json`" + prose exit criteria |
| Claims cite `path:line` as normative grounding | **Provenance footnote** per epic: pinned `path:line` citations, clearly marked *non-normative* — they prove the behavior is real without binding the rebuilder |
| — | **New section `## Perspectives`**: for this slice, what each manifest-taxonomy perspective may do and the boundary invariants a rebuild must preserve (e.g. "consumers cannot alter deck vocabulary"; "domain events X, Y are observable without side effects"). Perspectives irrelevant to the slice are listed as N/A in one line. |
| — | **New section `## Not specified (implementer's choice)`**: explicitly names the freedoms (memory layout, error style, module structure). Silence is ambiguous; a named freedom is not. |

## Pipeline

**Phase 1 — Survey.** Subagent fan-out over four signals: public API
inventory, test-suite mining (what the authors promised), docs mining (what
they intended), and **perspective analysis** (which actor perspectives the
code supports, at what rating, behind what boundaries — including
observability: which domain events an operator can see without side effects).
Output: a behavior inventory, each entry tagged **domain-essential** or
**implementation-accident**, plus the draft perspectives taxonomy. Where the
signals disagree, the disagreement is recorded and surfaced in the relevant
epic as a flagged **spec decision** — never silently resolved.

**Phase 2 — Map.** Cluster the inventory into epics with a dependency order;
draft `MANIFEST.md`; pause once (AskUserQuestion) showing the proposed epic
list, scopes, and out-of-scope items. User approval — with edits — is the gate.

**Phase 3 — Generate.** Per epic: write the doc, and extract golden vectors by
running the original code. The skill writes a **dumper program** in the source
language using only the crate's public API (for cardpack:
`examples/deconstruct_vectors.rs`) that serializes deck compositions,
orderings, parse/format round-trips, localized names, and seeded-shuffle
outputs to JSON under `vectors/`. The dumper is committed to the source repo so
vectors are regenerable when the source moves; the JSON travels with the pack.

**Phase 4 — Audit.** Two passes:

1. **Coverage** — walk the public surface and confirm every observable
   behavior lands in the manifest's coverage table (mapped to an epic or
   explicitly out-of-scope).
2. **Anti-constraint lint** — scan epic bodies for source-language type names
   and idioms outside Provenance sections; rewrite neutrally or move to
   Provenance.
3. **Perspective consistency** — every epic's `## Perspectives` section uses
   only perspectives defined in the manifest taxonomy (or N/A), and every
   non-Absent perspective in the taxonomy is addressed by at least one epic.

## Edge cases

- **Existing pack detected** → update mode: refresh vectors from the current
  commit, reconcile epics, note behavior drift in the manifest. Never clobber.
- **Scoped run** (`/deconstruct tarot`) → produce/update only the touched
  epics; re-derive the manifest coverage table.
- **Poorly tested source** → vectors still come from the dumper (it exercises
  the public API directly), but the survey notes the weak test grounding so
  spec-decision flags get extra scrutiny.

## Acceptance — first run on cardpack.rs

1. Manifest checkpoint presented and approved.
2. All epics generated in the deconstruct profile.
3. `cargo run --example deconstruct_vectors` reproduces `vectors/`
   byte-identically.
4. Audit coverage table has no unmapped public behavior.
5. Spot check: an agent given only the pack correctly answers "what cards are
   in a Pinochle deck and how do Ten and King rank?" without source access.
   (Originally a Ganjifa question; the survey found Ganjifa is design-doc-only
   with zero implementation, so it moved out of scope — checkpoint-approved.)
6. Perspectives check: the manifest taxonomy rates all four canonical actor
   perspectives and both quality lenses for cardpack with evidence (expected:
   god-mode and user/client present, observer/operator partial via logging,
   administrative likely absent, flexibility strong given no_std/wasm support,
   performant evidenced by the benches/ suite — whatever the survey actually
   finds), and each epic carries a `## Perspectives` section consistent with
   it.

## Non-goals

- API-compatible rebuilds (the pack does not pin type or function signatures).
- Regenerating engineering traits (`no_std`, wasm targets, performance
  characteristics) — recorded as appendix context only.
- Teaching material or kata generation (`/codebase-kata` owns that).
