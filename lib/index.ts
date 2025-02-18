// src/index.js
import { __wbg_set_wasm as init } from "../dist/jazz_crypto_rs_bg.js";

let initialized = false;
let initPromise: Promise<void> | null = null;

export async function initialize() {
	if (initialized) return;
	if (initPromise) return initPromise;

	initPromise = (async () => {
		if (typeof window === "undefined") {
			// Node.js environment
			await init(require("../dist/jazz_crypto_rs_bg.wasm"));
		} else {
			// Browser environment
			await init();
		}
		initialized = true;
	})();

	return initPromise;
}

// Re-export everything from wasm
export * from "../dist/jazz_crypto_rs.js";
export type * from "../dist/jazz_crypto_rs_bg.d.ts";
