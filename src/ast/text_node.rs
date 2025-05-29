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

    pub fn without_brackets(
        pair: &pest::iterators::Pair<Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        let mut text_node = Self::from_pair(pair, position_offset);
        let new_value = text_node
            .value
            .trim_matches(|c| c == '{' || c == '}' || c == '[' || c == ']')
            .to_string();
        if new_value == text_node.value {
            return text_node; // No change needed
        }
        text_node.value = new_value;

        text_node.position.shift_start(1); // Adjust position to account for removed brackets
        text_node.position.shift_end_down(1); // Adjust end position as well
        text_node
    }

    pub fn trim_content_start(&mut self, to_strip: &str) {
        if self.value.starts_with(to_strip) {
            self.value = self.value.trim_start_matches(to_strip).to_string();
            self.position.shift_start(to_strip.len());
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.value
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}
