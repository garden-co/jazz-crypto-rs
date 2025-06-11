#![feature(test)]
extern crate test;

static BAD_ALPHABET: std::sync::LazyLock<bs58::Alphabet> = std::sync::LazyLock::new(|| {
    bs58::Alphabet::new(b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY")
        .unwrap_or(bs58::Alphabet::DEFAULT.to_owned())
});

#[cfg(test)]
mod tests {
    use crate::BAD_ALPHABET;
    use jazz_crypto_rs::crypto::x25519::*;
    use jazz_crypto_rs::CryptoError;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_x25519_key_generation(b: &mut Bencher) {
        let x25519_key_generation = || -> Result<(), JsError> {
            // Test that we get the correct length keys
            let private_key = new_x25519_private_key();
            assert_eq!(private_key.len(), 32);

            // Test that public key generation works and produces correct length
            let public_key = x25519_public_key(&private_key)?;
            assert_eq!(public_key.len(), 32);

            // Test that different private keys produce different public keys
            let private_key2 = new_x25519_private_key();
            let public_key2 = x25519_public_key(&private_key2)?;
            assert_ne!(public_key, public_key2);
            Ok(())
        };
        b.iter(|| {
            let n = test::black_box(1000);
            (0..n).for_each(|_| {
                if let Err(e) = x25519_key_generation() {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_x25519_key_exchange(b: &mut Bencher) {
        let x25519_key_exchange = || -> Result<(), JsError> {
            // Generate sender's keypair
            let sender_private = new_x25519_private_key();
            let sender_public = x25519_public_key(&sender_private)?;

            // Generate recipient's keypair
            let recipient_private = new_x25519_private_key();
            let recipient_public = x25519_public_key(&recipient_private)?;

            // Test properties we expect from the shared secret
            let shared_secret1 = x25519_diffie_hellman(&sender_private, &recipient_public)?;
            let shared_secret2 = x25519_diffie_hellman(&recipient_private, &sender_public)?;

            // Both sides should arrive at the same shared secret
            assert_eq!(shared_secret1, shared_secret2);

            // Shared secret should be 32 bytes
            assert_eq!(shared_secret1.len(), 32);

            // Different recipient should produce different shared secret
            let other_recipient_private = new_x25519_private_key();
            let other_recipient_public = x25519_public_key(&other_recipient_private)?;
            let different_shared_secret =
                x25519_diffie_hellman(&sender_private, &other_recipient_public)?;
            assert_ne!(shared_secret1, different_shared_secret);
            Ok(())
        };
        b.iter(|| {
            let n = test::black_box(1000);
            (0..n).for_each(|_| {
                if let Err(e) = x25519_key_exchange() {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_get_sealer_id(b: &mut Bencher) {
        let get_sealer_id = |sealer_secret_invalid_format: &str,
                             sealer_secret_invalid_encoding: &str|
         -> Result<(), JsError> {
            // Create a test private key
            let private_key = new_x25519_private_key();
            let secret = format!("sealerSecret_z{}", bs58::encode(&private_key).into_string());

            // Get sealer ID
            let sealer_id = get_sealer_id(&secret.as_bytes())?;
            assert!(sealer_id.starts_with("sealer_z"));

            // Test that same secret produces same ID
            let sealer_id2 = get_sealer_id(&secret.as_bytes())?;
            assert_eq!(sealer_id, sealer_id2);

            // Test invalid secret format
            let result = get_sealer_id_internal(sealer_secret_invalid_format);
            assert!(matches!(
                result,
                Err(CryptoError::InvalidPrefix(
                    "sealerSecret_z",
                    "sealer secret"
                ))
            ));

            // Test invalid base58
            let result = get_sealer_id_internal(sealer_secret_invalid_encoding);
            assert!(matches!(result, Err(CryptoError::Base58Error(_))));
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = get_sealer_id("invalid_secret", "sealerSecret_z!!!invalid!!!") {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut sealer_secret_invalid_format = [0u8; 32];
                let mut sealer_secret_invalid_encoding = [0u8; 32];
                rng.fill(&mut sealer_secret_invalid_format[..]);
                rng.fill(&mut sealer_secret_invalid_encoding[..]);
                let sealer_secret_invalid_encoding_string = format!(
                    "sealerSecret_z{}",
                    bs58::encode(sealer_secret_invalid_encoding)
                        .with_alphabet(&BAD_ALPHABET)
                        .into_string()
                );
                if let Err(e) = get_sealer_id(
                    &bs58::encode(sealer_secret_invalid_format).into_string(),
                    &sealer_secret_invalid_encoding_string,
                ) {
                    eprintln!("{e:?}");
                }
            });
        });
    }
}
