use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> Vec<String> {
    vec![name.to_string(), "Hello, lfanalysis-wasm!".to_string()]
}
