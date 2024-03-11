mod interface_types;

use std::{io, path::Path};

use wasm_bindgen::prelude::*;

use crate::interface_types::JsBindingCommand;

/// ## Panics
/// Panics if js_file_system_read does not take in a string (representing a path) and return a
/// string (representing the contents of the file at that path) or undefined (if the file does not
/// exist).
fn do_fs_read(path: &Path, js_file_system_read: &js_sys::Function) -> io::Result<String> {
    js_file_system_read
        .call1(&JsValue::null(), &JsValue::from_str(path.to_str().unwrap()))
        .map_or_else(
            |err| {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("error occurred during file system read: {:#?}", err),
                ))
            },
            |ok| {
                if let Some(contents) = ok.as_string() {
                    Ok(Some(contents))
                } else if ok.is_undefined() {
                    Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("No file contents found at {}", path.to_string_lossy()),
                    ))
                } else {
                    Ok(None)
                }
            },
        )
        .transpose()
        .unwrap_or_else(|| {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "JS closure js_file_system_read({}) did not return a string or undefined",
                    path.to_string_lossy()
                ),
            ))
        })
}

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
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
pub fn lfc_json(
    lf_file: &str,
    js_file_system_read: &js_sys::Function,
) -> Result<Option<String>, String> {
    lfbuildconfig::lfc_json(
        Path::new(lf_file),
        Box::new(|p: &Path| do_fs_read(p, js_file_system_read)),
    )
    .map_err(|ioerr| format!("an I/O exception occurred: {}", ioerr))
}

#[wasm_bindgen]
pub fn path(lf_file: &str) -> String {
    lfanalysis::path(lf_file).to_string_lossy().to_string()
}
