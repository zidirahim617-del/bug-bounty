# ISSUE_067 — Go-Hybrid #982 → PR #1723 (`(T, error)` Return Signature Slot Enforcement HYB-E1002)

**Date opened:** 2026-10-22
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #982 — `(T, error)` return signatures accept non-error values in error slot
**PR:** #1723 — https://github.com/limitlessprojects/Go-Hybrid/pull/1723 (head `zidirahim617-del:fix/982-error-return-signature`)
**Issue comment:** issuecomment-6398093824 (PR linked)
**Branch:** `fix/982-error-return-signature` (based on main 9f61f54, commit f80fa3b)
**Track:** soundness / BUILDABLE fix

## Scope delivered
- `buildable.rs`: added `OpKind::Error` variant for Go's builtin `error` interface.
- `kind_of_named`: mapped `"error"` to `OpKind::Error`.
- `kinds_conflict`: configured `Error` targets to reject concrete primitive scalars (`Int`, `Float`, `Bool`, `Str`, `Slice`, `Array`, `Map`, `Chan`, `Pointer`) with **HYB-E1002**.
- New test suite: `compiler/crates/hy-check/tests/return_error_slot.rs` (4 test cases).

## Verification
- `return_error_slot.rs`: 4/4 pass.
- Full `hy-check` suite: 100% pass.
- Corpus harness: **126/126 verdict agreement**.
