#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use jazz_crypto_rs::crypto::encrypt::*;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_encrypt_decrypt(b: &mut Bencher) {
        let encrypt_decrypt =
            |plaintext: &[u8], key_secret: &str, nonce_material: &[u8]| -> Result<(), JsError> {
                // Test encryption
                let ciphertext = encrypt_internal(plaintext, key_secret, nonce_material)?;
                assert!(!ciphertext.is_empty());

                // Test decryption
                let decrypted = decrypt_internal(&ciphertext, key_secret, nonce_material)?;
                assert_eq!(&*decrypted, plaintext);
                Ok(())
            };
        b.iter(|| {
            if let Err(e) = encrypt_decrypt(
                b"Hello, World!",
                "keySecret_z11111111111111111111111111111111",
                b"test_nonce_material",
            ) {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut plaintext = [0u8; 64];
                let mut key_secret = [0u8; 32];
                let mut nonce_material = [0u8; 24];
                rng.fill(&mut plaintext[..]);
                rng.fill(&mut key_secret[..]);
                rng.fill(&mut nonce_material[..]);
                if let Err(e) = encrypt_decrypt(
                    &plaintext,
                    &format!("keySecret_z{}", bs58::encode(key_secret).into_string()),
                    &nonce_material,
                ) {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_invalid_key_secret(b: &mut Bencher) {
        let invalid_key_secret =
            |plaintext: &[u8],
             nonce_material: &[u8],
             key_secret_invalid_format: &str,
             key_secret_invalid_encoding: &str| {
                // Test with invalid key secret format
                let result = encrypt_internal(plaintext, key_secret_invalid_format, nonce_material);
                assert!(result.is_err());

                // Test with invalid base58 encoding
                let result =
                    encrypt_internal(plaintext, key_secret_invalid_encoding, nonce_material);
                assert!(result.is_err());
            };
        b.iter(|| {
            invalid_key_secret(b"test", b"nonce", "invalid_key", "keySecret_z!!!!");

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut plaintext = [0u8; 64];
                let mut nonce_material = [0u8; 24];
                let mut key_secret_invalid_format = [0u8; 32];
                let mut key_secret_invalid_encoding = [0u8; 32];
                rng.fill(&mut plaintext[..]);
                rng.fill(&mut nonce_material[..]);
                rng.fill(&mut key_secret_invalid_format[..]);
                rng.fill(&mut key_secret_invalid_encoding[..]);
                let key_secret_invalid_encoding_string = format!(
                    "keySecret_z{}",
                    key_secret_invalid_encoding
                        .iter()
                        .map(|b| format!("{:x}", b))
                        .collect::<String>()
                );
                invalid_key_secret(
                    &plaintext,
                    &nonce_material,
                    &bs58::encode(key_secret_invalid_format).into_string(),
                    &key_secret_invalid_encoding_string,
                );
            });
        });
    }
}
