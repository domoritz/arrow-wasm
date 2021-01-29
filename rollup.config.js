import rust from "@wasm-tool/rollup-plugin-rust";

export default {
  input: "index.js",
  output: {
    file: "dist/arrow.js",
    format: "umd",
    sourcemap: true,
    name: "arrow",
  },
  plugins: [
    rust({
      inlineWasm: true,
    }),
  ],
};
