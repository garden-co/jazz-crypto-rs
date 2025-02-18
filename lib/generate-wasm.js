import { readFileSync } from "node:fs";
const wasm = readFileSync(
	new URL("../dist/jazz_crypto_rs_bg.wasm", import.meta.url),
);
import { writeFileSync } from "node:fs";
const base64Wasm = `data:application/wasm;base64,${wasm.toString("base64")}`;
writeFileSync(
	new URL("../dist/wasm.js", import.meta.url),
	`export const data = "${base64Wasm}";`,
);
