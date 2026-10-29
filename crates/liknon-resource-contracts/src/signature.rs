use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signature, VerifyingKey};
#[cfg(feature = "signing")]
use ed25519_dalek::{Signer, SigningKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ContractError, SIGNATURE_SCHEMA_VERSION, TRUST_ROOT_SCHEMA_VERSION};

/// Signature algorithm supported by the version 1 trust contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureAlgorithm {
    /// Strict Ed25519 verification.
    Ed25519,
}

/// Lifecycle state of one trusted metadata-signing key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TrustKeyStatus {
    /// May sign current metadata.
    Active,
    /// May verify historical metadata signed during its validity interval.
    Retired,
    /// Must never authenticate metadata.
    Revoked,
}

/// One public key in a separately distributed trust root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustKey {
    /// Stable key identifier.
    pub key_id: String,
    /// Cryptographic algorithm.
    pub algorithm: SignatureAlgorithm,
    /// Standard Base64-encoded 32-byte Ed25519 public key.
    pub public_key: String,
    /// First authenticated Unix second when signatures are accepted.
    pub not_before_unix: u64,
    /// Last authenticated Unix second when signatures are accepted.
    pub not_after_unix: u64,
    /// Key lifecycle state.
    pub status: TrustKeyStatus,
}

/// Independently established Hub metadata trust root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustRoot {
    /// Exact trust-root schema version.
    pub schema_version: u32,
    /// Unique Hub authority name.
    pub authority: String,
    /// Trusted keys sorted by key ID.
    pub keys: Vec<TrustKey>,
}

/// Detached signature over exact metadata bytes and authenticated context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DetachedSignature {
    /// Exact detached-signature schema version.
    pub schema_version: u32,
    /// Trust-root key identifier.
    pub key_id: String,
    /// Cryptographic algorithm.
    pub algorithm: SignatureAlgorithm,
    /// Authenticated signing time used for key-lifecycle checks.
    pub signed_at_unix: u64,
    /// Standard Base64-encoded detached signature.
    pub signature: String,
}

/// Authenticates exact metadata bytes against a separately obtained trust root.
pub fn verify_detached(
    payload: &[u8],
    signature: &DetachedSignature,
    trust_root: &TrustRoot,
) -> Result<(), ContractError> {
    if signature.schema_version != SIGNATURE_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "detached signature",
            expected: SIGNATURE_SCHEMA_VERSION,
            actual: signature.schema_version,
        });
    }
    if trust_root.schema_version != TRUST_ROOT_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "trust root",
            expected: TRUST_ROOT_SCHEMA_VERSION,
            actual: trust_root.schema_version,
        });
    }
    let key = trust_root
        .keys
        .iter()
        .find(|key| key.key_id == signature.key_id)
        .ok_or_else(|| ContractError::UntrustedKey(signature.key_id.clone()))?;
    if key.algorithm != signature.algorithm
        || key.status == TrustKeyStatus::Revoked
        || signature.signed_at_unix < key.not_before_unix
        || signature.signed_at_unix > key.not_after_unix
    {
        return Err(ContractError::UntrustedKey(signature.key_id.clone()));
    }
    let public_bytes = decode_array::<32>(&key.public_key)?;
    let signature_bytes = decode_array::<64>(&signature.signature)?;
    let verifying_key = VerifyingKey::from_bytes(&public_bytes)
        .map_err(|error| ContractError::SignatureEncoding(error.to_string()))?;
    let signature_value = Signature::from_bytes(&signature_bytes);
    verifying_key
        .verify_strict(
            &signature_message(
                payload,
                &signature.key_id,
                signature.algorithm,
                signature.signed_at_unix,
            ),
            &signature_value,
        )
        .map_err(|_| ContractError::SignatureVerification(signature.key_id.clone()))
}

