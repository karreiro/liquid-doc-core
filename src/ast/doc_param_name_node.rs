use serde::{Deserialize, Serialize};

use crate::{ast::position::Position, parser::Rule};

use super::text_node::TextNode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidDocParamNameNode {
    pub content: TextNode,
    pub position: Position,
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

        let source_str = pair.as_str();

        LiquidDocParamNameNode {
            content: TextNode::new(pair),
            position: Position::from_pair(pair),
            source: source_str.to_string(),
            required: !source_str.starts_with('[') && !source_str.ends_with(']'),
        }
    }
}
