#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use jazz_crypto_rs::crypto::ed25519::*;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_ed25519_key_generation_and_signing(b: &mut Bencher) {
        let ed25519_key_generation_and_signing = |message: &[u8],
                                                  wrong_message: &[u8]|
         -> Result<(), JsError> {
            // Test key generation
            let signing_key = new_ed25519_signing_key();
            assert_eq!(signing_key.len(), 32, "Signing key should be 32 bytes");

            // Test verifying key derivation
            let verifying_key = ed25519_verifying_key(&signing_key)?;
            assert_eq!(verifying_key.len(), 32, "Verifying key should be 32 bytes");

            // Test that different signing keys produce different verifying keys
            let signing_key2 = new_ed25519_signing_key();
            let verifying_key2 = ed25519_verifying_key(&signing_key2)?;
            assert_ne!(
                verifying_key, verifying_key2,
                "Different signing keys should produce different verifying keys"
            );

            // Test signing and verification
            let signature = ed25519_sign(&signing_key, message)?;
            assert_eq!(signature.len(), 64, "Signature should be 64 bytes");

            // Test successful verification
            let verification_result = ed25519_verify(&verifying_key, message, &signature)?;
            assert!(
                verification_result,
                "Valid signature should verify successfully"
            );

            // Test verification with wrong message
            let wrong_verification = ed25519_verify(&verifying_key, wrong_message, &signature)?;
            assert!(
                !wrong_verification,
                "Signature should not verify with wrong message"
            );

            // Test verification with wrong key
            let wrong_verification = ed25519_verify(&verifying_key2, message, &signature)?;
            assert!(
                !wrong_verification,
                "Signature should not verify with wrong key"
            );

            // Test verification with tampered signature
            let mut tampered_signature = signature.clone();
            tampered_signature[0] ^= 1;
            let wrong_verification = ed25519_verify(&verifying_key, message, &tampered_signature)?;
            assert!(!wrong_verification, "Tampered signature should not verify");
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = ed25519_key_generation_and_signing(b"Test message", b"Wrong message") {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut message = [0u8; 64];
                let mut wrong_message = [0u8; 64];
                rng.fill(&mut message[..]);
                rng.fill(&mut wrong_message[..]);
                if let Err(e) = ed25519_key_generation_and_signing(&message, &wrong_message) {
                    eprintln!("{e:?}");
                }
            });
        });
    }
}
