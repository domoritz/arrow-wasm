# WASM Arrow ![.github/workflows/test.yml](https://github.com/domoritz/arrow-wasm/workflows/.github/workflows/test.yml/badge.svg)

The idea here is to compile the Rust library to wasm and use that instead of the JS library. The question is how difficult it is and what the performance implications are.

This repo is currently work in progress and just a proof of concept.

## Building and testing

Run with `wasm-pack build --target nodejs && node example.js`.

To use a debug build, run `wasm-pack build --target nodejs --dev && node example.js`.

## Linting

Run `cargo fmt && cargo clippy` before committing.
