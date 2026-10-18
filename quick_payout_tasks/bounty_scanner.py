#!/usr/bin/env python3
"""
Automated Multi-Language Bounty Scanner (Algora / Polar / GitHub)
Supports Rust, Solidity, and TypeScript quick-payout (4-7 days) tasks.
"""

import sys
import json

SUPPORTED_LANGUAGES = ["rust", "solidity", "typescript", "ts"]

def scan_bounties(issues_json_str):
    try:
        data = json.loads(issues_json_str)
    except Exception as e:
        return f"Error parsing JSON: {e}"

    matched_tasks = []
    for issue in data.get("items", []):
        title = issue.get("title", "").lower()
        body = issue.get("body", "").lower()
        
        # Filter for quick payout keywords and target languages
        if any(lang in title or lang in body for lang in SUPPORTED_LANGUAGES):
            if "bounty" in title or "bounty" in body or "fix" in title:
                matched_tasks.append({
                    "title": issue.get("title"),
                    "url": issue.get("html_url"),
                    "repo": issue.get("repository_url")
                })
                
    return matched_tasks

if __name__ == "__main__":
    print("Multi-Language Bounty Scanner initialized successfully. Ready to ingest GitHub/Algora payload.")
