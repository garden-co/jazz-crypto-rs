#![feature(test)]
extern crate test;

static BAD_ALPHABET: std::sync::LazyLock<bs58::Alphabet> = std::sync::LazyLock::new(|| {
    bs58::Alphabet::new(b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY")
        .unwrap_or(bs58::Alphabet::DEFAULT.to_owned())
});

#[cfg(test)]
mod tests {
    use crate::BAD_ALPHABET;
    use jazz_crypto_rs::crypto::seal::*;
    use jazz_crypto_rs::crypto::x25519::*;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_seal_unseal(b: &mut Bencher) {
        let seal_unseal = |message: &[u8], nonce_material: &[u8]| -> Result<(), JsError> {
            // Generate real keys
            let sender_private = new_x25519_private_key();
            let sender_public = x25519_public_key(&sender_private)?;

            // Encode keys with proper prefixes
            let sender_secret = format!(
                "sealerSecret_z{}",
                bs58::encode(&sender_private).into_string()
            );
            let recipient_id = format!("sealer_z{}", bs58::encode(&sender_public).into_string());

            // Test sealing
            let sealed = seal(message, &sender_secret, &recipient_id, nonce_material)?;
            assert!(!sealed.is_empty());

            // Test unsealing (using same keys since it's a test)
            let unsealed = unseal(&sealed, &sender_secret, &recipient_id, nonce_material)?;
            assert_eq!(&*unsealed, message);
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = seal_unseal(b"Secret message", b"test_nonce_material") {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut message = [0u8; 64];
                let mut nonce_material = [0u8; 24];
                rng.fill(&mut message[..]);
                rng.fill(&mut nonce_material[..]);
                if let Err(e) = seal_unseal(&message, &nonce_material) {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_invalid_keys(b: &mut Bencher) {
        let invalid_keys = |message: &[u8],
                            nonce_material: &[u8],
                            sealer_secret: &str,
                            sealer_id: &str,
                            sealer_secret_invalid_format: &str,
                            sealer_secret_invalid_encoding: &str,
                            sealer_id_invalid_format: &str| {
            // Test with invalid sender secret format
            let result = seal_internal(
                message,
                sealer_secret_invalid_format,
                sealer_id,
                nonce_material,
            );
            assert!(result.is_err());

            // Test with invalid recipient ID format
            let result = seal_internal(
                message,
                sealer_secret,
                sealer_id_invalid_format,
                nonce_material,
            );
            assert!(result.is_err());

            // Test with invalid base58 encoding
            let result = seal_internal(
                message,
                sealer_secret_invalid_encoding,
                sealer_id,
                nonce_material,
            );
            assert!(result.is_err());
        };
        b.iter(|| {
            invalid_keys(
                b"test",
                b"nonce",
                "sealerSecret_z11111111111111111111111111111111",
                "sealer_z22222222222222222222222222222222",
                "invalid_key",
                "sealerSecret_z!!!!",
                "invalid_key",
            );

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut message = [0u8; 64];
                let mut nonce_material = [0u8; 24];
                let mut sealer_secret = [0u8; 32];
                let mut sealer_id = [0u8; 32];
                let mut sealer_secret_invalid_format = [0u8; 32];
                let mut sealer_secret_invalid_encoding = [0u8; 32];
                let mut sealer_id_invalid_format = [0u8; 32];
                rng.fill(&mut message[..]);
                rng.fill(&mut nonce_material[..]);
                rng.fill(&mut sealer_secret[..]);
                rng.fill(&mut sealer_id[..]);
                rng.fill(&mut sealer_secret_invalid_format[..]);
                rng.fill(&mut sealer_secret_invalid_encoding[..]);
                rng.fill(&mut sealer_id_invalid_format[..]);
                let sealer_secret_invalid_encoding_string = format!(
                    "sealerSecret_z{}",
                    bs58::encode(sealer_secret_invalid_encoding)
                        .with_alphabet(&BAD_ALPHABET)
                        .into_string()
                );
                invalid_keys(
                    &message,
                    &nonce_material,
                    &format!(
                        "sealerSecret_z{}",
                        bs58::encode(sealer_secret).into_string()
                    ),
                    &format!("sealer_z{}", bs58::encode(sealer_id).into_string()),
                    &bs58::encode(sealer_secret_invalid_format).into_string(),
                    &sealer_secret_invalid_encoding_string,
                    &bs58::encode(sealer_id_invalid_format).into_string(),
                );
            });
        });
    }
}