/// Signs exact metadata bytes with an Ed25519 private key.
#[cfg(feature = "signing")]
pub fn sign_detached(
    payload: &[u8],
    key_id: impl Into<String>,
    signed_at_unix: u64,
    private_key: &[u8; 32],
) -> DetachedSignature {
    let key_id = key_id.into();
    let algorithm = SignatureAlgorithm::Ed25519;
    let signing_key = SigningKey::from_bytes(private_key);
    let signature = signing_key.sign(&signature_message(
        payload,
        &key_id,
        algorithm,
        signed_at_unix,
    ));
    DetachedSignature {
        schema_version: SIGNATURE_SCHEMA_VERSION,
        key_id,
        algorithm,
        signed_at_unix,
        signature: STANDARD.encode(signature.to_bytes()),
    }
}

/// Derives the standard Base64 public key for one Ed25519 private key.
#[cfg(feature = "signing")]
#[must_use]
pub fn public_key_base64(private_key: &[u8; 32]) -> String {
    STANDARD.encode(
        SigningKey::from_bytes(private_key)
            .verifying_key()
            .to_bytes(),
    )
}

fn signature_message(
    payload: &[u8],
    key_id: &str,
    algorithm: SignatureAlgorithm,
    signed_at_unix: u64,
) -> Vec<u8> {
    let algorithm = match algorithm {
        SignatureAlgorithm::Ed25519 => "ed25519",
    };
    let mut message = format!(
        "liknon-metadata-signature-v1\n{key_id}\n{algorithm}\n{signed_at_unix}\n{}\n",
        payload.len()
    )
    .into_bytes();
    message.extend_from_slice(payload);
    message
}

fn decode_array<const N: usize>(value: &str) -> Result<[u8; N], ContractError> {
    let decoded = STANDARD
        .decode(value)
        .map_err(|error| ContractError::SignatureEncoding(error.to_string()))?;
    decoded.try_into().map_err(|bytes: Vec<u8>| {
        ContractError::SignatureEncoding(format!(
            "expected {N} decoded bytes, found {}",
            bytes.len()
        ))
    })
}

#[cfg(all(test, feature = "signing"))]
mod tests {
    use super::{public_key_base64, sign_detached, verify_detached, TrustKey, TrustRoot};
    use crate::{SignatureAlgorithm, TrustKeyStatus, TRUST_ROOT_SCHEMA_VERSION};

    #[test]
    fn signs_exact_bytes_and_rejects_mutation() {
        let private = [7u8; 32];
        let signature = sign_detached(b"catalog", "test-v1", 20, &private);
        let trust = TrustRoot {
            schema_version: TRUST_ROOT_SCHEMA_VERSION,
            authority: "fixture".to_owned(),
            keys: vec![TrustKey {
                key_id: "test-v1".to_owned(),
                algorithm: SignatureAlgorithm::Ed25519,
                public_key: public_key_base64(&private),
                not_before_unix: 10,
                not_after_unix: 30,
                status: TrustKeyStatus::Active,
            }],
        };
        verify_detached(b"catalog", &signature, &trust).unwrap();
        assert!(verify_detached(b"changed", &signature, &trust).is_err());
    }

    #[test]
    fn enforces_key_validity_retirement_and_revocation() {
        let private = [11u8; 32];
        let signature = sign_detached(b"catalog", "rotated-v1", 20, &private);
        let root = |status| TrustRoot {
            schema_version: TRUST_ROOT_SCHEMA_VERSION,
            authority: "fixture".to_owned(),
            keys: vec![TrustKey {
                key_id: "rotated-v1".to_owned(),
                algorithm: SignatureAlgorithm::Ed25519,
                public_key: public_key_base64(&private),
                not_before_unix: 10,
                not_after_unix: 30,
                status,
            }],
        };

        verify_detached(b"catalog", &signature, &root(TrustKeyStatus::Retired)).unwrap();
        assert!(verify_detached(b"catalog", &signature, &root(TrustKeyStatus::Revoked)).is_err());
        assert!(verify_detached(
            b"catalog",
            &sign_detached(b"catalog", "rotated-v1", 9, &private),
            &root(TrustKeyStatus::Active),
        )
        .is_err());
        assert!(verify_detached(
            b"catalog",
            &sign_detached(b"catalog", "rotated-v1", 31, &private),
            &root(TrustKeyStatus::Active),
        )
        .is_err());
    }
}
