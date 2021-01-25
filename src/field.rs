use wasm_bindgen::prelude::*;
use arrow::datatypes;

#[wasm_bindgen]
pub struct Field {
    field: datatypes::Field,
}

#[wasm_bindgen]
impl Field {
    pub fn name(&self) -> String {
        self.field.name().clone()
    }

    #[wasm_bindgen(js_name = dataType)]
    pub fn data_type(&self) -> JsValue {
        let json = self.field.data_type().to_json();
        let dtype = json.as_object().unwrap();
        JsValue::from_serde(dtype).unwrap()
    }
}

impl Field {
    pub fn from(field: datatypes::Field) -> Field {
        Field {field}
    }
}