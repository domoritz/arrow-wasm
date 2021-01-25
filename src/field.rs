use arrow::datatypes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Field(datatypes::Field);

impl_to_json!(Field);
impl_to_string!(Field);

#[wasm_bindgen]
impl Field {
    pub fn name(&self) -> String {
        self.0.name().clone()
    }

    #[wasm_bindgen(js_name = isNullable)]
    pub fn is_nullable(&self) -> bool {
        self.0.is_nullable()
    }

    #[wasm_bindgen(js_name = dictId)]
    pub fn dict_id(&self) -> Option<i64> {
        self.0.dict_id()
    }

    #[wasm_bindgen(js_name = dictIsOrdered)]
    pub fn dict_is_ordered(&self) -> Option<bool> {
        self.0.dict_is_ordered()
    }

    #[wasm_bindgen(js_name = dataType)]
    pub fn data_type(&self) -> JsValue {
        // TODO: return DataType type
        JsValue::from_serde(&self.0.data_type().to_json()).unwrap()
    }
}

impl Field {
    pub fn new(field: datatypes::Field) -> Field {
        Field { 0: field }
    }
}
