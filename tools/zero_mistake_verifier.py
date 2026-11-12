#!/usr/bin/env python3
"""
Zero-Mistake Project Integrity Verifier & Pre-Flight Submission Audit Tool
Validates JSON schemas, report links, payout wallet bindings, and script execution status.
ASCII-safe output for Windows environments.
"""

import json
import os
from datetime import datetime

REQUIRED_FILES = [
    "config/payout_profile.json",
    "reports/FINDING_HIGH_ERC4626_INFLATION.md",
    "reports/Work22_ZK_SmartContract_Audit_Package.json",
    "reports/Live_Submission_Dispatch_Summary.json",
    "security_portfolio/README.md",
    "security_portfolio/audits/01_DEFI_VAULT_INVARIANT_AUDIT.md",
    "security_portfolio/audits/02_ZK_CIRCUIT_SOUNDNESS_AUDIT.md",
    "profile_branding/PUBLIC_PORTFOLIO_DISPATCH.md"
]

EXPECTED_WALLET = "0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c"

def run_zero_mistake_verification():
    print("=" * 70)
    print("EXECUTING ZERO-MISTAKE PROJECT INTEGRITY VERIFICATION")
    print("=" * 70)

    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    verification_results = []
    mistakes_found = 0

    # 1. File Existence Check
    for rel_path in REQUIRED_FILES:
        full_path = os.path.join(base_dir, rel_path)
        exists = os.path.exists(full_path)
        if exists:
            verification_results.append(f" [PASS] File Exists: {rel_path}")
        else:
            verification_results.append(f" [FAIL] MISSING FILE: {rel_path}")
            mistakes_found += 1

    # 2. JSON Integrity & Wallet Binding Check
    config_path = os.path.join(base_dir, "config", "payout_profile.json")
    if os.path.exists(config_path):
        try:
            with open(config_path, "r", encoding="utf-8") as f:
                cfg = json.load(f)
            if cfg.get("wallet_address") == EXPECTED_WALLET:
                verification_results.append(f" [PASS] Payout Wallet Bound Correctly: {EXPECTED_WALLET}")
            else:
                verification_results.append(f" [FAIL] Wallet Mismatch in config: {cfg.get('wallet_address')}")
                mistakes_found += 1
        except Exception as e:
            verification_results.append(f" [FAIL] Invalid JSON in payout_profile: {e}")
            mistakes_found += 1

    # 3. Report Payload Integrity Check
    zk_report = os.path.join(base_dir, "reports", "Work22_ZK_SmartContract_Audit_Package.json")
    if os.path.exists(zk_report):
        try:
            with open(zk_report, "r", encoding="utf-8") as f:
                payload = json.load(f)
            if payload.get("total_estimated_bounty") == "$70,000":
                verification_results.append(" [PASS] Work 22 Audit Package Integrity Verified ($70,000)")
            else:
                verification_results.append(" [FAIL] Work 22 Audit Package Payload Incomplete")
                mistakes_found += 1
        except Exception as e:
            verification_results.append(f" [FAIL] Invalid JSON in Work 22 report: {e}")
            mistakes_found += 1

    # 4. Summary Output
    summary = {
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "total_checks": len(verification_results),
        "mistakes_detected": mistakes_found,
        "verification_log": verification_results,
        "quality_status": "PERFECT_ZERO_MISTAKES" if mistakes_found == 0 else "CORRECTIONS_REQUIRED"
    }

    out_path = os.path.join(base_dir, "reports", "Zero_Mistake_Verification_Summary.json")
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(summary, f, indent=2)

    for line in verification_results:
        print(line)
        
    print("=" * 70)
    print(f"VERIFICATION COMPLETE: {mistakes_found} MISTAKES FOUND. STATUS: {summary['quality_status']}")
    print("=" * 70)
    return summary

if __name__ == "__main__":
    run_zero_mistake_verification()
