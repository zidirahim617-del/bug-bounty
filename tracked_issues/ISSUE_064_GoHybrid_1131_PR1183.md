# ISSUE_064 — Go-Hybrid #1131 → PR #1183 (Bare Unknown Dotted Root HYB-E1001)

**Date opened:** 2026-10-21
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #1131 — bare unknown dotted root (no import, e.g. `widgets.count`) passes check and fails `go build`
**PR:** #1183 — https://github.com/limitlessprojects/Go-Hybrid/pull/1183 (head `zidirahim617-del:fix/1131-bare-unknown-dotted-root`)
**Issue comment:** issuecomment-6340883956 (PR linked, controls verified)
**Branch:** `fix/1131-bare-unknown-dotted-root` (based on main 9f61f54, commit 98b7147)
**Track:** free open issue (soundness / BUILDABLE)

## Scope delivered
- `resolve.rs` (`Expr::Field` arm): emits HYB-E1001 when a dotted root is not found in `allowed`, `file_decls`, `packages`, `BUILTIN_PACKAGES`, or `is_curated_package_root`.
- `undefined_names.rs`: updated to reject unknown roots while keeping curated roots (`hex.encode`) accepted.
- New test suite: `compiler/crates/hy-check/tests/unknown_dotted_root.rs` (7 cases).

## Verification
- `unknown_dotted_root.rs`: 7/7 pass.
- `undefined_names.rs`: 23/23 pass.
- Full `hy-check` suite: 100% pass.
- Corpus harness: **126/126 verdict agreement** (0 mismatches, 0 tripwires).

## Ladder positioning
- P2 / Medium soundness bug fix directly closing issue #1131.
- Contends for November 1 cycle ladder allocation ($45–$75 range).
