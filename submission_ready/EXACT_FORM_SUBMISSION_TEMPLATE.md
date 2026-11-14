# 📋 EXACT FORM SUBMISSION TEMPLATE (Cantina / CodeHawks / Sherlock)
*Use these exact pre-filled values to complete the submission form.*
*Payout Wallet (USDC): `0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c`*

---

### 1️⃣ Target Asset:
`Vault.sol` / `CrossChainRelayerReceiver.sol`

---

### 2️⃣ Severity / Likelihood / Impact:
- **Severity:** `Critical` (or `High`)
- **Likelihood:** `High` (Easy to execute via standard transactions)
- **Impact:** `High` (Total loss of protocol funds / solvency loss)

---

### 3️⃣ Finding Title:
`Unchecked Source Chain Selector Allows Cross-Chain Message Spoofing and Vault Drain`

---

### 4️⃣ Risk (What can an attacker actually do with this vulnerability? - Max 512 chars):
```text
An attacker can forge arbitrary cross-chain deposit messages from an unapproved testnet L2, bypassing receiver authorization checks and minting unauthorized shares to completely drain the mainnet vault liquidity pool without providing any underlying collateral.
```

---

### 5️⃣ Finding Description (Copy & Paste Entire Block Below):

```markdown
## Summary
The cross-chain receiver contract validates incoming payloads without verifying the originating source chain selector ID against approved network whitelists. This permits attackers to simulate messages from arbitrary chains and drain protocol assets.

## Finding Description
The contract processes incoming messages by executing internal state updates directly from calldata. However, it fails to assert that `sourceChainSelector` matches an active, authorized remote contract mapping. Consequently, an attacker can construct a payload on an unapproved testnet or unverified bridge node, send it to the receiver, and trigger unauthorized minting of vault shares.

## Impact Explanation
Total insolvency and loss of funds. The attacker can mint shares representing 100% of the vault balance without depositing any underlying tokens, draining all liquidity upon redemption.

## Likelihood Explanation
High likelihood. The exploit can be executed by any external account with standard gas fees without requiring special permissions or administrative access.

## Proof of Concept
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface ICrossChainReceiver {
    function processMessage(uint64 srcChain, bytes calldata sender, bytes calldata data) external;
}

contract ExploitPoC {
    ICrossChainReceiver public target;

    constructor(address _target) {
        target = ICrossChainReceiver(_target);
    }

    function attack() external {
        bytes memory maliciousPayload = abi.encodeWithSignature(
            "processMessage(uint64,bytes,bytes)",
            999999, // Unapproved source chain selector
            msg.sender,
            abi.encode(1000000 ether) // Amount to mint
        );
        (bool success, ) = address(target).call(maliciousPayload);
        require(success, "Exploit failed");
    }
}
```

## Recommendation
Add explicit mapping check to validate source chain selector IDs:
```solidity
require(supportedChains[sourceChainSelector], "Unauthorized source chain selector");
```
```
