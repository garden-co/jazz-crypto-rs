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
        b.iter(|| {
            // Initial state
            let mut state = blake3_empty_state();

            // First update with [1,2,3,4,5]
            let data1 = &[1u8, 2, 3, 4, 5];
            blake3_update_state(&mut state, data1);

            // Check that this matches a direct hash
            let direct_hash = blake3_hash_once(data1);
            let state_hash = state.finalize();
            assert_eq!(
                state_hash, direct_hash,
                "First update should match direct hash"
            );

            // Create new state for second test
            let mut state = blake3_empty_state();
            blake3_update_state(&mut state, data1);

            // Verify the exact expected hash from the TypeScript test for the first update
            let expected_first_hash = [
                2, 79, 103, 192, 66, 90, 61, 192, 47, 186, 245, 140, 185, 61, 229, 19, 46, 61, 117,
                197, 25, 250, 160, 186, 218, 33, 73, 29, 136, 201, 112, 87,
            ]
            .to_vec()
            .into_boxed_slice();
            assert_eq!(
                state.finalize(),
                expected_first_hash,
                "First update should match expected hash"
            );

            // Test with two updates
            let mut state = blake3_empty_state();
            let data1 = &[1u8, 2, 3, 4, 5];
            let data2 = &[6u8, 7, 8, 9, 10];
            blake3_update_state(&mut state, data1);
            blake3_update_state(&mut state, data2);

            // Compare with a single hash of all data
            let mut all_data = Vec::new();
            all_data.extend_from_slice(data1);
            all_data.extend_from_slice(data2);
            let direct_hash_all = blake3_hash_once(&all_data);
            assert_eq!(
                state.finalize(),
                direct_hash_all,
                "Final state should match direct hash of all data"
            );

            // Test final hash matches expected value
            let mut state = blake3_empty_state();
            blake3_update_state(&mut state, data1);
            blake3_update_state(&mut state, data2);

            let expected_final_hash = [
                165, 131, 141, 69, 2, 69, 39, 236, 196, 244, 180, 213, 147, 124, 222, 39, 68, 223,
                54, 176, 242, 97, 200, 101, 204, 79, 21, 233, 56, 51, 1, 199,
            ]
            .to_vec()
            .into_boxed_slice();
            assert_eq!(
                state.finalize(),
                expected_final_hash,
                "Final state should match expected hash"
            );
        });
    }
}
