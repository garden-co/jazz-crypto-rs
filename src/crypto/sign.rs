use crate::crypto::ed25519::{
    ed25519_sign_internal, ed25519_verify_internal, ed25519_verifying_key_internal,
};
use bs58;
use wasm_bindgen::prelude::*;

/// Internal function to sign a message using Ed25519.
/// - `message`: Raw bytes to sign
/// - `secret`: Base58-encoded signing key with "signerSecret_z" prefix
/// Returns base58-encoded signature with "signature_z" prefix or error string.
pub(crate) fn sign_internal(message: &[u8], secret: &str) -> Result<String, String> {
    let secret_bytes = bs58::decode(&secret["signerSecret_z".len()..])
        .into_vec()
        .map_err(|e| format!("Invalid base58 in secret: {:?}", e))?;

    let signature = ed25519_sign_internal(&secret_bytes, message)
        .map_err(|e| format!("Signing failed: {:?}", e))?;
    Ok(format!(
        "signature_z{}",
        bs58::encode(signature).into_string()
    ))
}

/// Internal function to verify an Ed25519 signature.
/// - `signature`: Base58-encoded signature with "signature_z" prefix
/// - `message`: Raw bytes that were signed
/// - `id`: Base58-encoded verifying key with "signer_z" prefix
/// Returns true if signature is valid, false otherwise, or error string if formats are invalid.
pub(crate) fn verify_internal(signature: &str, message: &[u8], id: &str) -> Result<bool, String> {
    if !signature.starts_with("signature_z") {
        return Err("Invalid signature format: must start with 'signature_z'".to_string());
    }
    if !id.starts_with("signer_z") {
        return Err("Invalid signer ID format: must start with 'signer_z'".to_string());
    }

    let verifying_key = bs58::decode(&id["signer_z".len()..])
        .into_vec()
        .map_err(|e| format!("Invalid base58 in id: {:?}", e))?;

    let signature_bytes = bs58::decode(&signature["signature_z".len()..])
        .into_vec()
        .map_err(|e| format!("Invalid base58 in signature: {:?}", e))?;

    ed25519_verify_internal(&verifying_key, message, &signature_bytes)
        .map_err(|e| format!("Verification failed: {:?}", e))
}

/// Internal function to derive a signer ID from a signing key.
/// - `secret`: Base58-encoded signing key with "signerSecret_z" prefix
/// Returns base58-encoded verifying key with "signer_z" prefix or error string.
pub(crate) fn get_signer_id_internal(secret: &str) -> Result<String, String> {
    if !secret.starts_with("signerSecret_z") {
        return Err("Invalid signer secret format: must start with 'signerSecret_z'".to_string());
    }

    let secret_bytes = bs58::decode(&secret["signerSecret_z".len()..])
        .into_vec()
        .map_err(|e| format!("Invalid base58 in secret: {:?}", e))?;

    let verifying_key = ed25519_verifying_key_internal(&secret_bytes)
        .map_err(|e| format!("Failed to get verifying key: {:?}", e))?;

    Ok(format!(
        "signer_z{}",
        bs58::encode(verifying_key).into_string()
    ))
}

/// WASM-exposed function to sign a message using Ed25519.
/// - `message`: Raw bytes to sign
/// - `secret`: Raw Ed25519 signing key bytes
/// Returns base58-encoded signature with "signature_z" prefix or throws JsError if signing fails.
#[wasm_bindgen(js_name = sign)]
pub fn sign(message: &[u8], secret: &[u8]) -> Result<String, JsError> {
    let secret_str = std::str::from_utf8(secret)
        .map_err(|e| JsError::new(&format!("Invalid UTF-8 in secret: {:?}", e)))?;
    sign_internal(message, secret_str).map_err(|e| JsError::new(&e))
}

/// WASM-exposed function to verify an Ed25519 signature.
/// - `signature`: Raw signature bytes
/// - `message`: Raw bytes that were signed
/// - `id`: Raw Ed25519 verifying key bytes
/// Returns true if signature is valid, false otherwise, or throws JsError if verification fails.
#[wasm_bindgen(js_name = verify)]
pub fn verify(signature: &[u8], message: &[u8], id: &[u8]) -> Result<bool, JsError> {
    let signature_str = std::str::from_utf8(signature)
        .map_err(|e| JsError::new(&format!("Invalid UTF-8 in signature: {:?}", e)))?;
    let id_str = std::str::from_utf8(id)
        .map_err(|e| JsError::new(&format!("Invalid UTF-8 in id: {:?}", e)))?;
    verify_internal(signature_str, message, id_str).map_err(|e| JsError::new(&e))
}

/// WASM-exposed function to derive a signer ID from a signing key.
/// - `secret`: Raw Ed25519 signing key bytes
/// Returns base58-encoded verifying key with "signer_z" prefix or throws JsError if derivation fails.
#[wasm_bindgen(js_name = get_signer_id)]
pub fn get_signer_id(secret: &[u8]) -> Result<String, JsError> {
    let secret_str = std::str::from_utf8(secret)
        .map_err(|e| JsError::new(&format!("Invalid UTF-8 in secret: {:?}", e)))?;
    get_signer_id_internal(secret_str).map_err(|e| JsError::new(&e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::new_ed25519_signing_key;

    #[test]
    fn test_sign_and_verify() {
        let message = b"hello world";

        // Create a test signing key
        let signing_key = new_ed25519_signing_key();
        let secret = format!("signerSecret_z{}", bs58::encode(&signing_key).into_string());

        // Sign the message
        let signature = sign_internal(message, &secret).unwrap();

        // Get the public key for verification
        let secret_bytes = bs58::decode(&secret["signerSecret_z".len()..])
            .into_vec()
            .unwrap();
        let verifying_key = ed25519_verifying_key_internal(&secret_bytes).unwrap();
        let signer_id = format!("signer_z{}", bs58::encode(&verifying_key).into_string());

        // Verify the signature
        assert!(verify_internal(&signature, message, &signer_id).unwrap());
    }

    #[test]
    fn test_invalid_inputs() {
        let message = b"hello world";

        // Test invalid base58 in secret
        let result = sign_internal(message, "signerSecret_z!!!invalid!!!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base58 in secret"));

        // Test invalid signature format
        let result = verify_internal("not_a_signature", message, "signer_z123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid signature format"));

        // Test invalid signer ID format
        let result = verify_internal("signature_z123", message, "not_a_signer");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid signer ID format"));
    }

    #[test]
    fn test_get_signer_id() {
        // Create a test signing key
        let signing_key = new_ed25519_signing_key();
        let secret = format!("signerSecret_z{}", bs58::encode(&signing_key).into_string());

        // Get signer ID
        let signer_id = get_signer_id_internal(&secret).unwrap();
        assert!(signer_id.starts_with("signer_z"));

        // Test that same secret produces same ID
        let signer_id2 = get_signer_id_internal(&secret).unwrap();
        assert_eq!(signer_id, signer_id2);

        // Test invalid secret format
        let result = get_signer_id_internal("invalid_secret");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid signer secret format"));

        // Test invalid base58
        let result = get_signer_id_internal("signerSecret_z!!!invalid!!!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base58 in secret"));
    }
}
