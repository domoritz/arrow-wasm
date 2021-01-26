use arrow::datatypes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DataType(datatypes::DataType);

impl_to_json!(DataType);
impl_to_string!(DataType);

impl DataType {
    pub fn new(datatype: datatypes::DataType) -> DataType {
        DataType { 0: datatype }
    }
}
