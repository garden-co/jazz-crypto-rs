use crate::error::CryptoError;
use crate::hash::blake3::generate_nonce;
use bs58;
use wasm_bindgen::prelude::*;

/// Internal function to encrypt bytes with a key secret and nonce material
fn encrypt_internal(
    plaintext: &[u8],
    key_secret: &str,
    nonce_material: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // Decode the base58 key secret (removing the "keySecret_z" prefix)
    let key_secret = key_secret
        .strip_prefix("keySecret_z")
        .ok_or(CryptoError::InvalidKeyLength)?;
    let key = bs58::decode(key_secret)
        .into_vec()
        .map_err(|_| CryptoError::InvalidKeyLength)?;

    // Generate nonce from nonce material
    let nonce = generate_nonce(nonce_material);

    // Encrypt using XSalsa20
    super::xsalsa20::encrypt_xsalsa20_raw_internal(&key, &nonce, plaintext)
}

/// Internal function to decrypt bytes with a key secret and nonce material
fn decrypt_internal(
    ciphertext: &[u8],
    key_secret: &str,
    nonce_material: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // Decode the base58 key secret (removing the "keySecret_z" prefix)
    let key_secret = key_secret
        .strip_prefix("keySecret_z")
        .ok_or(CryptoError::InvalidKeyLength)?;
    let key = bs58::decode(key_secret)
        .into_vec()
        .map_err(|_| CryptoError::InvalidKeyLength)?;

    // Generate nonce from nonce material
    let nonce = generate_nonce(nonce_material);

    // Decrypt using XSalsa20
    super::xsalsa20::decrypt_xsalsa20_raw_internal(&key, &nonce, ciphertext)
}

/// Encrypt bytes with a key secret and nonce material
#[wasm_bindgen(js_name = encrypt)]
pub fn encrypt(value: &[u8], key_secret: &str, nonce_material: &[u8]) -> Result<Vec<u8>, JsError> {
    encrypt_internal(value, key_secret, nonce_material).map_err(|e| JsError::new(&e.to_string()))
}

/// Decrypt bytes with a key secret and nonce material
#[wasm_bindgen(js_name = decrypt)]
pub fn decrypt(
    ciphertext: &[u8],
    key_secret: &str,
    nonce_material: &[u8],
) -> Result<Vec<u8>, JsError> {
    decrypt_internal(ciphertext, key_secret, nonce_material)
        .map_err(|e| JsError::new(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        // Test data
        let plaintext = b"Hello, World!";
        let key_secret = "keySecret_z11111111111111111111111111111111"; // Example base58 encoded key
        let nonce_material = b"test_nonce_material";

        // Test encryption
        let ciphertext = encrypt_internal(plaintext, key_secret, nonce_material).unwrap();
        assert!(!ciphertext.is_empty());

        // Test decryption
        let decrypted = decrypt_internal(&ciphertext, key_secret, nonce_material).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_invalid_key_secret() {
        let plaintext = b"test";
        let nonce_material = b"nonce";

        // Test with invalid key secret format
        let result = encrypt_internal(plaintext, "invalid_key", nonce_material);
        assert!(result.is_err());

        // Test with invalid base58 encoding
        let result = encrypt_internal(plaintext, "keySecret_z!!!!", nonce_material);
        assert!(result.is_err());
    }
}
