use arrow::record_batch;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RecordBatch(record_batch::RecordBatch);

#[wasm_bindgen]
impl RecordBatch {
    #[wasm_bindgen(js_name = numRows)]
    #[wasm_bindgen(getter)]
    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    #[wasm_bindgen(js_name = numColumns)]
    #[wasm_bindgen(getter)]
    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }

    /// Returns the schema of the record batches.
    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> crate::schema::Schema {
        crate::schema::Schema::new(self.0.schema())
    }

    /// Get a column's vector by index.
    pub fn column(&self, index: usize) -> crate::vector::Vector {
        crate::vector::Vector::new(self.0.column(index).clone())
    }

    /// Get a column's vector by name.
    #[wasm_bindgen(js_name = columnWithName)]
    pub fn column_with_name(&self, name: &str) -> Result<crate::vector::Vector, JsValue> {
        let index = self
            .0
            .schema()
            .index_of(name)
            .expect("Could not find field in schema");
        Ok(self.column(index))
    }
}

impl RecordBatch {
    pub fn new(batch: record_batch::RecordBatch) -> RecordBatch {
        RecordBatch { 0: batch }
    }
}
