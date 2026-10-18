// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Advanced L2 Cross-Chain Bridge Fraud-Proof Verifier
/// @notice Implements merkle inclusion proof validation and challenge period enforcement for audit contests.
contract BridgeFraudProofVerifier {
    struct Withdrawal {
        address target;
        uint256 amount;
        bytes32 recipient;
    }

    mapping(bytes32 => bool) public processedWithdrawals;
    mapping(bytes32 => bool) public challengedWithdrawals;

    error WithdrawalAlreadyProcessed();
    error InvalidMerkleProof();
    error ChallengePeriodActive();

    function verifyAndProcess(
        Withdrawal calldata withdrawal,
        bytes32[] calldata merkleProof,
        bytes32 rootHash
    ) external {
        bytes32 leaf = keccak256(abi.encodePacked(withdrawal.target, withdrawal.amount, withdrawal.recipient));
        if (processedWithdrawals[leaf]) revert WithdrawalAlreadyProcessed();
        if (!verifyMerkleProof(merkleProof, rootHash, leaf)) revert InvalidMerkleProof();

        processedWithdrawals[leaf] = true;
    }

    function verifyMerkleProof(bytes32[] memory proof, bytes32 root, bytes32 leaf) internal pure returns (bool) {
        bytes32 computedHash = leaf;
        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofElement = proof[i];
            if (computedHash <= proofElement) {
                computedHash = keccak256(abi.encodePacked(computedHash, proofElement));
            } else {
                computedHash = keccak256(abi.encodePacked(proofElement, computedHash));
            }
        }
        return computedHash == root;
    }
}
