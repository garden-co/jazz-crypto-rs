import __wbg_init from "../dist/jazz_crypto_rs.js";
let isInitialized = false;

export async function initialize() {
	if (isInitialized) return;
	isInitialized = true;
	return await __wbg_init();
}

// Re-export everything from wasm
export * from "../dist/jazz_crypto_rs.js";
export type * from "../dist/jazz_crypto_rs.d.ts";
