import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
	build: {
		lib: {
			entry: "./wrapper.ts",
			formats: ["es"],
			fileName: "wrapper",
		},
		target: "esnext",
		emptyOutDir: false,
	},
	plugins: [wasm(), topLevelAwait()],
});
