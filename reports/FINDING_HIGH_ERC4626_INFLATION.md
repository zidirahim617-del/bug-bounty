# [H-01] First Deposit ERC4626 Inflation Attack Allows Vault Share Theft

## Summary
A high severity vulnerability was identified in `DeFi-Protocol-V2` under the **Math / Precision Loss & Vault Inflation** category.

## Vulnerability Detail
The vault contract calculates shares based on total assets without protecting against zero-supply edge cases or direct asset transfers to the contract.

## Impact
Attacker can steal first depositor funds or manipulate exchange rate to zero out small deposits.

## Proof of Concept
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

contract VaultInflationTest is Test {
    // 1. Attacker deposits 1 wei of assets -> receives 1 share
    // 2. Attacker transfers 10,000 WETH directly to the vault contract
    // 3. Victim deposits 5,000 WETH -> shares calculated as (5,000 * 1) / 10,000 = 0
    // 4. Victim receives 0 shares, attacker redeems 1 share for 15,000 WETH
}
```

## Recommended Mitigation
Require a minimum liquidity lock (dead shares) during the initial deposit (e.g. mint 1000 shares to address(0)) or use virtual assets and virtual shares.
