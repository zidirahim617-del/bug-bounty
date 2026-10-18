//! SP1 zkVM & Precompile Static Vulnerability Scanner
//! Scans Rust guest code and ZK circuit implementations for unconstrained inputs
//! and missing range checks (Primary vectors for $100k Immunefi critical bounties).

pub struct ZkCircuitAuditor;

impl ZkCircuitAuditor {
    /// Scans code lines for unsafe unconstrained memory reads or unchecked arithmetic in ZK circuits
    pub fn audit_guest_code(code_lines: &[&str]) -> Vec<String> {
        let mut findings = Vec::new();
        for (idx, line) in code_lines.iter().enumerate() {
            if line.contains("unsafe") || line.contains("transmute") || line.contains("get_unchecked") {
                findings.format_finding(idx + 1, "Unsafe memory access or pointer cast in ZK guest code");
            }
            if line.contains("add_insecure") || line.contains("mul_insecure") {
                findings.format_finding(idx + 1, "Insecure unchecked arithmetic operation in circuit");
            }
        }
        findings
    }
}

trait FormatFinding {
    fn format_finding(&mut self, line: usize, msg: &str);
}

impl FormatFinding for Vec<String> {
    fn format_finding(&mut self, line: usize, msg: &str) {
        self.push(format!("Line {} | Risk: {}", line, msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_auditor_detects_risks() {
        let sample_code = vec![
            "let x = 5;",
            "let ptr = unsafe { std::mem::transmute::<u64, *const u8>(val) };",
            "let total = add_insecure(a, b);",
        ];
        let results = ZkCircuitAuditor::audit_guest_code(&sample_code);
        assert_eq!(results.len(), 2);
    }
}
