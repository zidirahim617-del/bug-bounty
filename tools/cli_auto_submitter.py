#!/usr/bin/env python3
"""
CLI Auto-Submitter & Form Payload Formatter
Generates and displays the exact, submission-ready markdown texts and JSON payloads
for Sherlock, Code4rena, and Immunefi official contest forms.
"""

import json
import os

WALLET_ADDRESS = "0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c"

def render_submission_forms():
    print("=" * 70)
    print("SHERLOCK & CODE4RENA & IMMUNEFI OFFICIAL FORM PAYLOADS")
    print("=" * 70)
    
    sherlock_payload = {
        "platform": "Sherlock",
        "form_field_title": "[H-01] First Deposit ERC4626 Inflation Attack Allows Vault Share Theft",
        "form_field_severity": "High",
        "form_field_vulnerability_details": "The vault contract calculates shares based on total assets without protecting against zero-supply edge cases or direct asset transfers. An attacker can frontrun deposits, inflate assets, and cause subsequent depositors to receive 0 shares.",
        "form_field_proof_of_concept": "See reports/FINDING_HIGH_ERC4626_INFLATION.md for complete Foundry PoC test contract.",
        "payout_wallet": WALLET_ADDRESS
    }

    code4rena_payload = {
        "platform": "Code4rena",
        "form_field_title": "Unconstrained Polynomial Witness Evaluation in ZK Verifier Circuit",
        "form_field_severity": "High / Critical",
        "form_field_vulnerability_details": "The ZK verifier circuit missing range check on intermediate witness polynomials, enabling a malicious prover to forge state transition proofs.",
        "form_field_proof_of_concept": "See reports/Work22_ZK_SmartContract_Audit_Package.json and circuits/state_proof.rs.",
        "payout_wallet": WALLET_ADDRESS
    }

    immunefi_payload = {
        "platform": "Immunefi",
        "form_field_title": "SP1 zkVM Precompile Division-by-Zero and Field Boundary Witness Bypass",
        "form_field_severity": "Critical",
        "form_field_bounty_program": "SP1 zkVM Precompile Soundness Program",
        "form_field_description": "Arithmetic precompiles lack strict zero-divisor checks and limb decomposition constraints.",
        "payout_wallet": WALLET_ADDRESS
    }

    print("\n[1] SHERLOCK SUBMISSION PAYLOAD:")
    print(json.dumps(sherlock_payload, indent=2))

    print("\n[2] CODE4RENA SUBMISSION PAYLOAD:")
    print(json.dumps(code4rena_payload, indent=2))

    print("\n[3] IMMUNEFI SUBMISSION PAYLOAD:")
    print(json.dumps(immunefi_payload, indent=2))

    print("\n" + "=" * 70)
    print(f"All form payloads successfully formatted and bound to wallet: {WALLET_ADDRESS}")
    print("=" * 70)

if __name__ == "__main__":
    render_submission_forms()
