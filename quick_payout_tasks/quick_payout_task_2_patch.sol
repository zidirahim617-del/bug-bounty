// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Quick Payout Task 2: Solidity Access Control Patch
/// @notice Target: Polar / Algora 4-6 Day Bounties
/// @dev Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c
contract QuickPayoutAccessPatch {
    address public owner;
    bool public paused;

    error Unauthorized();
    error ContractPaused();

    constructor() {
        owner = msg.sender;
        paused = false;
    }

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    modifier whenNotPaused() {
        if (paused) revert ContractPaused();
        _;
    }

    function sensitiveAction() external onlyOwner whenNotPaused {
        // Secure execution flow
    }
}
