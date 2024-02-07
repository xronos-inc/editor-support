use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn federate_names(lf_file: String) -> Vec<String> {
    lfanalysis::federate_names(lf_file)
}
