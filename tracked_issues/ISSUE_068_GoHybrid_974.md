# ISSUE_068 — Go-Hybrid #974 (Conditional Branch Rebinding & Inner-Block Shadowing in Sharing Analysis)

**Date opened:** 2026-10-22
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #974 — conditional branch rebinding and inner-block shadowing clobber return-origin and multi-return alias tracking in `hy-check` (P0 Critical Soundness)
**Track:** soundness / BUILDABLE fix

## Scope & Root Cause Analysis
- **Root Cause:** `collect_callee_const_return_roots` and `callee_shared_return_pairs` in `loans.rs` performed linear statement walks without block scoping, branch merging, or clean callee state separation.
- **Impact:** Allowed conditional branch reassignments and lexical shadowing to bypass sharing/const-origin checks (`HYB-E3003`, `HYB-E3006`, `HYB-E3004`).

## Status
- Identified and analyzed; designated for next batch deployment alongside Works 6–10.
