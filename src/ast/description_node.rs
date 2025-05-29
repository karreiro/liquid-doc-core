use serde::{Deserialize, Serialize};

use super::{position::Position, TextNode};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocDescriptionNode {
    pub name: String,
    pub position: Position,
    pub source: String,
    pub content: TextNode,
    #[serde(rename = "isImplicit")]
    pub is_implicit: bool,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
}

impl LiquidDocDescriptionNode {
    fn new(
        content: TextNode,
        is_implicit: bool,
        is_inline: bool,
        position: Position,
        source: String,
    ) -> Self {
        LiquidDocDescriptionNode {
            content,
            is_implicit,
            is_inline,
            position,
            source,
            name: "description".to_string(), // The node name is always "description"
        }
    }
    pub fn explicit(
        pair: &pest::iterators::Pair<crate::parser::Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        assert!(
            pair.as_rule() == crate::parser::Rule::descriptionNode,
            "Expected a descriptionNode, found {:?}",
            pair.as_rule()
        );

        let mut content = TextNode::from_pair(pair, position_offset);
        content.trim_content_start("@description ");
        let source_str = pair.as_str();

        LiquidDocDescriptionNode::new(
            content,
            false,
            true,
            Position::from_pair(pair, position_offset),
            source_str.to_string(),
        )
    }
    pub fn implicit(
        pair: &pest::iterators::Pair<crate::parser::Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        assert!(
            pair.as_rule() == crate::parser::Rule::ImplicitDescription,
            "Expected a ImplicitDescription, found {:?}",
            pair.as_rule()
        );

        let content_node = pair.clone().into_inner().next().unwrap();

        let content = TextNode::from_pair(&content_node, position_offset);
        let source_str = pair.as_str();

        LiquidDocDescriptionNode::new(
            content,
            true,
            true,
            Position::from_pair(pair, position_offset),
            source_str.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_json_output, ast::LiquidNode, parser::parse_liquid_string};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_liquid_doc_implicit_description_node() {
        let input =
            "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input, None).unwrap();
        let description_node = match ast.nodes.first() {
            Some(LiquidNode::LiquidDocDescriptionNode(node)) => node,
            _ => panic!("Expected a LiquidDocDescriptionNode"),
        };

        assert_eq!(description_node.name, "description");
        assert!(description_node.is_implicit);
        assert!(description_node.is_inline);
        assert_eq!(description_node.content.value, "kdkd\n\n");
    }
    #[test]
    fn test_liquid_doc_explicit_description_node() {
        let input = "@description kdkd
            @param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input, None).unwrap();
        let description_node = match ast.nodes.first() {
            Some(LiquidNode::LiquidDocDescriptionNode(node)) => node,
            _ => panic!("Expected a LiquidDocDescriptionNode"),
        };

        assert_eq!(description_node.name, "description");
        assert!(!description_node.is_implicit);
        assert!(description_node.is_inline);
        assert_eq!(description_node.content.value, "kdkd\n");
    }

    #[test]
    fn test_serialization_round_trip() {
        assert_json_output!("@description kdkd\n");
    }

    #[test]
    pub fn test_serialization_round_trip_with_implicit_description() {
        assert_json_output!(
            "kdkd

@param {sometype} requiredParamWithSomeType - This is a cool parameter"
        );
    }

    #[test]
    pub fn test_serialization_round_trip_with_explicit_description() {
        assert_json_output!(
            "@description kdkd

@param {sometype} requiredParamWithSomeType - This is a cool parameter"
        );
    }
}
