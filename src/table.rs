use std::io::Cursor;

use arrow::ipc;
use arrow::{datatypes, error::ArrowError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Table {
    schema: datatypes::SchemaRef,

    record_batches: Vec<arrow::record_batch::RecordBatch>,
}

#[wasm_bindgen]
impl Table {
    /// Returns the schema of the record batches.
    #[wasm_bindgen(getter)]
    pub fn schema(&self) -> crate::schema::Schema {
        crate::schema::Schema::new(self.schema.clone())
    }

    #[wasm_bindgen(js_name = recordBatch)]
    pub fn record_batch(&self, index: usize) -> Option<crate::record_batch::RecordBatch> {
        let batch = self.record_batches.get(index)?;

        Some(crate::record_batch::RecordBatch::new(batch.clone()))
    }

    /// Return the number of batches in the file
    #[wasm_bindgen(getter, js_name = numBatches)]
    pub fn num_batches(&self) -> usize {
        self.record_batches.len()
    }

    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(contents: js_sys::Uint8Array) -> Result<Table, JsValue> {
        let vec = contents.to_vec();
        let cursor: Cursor<&[u8]> = std::io::Cursor::new(vec.as_slice());
        let reader = match arrow::ipc::reader::FileReader::try_new(cursor) {
            Ok(reader) => reader,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let schema = reader.schema();
        match reader.collect() {
            Ok(record_batches) => Ok(Table {
                schema,
                record_batches,
            }),
            Err(error) => Err(format!("{}", error).into()),
        }
    }

    pub fn from(contents: &[u8]) -> Result<Table, JsValue> {
        let cursor = std::io::Cursor::new(contents);
        let reader = match arrow::ipc::reader::FileReader::try_new(cursor) {
            Ok(reader) => reader,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let schema = reader.schema();
        match reader.collect() {
            Ok(record_batches) => Ok(Table {
                schema,
                record_batches,
            }),
            Err(error) => Err(format!("{}", error).into()),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, JsValue> {
        let mut file = Vec::new();
        {
            let mut writer = ipc::writer::StreamWriter::try_new(&mut file, &self.schema).unwrap();

            let result: Result<Vec<()>, ArrowError> = self
                .record_batches
                .iter()
                .map(|batch| writer.write(batch))
                .collect();
            if let Err(error) = result {
                return Err(format!("{}", error).into());
            }

            if let Err(error) = writer.finish() {
                return Err(format!("{}", error).into());
            }
        };

        Ok(file)
    }
}
