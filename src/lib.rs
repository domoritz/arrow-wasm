#[macro_use]
mod utils;

mod field;
mod record_batch;
mod schema;
mod vector;

use std::io::Cursor;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn test(contents: &[u8]) -> record_batch::RecordBatch {
    crate::utils::set_panic_hook();

    // let field = arrow::datatypes::Field::new("c1", arrow::datatypes::DataType::Int64, false);
    // field::Field::new(field)

    let cursor = Cursor::new(contents);
    let mut reader = arrow::ipc::reader::FileReader::try_new(cursor).unwrap();

    let schema = reader.schema();
    let batch = reader.next().unwrap().unwrap();

    // schema::Schema::new(schema)

    record_batch::RecordBatch::new(batch)
}
