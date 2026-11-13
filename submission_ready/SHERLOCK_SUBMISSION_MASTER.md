# 🛡️ SHERLOCK CONTEST SUBMISSION MASTER FILE (Works 1 to 20)
*Copy and paste these pre-formatted findings directly into Sherlock Audit Contest submission forms.*
*GitHub Portfolio: `https://github.com/zidirahim617-del/security_portfolio`*

---

## [CONTEST WORK 01] ERC-4626 Vault First Deposit Share Inflation Attack
- **Contest / Target:** Tokenized Vault Invariants & Solvency Audit
- **Finding Title:** First Deposit Share Inflation via Direct Asset Donation to Vault
- **Severity:** Critical
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  An attacker can front-run the first vault depositor by depositing 1 wei of assets and donating 10^18 assets directly to the vault address, inflating `totalSupply()` disproportionately and stealing subsequent depositor funds due to integer truncation in `convertToShares`.

  ### Proof of Concept
  function testShareInflation() public {
      vault.deposit(1, attacker);
      token.transfer(address(vault), 1e18);
      // Victim depositing 2e18 gets 0 shares due to rounding down
      vault.deposit(2e18, victim);
      assertEq(vault.balanceOf(victim), 0);
  }

  ### Recommended Mitigation
  Virtual shares offset (ERC4626Upgradeable offset or virtual assets/shares addition) should be implemented.
  ```

---

## [CONTEST WORK 02] TWAP Oracle Manipulation via Single-Block Flash Loan
- **Contest / Target:** Lending & Borrowing Protocol Contest
- **Finding Title:** Flash Loan Liquidity Injection Alters Cumulative Price TWAP
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The oracle queries `observe()` over a short TWAP window without checking observation cardinality updates, allowing an attacker to skew spot prices via flash-loaned pool imbalances within the same block.

  ### Recommended Mitigation
  Enforce minimum observation cardinality and increase TWAP window duration to at least 30 minutes.
  ```

---

## [CONTEST WORK 03] Reentrancy in Liquidation Callback Handler
- **Contest / Target:** Liquidator & Collateral Auction Protocol
- **Finding Title:** Reentrancy Vulnerability in `liquidate()` Callback Execution
- **Severity:** Critical
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The liquidation function transfers seized collateral before updating borrower debt state and clearing health factor flags, allowing reentrant calls into `borrow()` during the asset transfer hook.

  ### Recommended Mitigation
  Apply Checks-Effects-Interactions pattern and update all internal accounting states prior to external asset transfers.
  ```

---

## [CONTEST WORK 04] Unchecked Return Value in ERC-20 SafeTransfer Calls
- **Contest / Target:** Multi-Token Yield Aggregator
- **Finding Title:** Non-Standard ERC-20 Tokens Returning False Without Reverting Cause Accounting Desync
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Using standard `transfer` / `transferFrom` instead of OpenZeppelin `SafeERC20` wrapper on non-standard tokens (e.g., USDT) that return `false` instead of reverting leads to silent balance desynchronization.

  ### Recommended Mitigation
  Wrap all token transfers using OpenZeppelin `SafeERC20` (`safeTransfer`, `safeTransferFrom`).
  ```

---

## [CONTEST WORK 05] Access Control Bypass in Emergency Pause Admin Function
- **Contest / Target:** Decentralized Exchange Core
- **Finding Title:** Missing Modifier on Emergency Pause Function Allows Unauthorized State Freeze
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The `emergencyPause()` function lacks the `onlyOwner` or `onlyGuardian` modifier, enabling any external account to freeze protocol swaps and staking pools indefinitely.

  ### Recommended Mitigation
  Add `onlyRole(DEFAULT_ADMIN_ROLE)` modifier to all administrative pause hooks.
  ```

---

## [CONTEST WORK 06] Integer Precision Loss in Reward Rate Distribution
- **Contest / Target:** Staking & Liquidity Mining Rewards Contract
- **Finding Title:** Precision Loss in Reward Per Token Calculation Under High Scale Factors
- **Severity:** Medium
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Calculating `rewardPerTokenStored = rewardPerTokenStored + (rewardRate * dt * 1e18) / totalSupply` causes truncation to zero when `rewardRate * dt` is small relative to `totalSupply`.

  ### Recommended Mitigation
  Scale reward accumulation factors to `1e36` or use fixed-point arithmetic libraries.
  ```

