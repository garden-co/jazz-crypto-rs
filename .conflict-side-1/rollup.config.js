import ts from "@rollup/plugin-typescript";
import dts from "rollup-plugin-dts";
import externals from "rollup-plugin-node-externals";

export const config = [
	{
		input: "./src/index.ts",
		output: [
			{
				dir: "./dist/esm",
				format: "esm",
				exports: "named",
				preserveModules: true,
				// sourcemap: true,
				entryFileNames: "[name].mjs",
			},
			{
				dir: "./dist/cjs",
				format: "cjs",
				exports: "named",
				preserveModules: true,
				// sourcemap: true,
				entryFileNames: "[name].cjs",
			},
		],
		plugins: [externals(), ts()],
	},
	{
		input: "./src/index.ts",
		output: [
			{
				dir: "./dist/types",
				format: "esm",
				exports: "named",
				preserveModules: true,
				sourcemap: false,
				entryFileNames: "[name].d.ts",
			},
		],
		plugins: [externals(), ts(), dts()],
		external: [/^lib/],
	},
];

export default config;
