use crate::crypto::x25519::x25519_diffie_hellman_internal;
use crate::crypto::xsalsa20::{decrypt_xsalsa20_poly1305, encrypt_xsalsa20_poly1305};
use crate::error::CryptoError;
use crate::hash::blake3::generate_nonce;
use bs58;
use wasm_bindgen::prelude::*;

/// Internal function to seal a message using X25519 + XSalsa20-Poly1305.
/// - `message`: Raw bytes to seal
/// - `sender_secret`: Base58-encoded sender's private key with "sealerSecret_z" prefix
/// - `recipient_id`: Base58-encoded recipient's public key with "sealer_z" prefix
/// - `nonce_material`: Raw bytes used to generate the nonce
/// Returns sealed bytes or CryptoError if key formats are invalid.
///
/// The sealing process:
/// 1. Decode base58 keys and validate prefixes
/// 2. Generate shared secret using X25519 key exchange
/// 3. Generate nonce from nonce material using BLAKE3
/// 4. Encrypt message using XSalsa20-Poly1305 with the shared secret
pub fn seal_internal(
    message: &[u8],
    sender_secret: &str,
    recipient_id: &str,
    nonce_material: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // Decode the base58 sender secret (removing the "sealerSecret_z" prefix)
    let sender_secret =
        sender_secret
            .strip_prefix("sealerSecret_z")
            .ok_or(CryptoError::InvalidPrefix(
                "sealer secret",
                "sealerSecret_z",
            ))?;
    let sender_private_key = bs58::decode(sender_secret)
        .into_vec()
        .map_err(|e| CryptoError::Base58Error(e.to_string()))?;

    // Decode the base58 recipient ID (removing the "sealer_z" prefix)
    let recipient_id = recipient_id
        .strip_prefix("sealer_z")
        .ok_or(CryptoError::InvalidPrefix("sealer ID", "sealer_z"))?;
    let recipient_public_key = bs58::decode(recipient_id)
        .into_vec()
        .map_err(|e| CryptoError::Base58Error(e.to_string()))?;

    let nonce = generate_nonce(nonce_material);

    // Generate shared secret using X25519
    let shared_secret = x25519_diffie_hellman_internal(&sender_private_key, &recipient_public_key)?;

    // Encrypt message using XSalsa20-Poly1305
    Ok(encrypt_xsalsa20_poly1305(&shared_secret, &nonce, message)?.into())
}

/// Internal function to unseal a message using X25519 + XSalsa20-Poly1305.
/// - `sealed_message`: The sealed bytes to decrypt
/// - `recipient_secret`: Base58-encoded recipient's private key with "sealerSecret_z" prefix
/// - `sender_id`: Base58-encoded sender's public key with "sealer_z" prefix
/// - `nonce_material`: Raw bytes used to generate the nonce (must match sealing)
/// Returns unsealed bytes or CryptoError if key formats are invalid or authentication fails.
///
/// The unsealing process:
/// 1. Decode base58 keys and validate prefixes
/// 2. Generate shared secret using X25519 key exchange
/// 3. Generate nonce from nonce material using BLAKE3
/// 4. Decrypt and authenticate message using XSalsa20-Poly1305 with the shared secret
fn unseal_internal(
    sealed_message: &[u8],
    recipient_secret: &str,
    sender_id: &str,
    nonce_material: &[u8],
) -> Result<Box<[u8]>, CryptoError> {
    // Decode the base58 recipient secret (removing the "sealerSecret_z" prefix)
    let recipient_secret =
        recipient_secret
            .strip_prefix("sealerSecret_z")
            .ok_or(CryptoError::InvalidPrefix(
                "sealer secret",
                "sealerSecret_z",
            ))?;
    let recipient_private_key = bs58::decode(recipient_secret)
        .into_vec()
        .map_err(|e| CryptoError::Base58Error(e.to_string()))?;

    // Decode the base58 sender ID (removing the "sealer_z" prefix)
    let sender_id = sender_id
        .strip_prefix("sealer_z")
        .ok_or(CryptoError::InvalidPrefix("sealer ID", "sealer_z"))?;
    let sender_public_key = bs58::decode(sender_id)
        .into_vec()
        .map_err(|e| CryptoError::Base58Error(e.to_string()))?;

    let nonce = generate_nonce(nonce_material);

    // Generate shared secret using X25519
    let shared_secret = x25519_diffie_hellman_internal(&recipient_private_key, &sender_public_key)?;

    // Decrypt message using XSalsa20-Poly1305
    Ok(decrypt_xsalsa20_poly1305(&shared_secret, &nonce, sealed_message)?.into())
}

