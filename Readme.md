# WASM Arrow ![.github/workflows/test.yml](https://github.com/domoritz/arrow-wasm/workflows/.github/workflows/test.yml/badge.svg) [![npm version](https://img.shields.io/npm/v/arrow-wasm.svg)](https://www.npmjs.com/package/arrow-wasm)

The idea here is to compile the Rust library to wasm and use that instead of the JS library. The question is how difficult it is and what the performance implications are.

This repo is currently work in progress and just a proof of concept.

## Building and testing

### Node

Run with `wasm-pack build --target nodejs && node example.js`.

To use a debug build, run `wasm-pack build --target nodejs --dev && node example.js`.

### Browser

Build with `wasm-pack build --target web`. Then run `python3 -m http.server` and open http://localhost:8000/.

## Publishing

```bash
trash pkg
wasm-pack build --target web
wasm-pack pack
wasm-pack publish
```

## Linting

Run `cargo fmt && cargo clippy` before committing.

## Check file size

We can check how large the wasm file is after compression (which every web server probably does).

`gzip -9 <pkg//arrow_wasm_bg.wasm | wc -c`