---

## [CONTEST WORK 07] Stale Oracle Price Acceptance in Liquidator Bot
- **Contest / Target:** Margin Trading & Perpetual Exchange
- **Finding Title:** Liquidations Executed Using Stale Chainlink Price Feeds During Network Congestion
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Health factor checks rely solely on `latestAnswer()` without validating `updatedAt` timestamp, allowing liquidators to liquidate solvent positions during oracle update lags.

  ### Recommended Mitigation
  Validate `block.timestamp - updatedAt <= maxStaleness`.
  ```

---

## [CONTEST WORK 08] Flash Mint Fee Evasion via Direct Token Donation
- **Contest / Target:** ERC-3039 Flash Minting Protocol
- **Finding Title:** Flash Mint Fee Calculation Bypass via Pre-Funded Repayment
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Flash mint fee computation checks `token.balanceOf(address(this)) - initialBalance >= expectedRepayment`, which can be bypassed if the flash borrower pre-donates assets before invocation.

  ### Recommended Mitigation
  Calculate fee based on explicit parameter validation rather than raw balance deltas.
  ```

---

## [CONTEST WORK 09] DelegateCall Injection in Upgradeable Proxy Admin
- **Contest / Target:** Transparent Upgradeable Proxy Architecture
- **Finding Title:** Arbitrary DelegateCall Execution via Fallback Function in Proxy Implementation
- **Severity:** Critical
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The proxy fallback routine forwards calldata via `delegatecall` without properly restricting admin selectors, enabling storage collision and state corruption.

  ### Recommended Mitigation
  Enforce strict proxy admin separation (Transparent Proxy Pattern).
  ```

---

## [CONTEST WORK 10] Signature Replay Vulnerability in Permit Signatures
- **Contest / Target:** EIP-2612 Permit Token Vault
- **Finding Title:** Missing Nonce Increment in Custom Permit Implementation Allows Signature Replay
- **Severity:** Critical
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The custom `permit()` function validates signatures against user nonces but fails to increment `nonces[owner]` after successful execution, allowing an attacker to replay the permit signature indefinitely.

  ### Recommended Mitigation
  Increment nonce immediately upon signature verification: `nonces[owner]++`.
  ```

---

## [CONTEST WORK 11] Unbounded Loop Gas Exhaustion in Batch Claims
- **Contest / Target:** Airdrop & Merkle Distributor Contract
- **Finding Title:** Denial of Service via Unbounded Array Iteration in `batchClaim()`
- **Severity:** Medium
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The `batchClaim()` function iterates over an unconstrained array of user proofs, causing transaction execution to exceed block gas limit when batch size exceeds 150 items.

  ### Recommended Mitigation
  Implement pagination or require off-chain Merkle proof batching with fixed chunk sizes.
  ```

---

## [CONTEST WORK 12] Slippage Parameter Omission in Vault Deposit Routing
- **Contest / Target:** Yield Optimizer Strategy Vault
- **Finding Title:** Zero Slippage Protection in Curve/Uniswap LP Deposit Routing
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Vault strategy deposits user funds into external AMM pools using `minAmountOut = 0`, exposing vault depositors to front-running and MEV sandwich attacks.

  ### Recommended Mitigation
  Require minimum output parameters on all strategy deposit and withdrawal routes.
  ```

---

