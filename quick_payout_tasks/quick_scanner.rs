//! Algora & Polar Quick-Payout Rust Bug Fix Scanner
//! Scans Rust code for unhandled Result/Option unwraps and panic vectors
//! to quickly fix issues in bounty repositories.

use std::fs;
use walkdir::WalkDir;

pub fn scan_quick_bounty_risks(dir: &str) -> Vec<String> {
    let mut vulnerabilities = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "rs") {
            if let Ok(content) = fs::read_to_string(path) {
                for (idx, line) in content.lines().enumerate() {
                    if line.contains(".unwrap()") || line.contains(".expect(") {
                        vulnerabilities.push(format!(
                            "File: {} | Line: {} | Risk: Potential Panic via unwrap/expect",
                            path.display(),
                            idx + 1
                        ));
                    }
                }
            }
        }
    }
    vulnerabilities
}
