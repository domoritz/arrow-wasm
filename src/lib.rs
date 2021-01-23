mod utils;

use arrow::ipc;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello() {
    console::log_1(&"Hello, world!".into());
}

#[wasm_bindgen]
pub fn parse(contents: &[u8]) -> usize {
    let cursor = Cursor::new(contents);
    let mut reader = ipc::reader::FileReader::try_new(cursor).unwrap();

    assert!(reader.num_batches() == 1);

    reader.next().unwrap().unwrap().num_rows()
}
