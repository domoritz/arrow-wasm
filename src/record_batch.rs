use arrow::datatypes;
use arrow::record_batch;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RecordBatch(record_batch::RecordBatch);

#[wasm_bindgen]
impl RecordBatch {
    #[wasm_bindgen(js_name = numRows)]
    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    #[wasm_bindgen(js_name = numColumns)]
    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }

    pub fn schema(&self) -> crate::schema::Schema {
        crate::schema::Schema::new(self.0.schema())
    }

    pub fn column(&self, index: usize) -> crate::vector::Vector {
        crate::vector::Vector::new(self.0.column(index).clone())
    }
}

impl RecordBatch {
    pub fn new(batch: record_batch::RecordBatch) -> RecordBatch {
        RecordBatch { 0: batch }
    }
}
