use sha2::{Digest, Sha256};

use crate::ContractError;

/// Computes a lower-case hexadecimal SHA-256 digest.
#[must_use]
pub fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(output, "{byte:02x}");
    }
    output
}

/// Serializes a contract with its declared field order and hashes those bytes.
///
/// Contract structures avoid unordered maps so this representation is stable
/// for equivalent typed values.
pub fn canonical_json_sha256<T: serde::Serialize>(value: &T) -> Result<String, ContractError> {
    Ok(sha256_bytes(&serde_json::to_vec(value)?))
}

/// Validates an exact lower-case hexadecimal SHA-256 digest.
pub fn validate_sha256(value: &str) -> Result<(), ContractError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(ContractError::InvalidDigest(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{sha256_bytes, validate_sha256};

    #[test]
    fn hashes_and_validates_exact_lowercase_sha256() {
        let digest = sha256_bytes(b"liknon");
        assert_eq!(digest.len(), 64);
        validate_sha256(&digest).unwrap();
        assert!(validate_sha256(&digest.to_uppercase()).is_err());
    }
}
