extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

mod vector;
mod vector_pool;

#[inline]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}