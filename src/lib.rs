mod utils;
use arrow::array::Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Int32Vector {
    array: arrow::array::Int32Array,
}

#[wasm_bindgen]
impl Int32Vector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Int32Vector {
        let array: Vec<i32> = Vec::new();
        Int32Vector {
            array: arrow::array::Int32Array::from(array),
        }
    }

    #[wasm_bindgen(catch)]
    pub fn from(data: Vec<i32>) -> Result<Int32Vector, JsValue> {
        let array = arrow::array::Int32Array::from(data);
        Ok(Self { array })
    }

    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.array.len()
    }

    pub fn get(&self, index: usize) -> i32 {
        self.array.value(index)
    }

    #[wasm_bindgen(js_name = isValid)]
    pub fn is_valid(&self, index: usize) -> bool {
        self.array.is_valid(index)
    }

    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self, index: usize) -> bool {
        self.array.is_null(index)
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    #[wasm_bindgen(js_name = nullCount)]
    pub fn null_count(&self) -> usize {
        self.array.null_count()
    }

    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Vec<i32> {
        self.array.values().to_vec()
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&self.array.values()).unwrap()
    }
}
