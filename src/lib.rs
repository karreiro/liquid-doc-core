use pest::Parser;
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
        a,
        b,
        result: a + b,
    };
    serde_wasm_bindgen::to_value(&result).expect("The struct must be serializable")
}

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidAst {
    pub nodes: Vec<LiquidNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LiquidNode {
    LiquidRawTag(LiquidRawTagNode),
    TextNode(TextNode),
    LiquidDocParamNode(LiquidDocParamNode),
    LiquidDocParamNameNode(LiquidDocParamNameNode),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextNode {
    pub value: String,
    #[serde(rename = "locStart")]
    pub loc_start: usize,
    #[serde(rename = "locEnd")]
    pub loc_end: usize,
    pub source: String,
}

impl TextNode {
    pub fn new(pair: &pest::iterators::Pair<Rule>) -> Self {
        let span = pair.as_span();
        let source_str = span.as_str();
        TextNode {
            value: source_str.to_string(),
            loc_start: span.start(),
            loc_end: span.end(),
            source: source_str.to_string(),
        }
    }

    fn new_from_indices(pair: &pest::iterators::Pair<Rule>, start: usize, end: usize) -> Self {
        let span = pair.as_span();
        let source_str = span.as_str();
        let original_start = span.start();
        let loc_start = original_start + start;
        let loc_end = original_start + end;
        TextNode {
            value: source_str[start..end].to_string(),
            loc_start,
            loc_end,
            source: source_str.to_string(),
        }
    }

    pub fn new_trim_ends(pair: &pest::iterators::Pair<Rule>) -> Self {
        Self::new_from_indices(pair, 1, pair.as_str().len() - 1)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidRawTagNode {
    pub name: String,
    pub body: String,
    pub children: Vec<LiquidNode>,
    #[serde(rename = "whitespaceStart")]
    pub whitespace_start: String,
    #[serde(rename = "whitespaceEnd")]
    pub whitespace_end: String,
    #[serde(rename = "delimiterWhitespaceStart")]
    pub delimiter_whitespace_start: String,
    #[serde(rename = "delimiterWhitespaceEnd")]
    pub delimiter_whitespace_end: String,
    #[serde(rename = "locStart")]
    pub loc_start: usize,
    #[serde(rename = "locEnd")]
    pub loc_end: usize,
    pub source: String,
    #[serde(rename = "blockStartLocStart")]
    pub block_start_loc_start: usize,
    #[serde(rename = "blockStartLocEnd")]
    pub block_start_loc_end: usize,
    #[serde(rename = "blockEndLocStart")]
    pub block_end_loc_start: usize,
    #[serde(rename = "blockEndLocEnd")]
    pub block_end_loc_end: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidDocParamNode {
    pub name: String,
    #[serde(rename = "locStart")]
    pub loc_start: usize,
    #[serde(rename = "locEnd")]
    pub loc_end: usize,
    pub source: String,
    #[serde(rename = "paramType")]
    pub param_type: Option<TextNode>,
    #[serde(rename = "paramName")]
    pub param_name: LiquidDocParamNameNode,
    #[serde(rename = "paramDescription")]
    pub param_description: Option<TextNode>,
}
impl LiquidDocParamNode {
    pub fn new(pair: &pest::iterators::Pair<Rule>) -> Self {
        println!("{:#?}", pair);
        assert!(
            pair.as_rule() == Rule::paramNode,
            "Expected a paramNode, found {:?}",
            pair.as_rule()
        );

        let mut inner = pair.clone().into_inner();

        let first = inner.next().expect("Expected at least one inner pair");

        let (param_type, name) = if let Rule::paramType = first.as_rule() {
            (
                // Remove the curly braces from the type string
                Some(TextNode::new_trim_ends(&first)),
                inner.next().expect("Expected a paramName after paramType"),
            )
        } else {
            (None, first)
        };

        let param_name = LiquidDocParamNameNode::new(&name);

        let description = inner.next().and_then(|t| {
            if !t.as_str().is_empty() {
                Some(TextNode::new(&t))
            } else {
                None
            }
        });

        // let pair = pair.next
        let source_str = pair.as_str();
        let span = pair.as_span();
        LiquidDocParamNode {
            name: "param".to_string(), // The node name is always "param"
            loc_start: span.start(),
            loc_end: span.end(),
            source: source_str.to_string(),
            param_type, // Default to None, can be set later
            param_name,
            param_description: description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidDocParamNameNode {
    pub content: TextNode,
    #[serde(rename = "locStart")]
    pub loc_start: usize,
    #[serde(rename = "locEnd")]
    pub loc_end: usize,
    pub source: String,
    pub required: bool,
}

impl LiquidDocParamNameNode {
    pub fn new(pair: &pest::iterators::Pair<Rule>) -> Self {
        assert!(
            pair.as_rule() == Rule::paramName,
            "Expected a paramName, found {:?}",
            pair.as_rule()
        );

        let span = pair.as_span();
        let source_str = span.as_str();

        LiquidDocParamNameNode {
            content: TextNode::new(pair),
            loc_start: span.start(),
            loc_end: span.end(),
            source: source_str.to_string(),
            required: !source_str.starts_with('[') && !source_str.ends_with(']'),
        }
    }
}

pub fn visit(builder: &mut LiquidASTBuilder, pair: pest::iterators::Pair<Rule>) {
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

fn parse_liquid_string(input: &str) -> Option<LiquidAst> {
    let text = LiquidParser::parse(Rule::LiquidDocNode, input)
        .map_err(|e| println!("Parsing error: {}", e))
        .ok()?;

    let mut builder = LiquidASTBuilder::new();
    for pair in text {
        visit(&mut builder, pair);
    }

    Some(builder.build())
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_liquid_string(input))
        .expect("The LiquidAst was not in the correct format")
}

pub struct LiquidASTBuilder {
    nodes: Vec<LiquidNode>,
}

impl Default for LiquidASTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidASTBuilder {
    pub fn new() -> Self {
        LiquidASTBuilder { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: LiquidNode) {
        self.nodes.push(node);
    }

    pub fn build(self) -> LiquidAst {
        LiquidAst { nodes: self.nodes }
    }
}

// Add additional node types and utility functions as needed

#[cfg(test)]
mod tests {
    use super::*;

    /*
        """@param requiredParamWithNoType
            @param {String} paramWithDescription - param with description and `punctation`. This is still a valid param description.
            @param {String} paramWithNoDescription
            @param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
            @param [optionalParameterWithDescription] - optional parameter description
            @param {String} [optionalParameterWithType]
            @unsupported this node falls back to a text node
    """
         */
    #[test]
    fn test_parse_param_with_type() {
        let input = "@param {sometype} requiredParamWithNoType";
        let result = parse_liquid_string(input);

        println!("{:#?}", result);
        assert!(result.is_some());
        let node = result.unwrap().nodes[0].clone();

        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(
                param_node.param_name.content.as_str(),
                "requiredParamWithNoType"
            );
            assert!(param_node.param_description.is_none());
            assert!(param_node.param_type.is_some());
            assert_eq!(param_node.param_type.unwrap().as_str(), "sometype");
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }
    #[test]
    fn test_parse_param_with_type_and_description() {
        let input = "@param {sometype} requiredParamWithNoType - This is a cool parameter";
        let result = parse_liquid_string(input);

        println!("{:#?}", result);
        assert!(result.is_some());
        let node = result.unwrap().nodes[0].clone();
        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(
                param_node.param_name.content.as_str(),
                "requiredParamWithNoType"
            );
            assert_eq!(
                param_node.param_description.unwrap().as_str(),
                "This is a cool parameter"
            );
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }
}
