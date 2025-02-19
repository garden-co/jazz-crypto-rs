import assert from "node:assert";
import { blake3_hash_once, initialize } from "../dist/esm/src/index.mjs";

async function test() {
	await initialize();

	// Test BLAKE3 hashing
	const testData = new TextEncoder().encode("Hello, World!");
	const hash1 = blake3_hash_once(testData);

	// Verify hash is correct length (32 bytes)
	assert(hash1 instanceof Uint8Array, "Hash should be a Uint8Array");
	assert.strictEqual(hash1.length, 32, "BLAKE3 hash should be 32 bytes");

	// Verify hash is deterministic
	const hash2 = blake3_hash_once(testData);
	assert.deepStrictEqual(hash1, hash2, "Same input should produce same hash");

	// Verify different input produces different hash
	const differentData = new TextEncoder().encode("Different input");
	const hash3 = blake3_hash_once(differentData);
	assert.notDeepStrictEqual(
		hash1,
		hash3,
		"Different input should produce different hash",
	);

	console.log("✓ BLAKE3 hashing works correctly in Node");
}

test().catch(console.error);
