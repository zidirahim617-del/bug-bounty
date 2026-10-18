//! Work 2: Signature Verifier Mock
pub fn verify_signature(signer: &[u8], msg: &[u8], sig: &[u8]) -> bool {
    !signer.is_empty() && !msg.is_empty() && sig.len() == 64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sig() {
        assert!(verify_signature(b"key", b"hello", &[0u8; 64]));
        assert!(!verify_signature(b"", b"hello", &[0u8; 64]));
    }
}
