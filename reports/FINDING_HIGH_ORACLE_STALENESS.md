# [H-01] Missing Chainlink Staleness and L2 Sequencer Uptime Check Leads to Bad Debt

## Summary
A high severity vulnerability was identified in `DeFi-Protocol-V2` under the **Oracle Security** category.

## Vulnerability Detail
The oracle integration queries latestRoundData() without verifying updatedDataTimestamp or checking the L2 sequencer uptime feed.

## Impact
Protocol relies on outdated or stale oracle prices during L2 sequencer downtime or flash crashes, allowing undercollateralized loans.

## Proof of Concept
```solidity
(uint80 roundId, int256 price,, uint256 updatedAt, uint80 answeredInRound) = priceFeed.latestRoundData();
require(price > 0, "Invalid price");
// Missing: require(block.timestamp - updatedAt <= TIMEOUT, "Stale price");
// Missing: require(answeredInRound >= roundId, "Stale round");
```

## Recommended Mitigation
Add strict timestamp staleness checks and integrate the Chainlink L2 Sequencer Uptime Feed.
