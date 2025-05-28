use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
        Self {
            a,
            b,
            result,
        }
    }
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> JsValue {
    let var_name = CalculationResult{
        a: a,
        b: b,
        result: a + b
    };
    serde_wasm_bindgen::to_value(&var_name).unwrap()

}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("WebAssembly Calculator module loaded! 🦀");
}
