//! QP Task 13: Whitelist Check
pub fn is_whitelisted(addr: &str, whitelist: &[&str]) -> bool {
    whitelist.contains(&addr)
}
