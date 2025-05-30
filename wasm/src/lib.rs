use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    match liquid_doc_parser::parse_liquid_string(input, None) {
        Some(ast) => {
            serde_wasm_bindgen::to_value(&ast).expect("The LiquidAst was not in the correct format")
        }
        None => JsValue::NULL,
    }
}

// Re-export the parser function for other Rust crates
pub use liquid_doc_parser::parse_liquid_string;
