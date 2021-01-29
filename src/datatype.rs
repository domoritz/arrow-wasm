use crate::{impl_to_json, impl_to_string};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DataType(arrow::datatypes::DataType);

impl_to_json!(DataType);
impl_to_string!(DataType);

impl DataType {
    pub fn new(datatype: arrow::datatypes::DataType) -> DataType {
        DataType { 0: datatype }
    }
}
