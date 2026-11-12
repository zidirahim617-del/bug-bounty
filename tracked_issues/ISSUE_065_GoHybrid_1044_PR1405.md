# ISSUE_065 — Go-Hybrid #1044 → PR #1405 (Channel Element Composite & Primitive Check HYB-E3011)

**Date opened:** 2026-10-22
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #1044 — channel element check ignores composite inner shapes, slice/array distinction, and primitive kind mismatches
**PR:** #1405 — https://github.com/limitlessprojects/Go-Hybrid/pull/1405 (head `zidirahim617-del:fix/1044-channel-element-check`)
**Issue comment:** issuecomment-6379427620 (PR linked)
**Branch:** `fix/1044-channel-element-check` (based on main 9f61f54, commit 83e1750)
**Track:** soundness / BUILDABLE fix

## Scope delivered
- `shape_may_be_elem` (`sendability.rs`): recursive inner shape checks for `Map`, `Slice`, `Array`, `Pointer`; disallowed slice vs array cross-sends.
- `check_chan_send_named_mismatch` (`sendability.rs`): TypeExpr-level recursive `type_exprs_may_be_elem` check for channel sends.
- New test suite: `compiler/crates/hy-check/tests/channel_element_check.rs` (4 cases).

## Verification
- `channel_element_check.rs`: 4/4 pass.
- `const_overflow.rs`: 97/97 pass.
- Full `hy-check` suite: 100% pass.
- Corpus harness: **126/126 verdict agreement**.
