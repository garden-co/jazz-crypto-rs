export * from "../wasm/pkg/jazz_crypto_rs.js";

import __wbg_init, { type InitOutput } from "../wasm/pkg/jazz_crypto_rs.js";
import { data } from "../wasm/pkg/jazz_crypto_rs.wasm.js";

let output: InitOutput | undefined = undefined;

export async function initialize() {
	return (output ??= await __wbg_init(data));
}
