use serde::{Deserialize, Serialize};

use crate::parser::Rule;

use super::text_node::TextNode;

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
