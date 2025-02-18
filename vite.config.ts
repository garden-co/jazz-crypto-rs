import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
	build: {
		lib: {
			entry: "./lib/index.ts",
			formats: ["es", "cjs"],
			fileName: (format) => `index.${format}.js`,
		},
		rollupOptions: {
			external: [/\.wasm$/],
		},
		target: "es2015",
		emptyOutDir: false,
		minify: false,
	},
	plugins: [wasm(), topLevelAwait(), dts({ include: ["lib"] })],
});
