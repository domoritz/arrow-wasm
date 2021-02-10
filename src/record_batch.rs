use crate::{schema, vector};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RecordBatch(arrow::record_batch::RecordBatch);

#[wasm_bindgen]
impl RecordBatch {
    #[wasm_bindgen(getter, js_name = numRows)]
    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    #[wasm_bindgen(getter, js_name = numColumns)]
    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }

    /// Returns the schema of the record batches.
    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> schema::Schema {
        schema::Schema::new(self.0.schema())
    }

    /// Get a column's vector by index.
    pub fn column(&self, index: usize) -> vector::Vector {
        vector::Vector::new(self.0.column(index).clone())
    }

    /// Get all columns in the record batch.
    // TODO: specify that the output type is Array<Vector>, not Array<any>
    #[wasm_bindgen(getter)]
    pub fn columns(&self) -> Array {
        let vectors: Vec<vector::Vector> = self
            .0
            .columns()
            .into_iter()
            .map(|column| vector::Vector::new(column.clone()))
            .collect();

        vectors.into_iter().map(JsValue::from).collect()
    }

    /// Get a column's vector by name.
    #[wasm_bindgen(js_name = columnWithName)]
    pub fn column_with_name(&self, name: &str) -> Result<crate::vector::Vector, JsValue> {
        match self.0.schema().index_of(name) {
            Ok(index) => Ok(self.column(index)),
            Err(error) => Err(format!("{}", error).into()),
        }
    }
}

impl RecordBatch {
    pub fn new(batch: arrow::record_batch::RecordBatch) -> RecordBatch {
        RecordBatch { 0: batch }
    }
}
