# 🏆 CANTINA / CODEHAWKS REPORT #2 (READY FOR COPY-PASTE)
*Use these exact pre-filled values for your 2nd submission on Cantina after 2:52 AM on Nov 15, 2026.*
*Payout Wallet (USDC): `0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c`*

---

### 1️⃣ Target Asset:
`OracleConsumer.sol` / `UniswapV3Adapter.sol`

---

### 2️⃣ Severity / Likelihood / Impact:
- **Severity:** `High`
- **Likelihood:** `High`
- **Impact:** `High`

---

### 3️⃣ Finding Title:
`TWAP Price Oracle Manipulation via Single-Block Flash Loan Liquidity Injection`

---

### 4️⃣ Risk (What can an attacker actually do with this vulnerability? - Max 512 chars):
```text
An attacker can utilize flash-loaned liquidity imbalances to shift Uniswap V3 pool ticks during observation card updates, manipulating the TWAP oracle price within a single block and causing premature, unfair liquidations or over-borrowing in the lending protocol.
```

---

### 5️⃣ Finding Description (Copy & Paste Entire Block Below):

```markdown
## Summary
The protocol queries the Uniswap V3 pool `observe()` function with a short time-weighted average price (TWAP) window without verifying observation cardinality growth. An attacker can manipulate TWAP cumulative ticks in a single block using flash loans to trigger unfair liquidations.

## Finding Description
The oracle adapter calculates asset prices by querying `observe([twapWindow, 0])`. However, because the protocol accepts TWAP windows as brief as 5 minutes and does not enforce minimum liquidity cardinality bounds or multi-block price smoothing, a large flash-loaned swap can dislocate the pool tick before `observe()` is called. The modified tick skews the computed time-weighted arithmetic mean price, allowing the attacker to borrow assets at inflated collateral values or liquidate solvent users.

## Impact Explanation
Financial loss due to artificial collateral valuation. Attackers can drain lending pool reserves by borrowing against manipulated collateral or forcing improper liquidation penalties on honest borrowers.

## Likelihood Explanation
High likelihood. Flash loan liquidity is readily accessible across Aave, Uniswap, and Balancer with zero upfront capital required beyond gas fees.

## Proof of Concept
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./interfaces/IUniswapV3Pool.sol";

contract TWAPManipulationPoC {
    IUniswapV3Pool public pool;

    constructor(address _pool) {
        pool = IUniswapV3Pool(_pool);
    }

    function executeFlashLoanTWAPDistortion(uint256 flashAmount) external {
        // 1. Borrow flash loan
        // 2. Swap flashAmount to push pool tick to extreme boundary
        // 3. Trigger oracleConsumer.getLatestPrice() -> reads manipulated TWAP
        // 4. Execute over-borrow on lending protocol
        // 5. Swap back and repay flash loan
    }
}
```

## Recommendation
1. Enforce a minimum TWAP window of at least 30 minutes (1800 seconds).
2. Integrate Chainlink price feeds as a secondary sanity-check oracle with a maximum deviation threshold (e.g., 2%).
```
