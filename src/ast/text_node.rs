use serde::{Deserialize, Serialize};

use crate::parser::Rule;

use super::position::Position;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextNode {
    pub value: String,
    pub position: Position,
    pub source: String,
    #[serde(rename = "type", skip_deserializing)]
    pub type_: &'static str,
}

impl TextNode {
    pub fn new(value: String, position: Position, source: String) -> Self {
        TextNode {
            value,
            position,
            source,
            type_: "TextNode",
        }
    }
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>, position_offset: Option<usize>) -> Self {
        let source_str = pair.as_str();
        TextNode::new(
            source_str.to_string(),
            Position::from_pair(pair, position_offset),
            source_str.to_string(),
        )
    }

    fn new_from_indices(
        pair: &pest::iterators::Pair<Rule>,
        start: usize,
        end: usize,
        position_offset: Option<usize>,
    ) -> Self {
        let span = pair.as_span();
        let source_str = span.as_str();
        let original_start = span.start();
        let loc_start = original_start + start;
        let loc_end = original_start + end;
        TextNode::new(
            source_str[start..end].to_string(),
            Position::new(loc_start, loc_end, position_offset),
            source_str.to_string(),
        )
    }

    pub fn new_trim_ends(
        pair: &pest::iterators::Pair<Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        Self::new_from_indices(pair, 1, pair.as_str().len() - 1, position_offset)
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.value
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
