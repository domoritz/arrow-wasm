use crate::{impl_to_json, impl_to_string};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DataType(arrow::datatypes::DataType);

impl_to_json!(DataType);
impl_to_string!(DataType);

#[wasm_bindgen]
impl DataType {
    /// Parse a `Field` definition from a JSON representation.
    pub fn from(json: &JsValue) -> DataType {
        let value = json.into_serde().unwrap();
        let data_type: arrow::datatypes::DataType = arrow::datatypes::DataType::from(value);
        DataType { 0: data_type }
    }
}

impl DataType {
    pub fn new(datatype: arrow::datatypes::DataType) -> DataType {
        DataType { 0: datatype }
    }
}
