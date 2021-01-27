
import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: "index.js",
    output: {
        file: "dist/arrow.js",
        format: "umd",
        sourcemap: true,
        name: "arrow",
        inlineDynamicImports: true
    },
    plugins: [
        rust({
            nodejs: true,
            serverPath: "dist/",
        }),
    ],
};