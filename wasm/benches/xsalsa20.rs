#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use jazz_crypto_rs::crypto::xsalsa20::*;
    use test::Bencher;
    use wasm_bindgen::JsError;

    #[bench]
    fn bench_xsalsa20(b: &mut Bencher) {
        let xsalsa20 = |plaintext: &[u8],
                        key: &[u8; 32],
                        nonce: &[u8; 24],
                        key2: &[u8; 32],
                        nonce2: &[u8; 24]|
         -> Result<(), JsError> {
            // Test encryption
            let ciphertext = encrypt_xsalsa20(&key[..], &nonce[..], plaintext)?;
            assert_ne!(&*ciphertext, plaintext); // Ciphertext should be different from plaintext

            // Test decryption
            let decrypted = decrypt_xsalsa20(&key[..], &nonce[..], &ciphertext)?;
            assert_eq!(&*decrypted, plaintext);

            // Test that different nonce produces different ciphertext
            let ciphertext2 = encrypt_xsalsa20(&key[..], &nonce2[..], plaintext)?;
            assert_ne!(ciphertext, ciphertext2);

            // Test that different key produces different ciphertext
            let ciphertext3 = encrypt_xsalsa20(&key2[..], &nonce[..], plaintext)?;
            assert_ne!(ciphertext, ciphertext3);

            // Test invalid key length
            assert!(encrypt_xsalsa20_raw_internal(&key[..31], &nonce[..], plaintext).is_err());
            assert!(decrypt_xsalsa20_raw_internal(&key[..31], &nonce[..], &ciphertext).is_err());

            // Test invalid nonce length
            assert!(encrypt_xsalsa20_raw_internal(&key[..], &nonce[..23], plaintext).is_err());
            assert!(decrypt_xsalsa20_raw_internal(&key[..], &nonce[..23], &ciphertext).is_err());
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = xsalsa20(
                b"Hello, World!",
                &[0u8; 32],
                &[0u8; 24],
                &[1u8; 32],
                &[1u8; 24],
            ) {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut plaintext = [0u8; 64];
                let mut key = [0u8; 32];
                let mut nonce = [0u8; 24];
                let mut key2 = [0u8; 32];
                let mut nonce2 = [0u8; 24];
                rng.fill(&mut plaintext[..]);
                rng.fill(&mut key[..]);
                rng.fill(&mut nonce[..]);
                rng.fill(&mut key2[..]);
                rng.fill(&mut nonce2[..]);
                if let Err(e) = xsalsa20(&plaintext, &key, &nonce, &key2, &nonce2) {
                    eprintln!("{e:?}");
                }
            });
        });
    }

    #[bench]
    fn bench_xsalsa20_poly1305(b: &mut Bencher) {
        let xsalsa20_poly1305 = |plaintext: &[u8],
                                 key: &[u8; 32],
                                 nonce: &[u8; 24],
                                 key2: &[u8; 32],
                                 nonce2: &[u8; 24]|
         -> Result<(), JsError> {
            // Test encryption
            let ciphertext = encrypt_xsalsa20_poly1305(&key[..], &nonce[..], plaintext)?;
            assert!(ciphertext.len() > plaintext.len()); // Should include authentication tag

            // Test decryption
            let decrypted = decrypt_xsalsa20_poly1305(&key[..], &nonce[..], &ciphertext)?;
            assert_eq!(&*decrypted, plaintext);

            // Test that different nonce produces different ciphertext
            let ciphertext2 = encrypt_xsalsa20_poly1305(&key[..], &nonce2[..], plaintext)?;
            assert_ne!(ciphertext, ciphertext2);

            // Test that different key produces different ciphertext
            let ciphertext3 = encrypt_xsalsa20_poly1305(&key2[..], &nonce[..], plaintext)?;
            assert_ne!(ciphertext, ciphertext3);

            // Test that decryption fails with wrong key
            assert!(decrypt_xsalsa20_poly1305(&key2[..], &nonce[..], &ciphertext).is_err());

            // Test that decryption fails with wrong nonce
            assert!(decrypt_xsalsa20_poly1305(&key[..], &nonce2[..], &ciphertext).is_err());

            // Test that decryption fails with tampered ciphertext
            let mut tampered = ciphertext.clone();
            tampered[0] ^= 1;
            assert!(decrypt_xsalsa20_poly1305(&key[..], &nonce[..], &tampered).is_err());
            Ok(())
        };
        b.iter(|| {
            if let Err(e) = xsalsa20_poly1305(
                b"Hello, World!",
                &[0u8; 32],
                &[0u8; 24],
                &[1u8; 32],
                &[1u8; 24],
            ) {
                eprintln!("{e:?}");
            }

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut plaintext = [0u8; 64];
                let mut key = [0u8; 32];
                let mut nonce = [0u8; 24];
                let mut key2 = [0u8; 32];
                let mut nonce2 = [0u8; 24];
                rng.fill(&mut plaintext[..]);
                rng.fill(&mut key[..]);
                rng.fill(&mut nonce[..]);
                rng.fill(&mut key2[..]);
                rng.fill(&mut nonce2[..]);
                if let Err(e) = xsalsa20_poly1305(&plaintext, &key, &nonce, &key2, &nonce2) {
                    eprintln!("{e:?}");
                }
            });
        });
    }
}
