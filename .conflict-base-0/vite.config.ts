import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
	build: {
		lib: {
			entry: {
				index: "./lib/index.ts",
			},
			formats: ["es"],
			fileName: (_, entryName) => `${entryName}.js`,
		},
		target: "esnext",
		emptyOutDir: false,
	},
	plugins: [
		topLevelAwait(),
		dts({
			include: ["lib/index.ts"],
			rollupTypes: true,
		}),
	],
});
