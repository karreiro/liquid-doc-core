use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalculationResult {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

impl CalculationResult {
    pub fn new(a: i32, b: i32, result: i32) -> Self {
        Self { a, b, result }
    }
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> JsValue {
    let result = CalculationResult {
        a,
        b,
        result: a + b,
    };
    serde_wasm_bindgen::to_value(&result).expect("The struct must be serializable")
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
