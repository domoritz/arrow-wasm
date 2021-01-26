use arrow::datatypes;
use arrow::ipc;
use std::io::Cursor;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Table {
    schema: datatypes::SchemaRef,

    record_batches: Vec<arrow::record_batch::RecordBatch>,
}

#[wasm_bindgen]
impl Table {
    /// Returns the schema of the record batches.
    pub fn schema(&self) -> crate::schema::Schema {
        crate::schema::Schema::new(self.schema.clone())
    }

    #[wasm_bindgen(js_name = recordBatch)]
    pub fn record_batch(&self, index: usize) -> Option<crate::record_batch::RecordBatch> {
        let batch = self.record_batches.get(index)?;

        Some(crate::record_batch::RecordBatch::new(batch.clone()))
    }

    /// Return the number of batches in the file
    #[wasm_bindgen(js_name = numBatches)]
    pub fn num_batches(&self) -> usize {
        self.record_batches.len()
    }

    pub fn from(contents: &[u8]) -> Result<Table, JsValue> {
        let cursor = Cursor::new(contents);
        let mut reader =
            arrow::ipc::reader::FileReader::try_new(cursor).expect("Could not read ipc");

        let schema = reader.schema();
        let record_batches = (0..reader.num_batches())
            .map(|_| reader.next().unwrap().unwrap())
            .collect();

        Ok(Table {
            schema,
            record_batches,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, JsValue> {
        let mut file = Vec::new();
        let mut writer = ipc::writer::FileWriter::try_new(&mut file, &self.schema).unwrap();
        self.record_batches
            .iter()
            .for_each(|batch| writer.write(batch).expect("Could not write batch"));
        writer.finish().expect("Could not finish writer");
        drop(writer);

        Ok(file)
    }
}
