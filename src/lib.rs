#[macro_use]
mod utils;

mod datatype;
mod field;
mod record_batch;
mod schema;
mod table;
mod vector;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
