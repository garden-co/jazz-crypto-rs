// import __wbg_init from "../wasm/pkg/jazz_crypto_rs.js";
// let isInitialized = false;

// export async function initialize() {
// 	if (isInitialized) return;
// 	isInitialized = true;
// 	return await __wbg_init();
// }

// // Re-export everything from wasm
// export * from "../wasm/pkg/jazz_crypto_rs.js";
// export type * from "../wasm/pkg/jazz_crypto_rs";

export * from "../wasm/pkg/jazz_crypto_rs.js";

import __wbg_init, { type InitOutput } from "../wasm/pkg/jazz_crypto_rs.js";
import { data } from "../wasm/pkg/jazz_crypto_rs.wasm.js";

let output: InitOutput | undefined = undefined;

export async function initialize() {
	return (output ??= await __wbg_init(data));
}
