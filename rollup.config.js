import typescript from "@rollup/plugin-typescript";
import copy from "rollup-plugin-copy";

export default {
  input: "index.ts",
  output: {
    file: "dist/arrow.js",
    format: "umd",
    sourcemap: true,
    name: "arrow",
  },
  plugins: [
    copy({
      targets: [{ src: "pkg/arrow_wasm_bg.wasm", dest: "dist/" }],
    }),
    typescript(),
  ],
};
