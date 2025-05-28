use crate::ast::{LiquidDocParamNameNode, LiquidDocParamNode, LiquidRawTagNode, TextNode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LiquidNode {
    LiquidRawTag(LiquidRawTagNode),
    TextNode(TextNode),
    LiquidDocParamNode(LiquidDocParamNode),
    LiquidDocParamNameNode(LiquidDocParamNameNode),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn head(&self) -> LiquidNode {
        self.nodes[0].clone()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
