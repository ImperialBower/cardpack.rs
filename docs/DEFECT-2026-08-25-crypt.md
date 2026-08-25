# DEFECT Report: branch `crypt`

> Review date: 2026-08-25
>
> Scope: code review of the `crypt` branch after the sealed-deck / commit-reveal / holder-key seal merge set.
>
> Status: open. This report records review findings only; it does not apply code changes.

## Summary

Two review findings stood out as likely user-visible defects:

1. A commit-reveal participant can reveal twice and overwrite their earlier reveal, which breaks the binding property of the round.
2. `HolderKeySeal` truncates an overlong deck name length field in associated data without rejecting the input, which can produce malformed AD if the deck name ever exceeds 65535 bytes.

## Defect 1: duplicate reveals overwrite the first contribution

**Severity:** High

**Location:** [src/seal/commit/round.rs](../src/seal/commit/round.rs#L239)

**Observed behavior:** `ShuffleRound::reveal` checks that the participant is known, that every participant has committed, and that the reveal opens the stored commitment. It then inserts the contribution into `self.reveals` without checking whether that participant already revealed.

**Why this is a defect:** `BTreeMap::insert` replaces any existing value for the same key. That means a participant can reveal a valid contribution and later call `reveal` again with a different contribution that also opens the same commitment. The second reveal silently replaces the first, and `is_complete()` still returns true because it only checks map length. The final seed is therefore not bound to the first accepted reveal.

**Evidence:**

- `reveal` ends with `self.reveals.insert(who, c);` and no duplicate guard.
- `is_complete()` is `self.reveals.len() == self.participants.len()`, so an overwrite does not change completeness.

**Recommended fix:** Reject duplicate reveals explicitly before insertion, and add a unit test that attempts a second reveal for the same participant and expects an error.

## Defect 2: associated-data name length is truncated instead of validated

**Severity:** Medium

**Location:** [src/seal/aead/holder_key_seal.rs](../src/seal/aead/holder_key_seal.rs#L164)

**Observed behavior:** `associated_data` computes `name_len` with `u16::try_from(name.len()).unwrap_or(u16::MAX)` and then appends the full deck name bytes to the AD buffer.

**Why this is a defect:** If `deck_name` ever exceeds 65535 bytes, the encoded length prefix no longer matches the bytes that follow it. The comment assumes deck names are always short, but the implementation does not enforce that assumption. The result is malformed associated data and decryption failure that is avoidable at the boundary.

**Evidence:**

- The length prefix is truncated to `u16::MAX`.
- The full `name` is still appended, so the length field and payload can disagree.

**Recommended fix:** Validate the name length up front and return a dedicated error if it exceeds `u16::MAX`, or otherwise guarantee the invariant before constructing AD.

## Notes

- I did not find additional high-confidence correctness issues in the new sealed-deck core during this review.
- The rest of the branch appears internally consistent with the documented design and test vectors.

---

## Resolution (2026-08-25)

**Status: closed.** Both findings were verified against the code, tested, and
fixed. The original text above is left unedited; this section records what
was confirmed, what was refuted, and what shipped.

### Defect 1 — duplicate reveals

| Claim | Verdict |
|---|---|
| `reveal` has no duplicate guard; `insert` overwrites | **Confirmed** |
| `is_complete()` only checks map length | **Confirmed** |
| A participant can reveal a *different* contribution that also opens the commitment | **Refuted** |
| "The final seed is not bound to the first accepted reveal" | **Refuted** |
| Severity High | **Downgraded to Low** |

A `Commitment` is `SHA-256(tag ‖ 32 bytes)`. Two distinct contributions
opening one commitment is a SHA-256 collision, so the attack step is not
available. Measured:

```
second reveal, different contribution -> Err(CommitmentMismatch(1))   (rejected)
second reveal, same contribution      -> Ok(())                       (no guard)
                     seed changed?    -> false                        (no-op)
```

Binding held, and it held because of the hash. The missing guard was still a
real inconsistency — `commit` rejects a repeat, and so does
`Revealed::reveal` — so `reveal` now returns `CardError::AlreadyRevealed`.
The guard sits *after* the commitment check, so a wrong contribution still
reads as a mismatch rather than a duplicate.

### Defect 2 — truncated associated-data length

| Claim | Verdict |
|---|---|
| `unwrap_or(u16::MAX)` truncates while the full name is still appended | **Confirmed** |
| "malformed associated data **and decryption failure**" | **Refuted** |
| Severity Medium | **Downgraded to Low** |

`seal` and `unseal` both call `associated_data`, so a truncated prefix is
symmetric and the payload round-trips. The genuine costs are that the AD stops
parsing unambiguously, and — the stronger point, which the report did not
make — that `Codebook::encode_pile` **rejects** this same input while the
AEAD backend truncated it.

Reachability is effectively zero: every shipped deck name is 4–19 bytes, so
only a consumer's own `DeckedBase` returning a >64 KiB name can reach it.
Fixed with `AeadSealError::DeckNameTooLong`. `associated_data` became
fallible and `seal`/`unseal` propagate, so **no public signature changed**.

### A third site the report missed

The same truncate-don't-reject pattern was in `CombinedSeed::combine` and
therefore `ShuffleRound::new`. The participant count is part of the frozen
`v1` preimage, so truncating it would return a seed no verifier could
reproduce. `combine` now returns `Result<Self, CardError>` and both reject
with `CardError::TooManyParticipants`. **No wire format changed.**

Two further hits of the same pattern in `Permutation` (`canonical_bytes`,
`inverse`) are **dead code, not defects**: every constructor already rejects
`n > u16::MAX`.

### Verification

Six tests added; all four new guards mutation-checked (deleting each one
reddens its named test). Full suite 730 passed / 0 failed under
`full,crypto,seal-test-double`; clippy-pedantic, `cargo doc -D warnings` on
three feature sets, `make no-std`, thumb, wasm, `cargo deny`, and MSRV 1.85
all green.

### Note on the report itself

Its two line citations were stale (`round.rs#L239`, `holder_key_seal.rs#L164`
pointed into doc comments after an intervening commit). Both severities were
wrong in the same direction, and each was refuted by a single test. If this
report was produced by an automated reviewer, treat its impact claims as
hypotheses to verify rather than as findings.
