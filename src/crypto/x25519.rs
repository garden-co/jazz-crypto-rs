use crate::error::CryptoError;
use bs58;
use wasm_bindgen::prelude::*;
use x25519_dalek::{PublicKey, StaticSecret};

/// Generate a new X25519 private key that can be reused
#[wasm_bindgen]
pub fn new_x25519_private_key() -> Vec<u8> {
    let secret = StaticSecret::random();
    secret.to_bytes().to_vec()
}

/// Internal pure Rust functions (no wasm_bindgen)
pub(crate) fn x25519_public_key_internal(private_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let bytes: [u8; 32] = private_key
        .try_into()
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    let secret = StaticSecret::from(bytes);
    Ok(PublicKey::from(&secret).to_bytes().to_vec())
}

#[wasm_bindgen]
pub fn x25519_public_key(private_key: &[u8]) -> Result<Vec<u8>, JsError> {
    x25519_public_key_internal(private_key).map_err(|e| JsError::new(&e.to_string()))
}

pub(crate) fn x25519_diffie_hellman_internal(
    private_key: &[u8],
    public_key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let private_bytes: [u8; 32] = private_key
        .try_into()
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    let public_bytes: [u8; 32] = public_key
        .try_into()
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    let secret = StaticSecret::from(private_bytes);
    let public = PublicKey::from(public_bytes);
    Ok(secret.diffie_hellman(&public).to_bytes().to_vec())
}

#[wasm_bindgen]
pub fn x25519_diffie_hellman(private_key: &[u8], public_key: &[u8]) -> Result<Vec<u8>, JsError> {
    x25519_diffie_hellman_internal(private_key, public_key)
        .map_err(|e| JsError::new(&e.to_string()))
}

pub(crate) fn get_sealer_id_internal(secret: &str) -> Result<String, String> {
    if !secret.starts_with("sealerSecret_z") {
        return Err("Invalid sealer secret format: must start with 'sealerSecret_z'".to_string());
    }

    let private_bytes = bs58::decode(&secret["sealerSecret_z".len()..])
        .into_vec()
        .map_err(|e| format!("Invalid base58 in secret: {:?}", e))?;

    let public_bytes = x25519_public_key_internal(&private_bytes)
        .map_err(|e| format!("Failed to get public key: {:?}", e))?;

    Ok(format!(
        "sealer_z{}",
        bs58::encode(public_bytes).into_string()
    ))
}

#[wasm_bindgen]
pub fn get_sealer_id(secret: &[u8]) -> Result<String, JsError> {
    let secret_str = std::str::from_utf8(secret)
        .map_err(|e| JsError::new(&format!("Invalid UTF-8 in secret: {:?}", e)))?;
    get_sealer_id_internal(secret_str).map_err(|e| JsError::new(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x25519_key_generation() {
        // Test that we get the correct length keys
        let private_key = new_x25519_private_key();
        assert_eq!(private_key.len(), 32);

        // Test that public key generation works and produces correct length
        let public_key = x25519_public_key_internal(&private_key).unwrap();
        assert_eq!(public_key.len(), 32);

        // Test that different private keys produce different public keys
        let private_key2 = new_x25519_private_key();
        let public_key2 = x25519_public_key_internal(&private_key2).unwrap();
        assert_ne!(public_key, public_key2);
    }

    #[test]
    fn test_x25519_key_exchange() {
        // Generate sender's keypair
        let sender_private = new_x25519_private_key();
        let sender_public = x25519_public_key_internal(&sender_private).unwrap();

        // Generate recipient's keypair
        let recipient_private = new_x25519_private_key();
        let recipient_public = x25519_public_key_internal(&recipient_private).unwrap();

        // Test properties we expect from the shared secret
        let shared_secret1 =
            x25519_diffie_hellman_internal(&sender_private, &recipient_public).unwrap();
        let shared_secret2 =
            x25519_diffie_hellman_internal(&recipient_private, &sender_public).unwrap();

        // Both sides should arrive at the same shared secret
        assert_eq!(shared_secret1, shared_secret2);

        // Shared secret should be 32 bytes
        assert_eq!(shared_secret1.len(), 32);

        // Different recipient should produce different shared secret
        let other_recipient_private = new_x25519_private_key();
        let other_recipient_public = x25519_public_key_internal(&other_recipient_private).unwrap();
        let different_shared_secret =
            x25519_diffie_hellman_internal(&sender_private, &other_recipient_public).unwrap();
        assert_ne!(shared_secret1, different_shared_secret);
    }

    #[test]
    fn test_get_sealer_id() {
        // Create a test private key
        let private_key = new_x25519_private_key();
        let secret = format!("sealerSecret_z{}", bs58::encode(&private_key).into_string());

        // Get sealer ID
        let sealer_id = get_sealer_id_internal(&secret).unwrap();
        assert!(sealer_id.starts_with("sealer_z"));

        // Test that same secret produces same ID
        let sealer_id2 = get_sealer_id_internal(&secret).unwrap();
        assert_eq!(sealer_id, sealer_id2);

        // Test invalid secret format
        let result = get_sealer_id_internal("invalid_secret");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid sealer secret format"));

        // Test invalid base58
        let result = get_sealer_id_internal("sealerSecret_z!!!invalid!!!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base58 in secret"));
    }
}
