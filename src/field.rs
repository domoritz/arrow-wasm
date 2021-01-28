use arrow::datatypes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Field(datatypes::Field);

impl_to_json!(Field);
impl_to_string!(Field);

#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name().clone()
    }

    #[wasm_bindgen(getter, js_name = isNullable)]
    pub fn is_nullable(&self) -> bool {
        self.0.is_nullable()
    }

    #[wasm_bindgen(getter, js_name = dictId)]
    pub fn dict_id(&self) -> Option<i64> {
        self.0.dict_id()
    }

    #[wasm_bindgen(getter, js_name = dictIsOrdered)]
    pub fn dict_is_ordered(&self) -> Option<bool> {
        self.0.dict_is_ordered()
    }

    #[wasm_bindgen(getter ,js_name = dataType)]
    pub fn data_type(&self) -> crate::datatype::DataType {
        crate::datatype::DataType::new(self.0.data_type().clone())
    }
}

impl Field {
    pub fn new(field: datatypes::Field) -> Field {
        Field { 0: field }
    }
}
