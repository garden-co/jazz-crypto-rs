import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";

const wasm = readFileSync("./wasm/pkg/jazz_crypto_rs_bg.wasm");

// Create dist directory if it doesn't exist
if (!existsSync("./dist")) {
	mkdirSync("./dist");
}

writeFileSync(
	"./wasm/pkg/jazz_crypto_rs.wasm.js",
	`export const data = "data:application/wasm;base64,${wasm.toString("base64")}";`,
);
writeFileSync(
	"./wasm/pkg/jazz_crypto_rs.wasm.d.ts",
	"export const data: string;",
);

const glueJs = readFileSync("./wasm/pkg/jazz_crypto_rs.js", "utf8").replace(
	"module_or_path = new URL('jazz_crypto_rs_bg.wasm', import.meta.url);",
	"throw new Error();",
);

writeFileSync("./wasm/pkg/jazz_crypto_rs.js", glueJs);
