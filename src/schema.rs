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

    #[wasm_bindgen(getter)]
    pub fn fields(&self) -> JsValue {
        JsValue::from_serde(&self.0.fields()).unwrap()
    }

    /// Look up a column by name and return a immutable reference to the column along with its index.
    #[wasm_bindgen(js_name = columnWithName)]
    pub fn column_with_name(&self, name: &str) -> Result<JsValue, JsValue> {
        let column = self.0.column_with_name(name).expect("Could not find field");
        Ok(JsValue::from_serde(&column).unwrap())
    }

    /// Find the index of the column with the given name.
    #[wasm_bindgen(js_name = indexOf)]
    pub fn index_of(&self, name: &str) -> Result<usize, JsValue> {
        Ok(self.0.index_of(name).expect("Could not find field"))
    }

    #[wasm_bindgen(js_name = fieldWithName)]
    pub fn field_with_name(&self, name: &str) -> Result<crate::field::Field, JsValue> {
        let field = self.0.field_with_name(name).expect("Could not find field");
        Ok(crate::field::Field::new(field.clone()))
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> JsValue {
        JsValue::from_serde(&self.0.metadata()).unwrap()
    }
}

impl Schema {
    pub fn new(schema: datatypes::SchemaRef) -> Schema {
        Schema { 0: schema }
    }
}
