# ISSUE_070 — Stellarcade Wave 11 Batch Preparation (Work 9)

**Date executed:** 2026-10-22
**Platform / target:** Stellarcade Protocol (`theblockcade/stellarcade`)
**Track:** Frontend A11y, Security Scanning, Contracts, Testnet Integration

## Batch Summary (10 PRs Prepared)
1. `fix(a11y)`: Keyboard and screen-reader support for coin-flip play panel (#1455).
2. `fix(indexer)`: Recover when saved cursor is older than RPC event retention (#1435).
3. `feat(dice-roll)`: Reject rolls the house cannot cover; fund_house tracking (#1443).
4. `ci(integration)`: Test oracle and indexer against local Soroban network (#1441).
5. `ci(contracts)`: Report WASM sizes and fail before Soroban upload limit (#1456).
6. `chore`: Add Dependabot for npm, Cargo, and GitHub Actions (#1438).
7. `test(coin-flip)`: Property-based tests for payout and exposure invariants (#1440).
8. `feat(security)`: Scan pull requests for committed secrets with gitleaks (#1439).
9. `test(smoke)`: Add dice-roll round to testnet smoke test (#1451).
10. `fix(ci)`: Deduplicate testnet smoke workflow steps and auto-issue on failure (#1432).

## Status
- All 10 PRs prepared, tested against local Soroban environment, and queued for Wave 11 launch.
