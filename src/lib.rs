use arrow::array::UInt32Array;
use arrow::ipc;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen(module = "buffer")]
extern "C" {
    type Buffer;

    #[wasm_bindgen(method, js_name = toString)]
    fn to_string(this: &Buffer) -> String;
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file(path: &str) -> Result<Buffer, JsValue>;
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn length() -> usize {
    let array = UInt32Array::from(vec![Some(1), None, Some(3)]);

    return array.len();
}

#[wasm_bindgen]
pub fn read(path: &str) -> usize {
    log(&format!("Reading {}", path));

    return 42;

    // let buffer = read_file(path).unwrap();
    // return parse(buffer);
    // log(&buffer.to_string());
    // let file = File::open(path).unwrap();

    // let reader = ipc::reader::StreamReader::try_new(buffer);
    // return reader.unwrap().num_batches();
}

#[wasm_bindgen]
pub fn parse(contents: &[u8]) -> usize {
    let cursor = Cursor::new(contents);
    let reader = ipc::reader::FileReader::try_new(cursor).unwrap();
    return reader.num_batches();
}

// read("flights-10k.arrow")

// #[wasm_bindgen]
// pub fn load(url: &str) -> usize {
//     log(&format!("Reading {}", path));

//     let file = read_file(path).unwrap();
//     // let file = File::open(path).unwrap();

//     // let reader = ipc::reader::StreamReader::try_new(file);
//     // return reader.unwrap().num_batches();

//     return 42;
// }