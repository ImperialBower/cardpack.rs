---
type: Reference
title: DEFECT report — branch `crypt` (2026-08-25)
description: A two-finding code review of the sealed-deck branch; both findings pointed at real code but overstated impact, and both were fixed as consistency defects along with a third truncation site the report missed.
tags: [defect, review, seal, crypto, resolved]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/DEFECT-2026-08-25-crypt.md
timestamp: 2026-08-25T12:00:00Z
---

# What it covers

A code review of branch `crypt` after the sealed-deck / commit–reveal /
holder-key seal work, raising two findings. The in-repo document is
`docs/DEFECT-2026-08-25-crypt.md`; it carries the original text plus an
appended resolution.

# Outcome

Both findings named **real code**. Both stated an **impact that testing
refuted**, and both were downgraded from High/Medium to Low and fixed anyway
as API-consistency defects.

* **#1 duplicate reveal** (`ShuffleRound::reveal` had no guard — true). The
  claimed break of binding does not hold: a second, *different* contribution
  opening one commitment is a SHA-256 collision, so a repeat is either
  rejected as `CommitmentMismatch` or an exact no-op, and the seed cannot
  move. Fixed with `CardError::AlreadyRevealed`, checked *after* the
  commitment so a bad contribution still reads as a mismatch.
* **#2 truncated AD name length** (`unwrap_or(u16::MAX)` — true). The claimed
  decryption failure does not hold: `seal` and `unseal` build the associated
  data identically, so it round-trips. The real costs are ambiguous parsing
  and disagreement with `Codebook::encode_pile`, which rejects the same
  input. Fixed with `AeadSealError::DeckNameTooLong`; `associated_data`
  became fallible, so no public signature changed.
* **A third site the report missed**: `CombinedSeed::combine` and
  `ShuffleRound::new` truncated the participant count. `combine` now returns
  `Result` and both reject with `CardError::TooManyParticipants`, since the
  count is part of the frozen `v1` preimage.

Two further `unwrap_or(u16::MAX)` hits in `Permutation` are **dead code, not
defects** — every constructor already rejects `n > u16::MAX`.

# Lesson worth keeping

Severity in a review is a claim like any other, and this report's two
severities were both wrong in the same direction. Each was refuted by a
single test. Treat an unvalidated impact claim as a hypothesis: verify the
mechanism before accepting the rating, and fix the underlying inconsistency
on its own merits. See [EPIC-04](epic-04-sealed-decks.md).
