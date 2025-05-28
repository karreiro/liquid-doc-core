use serde::{Deserialize, Serialize};

use crate::parser::Rule;

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

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
