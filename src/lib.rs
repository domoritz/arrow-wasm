mod utils;
use std::sync::Arc;
use arrow::array::Int32Array;
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
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

#[wasm_bindgen]
pub fn serialize_vector(data: Vec<i32>, nullable: bool) -> Result<Vec<u8>, JsValue> {
    let array = Int32Array::from(data);

    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("values", DataType::Int32, nullable)])),
        vec![Arc::new(array)]
    ).expect("Could not create batch");

    let mut file = Vec::new();
    let writer = ipc::writer::FileWriter::try_new(&mut file, &batch.schema());
    writer.unwrap().write(&batch).expect("Could not write batch");

    Ok(file)
}

#[wasm_bindgen]
pub fn serialize(contents: &[u8]) -> Vec<u8> {
    let cursor = Cursor::new(contents);
    let mut reader = ipc::reader::FileReader::try_new(cursor).unwrap();

    let schema = reader.schema();
    let batch = reader.next().unwrap().unwrap();

    let mut file = Vec::new();
    let writer = ipc::writer::FileWriter::try_new(&mut file, &schema);
    writer.unwrap().write(&batch).unwrap();

    file
}

// #[wasm_bindgen]
// pub struct Table {
//     batches: Vec<RecordBatch>,
// }

// #[wasm_bindgen]
// impl Table {
//     pub fn new(contents: &[u8]) -> Self {
//         let cursor = Cursor::new(contents);
//         let mut reader = ipc::reader::FileReader::try_new(cursor).unwrap();

//         let batches: Vec<RecordBatch> = reader.collect();

//         Self { batches: batches }
//     }
// }
