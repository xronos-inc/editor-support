use lfdeploy::CommandDescription;
use wasm_bindgen::prelude::*;

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
