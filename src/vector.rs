use arrow::array::{Array, BooleanArray, PrimitiveArray};
use arrow::datatypes::*;
use arrow::util::bit_util;
use wasm_bindgen::prelude::*;

macro_rules! impl_basic_vector {
    ($struct_name:ident; $N: ty) => {
        impl_to_string!($struct_name);

        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.0.len()
            }

            pub fn get(&self, index: usize) -> $N {
                self.0.value(index)
            }

            #[wasm_bindgen(js_name = isValid)]
            pub fn is_valid(&self, index: usize) -> bool {
                self.0.is_valid(index)
            }

            #[wasm_bindgen(js_name = isNull)]
            pub fn is_null(&self, index: usize) -> bool {
                self.0.is_null(index)
            }

            #[wasm_bindgen(js_name = isEmpty)]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[wasm_bindgen(js_name = nullCount)]
            pub fn null_count(&self) -> usize {
                self.0.null_count()
            }
        }
    };
}

macro_rules! declare_vector {
    ($struct_name:ident; $T:ty) => {
        declare_vector!($struct_name; PrimitiveArray<$T>; <$T as ArrowPrimitiveType>::Native);
    };
    ($struct_name:ident; $A:ty; $N: ty) => {
        #[wasm_bindgen]
        pub struct $struct_name($A);

        impl_basic_vector!($struct_name; $N);

        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            #[wasm_bindgen(catch)]
            pub fn from(data: Vec<$N>) -> Result<$struct_name, JsValue> {
                let array = <$A>::from(data);
                Ok(Self(array))
            }

            #[wasm_bindgen(js_name = toArray)]
            pub fn to_array(&self) -> Vec<$N> {
                self.0.values().to_vec()
            }

            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> JsValue {
                JsValue::from_serde(&self.0.values()).unwrap()
            }
        }
    };
}

declare_vector!(Int8Vector; Int8Type);
declare_vector!(Int16Vector; Int16Type);
declare_vector!(Int32Vector; Int32Type);
declare_vector!(Int64Vector; Int64Type);
declare_vector!(UInt8Vector; UInt8Type);
declare_vector!(UInt16Vector; UInt16Type);
declare_vector!(UInt32Vector; UInt32Type);
declare_vector!(UInt64Vector; UInt64Type);
declare_vector!(Float32Vector; Float32Type);
declare_vector!(Float64Vector; Float64Type);

// Boolean arrays are a bit special

#[wasm_bindgen]
pub struct BooleanVector(BooleanArray);

impl_basic_vector!(BooleanVector; bool);

#[wasm_bindgen]
impl BooleanVector {
    #[wasm_bindgen(catch)]
    pub fn from(data: Vec<u8>, length: usize) -> Result<BooleanVector, JsValue> {
        let vector: Vec<bool> = (0..length).map(|i| bit_util::get_bit(&data, i)).collect();
        Ok(Self(BooleanArray::from(vector)))
    }

    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Vec<u8> {
        self.0.values().to_vec()
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        let vector: Vec<bool> = (0..self.length()).map(|i| self.get(i)).collect();
        JsValue::from_serde(&vector).unwrap()
    }
}
