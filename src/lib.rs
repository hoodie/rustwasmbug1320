#[macro_use]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(msg: &str);
}

#[wasm_bindgen]
pub fn run_test() {
    debug(env!("RUSTC"));

    let cols = 7;
    let mut current_x_offset = -1f64;
    let mut prev_x_offset = -1f64;

    for yi in 0..=3 {
        let xi = 0;

        let x_offset = if yi % 2 == 1 { 23f64 } else { 0f64 };
        format!("x_offset: {}, yi: {}, cols: {}", x_offset, 42, cols); // FIXME: change 42 to `yi` and it no longer panics
        current_x_offset = x_offset;

        if yi % 2 == 0 && xi == 0 {
            format!("{:?}", ());
        }

        format!("{:?}", ());

        if yi % 2 == 1 && xi == cols - 1 {
            format!("{:?}", ());
        }

        if prev_x_offset == current_x_offset {
            format!("not altering");
            panic!("done");
        }
        prev_x_offset = current_x_offset;
    }
}
