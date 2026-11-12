# ISSUE_063 — Go-Hybrid #1112 → PR #1130 (Work 1 of 5, HYB-E1002 assign types)

**Date opened:** 2026-10-21
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #1112 — assignments between variables of different types pass the checker and emit unbuildable Go
**PR:** #1130 — https://github.com/limitlessprojects/Go-Hybrid/pull/1130 (head `zidirahim617-del:fix/1112-int-float64-assign`)
**Issue comment:** issuecomment-6322269153 (PR linked, ground truth + matrix stated)
**Branch:** `fix/1112-int-float64-assign` (based on main aedfa2d, commit 588cf1c)
**Track:** free open issue (no prior claim; verified free before starting)

## Scope delivered
- HYB-E1002 emitted in `process_assign` when both sides resolve to different predeclared scalars (alias-expanded, depth-cap 8).
- New `Checker.type_aliases` + `resolve_named_base` + `is_go_scalar` (model.rs).
- `process_local` keeps declared type on binding when inference resolves (shape priority unchanged).
- Untyped constants / same-type / alias-same-base / same-shape maps stay accepted.
- Closures: not covered (assign path is move-marking only) — declared as scope in PR.

## Verification
- tests/assign_types.rs: 8/8 pass.
- hy-check full: 213 tests, 0 failures.
- Full workspace: 27 suites, 0 failures (gnu toolchain).
- Toolchain note: `cargo +stable-x86_64-pc-windows-gnu` (MSVC link.exe absent on this machine).

## Ladder positioning
- Meaningful compiler-contract fix (BUILDABLE row 13); closes an open issue directly; not docs/cosmetic. Competing only against whoever claims before merge — issue was free at sweep time (only #1112/#1083 remained).
- Expected value: ladder slot candidate ($45–$75 range if merged and ranked in top 10 of cycle); cycle closes 2026-11-01 00:00 UTC.

## Next actions
- Monitor maintainer response; if changes requested, address same-day.
- Work 2: issue #1083 (const with non-constant type ascriptions) next.
- Re-run after any upstream main move (rebase if conflicts).
