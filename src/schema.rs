use arrow::datatypes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Schema(datatypes::SchemaRef);

impl_to_json!(Schema);
impl_to_string!(Schema);

#[wasm_bindgen]
impl Schema {
    pub fn field(&self, i: usize) -> crate::field::Field {
        crate::field::Field::new(self.0.field(i).clone())
    }

    #[wasm_bindgen(js_name = fieldWithName)]
    pub fn field_with_name(&self, name: &str) -> Result<crate::field::Field, JsValue> {
        let field = self.0.field_with_name(name).expect("Could not find field");
        Ok(crate::field::Field::new(field.clone()))
    }

    pub fn metadata(&self) -> JsValue {
        JsValue::from_serde(&self.0.metadata()).unwrap()
    }
}

impl Schema {
    pub fn new(schema: datatypes::SchemaRef) -> Schema {
        Schema { 0: schema }
    }
}
