# CLAUDE.md

## Knowledge bundle (OKF)

This repo maintains an [Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
bundle at [`.okf/`](.okf/index.md) — distilled knowledge about the crate's
architecture, deck catalog, workflows, and load-bearing decisions.

- **Consume:** before non-trivial work, read [`.okf/index.md`](.okf/index.md)
  and follow links into the concepts relevant to the task. In particular,
  check `.okf/decisions/` before "cleaning up" Cargo.toml or feature flags —
  it documents refactors that pass CI but break downstream consumers.
- **Maintain:** when a change affects documented knowledge (features, decks,
  workflows, invariants), update the affected `.okf/` concepts in the same
  change: refresh the body and `timestamp`, fix cross-links, update the
  directory `index.md`, and append a dated entry to `.okf/log.md`.
- **Validate:** every non-reserved `.md` in `.okf/` needs YAML frontmatter
  with a non-empty `type`. If the `okf` skill is available, use
  `/okf:validate .okf --strict` before committing bundle changes.
