use serde::{Deserialize, Serialize};

use super::{position::Position, LiquidNode};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub position: Position,
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
