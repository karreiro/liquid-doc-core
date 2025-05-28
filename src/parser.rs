use crate::ast::{LiquidDocParamNode, *};
use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

pub fn visit(builder: &mut LiquidAST, pair: pest::iterators::Pair<Rule>) {
    match pair.as_rule() {
        Rule::ImplicitDescription => {
            let _description_content = pair.into_inner();
            // Process description content
        }
        Rule::LiquidDocNode => {
            let mut content = pair.into_inner();
            let next = content.next().unwrap();
            match next.as_rule() {
                Rule::paramNode => {
                    let node = LiquidDocParamNode::new(&next);

                    builder.add_node(LiquidNode::LiquidDocParamNode(node));
                }
                Rule::exampleNode => {
                    // Process example node
                }
                Rule::descriptionNode => {
                    // Process description node
                }
                Rule::promptNode => {
                    // Process prompt node
                }
                Rule::fallbackNode => {
                    // Process fallback node
                }
                _ => {}
            }
        }
        Rule::TextNode => (), // Process text node
        Rule::EOI => (),
        _ => unreachable!(),
    }
}

pub(crate) fn parse_liquid_string(input: &str) -> Option<LiquidAST> {
    let text = LiquidParser::parse(Rule::LiquidDocNode, input)
        .map_err(|e| println!("Parsing error: {}", e))
        .ok()?;

    let mut builder = LiquidAST::new();
    for pair in text {
        visit(&mut builder, pair);
    }

    Some(builder)
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_liquid_string(input))
        .expect("The LiquidAst was not in the correct format")
}
