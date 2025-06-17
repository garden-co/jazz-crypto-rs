#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use jazz_crypto_rs::hash::*;
    use test::Bencher;

    #[bench]
    fn bench_nonce_generation(b: &mut Bencher) {
        let nonce_generation = |input: &[u8], different_input: &[u8]| {
            let nonce = generate_nonce(input);
            assert_eq!(nonce.len(), 24);

            // Same input should produce same nonce
            let nonce2 = generate_nonce(input);
            assert_eq!(nonce, nonce2);

            // Different input should produce different nonce
            let nonce3 = generate_nonce(different_input);
            assert_ne!(nonce, nonce3);
        };
        b.iter(|| {
            nonce_generation(b"test input", b"different input");

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut input = [0u8; 64];
                let mut different_input = [0u8; 64];
                rng.fill(&mut input[..]);
                rng.fill(&mut different_input[..]);
                nonce_generation(&input, &different_input);
            });
        });
    }

    #[bench]
    fn bench_blake3_hash_once(b: &mut Bencher) {
        let blake3_hash_once = |input: &[u8], different_input: &[u8]| {
            let hash = blake3_hash_once(input);

            // BLAKE3 produces 32-byte hashes
            assert_eq!(hash.len(), 32);

            // Same input should produce same hash
            let hash2 = blake3_hash_once(input);
            assert_eq!(hash, hash2);

            // Different input should produce different hash
            let hash3 = blake3_hash_once(different_input);
            assert_ne!(hash, hash3);
        };
        b.iter(|| {
            blake3_hash_once(b"test input", b"different input");

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut input = [0u8; 64];
                let mut different_input = [0u8; 64];
                rng.fill(&mut input[..]);
                rng.fill(&mut different_input[..]);
                blake3_hash_once(&input, &different_input);
            });
        });
    }

    #[bench]
    fn bench_blake3_hash_once_with_context(b: &mut Bencher) {
        let blake3_hash_once_with_context =
            |input: &[u8], context: &[u8], different_input: &[u8], different_context: &[u8]| {
                let hash = blake3_hash_once_with_context(input, context);

                // BLAKE3 produces 32-byte hashes
                assert_eq!(hash.len(), 32);

                // Same input and context should produce same hash
                let hash2 = blake3_hash_once_with_context(input, context);
                assert_eq!(hash, hash2);

                // Different input should produce different hash
                let hash3 = blake3_hash_once_with_context(different_input, context);
                assert_ne!(hash, hash3);

                // Different context should produce different hash
                let hash4 = blake3_hash_once_with_context(input, different_context);
                assert_ne!(hash, hash4);

                // Hash with context should be different from hash without context
                let hash_no_context = blake3_hash_once(input);
                assert_ne!(hash, hash_no_context);
            };
        b.iter(|| {
            blake3_hash_once_with_context(
                b"test input",
                b"test context",
                b"different input",
                b"different context",
            );

            let n = test::black_box(1000);
            use rand::Rng;
            let mut rng = rand::thread_rng();

            (0..n).for_each(|_| {
                let mut input = [0u8; 64];
                let mut context = [0u8; 64];
                let mut different_input = [0u8; 64];
                let mut different_context = [0u8; 64];
                rng.fill(&mut input[..]);
                rng.fill(&mut context[..]);
                rng.fill(&mut different_input[..]);
                rng.fill(&mut different_context[..]);
                blake3_hash_once_with_context(
                    &input,
                    &context,
                    &different_input,
                    &different_context,
                );
            });
        });
    }

    #[bench]
    fn bench_blake3_incremental(b: &mut Bencher) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let blake3_incremental = |rng: &mut rand::rngs::ThreadRng, n: u32| {
            let n = test::black_box(n);

            let mut all_data = Vec::new();

            // Initial state
            let mut state = blake3_empty_state();

            for _ in 0..n {
                let mut data = [0u8; 64];
                rng.fill(&mut data[..]);
                all_data.extend_from_slice(&data);
                blake3_update_state(&mut state, &data);
            }

            // Check that this matches a direct hash
            let direct_hash = blake3_hash_once(&all_data);
            let state_hash = state.finalize();
            assert_eq!(
                state_hash, direct_hash,
                "Final state should match direct hash of all data"
            );
        };
        b.iter(|| {
            let num_benches = test::black_box(1000);
            let n: u32 = rng.gen_range(1..10);

            (0..num_benches).for_each(|_| {
                blake3_incremental(&mut rng, n);
            });
        });
    }
}
