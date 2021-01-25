use arrow::array::{Array, ArrayRef, BooleanArray, Float32Array, Int32Array, PrimitiveArray};
use arrow::compute::kernels;
use arrow::datatypes::*;
use arrow::util::bit_util;
use wasm_bindgen::prelude::*;

macro_rules! impl_vector {
    ($struct_name:ident) => {
        impl_to_string!($struct_name);

        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            #[wasm_bindgen(getter)]
            #[inline]
            pub fn length(&self) -> usize {
                self.0.len()
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
    ($struct_name:ident; $N: ty) => {
        impl_vector!($struct_name);

        #[wasm_bindgen]
        impl $struct_name {
            pub fn get(&self, index: usize) -> $N {
                self.0.value(index)
            }
        }
    };
}

macro_rules! number_vector {
    ($struct_name:ident; $T:ty) => {
        number_vector!($struct_name; PrimitiveArray<$T>; <$T as ArrowPrimitiveType>::Native);
    };
    ($struct_name:ident; $A:ty; $N: ty) => {
        #[wasm_bindgen]
        pub struct $struct_name($A);

        impl_vector!($struct_name; $N);

        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            #[wasm_bindgen(catch)]
            pub fn from(data: Vec<$N>) -> Result<$struct_name, JsValue> {
                let array = <$A>::from(data);
                Ok(Self(array))
            }

            #[wasm_bindgen(js_name = toArray)]
            #[inline]
            pub fn to_array(&self) -> Vec<$N> {
                self.0.values().to_vec()
            }

            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> JsValue {
                JsValue::from_serde(&self.0.values()).unwrap()
            }

            // aggregations
            // TODO: think about how to support kernels

            #[wasm_bindgen(catch)]
            pub fn sum(&self) -> Option<$N>  {
                kernels::aggregate::sum(&self.0)
            }
        }

        impl $struct_name {
            pub fn new(vector: $A) -> $struct_name {
                $struct_name { 0: vector }
            }
        }
    };
}

// Number vectors

number_vector!(Int8Vector; Int8Type);
number_vector!(Int16Vector; Int16Type);
number_vector!(Int32Vector; Int32Type);
number_vector!(Int64Vector; Int64Type);
number_vector!(UInt8Vector; UInt8Type);
number_vector!(UInt16Vector; UInt16Type);
number_vector!(UInt32Vector; UInt32Type);
number_vector!(UInt64Vector; UInt64Type);
number_vector!(Float32Vector; Float32Type);
number_vector!(Float64Vector; Float64Type);

// Boolean vector (because boolean arrays are special)

#[wasm_bindgen]
pub struct BooleanVector(BooleanArray);

impl_vector!(BooleanVector; bool);

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

// Generic vector

#[wasm_bindgen]
pub struct Vector(ArrayRef);

#[wasm_bindgen]
impl Vector {
    pub fn as_i32_vector(&self) -> Result<Int32Vector, JsValue> {
        // TODO: This feels wrong somehow. Should we keep a reference in the Vector structs instead (e.g. Rc or Arc)?

        // let array = self
        //     .0
        //     .as_any()
        //     .downcast_ref::<arrow::array::Int32Array>()
        //     .expect("Failed to downcast");

        Ok(Int32Vector::new(Int32Array::from(self.0.data())))
    }

    pub fn as_f32_vector(&self) -> Result<Float32Vector, JsValue> {
        Ok(Float32Vector::new(Float32Array::from(self.0.data())))
    }
}

impl Vector {
    pub fn new(vector: ArrayRef) -> Vector {
        Vector { 0: vector }
    }
}

impl_vector!(Vector);
