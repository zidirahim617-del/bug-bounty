#!/usr/bin/env python3
"""
Fast-Payout Platforms Navigator & Immunefi Focus Tool
Identifies and prepares submission routing for top Web3 security platforms
known for rapid triage and fast payouts (3-7 days), including Immunefi,
Cantina, CodeHawks, and Algora/Polar.
"""

import json
import os
from datetime import datetime

class FastPayoutNavigator:
    def __init__(self):
        self.wallet = "0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c"
        self.platforms = [
            {
                "platform": "Immunefi",
                "focus": "Active Bug Bounties (Smart Contracts, Bridges, ZK)",
                "payout_speed": "3 - 7 Days (Upon Triaging & Confirmation)",
                "reward_range": "$1,000 - $100,000+",
                "url": "https://immunefi.com/"
            },
            {
                "platform": "Cantina (Competitions & Private Audits)",
                "focus": "High-Value Competitive Audits & Direct Engagements",
                "payout_speed": "5 - 10 Days After Judging",
                "reward_range": "$2,500 - $30,000",
                "url": "https://cantina.xyz/"
            },
            {
                "platform": "CodeHawks (Cyfrin)",
                "focus": "Smart Contract Audit Contests with Fast Judging",
                "payout_speed": "7 Days After Contest Close",
                "reward_range": "$1,000 - $20,000",
                "url": "https://codehawks.cyfrin.io/"
            },
            {
                "platform": "Algora & Polar (Quick Payout Bounties)",
                "focus": "GitHub Issues, Rust/Solidity Patches & Bug Fixes",
                "payout_speed": "4 - 6 Days (Upon PR Merge)",
                "reward_range": "$150 - $1,500",
                "url": "https://algora.io/"
            }
        ]

    def generate_navigation_report(self):
        print("[*] Generating Fast-Payout Platforms Navigation Report...")
        
        report = {
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "target_wallet": self.wallet,
            "recommended_platforms": self.platforms,
            "strategy": "Pivot focus entirely to Immunefi bug bounties, Cantina competitions, CodeHawks contests, and Algora quick-payout GitHub issues for rapid 3-7 day USDC liquidity.",
            "status": "READY_FOR_DEPLOYMENT"
        }

        reports_dir = os.path.join(os.path.dirname(__file__), "..", "reports")
        os.makedirs(reports_dir, exist_ok=True)
        out_path = os.path.join(reports_dir, "Fast_Payout_Platforms_Report.json")

        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2)

        print(f" [+] Fast-Payout Navigation Report saved to: {out_path}")
        return report

if __name__ == "__main__":
    print("=" * 70)
    print("FAST-PAYOUT PLATFORMS NAVIGATOR & IMMUNEFI FOCUS")
    print("=" * 70)
    nav = FastPayoutNavigator()
    res = nav.generate_navigation_report()
    print(json.dumps(res, indent=2))
    print("=" * 70)
