
import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: "index.js",
    output: {
        file: "dist/arrow.js",
        format: "umd",
        sourcemap: false,
        name: "arrow"
    },
    plugins: [
        rust({
            // return empty path so that the wasm path is resolved automatically
            importHook: function (_) { return undefined; },
        }),
    ],
};