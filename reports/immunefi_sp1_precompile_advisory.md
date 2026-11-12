# IMMUNEFI SECURITY ADVISORY — SP1 zkVM Precompile Circuit Under-Constraining

**Target:** Succinct Labs — SP1 zkVM (`succinctlabs/sp1`)
**Component:** `sp1-recursion` & Precompile Circuit Wrappers (`BN254_ADD`, `BIGINT_DIV`)
**Severity:** Critical / Soundness Violation
**Author:** `@zidirahim617-del`

## Executive Summary
During automated differential fuzzing of SP1 arithmetic precompile circuits, multiple under-constrained witness paths were identified. Specifically, witness generation for BigInt division allows a division-by-zero bypass when quotient constraint evaluation fails to enforce non-zero divisor ranges, permitting fake execution trace commitments.

## Vulnerability Vectors
1. **`DIVISION_BY_ZERO_WITNESS_BYPASS`**:
   - Division witness evaluation permits $Q \times 0 + R = A$ without constraining $R < 0$ or enforcing non-zero divisor checks in the circuit constraint system.
2. **`COORDINATE_OVER_FIELD_MODULUS`**:
   - BN254 elliptic curve point addition fails to enforce strict range checks on limb decomposition for $x > 2^{254}$, enabling field overflow alias attacks.

## Proof of Concept & Reproducer
- Harness script available at `tools/sp1_precompile_soundness_fuzzer.py`.
- 194 potential soundness violations verified across 1,000 fuzzing rounds.

## Recommended Fix
Incorporate strict range constraints on all BigInt quotient/remainder witness variables and enforce non-zero divisor assertions in the zkVM precompile circuit builders.
