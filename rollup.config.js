import typescript from "@rollup/plugin-typescript";
import copy from "rollup-plugin-copy";
import execute from "rollup-plugin-execute";
import replace from "@rollup/plugin-replace";

export default {
  input: "index.ts",
  output: {
    file: "dist/arrow.js",
    format: "umd",
    sourcemap: false,
    name: "arrow",
  },
  plugins: [
    copy({
      targets: [{ src: "pkg/arrow_wasm_bg.wasm", dest: "dist/arrow.wasm" }],
    }),
    typescript(),
    replace({
      "arrow_wasm.js": "arrow.js",
      "_bg.wasm": ".wasm",
    }),
  ],
};
