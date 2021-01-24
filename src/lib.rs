mod utils;
use crate::utils::set_panic_hook;
use arrow::array::Int32Array;
use arrow::datatypes;
use arrow::ipc;
use arrow::record_batch;
use std::io::Cursor;
use std::sync::Arc;
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

#[wasm_bindgen(catch)]
pub fn serialize_vector(data: Vec<i32>, nullable: bool) -> Result<Vec<u8>, JsValue> {
    let array = Int32Array::from(data);

    let batch = record_batch::RecordBatch::try_new(
        Arc::new(datatypes::Schema::new(vec![datatypes::Field::new(
            "values",
            datatypes::DataType::Int32,
            nullable,
        )])),
        vec![Arc::new(array)],
    )
    .expect("Could not create batch");

    let mut file = Vec::new();
    let writer = ipc::writer::FileWriter::try_new(&mut file, &batch.schema());
    writer
        .unwrap()
        .write(&batch)
        .expect("Could not write batch");

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

#[wasm_bindgen]
pub struct RecordBatch {
    batch: record_batch::RecordBatch,
}

#[wasm_bindgen]
impl RecordBatch {
    #[wasm_bindgen(catch)]
    pub fn from(data: Vec<i32>, nullable: bool) -> Result<RecordBatch, JsValue> {
        let array = Int32Array::from(data);

        let batch = record_batch::RecordBatch::try_new(
            Arc::new(datatypes::Schema::new(vec![datatypes::Field::new(
                "values",
                datatypes::DataType::Int32,
                nullable,
            )])),
            vec![Arc::new(array)],
        )
        .expect("Could not create batch");

        Ok(Self { batch })
    }

    pub fn schema(&mut self) -> JsValue {
        // serialize schema as JSON, only do this because schema is small
        JsValue::from_serde(&self.batch.schema()).unwrap()
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let mut file = Vec::new();
        let writer = ipc::writer::FileWriter::try_new(&mut file, &self.batch.schema());
        writer.unwrap().write(&self.batch).unwrap();

        file
    }

    #[wasm_bindgen(js_name = numRows)]
    pub fn num_rows(&mut self) -> usize {
        self.batch.num_rows()
    }

    #[wasm_bindgen(js_name = numColumns)]
    pub fn num_columns(&mut self) -> usize {
        self.batch.num_columns()
    }

    #[wasm_bindgen(js_name = nullCount)]
    pub fn null_count(&mut self) -> usize {
        self.batch.column(0).null_count()
    }

    pub fn value(&mut self, index: usize) -> i32 {
        let array = self.batch.column(0).as_any().downcast_ref::<Int32Array>().expect("Failed to downcast");
        array.value(index)
    }

    pub fn sum(&mut self) -> i32 {
        let array = self.batch.column(0).as_any().downcast_ref::<Int32Array>().expect("Failed to downcast");
        array.iter().filter_map(|e| e).sum()
    }
}

#[wasm_bindgen]
pub struct Table {
    batches: Vec<record_batch::RecordBatch>,
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(catch)]
    pub fn from(contents: &[u8]) -> Result<Table, JsValue> {
        set_panic_hook();

        let cursor = Cursor::new(contents);
        let mut reader = ipc::reader::FileReader::try_new(cursor).expect("Could not read buffer");

        let mut table = Self { batches: vec![] };
        let batch = reader.next().unwrap().unwrap();
        table.batches.push(batch);

        Ok(table)
    }

    #[wasm_bindgen(js_name = numRows)]
    pub fn num_rows(&mut self) -> usize {
        self.batches[0].num_rows()
    }
}
