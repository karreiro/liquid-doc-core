use crate::ast::{LiquidDocParamNode, *};
use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

pub fn visit(
    ast: &mut LiquidAST,
    pair: pest::iterators::Pair<Rule>,
    position_offset: Option<usize>,
) {
    match pair.as_rule() {
        Rule::LiquidDocNode | Rule::Document => {
            for inner_pair in pair.into_inner() {
                visit(ast, inner_pair, position_offset);
            }
        }
        Rule::ImplicitDescription => {
            let node = LiquidDocDescriptionNode::implicit(&pair, position_offset);
            if !node.value().is_empty() {
                // If the description starts with '@', it's a fallback, treat it as a text node
                if node.value().starts_with("@") {
                    ast.add_node(LiquidNode::TextNode(TextNode::new(
                        node.value().to_string(),
                        node.position,
                        node.source.clone(),
                    )));
                } else {
                    ast.add_node(LiquidNode::LiquidDocDescriptionNode(node));
                }
            }
        }
        Rule::ParamNode => {
            let node = LiquidDocParamNode::from_pair(&pair, position_offset);

            ast.add_node(LiquidNode::LiquidDocParamNode(node));
        }
        Rule::ExampleNode => {
            let node = LiquidDocExampleNode::from_pair(&pair, position_offset);
            ast.add_node(LiquidNode::LiquidDocExampleNode(node));
        }
        Rule::DescriptionNode => {
            let node = LiquidDocDescriptionNode::explicit(&pair, position_offset);
            ast.add_node(LiquidNode::LiquidDocDescriptionNode(node));
        }
        Rule::PromptNode | Rule::FallbackNode => {
            let text_node = TextNode::from_pair(&pair, position_offset);
            if !text_node.is_empty() {
                ast.add_node(LiquidNode::TextNode(text_node));
            }
        }
        Rule::TextNode => {
            let text_node = TextNode::from_pair(&pair, position_offset);
            if !text_node.is_empty() {
                ast.add_node(LiquidNode::TextNode(text_node));
            }
        }
        _ => unimplemented!("Handle rule: {:?}", pair.as_rule()),
    }
}

pub(crate) fn parse_liquid_string(
    input: &str,
    position_offset: Option<usize>,
) -> Option<LiquidAST> {
    let text = LiquidParser::parse(Rule::Document, input)
        .map_err(|e| println!("Parsing error: {}", e))
        .ok()?;

    let mut ast = LiquidAST::new();
    for pair in text {
        visit(&mut ast, pair, position_offset);
    }

    Some(ast)
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_liquid_string(input, None))
        .expect("The LiquidAst was not in the correct format")
}
