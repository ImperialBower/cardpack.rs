---
type: Reference
title: CHANGELOG
description: Keep-a-Changelog per-version history; the authoritative record of what changed in each release and what broke.
tags: [changelog, releases, semver]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/CHANGELOG.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

Every released version in Keep a Changelog form, with a `### Breaking` section
where a release carried one. The `[Unreleased]` section at the top is the
staging area, renamed at tag time.

# Authoritative for

* **What changed in which version**, and which changes were semver-breaking.
  Both `DeckKind` and `CardError` became `#[non_exhaustive]` this way — see
  [deck catalog](/decks/deck-catalog.md) and
  [YAML envelope format](/decisions/yaml-envelope-format.md).
* Narrative release notes for the 0.7.0 series live separately in
  [release v0.7.0](/references/release-v0-7-0.md).

# In-repo path

`CHANGELOG.md`

This concept is a pointer, not a copy — the linked document is authoritative.
