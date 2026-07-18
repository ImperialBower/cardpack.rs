# Cardpack.rs Domain-Kernel Audit

**Date:** 2026-07-18
**Branch:** main
**Version:** 0.7.1
**Auditor:** Claude Code (claude-opus-4-8)
**Framework:** [`domain-kernel` skill](https://) — Mode A (Assess)
**Related:** [`docs/audit-2026-04-29.md`](./audit-2026-04-29.md), [`docs/audit-2026-04-09.md`](./audit-2026-04-09.md)

---

**Summary:** This audit assesses `cardpack` against the **domain-kernel** pattern — a
pure, delivery-agnostic core of one domain's logic behind a narrow, stable
boundary. The crate is closer than most: its public error surface is already
clean (errors are boxed, not `serde_norway::Error`), and no_std/alloc/wasm/
bare-metal builds are gated in CI. Two structural gaps stand between it and an
honest "domain kernel" label: **the core performs filesystem I/O** (Invariant 1),
and **`default` features turn on presentation, a YAML parser, i18n, and
fs-loading** (Invariants 3 & 4). The deterministic purity checker reported
**0 hard leaks, 2 warnings**. Fixes are sequenced in §5.

---

## 1. Framing: this is a value-type kernel, not a state-machine kernel

The domain-kernel invariants are written primarily for **state-machine kernels**
(game state + actions + emitted events, e.g. poker *gameplay*). `cardpack` is a
**value/collection library** — cards, pips, piles, decks. It has no game state,
no acting parties, and no hidden information.

That distinction is load-bearing for the assessment:

- **Invariant 5** (hidden-information projection, `view_for(state, actor)`) — **N/A.**
  There is nothing to project.
- **Invariant 6** (narrow transition surface `to_act`/`apply`/`legal_actions` →
  WIT world) — **N/A in the ideal form.** `cardpack`'s surface is inherently broad
  (13 decks, `Pile`, `Card`, `BasicCard`, combos, the `funky`/Balatro module). A
  language-neutral WIT boundary (**Mode C**) is a poor fit for this crate and is
  **not recommended** unless a specific cross-language consumer appears. The kernel
  value here is **purity + portability** (no_std/wasm), not a transition contract.

Everything below therefore focuses on Invariants 1–4.

---

## 2. Methodology

- Ran `scripts/check_purity.py` against the crate root (0 hard, 2 warn).
- Read `references/invariants.md` and checked each invariant by hand.
- Reviewed: `Cargo.toml`, `src/lib.rs`, `src/basic/types/basic_card.rs`,
  `src/basic/decks/razz.rs`, `src/localization.rs`, `src/common/errors.rs`,
  `deny.toml`, `clippy.toml`, `.github/workflows/CI.yaml`.
- Grepped non-test `src/` for `std::fs`/`std::net`/`std::env`/`Path`/`tokio`/
  `reqwest` and for format-crate/i18n leaks.

---

## 3. Scorecard

| # | Invariant | Status | Severity |
|---|-----------|--------|----------|
| 1 | Pure — no I/O of its own | 🔴 Violated | High |
| 2 | No format/transport crate in public API | 🟢 Satisfied | — |
| 3 | Pure by default | 🔴 Violated | High |
| 4 | Delivery-agnostic | 🟡 Partial | Medium |
| 5 | Hidden-information projection | ⚪ N/A | — |
| 6 | Narrow, stable transition boundary | ⚪ N/A | — |

---

## 4. Findings

### 🔴 Invariant 1 — Pure (no I/O of its own) — VIOLATED

#### Finding 1a — a deck reads a hardcoded path at construction time *(highest leverage)*

`src/basic/decks/razz.rs:30`

```rust
fn base_vec() -> Vec<BasicCard> {
    BasicCard::cards_from_yaml_file("src/basic/decks/yaml/razz.yaml").unwrap_or_else(|e| {
        error!("Failed to load Razz deck from YAML: {e}");
        Vec::default()
    })
}
```

The most serious purity violation in the crate. Building `Pile::<Razz>::deck()`
opens a file **relative to the current working directory**. It only works because
tests run from the crate root with the `.yaml` bundled — for any real downstream
consumer, `Razz::base_vec()` silently returns an **empty deck** and logs an error.
The kernel is asserting a CWD layout, performing I/O, *and* swallowing the failure.
Every other deck (`french.rs`, etc.) builds `base_vec` programmatically from
`const` data; Razz is the lone outlier.

> Note: the 2026-04-29 audit recorded the `log::error!` addition here as a *fix*
> for silent failure. From a kernel-purity standpoint the logging is a symptom —
> the underlying runtime fs read is the defect.

**Fix (near-zero risk):** embed the data at compile time instead of reading it at runtime.

```rust
fn base_vec() -> Vec<BasicCard> {
    // include_str! bakes the file into the binary at build time — no fs, no CWD.
    BasicCard::cards_from_yaml_str(include_str!("yaml/razz.yaml"))
        .expect("razz.yaml is bundled and validated in tests")
}
```

Keeps the "yaml-driven deck" demonstration, removes the runtime filesystem
dependency and the silent-empty-deck failure mode. Couples Razz to the `yaml`
feature, which is the correct coupling.

#### Finding 1b — a public constructor takes a path and reads it

`src/basic/types/basic_card.rs:78`

```rust
#[cfg(feature = "yaml")]
pub fn cards_from_yaml_file(file_path: &str) -> Result<Vec<Self>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    ...
}
```

Per Invariant 1, "a method that takes a path is I/O policy living in the kernel."
The pure sibling `cards_from_yaml_str(&str)` (line 93) already does the real work;
`cards_from_yaml_file` is a thin fs wrapper.

**Fix:** relocate `cards_from_yaml_file` to an adapter (an example, a `cardpack-io`
companion, or a `std-io` feature that is **not** in `default`). Keep
`cards_from_yaml_str` in the core. After Finding 1a, nothing inside the crate needs
the file variant.

> `static_loader!` in `src/localization.rs:8` reads locale files at **compile
> time** (fluent-templates embeds them), so it is not a runtime purity violation.
> It is still a heavy default dependency — see Invariant 3.

---

### 🟢 Invariant 2 — No format/transport crate in the public API — SATISFIED

`serde_norway` never appears in a public signature — the YAML methods return
`Result<_, Box<dyn Error>>`, boxing the format error at the seam exactly as the
invariant prescribes. `serde::{Serialize, Deserialize}` are derived under
`#[cfg(feature = "serde")]` — a *trait* dependency, not a concrete format, which is
allowed.

Only cosmetic residue remains: method *names* (`cards_from_yaml_*`) and
`CardError::InvalidFilePath` (`src/common/errors.rs:15`) mention format/fs
concerns. These are later renames, not couplings — defer until after the
structural fixes.

---

### 🔴 Invariant 3 — Pure by default — VIOLATED *(highest leverage, tie with 1a)*

`Cargo.toml`:

```toml
default = ["std", "i18n", "colored-display", "yaml", "serde"]
```

A bare `cargo add cardpack` currently pulls in:

| Default feature | Drags in | Kernel concern? |
|---|---|---|
| `std` | libstd | couples to std; a kernel should default to `alloc` |
| `i18n` | `fluent-templates` (+ embedded locales) | presentation / localization |
| `colored-display` | `colored` | **terminal presentation** (delivery — see Inv. 4) |
| `yaml` | `serde_norway` | **format crate** + unlocks the fs-reading methods |
| `serde` | `serde` | serialization |

This is the standard kernel headline fix, and the crate is already structured for
it — every feature is cleanly gated and a `no-std-build` CI job already proves
`--no-default-features` compiles:

```toml
default = []                                                 # pure core (or ["alloc"])
full = ["std", "i18n", "colored-display", "yaml", "serde"]   # opt-in umbrella
```

**Ripple to handle:** examples/tests with `required-features` **error rather than
skip** under empty defaults, and Makefile/CI invocations that relied on default
features must pass `--features full`.

---

### 🟡 Invariant 4 — Delivery-agnostic — PARTIAL

`colored-display` puts `colored::Color` into the core (`colors() -> HashMap<Pip,
Color>` in the deck impls) and it is **on by default**. Terminal coloring is a
*rendering* concern — textbook adapter material. It is feature-gated, so the
immediate fix is removing it from `default` (Invariant 3); the longer-term fix is
relocating the color maps into a `cardpack-display` adapter.

No `clap`/`axum`/`tonic`/web types in the core (`clap` is a dev-dep for examples
only). `log::error!` in `razz.rs` is a facade call — minor, and it disappears when
Finding 1a is fixed.

---

## 5. Recommended sequence

Ordered by leverage-to-risk (matches the skill's "flip default + de-leak first"
guidance):

1. **Fix Razz's hidden fs I/O** (1a) — swap runtime `cards_from_yaml_file` for
   compile-time `include_str!`. Removes the worst violation and the silent
   empty-deck bug. ~10 lines, near-zero risk.
2. **Flip `default` to `[]` (or `["alloc"]`) and add a `full` umbrella** (Inv. 3).
   Wire `--features full` into the Makefile/CI targets that assumed default
   features. Biggest purity win; the gating already exists.
3. **Move `cards_from_yaml_file` to an adapter / `std-io` opt-in feature** (1b),
   keeping the pure `cards_from_yaml_str` in the core.
4. **(Enforce — Mode B)** Add `clippy.toml` `disallowed-types`/`disallowed-methods`
   for `std::fs`/`std::net`/`std::env`, and a CI job asserting banned crates
   (`serde_norway`, `colored`, `fluent-templates`) are absent from the
   `--no-default-features` `cargo tree`. Makes "kernel" a testable property. The
   existing `deny.toml` and `no-std-build` job are ~80% of the scaffolding.
5. **(Later, cosmetic)** Relocate `colored`/i18n presentation into adapter crates;
   rename `InvalidFilePath` and the `*_yaml_*` methods.

**Not recommended:** Mode C (WIT / wasm-component boundary). `cardpack` is a
value-type library with a deliberately broad surface; a language-neutral
transition contract does not fit and would add ceremony without a consumer. Revisit
only if a non-Rust driver materializes.

---

## 6. What "done" looks like

- `cargo build --no-default-features` yields a pure `alloc`-only kernel: no
  `std::fs`, no `colored`, no `serde_norway`, no `fluent-templates`.
- `cargo tree --no-default-features` contains none of the banned crates, asserted
  in CI.
- Constructing any deck — including Razz — performs zero runtime I/O.
- Convenience (YAML load-from-file, colored display, i18n, serde) lives behind the
  `full` umbrella or in adapter crates, opt-in.