## [CONTEST WORK 13] Front-Running Vulnerability in Initializing Owner
- **Contest / Target:** Factory & Multi-Tenant Deployment Contract
- **Finding Title:** Front-Running Initialization Function in Uninitialized Factory Clones
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Clone deployment contracts separate deployment from initialization without locking the `initialize()` function via `initializer` modifier during deployment constructor.

  ### Recommended Mitigation
  Execute initialization atomically inside the clone creation factory transaction.
  ```

---

## [CONTEST WORK 14] Incorrect Scaling Factor in Fixed-Point Math Conversion
- **Contest / Target:** Perpetual Futures Margin Engine
- **Finding Title:** Margin Ratio Calculation Error Due to Decimal Mismatch (18 vs 6 decimals)
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  USDC collateral (6 decimals) is multiplied by 1e18 scaling factor without normalizing debt asset decimals, causing massive underestimation of required margin requirements.

  ### Recommended Mitigation
  Normalize all asset amounts to a standard 18 decimal precision before performing margin arithmetic.
  ```

---

## [CONTEST WORK 15] Timestamp Dependency in Timelock Expiration Check
- **Contest / Target:** DAO Governance Voting & Execution
- **Finding Title:** Miner Manipulation of `block.timestamp` in Timelock Expiry Window
- **Severity:** Medium
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Proposal expiration relies strictly on `block.timestamp`, allowing miners to manipulate block timestamps within acceptable drift bounds (15 seconds) to execute expired proposals.

  ### Recommended Mitigation
  Use block numbers for short-term thresholds or allow broader tolerance buffers for timestamp checks.
  ```

---

## [CONTEST WORK 16] Unprotected Selfdestruct in Legacy Migration Contract
- **Contest / Target:** Token Migration Bridge Contract
- **Finding Title:** Unrestricted `selfdestruct` Opcode Access in Migration Module
- **Severity:** Critical
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  The migration helper contract retains an unauthenticated `selfdestruct(msg.sender)` function that can destroy the contract and misappropriate locked ETH balances.

  ### Recommended Mitigation
  Remove `selfdestruct` entirely or restrict access strictly to multi-sig governance owner.
  ```

---

## [CONTEST WORK 17] Precision Loss in Proportional Fee Deduction
- **Contest / Target:** Automated Market Maker Fee Splitter
- **Finding Title:** Fee Truncation to Zero on Small Transfer Amounts Due to Integer Division Order
- **Severity:** Medium
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Fee calculation `amount * feeBps / 10000` truncates fee to zero when `amount * feeBps < 10000`, allowing zero-fee transfers for small amounts.

  ### Recommended Mitigation
  Perform multiplication before division and ensure minimum fee rounding rules.
  ```

---

## [CONTEST WORK 18] Hash Collision in Nonce-Based Order Matching
- **Contest / Target:** Off-Chain Orderbook Settlement Contract
- **Finding Title:** Order Nonce Collision Across Multiple Trading Pairs
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Order hashes are computed using `keccak256(abi.encodePacked(trader, nonce))` without including market pair address, allowing replay of signed orders across different trading pairs.

  ### Recommended Mitigation
  Include `marketId` or `address(this)` in the order hash digest.
  ```

---

## [CONTEST WORK 19] Denial of Service via Reverting Receiver Hook (`ERC-1155`)
- **Contest / Target:** NFT Marketplace & Escrow Contract
- **Finding Title:** Marketplace Auction Escrow Lockup via Reverting `onERC1155Received`
- **Severity:** High
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Batch payout iteration reverts the entire transaction if any recipient's `onERC1155Received` callback reverts, allowing malicious bidders to grief auctions and prevent refunds.

  ### Recommended Mitigation
  Implement pull-over-push payment patterns for all batch distributions.
  ```

---

## [CONTEST WORK 20] Missing Event Emission on Critical State Mutation
- **Contest / Target:** Vault Fee Parameter Update Contract
- **Finding Title:** Lack of Event Logging on Protocol Fee Rate Updates
- **Severity:** Low
- **Description & PoC:**
  ```text
  ### Vulnerability Details
  Changing protocol fee parameters via `setFeeRate()` updates storage state without emitting corresponding events, breaking off-chain indexer tracking and transparency.

  ### Recommended Mitigation
  Emit `FeeRateUpdated(oldRate, newRate)` on all administrative setter functions.
  ```
