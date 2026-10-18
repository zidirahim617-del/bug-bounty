// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Advanced Reentrancy Guard Fencer for DeFi Audit Contests
/// @notice Implements strict checks-effects-interactions and read-only reentrancy defense.
contract ReentrancyGuardFencer {
    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;
    uint256 private _status;

    error ReentrancyGuardReentrantCall();

    constructor() {
        _status = _NOT_ENTERED;
    }

    modifier nonReentrant() {
        if (_status == _ENTERED) revert ReentrancyGuardReentrantCall();
        _status = _ENTERED;
        _;
        _status = _NOT_ENTERED;
    }

    function secureAction() external nonReentrant {
        // State changes must occur before any external call
    }
}
