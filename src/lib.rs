use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn process_str(input: String) -> String {
    input
}

#[wasm_bindgen]
pub fn help_text() -> String {
    "Here's some help text to get started".to_string()
}
