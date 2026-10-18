//! Reentrancy Vulnerability Static Auditor
//! Target: Sherlock & Code4rena Smart Contract Audit Contests

pub struct ReentrancyAuditor;

impl ReentrancyAuditor {
    pub fn scan_contract_flow(lines: &[&str]) -> Vec<String> {
        let mut violations = Vec::new();
        let mut external_call_seen = false;
        
        for (idx, line) in lines.iter().enumerate() {
            if line.contains(".call{value:") || line.contains(".transfer(") || line.contains(".send(") {
                external_call_seen = true;
            }
            if external_call_seen && (line.contains("balances[msg.sender] =") || line.contains("state =")) {
                violations.push(format!("Line {} | Risk: State update after external call (Reentrancy violation)", idx + 1));
            }
        }
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reentrancy_detection() {
        let code = vec![
            "function withdraw() external {",
            "  (bool success, ) = msg.sender.call{value: amount}(\"\");",
            "  balances[msg.sender] = 0;", // State update after call!
            "}"
        ];
        let issues = ReentrancyAuditor::scan_contract_flow(&code);
        assert_eq!(issues.len(), 1);
    }
}
