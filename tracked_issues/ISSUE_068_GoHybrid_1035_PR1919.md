# ISSUE_068 — Go-Hybrid #1035 → PR #1919 (Package-Level Builtin Shadowing HYB-E1001)

**Date opened:** 2026-10-22
**Platform / repo:** Go-Hybrid (limitlessprojects/Go-Hybrid)
**Issue:** #1035 — user-declared `panic` silently defeats `option.unwrap()`
**PR:** #1919 — https://github.com/limitlessprojects/Go-Hybrid/pull/1919 (head `zidirahim617-del:fix/1035-user-panic-shadow`)
**Issue comment:** issuecomment-6422956256 (PR linked)
**Branch:** `fix/1035-user-panic-shadow` (based on main 9f61f54, commit ccdd0d5)
**Track:** soundness / BUILDABLE fix

## Scope delivered
- `resolve.rs` (`check_file_undefined_names`): added checks against top-level declarations (`Decl::Fn`, `Decl::Const`, `Decl::TypeAlias`, `Decl::Struct`, `Decl::Enum`) colliding with Go prelude builtins (`panic`, `len`, `make`, `delete`, `copy`, `append`, `close`, `recover`), emitting **HYB-E1001**.
- New test suite: `compiler/crates/hy-check/tests/user_panic_shadow.rs` (2 test cases).

## Verification
- `user_panic_shadow.rs`: 2/2 pass.
- Full `hy-check` suite: 100% pass.
- Corpus harness: **126/126 verdict agreement**.
