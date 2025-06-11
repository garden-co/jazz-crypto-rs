#![feature(test)]
extern crate test;

static BAD_ALPHABET: std::sync::LazyLock<bs58::Alphabet> = std::sync::LazyLock::new(|| {
    bs58::Alphabet::new(b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY")
        .unwrap_or(bs58::Alphabet::DEFAULT.to_owned())
});

#[cfg(test)]
mod tests {
    use crate::BAD_ALPHABET;
    use jazz_crypto_rs::crypto::ed25519::*;
    use jazz_crypto_rs::crypto::sign::*;
    use jazz_crypto_rs::CryptoError;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_sign_and_verify(b: &mut Bencher) {
        let sign_and_verify = |message: &[u8]| -> Result<(), JsError> {
            // Create a test signing key
            let signing_key = new_ed25519_signing_key();
            let secret = format!("signerSecret_z{}", bs58::encode(&signing_key).into_string());

            // Sign the message
            let signature = sign(message, &secret.as_bytes())?;

            // Get the public key for verification
            let secret_bytes =
                bs58::decode(secret.strip_prefix("signerSecret_z").unwrap_or_default())
                    .into_vec()?;
            let verifying_key = ed25519_verifying_key(&secret_bytes)?;
            let signer_id = format!("signer_z{}", bs58::encode(&verifying_key).into_string());

            // Verify the signature
            assert!(verify(
                &signature.as_bytes(),
                message,
                &signer_id.as_bytes()
            )?);
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = sign_and_verify(b"hello world") {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut message = [0u8; 64];
                rng.fill(&mut message[..]);
                if let Err(e) = sign_and_verify(&message) {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_invalid_inputs(b: &mut Bencher) {
        let invalid_inputs = |message: &[u8],
                              signer_id: &str,
                              signature: &str,
                              signer_secret_invalid_encoding: &str,
                              signature_invalid_format: &str,
                              signer_id_invalid_format: &str| {
            // Test invalid base58 in secret
            let result = sign_internal(message, signer_secret_invalid_encoding);
            assert!(matches!(result, Err(CryptoError::Base58Error(_))));

            // Test invalid signature format
            let result = verify_internal(signature_invalid_format, message, signer_id);
            assert!(matches!(
                result,
                Err(CryptoError::InvalidPrefix("signature_z", "signature"))
            ));

            // Test invalid signer ID format
            let result = verify_internal(signature, message, signer_id_invalid_format);
            assert!(matches!(
                result,
                Err(CryptoError::InvalidPrefix("signer_z", "signer ID"))
            ));
        };
        b.iter(|| {
            invalid_inputs(
                b"hello world",
                "signer_z123",
                "signature_z123",
                "signerSecret_z!!!invalid!!!",
                "not_a_signature",
                "not_a_signer",
            );

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut message = [0u8; 64];
                let mut signer_id = [0u8; 32];
                let mut signature = [0u8; 32];
                let mut signer_secret_invalid_encoding = [0u8; 32];
                let mut signature_invalid_format = [0u8; 32];
                let mut signer_id_invalid_format = [0u8; 32];
                rng.fill(&mut message[..]);
                rng.fill(&mut signer_id[..]);
                rng.fill(&mut signature[..]);
                rng.fill(&mut signer_secret_invalid_encoding[..]);
                rng.fill(&mut signature_invalid_format[..]);
                rng.fill(&mut signer_id_invalid_format[..]);
                let signer_secret_invalid_encoding_string = format!(
                    "signerSecret_z{}",
                    bs58::encode(signer_secret_invalid_encoding)
                        .with_alphabet(&BAD_ALPHABET)
                        .into_string()
                );
                invalid_inputs(
                    &message,
                    &format!("signer_z{}", bs58::encode(signer_id).into_string()),
                    &format!("signature_z{}", bs58::encode(signature).into_string()),
                    &signer_secret_invalid_encoding_string,
                    &bs58::encode(signature_invalid_format).into_string(),
                    &bs58::encode(signer_id_invalid_format).into_string(),
                );
            });
        });
    }

    #[bench]
    fn bench_get_signer_id(b: &mut Bencher) {
        let get_signer_id = |signer_secret_invalid_format: &str,
                             signer_secret_invalid_encoding: &str|
         -> Result<(), JsError> {
            // Create a test signing key
            let signing_key = new_ed25519_signing_key();
            let secret = format!("signerSecret_z{}", bs58::encode(&signing_key).into_string());

            // Get signer ID
            let signer_id = get_signer_id(&secret.as_bytes())?;
            assert!(signer_id.starts_with("signer_z"));

            // Test that same secret produces same ID
            let signer_id2 = get_signer_id(&secret.as_bytes())?;
            assert_eq!(signer_id, signer_id2);

            // Test invalid secret format
            let result = get_signer_id_internal(signer_secret_invalid_format);
            assert!(matches!(
                result,
                Err(CryptoError::InvalidPrefix(
                    "signerSecret_z",
                    "signer secret"
                ))
            ));

            // Test invalid base58
            let result = get_signer_id_internal(signer_secret_invalid_encoding);
            assert!(matches!(result, Err(CryptoError::Base58Error(_))));
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = get_signer_id("invalid_secret", "signerSecret_z!!!invalid!!!") {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut signer_secret_invalid_format = [0u8; 32];
                let mut signer_secret_invalid_encoding = [0u8; 32];
                rng.fill(&mut signer_secret_invalid_format[..]);
                rng.fill(&mut signer_secret_invalid_encoding[..]);
                let signer_secret_invalid_encoding_string = format!(
                    "signerSecret_z{}",
                    bs58::encode(signer_secret_invalid_encoding)
                        .with_alphabet(&BAD_ALPHABET)
                        .into_string()
                );
                if let Err(e) = get_signer_id(
                    &bs58::encode(signer_secret_invalid_format).into_string(),
                    &signer_secret_invalid_encoding_string,
                ) {
                    eprintln!("{e:?}");
                }
            });
        });
    }
}
