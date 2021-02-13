# WASM Arrow ![.github/workflows/test.yml](https://github.com/domoritz/arrow-wasm/workflows/.github/workflows/test.yml/badge.svg) [![npm version](https://img.shields.io/npm/v/arrow-wasm.svg)](https://www.npmjs.com/package/arrow-wasm)

This package compiles the Rust library of [Apache Arrow](https://arrow.apache.org/) to WebAssembly. This might be a viable alternative to the [pure JavaScript library](https://arrow.apache.org/docs/js/). Right now, this library is incomplete and the API will change so we recommend using JavaScript library.

You can try this library in Observable at https://observablehq.com/@domoritz/apache-arrow-in-webassembly.

## Documentation

Coming later. The API is somewhat similar to the Rust version (https://docs.rs/arrow/3.0.0/arrow/) although there are some differences to make the API more familiar to JavaScript users.

## Building and testing

### Node

Run with `wasm-pack build --target nodejs && node example/flights.js`.

To use a debug build, run `wasm-pack build --target nodejs --dev && node examples/flights.js`.

### Browser

Build with `wasm-pack build --target web`. Then run `python3 -m http.server` and open http://localhost:8000/examples/.

## Publishing

Run `npm publish` to build a bundle and release it to NPM.

## Linting

Run `cargo fmt && cargo clippy` before committing.

## Check file size

We can check how large the WASM file is after compression (which every web server probably does).

`gzip -9 <pkg//arrow_wasm_bg.wasm | wc -c`
