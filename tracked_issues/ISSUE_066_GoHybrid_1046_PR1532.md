# ISSUE_066 — Go-Hybrid #1046 → PR #1532 (Projected Map Element Borrows HYB-E3104)

**Date opened:** 2026-10-22
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #1046 — projected map-element borrows (`&m[k].f`, `&m[k][i]`) in argument and expression positions bypass HYB-E3104
**PR:** #1532 — https://github.com/limitlessprojects/Go-Hybrid/pull/1532 (head `zidirahim617-del:fix/1046-projected-map-borrow`)
**Issue comment:** issuecomment-6386762543 (PR linked)
**Branch:** `fix/1046-projected-map-borrow` (based on main 9f61f54, commit 0bf8826)
**Track:** soundness / BUILDABLE fix

## Scope delivered
- `map_slot_borrow_span` (`addr_loan.rs`): added recursive `has_map_index_root` helper to unwrap struct field projections and array index expressions down to a map index root, ensuring HYB-E3104 fires for `&m[k].f` and `&m[k][i]` in expression/argument positions.
- New test suite: `compiler/crates/hy-check/tests/map_borrow_check.rs` (2 test cases).

## Verification
- `map_borrow_check.rs`: 2/2 pass.
- Full `hy-check` suite: 100% pass.
- Corpus harness: **126/126 verdict agreement**.
