use crate::impl_to_string;
use arrow::array::{Array, ArrayRef, BooleanArray, PrimitiveArray};
use arrow::compute::kernels;
use arrow::datatypes::*;
use arrow::util::bit_util;
use paste::paste;
use wasm_bindgen::prelude::*;

macro_rules! impl_vector {
    ($struct_name:ident) => {
        impl_to_string!($struct_name);

        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            /// Get the length of the vector.
            #[wasm_bindgen(getter)]
            #[inline]
            pub fn length(&self) -> usize {
                self.0.len()
            }

            /// Returns whether the element at `index` is not null.
            #[wasm_bindgen(js_name = isValid)]
            pub fn is_valid(&self, index: usize) -> bool {
                self.0.is_valid(index)
            }

            /// Returns whether the element at `index` is null.
            #[wasm_bindgen(js_name = isNull)]
            pub fn is_null(&self, index: usize) -> bool {
                self.0.is_null(index)
            }

            /// Returns whether this vector is empty.
            #[wasm_bindgen(getter, js_name = isEmpty)]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns the total number of null values in this vector.
            #[wasm_bindgen(getter, js_name = nullCount)]
            pub fn null_count(&self) -> usize {
                self.0.null_count()
            }
        }
    };
    ($struct_name:ident; $N: ty) => {
        impl_vector!($struct_name);

        #[wasm_bindgen]
        impl $struct_name {
            /// Returns the primitive value at `index`.
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
            pub fn from(data: Vec<$N>) -> $struct_name {
                Self(<$A>::from(data))
            }

            /// Returns the contents of the vector as a typed array.
            #[wasm_bindgen(js_name = toArray)]
            #[inline]
            pub fn to_array(&self) -> Vec<$N> {
                // TODO: try to avoid this copy by writing into js_sys?
                self.0.values().to_vec()
            }

            /// Returns the contents of the vector as a JSON array.
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> JsValue {
                JsValue::from_serde(&self.0.values()).unwrap()
            }

            // aggregations
            // TODO: think about how to support kernels

            pub fn sum(&self) -> Option<$N>  {
                kernels::aggregate::sum(&self.0)
            }

            pub fn min(&self) -> Option<$N>  {
                kernels::aggregate::min(&self.0)
            }

            pub fn max(&self) -> Option<$N>  {
                kernels::aggregate::max(&self.0)
            }
        }

        paste! {
            #[wasm_bindgen]
            impl Vector {
                #[wasm_bindgen(js_name = as$struct_name)]
                #[doc = "Cast Vector as a `" $struct_name "`."]
                pub fn [<as$struct_name:snake>](&self) -> $struct_name {
                    $struct_name(<$A>::from(self.0.data()))
                }
            }
        }
    };
}

macro_rules! number_vector_view {
    ($struct_name:ident; $T:ty) => {
        /// Creates a JS typed array which is a view into wasm's linear memory at the slice specified.
        /// This function returns a new typed array which is a view into wasm's memory.
        /// This view does not copy the underlying data.
        #[wasm_bindgen]
        #[allow(clippy::new_without_default)]
        impl $struct_name {
            pub fn view(&self) -> $T {
                unsafe { <$T>::view(self.0.values()) }
            }
        }
    };
}

// Generic vector

#[wasm_bindgen]
pub struct Vector(ArrayRef);

impl Vector {
    pub fn new(vector: ArrayRef) -> Vector {
        Vector { 0: vector }
    }
}

impl_vector!(Vector);

// Number vectors

number_vector!(Int8Vector; Int8Type);
number_vector!(Int16Vector; Int16Type);
number_vector!(Int32Vector; Int32Type);
number_vector!(Int64Vector; Int64Type);
number_vector!(Uint8Vector; UInt8Type);
number_vector!(Uint16Vector; UInt16Type);
number_vector!(Uint32Vector; UInt32Type);
number_vector!(Uint64Vector; UInt64Type);
number_vector!(Float32Vector; Float32Type);
number_vector!(Float64Vector; Float64Type);

number_vector_view!(Int8Vector; js_sys::Int8Array);
number_vector_view!(Int16Vector; js_sys::Int16Array);
number_vector_view!(Int32Vector; js_sys::Int32Array);
number_vector_view!(Uint8Vector; js_sys::Uint8Array);
number_vector_view!(Uint16Vector; js_sys::Uint16Array);
number_vector_view!(Uint32Vector; js_sys::Uint32Array);
number_vector_view!(Float32Vector; js_sys::Float32Array);
number_vector_view!(Float64Vector; js_sys::Float64Array);

// Boolean vector (because boolean arrays are special)

#[wasm_bindgen]
pub struct BooleanVector(BooleanArray);

impl_vector!(BooleanVector; bool);

#[wasm_bindgen]
impl BooleanVector {
    pub fn from(data: &[u8], length: usize) -> BooleanVector {
        let vector: Vec<bool> = (0..length).map(|i| bit_util::get_bit(&data, i)).collect();
        Self(BooleanArray::from(vector))
    }

    /// Returns a `Buffer` holding all the values of this array.
    ///
    /// Note this doesn't take the offset of this array into account.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Vec<u8> {
        self.0.values().to_vec()
    }

    /// Creates a JS typed array which is a view into wasm's linear memory at the slice specified.
    /// This function returns a new typed array which is a view into wasm's memory. This view does not copy the underlying data.
    pub fn view(&self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view(self.0.values().as_slice()) }
    }

    /// Returns the contents of the vector as a JSON array.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        let vector: Vec<Option<bool>> = self.0.iter().collect();
        JsValue::from_serde(&vector).unwrap()
    }
}

#[wasm_bindgen]
impl Vector {
    /// Cast Vector as a `BooleanVector`.
    #[wasm_bindgen(js_name = asBooleanVector)]
    pub fn as_boolean_vector(&self) -> BooleanVector {
        BooleanVector(BooleanArray::from(self.0.data()))
    }
}
