export * from "./target/wasm-pack/arrow-wasm/index";

type Exports = typeof import("./target/wasm-pack/arrow-wasm/index");
declare const init: () => Promise<Exports>;
export default init;
