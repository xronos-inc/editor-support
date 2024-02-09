use std::process::Command;

use lfdeploy::CommandDescription;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn federate_names(lf_file: &str) -> Vec<String> {
    lfanalysis::federate_names(lf_file)
}

#[wasm_bindgen]
pub fn deploy_command(lf_file: &str) -> JsBindingCommand {
    console_error_panic_hook::set_once();
    lfdeploy::deploy_command(lf_file).into()
}

#[wasm_bindgen(getter_with_clone)]
pub struct JsBindingCommand {
    pub cwd: String,
    pub command: String,
    pub args: Vec<String>,
}

impl From<CommandDescription> for JsBindingCommand {
    fn from(value: CommandDescription) -> Self {
        let CommandDescription { cwd, command, args } = value;
        JsBindingCommand { cwd, command, args }
    }
}
