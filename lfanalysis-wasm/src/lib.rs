mod interface_types;

use std::process::Command;

use lfdeploy::CommandDescription;
use wasm_bindgen::prelude::*;

use crate::interface_types::JsBindingCommand;

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn federate_names(lf_file: &str) -> Vec<String> {
    lfanalysis::federate_names(lf_file)
}

#[wasm_bindgen]
pub fn deploy_command(lf_file: &str) -> JsBindingCommand {
    lfdeploy::deploy_command(lf_file).into()
}

#[wasm_bindgen]
pub fn main_reactor_name(lf_file: &str) -> Option<String> {
    lfanalysis::main_reactor_name(lf_file)
}

#[wasm_bindgen]
pub fn path(lf_file: &str) -> String {
    lfanalysis::path(lf_file).to_string_lossy().to_string()
}
