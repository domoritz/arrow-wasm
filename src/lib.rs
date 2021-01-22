use arrow::ipc;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn parse(contents: &[u8]) -> usize {
    let cursor = Cursor::new(contents);
    let mut reader = ipc::reader::FileReader::try_new(cursor).unwrap();

    assert!(reader.num_batches() == 1);

    reader.next().unwrap().unwrap().num_rows()
}
