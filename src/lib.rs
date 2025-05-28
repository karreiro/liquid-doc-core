use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
        Self { a, b, result }
    }
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> JsValue {
    let result = CalculationResult {
        a: a,
        b: b,
        result: a + b,
    };
    serde_wasm_bindgen::to_value(&result).expect("The struct must be serializable")
}

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

pub fn visit(builder: &mut LiquidASTBuilder, pair: pest::iterators::Pair<Rule>) {
    match pair.as_rule() {
        Rule::ImplicitDescription => {
            let descriptionContent = pair.into_inner();
        }
        Rule::LiquidDocNode => {
            let mut content = pair.into_inner();
            match content.next().unwrap().as_rule() {
                Rule::paramNode => {}
                Rule::exampleNode => {}
                Rule::descriptionNode => {}
                Rule::promptNode => {}
                Rule::fallbackNode => {}
                _ => {}
            }
        }
        Rule::TextNode => (),
        Rule::EOI => (),
        _ => unreachable!(),
    }
}

fn parse_liquid_string(pair: &str) -> Option<LiquidAst> {
    let text = LiquidParser::parse(Rule::LiquidDocNode, pair)
        .map_err(|e| console_log!("Parsing error: {}", e))
        .ok()?;

    let mut builder = LiquidASTBuilder {};
    for pair in text {
        visit(&mut builder, pair);
    }

    None
}

#[derive(Serialize, Debug, Clone)]
pub struct LiquidAst {}

pub struct LiquidASTBuilder {}

impl LiquidASTBuilder {
    pub fn new() -> Self {
        LiquidASTBuilder {}
    }
    pub fn build(self) -> LiquidAst {
        LiquidAst {}
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_liquid() {
//         let input = "\n        {% doc -%}\n        @param requiredParamWithNoType\n        @param {String} paramWithDescription - param with description and `punctation`. This is still a valid param description.\n        @param {String} paramWithNoDescription\n        @param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description\n        @param [optionalParameterWithDescription] - optional parameter description\n        @param {String} [optionalParameterWithType]\n        @unsupported this node falls back to a text node\n        {%- enddoc %}\n      ";
//         let result = parse_liquid(input);
//         assert!(result.is_empty());
//     }
// }
