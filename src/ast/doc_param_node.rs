use serde::{Deserialize, Serialize};

use crate::parser::Rule;

use super::position::Position;
use super::text_node::TextNode;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocParamNode {
    pub name: String,
    pub position: Position,
    pub source: String,
    pub required: bool,
    #[serde(rename = "paramType")]
    pub param_type: Option<TextNode>,
    #[serde(rename = "paramName")]
    pub param_name: TextNode,
    #[serde(rename = "paramDescription")]
    pub param_description: Option<TextNode>,
}
impl LiquidDocParamNode {
    pub fn new(pair: &pest::iterators::Pair<Rule>, position_offset: Option<usize>) -> Self {
        assert!(
            pair.as_rule() == Rule::paramNode,
            "Expected a paramNode, found {:?}",
            pair.as_rule()
        );

        let mut inner = pair.clone().into_inner();

        let first = inner.next().expect("Expected at least one inner pair");

        let (param_type, name) = if let Rule::paramType = first.as_rule() {
            (
                Some(TextNode::without_brackets(&first, position_offset)),
                inner.next().expect("Expected a paramName after paramType"),
            )
        } else {
            (None, first)
        };

        let param_name = TextNode::without_brackets(&name, position_offset);

        let description = inner.next().and_then(|t| {
            if !t.as_str().is_empty() {
                Some(TextNode::from_pair(&t, position_offset))
            } else {
                None
            }
        });

        // let pair = pair.next
        let source_str = pair.as_str();
        LiquidDocParamNode {
            name: "param".to_string(), // The node name is always "param"
            position: Position::from_pair(pair, position_offset),
            source: source_str.to_string(),
            param_type, // Default to None, can be set later
            param_name,
            param_description: description,
            required: !source_str.starts_with('[') && !source_str.ends_with(']'),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::LiquidNode;
    use crate::parser::parse_liquid_string;

    /*
        """@param requiredParamWithNoType
            @param {String} paramWithDescription - param with description and `punctation`. This is still a valid param description.
            @param {String} paramWithNoDescription
            @param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
            @param [optionalParameterWithDescription] - optional parameter description
            @param {String} [optionalParameterWithType]
            @unsupported this node falls back to a text node
    """
         */
    #[test]
    fn test_parse_param_with_type() {
        let input = "@param {sometype} requiredParamWithNoType";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();

        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(param_node.param_name.as_str(), "requiredParamWithNoType");
            assert!(param_node.param_description.is_none());
            assert!(param_node.param_type.is_some());
            assert_eq!(param_node.param_type.unwrap().as_str(), "sometype");
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }
    #[test]
    fn test_parse_param_with_type_and_description() {
        let input = "@param {sometype} requiredParamWithNoType - This is a cool parameter";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();
        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(param_node.param_name.as_str(), "requiredParamWithNoType");
            assert_eq!(
                param_node.param_description.unwrap().as_str(),
                "This is a cool parameter"
            );
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }

    #[test]
    fn test_parse_optional_param_with_type_and_description() {
        let input =
            "@param {sometype} [optionalParamWithTypeAndDescription] - This is a cool parameter";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();
        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(
                param_node.param_name.as_str(),
                "optionalParamWithTypeAndDescription"
            );
            assert_eq!(
                param_node.param_description.unwrap().as_str(),
                "This is a cool parameter"
            );
            assert!(param_node.required)
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }
}
