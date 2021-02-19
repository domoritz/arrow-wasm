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

    /// Create a table from IPC bytes. Use `fromWasmUint8Array` to avoid memory copies.
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

    /// Create a table from a pre-initialized buffer. The memory is passed without a copy.
    #[wasm_bindgen(js_name = fromWasmUint8Array)]
    pub fn from_wasm(data: &WasmUint8Array) -> Result<Table, JsValue> {
        Table::from(&data.0)
    }

    pub fn serialize(&self) -> Result<js_sys::Uint8Array, JsValue> {
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

        Ok(unsafe { js_sys::Uint8Array::view(&file) })
    }
}

#[wasm_bindgen]
pub struct WasmUint8Array(Vec<u8>);

#[wasm_bindgen]
impl WasmUint8Array {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        let buffer = vec![0; size];
        Self { 0: buffer }
    }

    #[wasm_bindgen(getter)]
    pub fn view(&mut self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view_mut_raw(self.0.as_mut_ptr(), self.0.len()) }
    }
}