/// WASM-exposed function for sealing a message using X25519 + XSalsa20-Poly1305.
/// Provides authenticated encryption with perfect forward secrecy.
/// - `message`: Raw bytes to seal
/// - `sender_secret`: Base58-encoded sender's private key with "sealerSecret_z" prefix
/// - `recipient_id`: Base58-encoded recipient's public key with "sealer_z" prefix
/// - `nonce_material`: Raw bytes used to generate the nonce
/// Returns sealed bytes or throws JsError if sealing fails.
#[wasm_bindgen(js_name = seal)]
pub fn seal(
    message: &[u8],
    sender_secret: &str,
    recipient_id: &str,
    nonce_material: &[u8],
) -> Result<Box<[u8]>, JsError> {
    Ok(seal_internal(message, sender_secret, recipient_id, nonce_material)?.into())
}

/// WASM-exposed function for unsealing a message using X25519 + XSalsa20-Poly1305.
/// Provides authenticated decryption with perfect forward secrecy.
/// - `sealed_message`: The sealed bytes to decrypt
/// - `recipient_secret`: Base58-encoded recipient's private key with "sealerSecret_z" prefix
/// - `sender_id`: Base58-encoded sender's public key with "sealer_z" prefix
/// - `nonce_material`: Raw bytes used to generate the nonce (must match sealing)
/// Returns unsealed bytes or throws JsError if unsealing fails.
#[wasm_bindgen(js_name = unseal)]
pub fn unseal(
    sealed_message: &[u8],
    recipient_secret: &str,
    sender_id: &str,
    nonce_material: &[u8],
) -> Result<Box<[u8]>, JsError> {
    Ok(unseal_internal(sealed_message, recipient_secret, sender_id, nonce_material)?.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::x25519::{new_x25519_private_key, x25519_public_key_internal};

    #[test]
    fn test_seal_unseal() {
        // Generate real keys
        let sender_private = new_x25519_private_key();
        let sender_public = x25519_public_key_internal(&sender_private).unwrap();

        // Encode keys with proper prefixes
        let sender_secret = format!(
            "sealerSecret_z{}",
            bs58::encode(&sender_private).into_string()
        );
        let recipient_id = format!("sealer_z{}", bs58::encode(&sender_public).into_string());

        // Test data
        let message = b"Secret message";
        let nonce_material = b"test_nonce_material";

        // Test sealing
        let sealed = seal_internal(message, &sender_secret, &recipient_id, nonce_material).unwrap();
        assert!(!sealed.is_empty());

        // Test unsealing (using same keys since it's a test)
        let unsealed =
            unseal_internal(&sealed, &sender_secret, &recipient_id, nonce_material).unwrap();
        assert_eq!(&*unsealed, message);
    }

    #[test]
    fn test_invalid_keys() {
        let message = b"test";
        let nonce_material = b"nonce";

        // Test with invalid sender secret format
        let result = seal_internal(
            message,
            "invalid_key",
            "sealer_z22222222222222222222222222222222",
            nonce_material,
        );
        assert!(result.is_err());

        // Test with invalid recipient ID format
        let result = seal_internal(
            message,
            "sealerSecret_z11111111111111111111111111111111",
            "invalid_key",
            nonce_material,
        );
        assert!(result.is_err());

        // Test with invalid base58 encoding
        let result = seal_internal(
            message,
            "sealerSecret_z!!!!",
            "sealer_z22222222222222222222222222222222",
            nonce_material,
        );
        assert!(result.is_err());
    }
}
