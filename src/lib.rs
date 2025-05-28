mod ast;
mod parser;
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

pub use parser::parse_liquid;
