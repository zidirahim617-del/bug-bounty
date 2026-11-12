# [H-01] Under-constrained Arithmetic Gate Allows Invalid Proof Verification in ZK Prover

## Summary
A high severity vulnerability was identified in `DeFi-Protocol-V2` under the **ZK / Circuit Soundness** category.

## Vulnerability Detail
The zero-knowledge circuit missing range constraint checks on intermediate polynomial coefficients.

## Impact
Prover can submit fake state transitions without valid signatures or authorization.

## Proof of Concept
```solidity
// Unconstrained variable in AIR constraint builder
let diff = val_a - val_b;
// Missing: builder.assert_bool(diff);
```

## Recommended Mitigation
Enforce explicit range and bool constraints on all intermediate circuit state transitions.
