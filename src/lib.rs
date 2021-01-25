mod utils;
mod vector;
mod field;

use arrow;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn test() -> field::Field {
    crate::utils::set_panic_hook();

    let field = arrow::datatypes::Field::new("c1", arrow::datatypes::DataType::Int64, false);
    field::Field::new(field)
}
