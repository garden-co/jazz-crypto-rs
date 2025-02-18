import { readFileSync } from "node:fs";
import { defineConfig } from "vite";
import type { Plugin } from "vite";
import dts from "vite-plugin-dts";
import topLevelAwait from "vite-plugin-top-level-await";

function wasmBase64Plugin(): Plugin {
	return {
		name: "wasm-base64",
		transform(code, id) {
			if (id.endsWith(".wasm?raw")) {
				const wasmPath = id.slice(0, -4); // remove "?raw"
				const wasmContent = readFileSync(wasmPath);
				const base64 = wasmContent.toString("base64");
				return {
					code: `export default "data:application/wasm;base64,${base64}";`,
				};
			}
		},
	};
}

export default defineConfig({
	build: {
		lib: {
			entry: {
				index: "./lib/index.ts",
				wasm: "./lib/wasm.ts",
			},
			formats: ["es"],
			fileName: (format, entryName) => `${entryName}.js`,
		},
		target: "esnext",
		emptyOutDir: false,
	},
	assetsInclude: ["**/*.wasm"],
	plugins: [
		topLevelAwait(),
		wasmBase64Plugin(),
		dts({
			include: ["lib/index.ts"],
			rollupTypes: true,
		}),
	],
});
