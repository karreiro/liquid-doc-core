use crate::ast::{LiquidDocDescriptionNode, LiquidDocParamNode, LiquidRawTagNode, TextNode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum LiquidNode {
    LiquidDocDescriptionNode(LiquidDocDescriptionNode),
    LiquidRawTag(LiquidRawTagNode),
    TextNode(TextNode),
    LiquidDocParamNode(LiquidDocParamNode),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidAST {
    pub nodes: Vec<LiquidNode>,
}

impl LiquidAST {
    pub fn new() -> Self {
        LiquidAST { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: LiquidNode) {
        self.nodes.push(node);
    }

    #[allow(dead_code)]
    pub fn head(&self) -> LiquidNode {
        self.nodes[0].clone()
    }
}
