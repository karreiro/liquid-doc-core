use crate::ast::{LiquidDocDescriptionNode, LiquidDocParamNode, TextNode};
use serde::{Deserialize, Serialize};

use super::LiquidDocExampleNode;

/// Represents the different types of nodes in a Liquid AST.
/// Each variant corresponds to a specific type of node in the Liquid template language.
/// If a node has a specific content type, it is represented as a `LiquidNode` variant, rather than the more specific type.
/// This instructs the serializer to use the `type` field to differentiate between node types, which the consuming code requires.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub enum LiquidNode {
    LiquidDocDescriptionNode(LiquidDocDescriptionNode),
    TextNode(TextNode),
    LiquidDocParamNode(LiquidDocParamNode),
    LiquidDocExampleNode(LiquidDocExampleNode),
}

impl LiquidNode {
    #[cfg(test)]
    pub fn as_text_node_unsafe(&self) -> &TextNode {
        if let LiquidNode::TextNode(text_node) = self {
            text_node
        } else {
            panic!("Tried to access TextNode from a non-TextNode LiquidNode");
        }
    }
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
