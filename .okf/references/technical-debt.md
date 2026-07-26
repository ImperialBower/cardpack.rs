---
type: Reference
title: Technical debt register
description: "Tracked debt sourced from TODO / TODO RF / TODO: HACK comments and EPIC docs, maintained by the /backlog skill."
tags: [debt, backlog, maintenance]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/TECHNICAL_DEBT.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The debt register: refactors flagged `TODO RF` in source, hacks flagged
`TODO: HACK`, and deferrals carried over from EPIC docs, each with a pointer
to where it lives in the code.

# Caveats

* Items tagged 🤖 were proposed by automated review — suggestions, not facts.
* It **mirrors** EPIC docs rather than superseding them, and has drifted out
  of sync with them before (noted in-file). When the two disagree, the EPIC
  wins.

# In-repo path

`docs/TECHNICAL_DEBT.md`

This concept is a pointer, not a copy — the linked document is authoritative.
