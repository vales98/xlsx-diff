use serde_json;
use std::error::Error;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn diff(old_file: &[u8], new_file: &[u8], raw_data: bool) -> String {
    match xlsx_diff::diff_xlsx_from_u8(old_file, new_file, raw_data) {
        Ok(res) => return serde_json::to_string(&res).unwrap(),
        Err(e) => return format!("Error: {}", e.to_string()),
    }
}
#[wasm_bindgen]
pub fn load_workbook(file: &[u8]) -> String {
    match xlsx_diff::core::parse::load_by_rs(file) {
        Ok(res) => return serde_json::to_string(&res).unwrap(),
        Err(e) => return format!("Error: {}", e.to_string()),
    }
}