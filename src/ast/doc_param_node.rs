use serde::{Deserialize, Serialize};

use crate::parser::Rule;

use super::position::Position;
use super::text_node::TextNode;
use super::LiquidNode;

const NODE_NAME: &str = "param";
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocParamNode {
    pub name: String,
    pub position: Position,
    pub source: String,
    #[serde(rename = "paramName")]
    pub param_name: Box<LiquidNode>,
    #[serde(rename = "paramDescription")]
    pub param_description: Option<Box<LiquidNode>>,
    #[serde(rename = "paramType")]
    pub param_type: Option<Box<LiquidNode>>,
    pub required: bool,
}
impl LiquidDocParamNode {
    fn new(
        position: Position,
        source: String,
        param_type: Option<TextNode>,
        param_name: TextNode,
        param_description: Option<TextNode>,
        required: bool,
    ) -> Self {
        LiquidDocParamNode {
            name: NODE_NAME.to_string(),
            position,
            source,
            param_type: param_type.map(|t| Box::new(LiquidNode::TextNode(t))),
            param_name: Box::new(LiquidNode::TextNode(param_name)),
            param_description: param_description.map(|d| Box::new(LiquidNode::TextNode(d))),
            required,
        }
    }
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>, position_offset: Option<usize>) -> Self {
        assert!(
            pair.as_rule() == Rule::ParamNode,
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
        let original_name_str = name.as_str();
        let required = !original_name_str.starts_with('[') && !original_name_str.ends_with(']');

        let param_name = TextNode::without_brackets(&name, position_offset);

        let description = inner.next().and_then(|t| {
            if !t.as_str().is_empty() {
                Some(TextNode::from_pair(&t, position_offset))
            } else {
                None
            }
        });

        LiquidDocParamNode::new(
            Position::from_pair(pair, position_offset),
            pair.as_str().to_string(),
            param_type,
            param_name,
            description,
            required,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_json_output;
    use crate::ast::LiquidNode;
    use crate::parser::parse_liquid_string;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_param_with_type() {
        let input = "@param {sometype} requiredParamWithNoType";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();

        if let LiquidNode::LiquidDocParamNode(param_node) = node {
            assert_eq!(
                param_node.param_name.as_text_node_unsafe().as_str(),
                "requiredParamWithNoType"
            );
            assert!(param_node.param_description.is_none());
            assert!(param_node.param_type.is_some());
            assert_eq!(
                param_node
                    .param_type
                    .unwrap()
                    .as_text_node_unsafe()
                    .as_str(),
                "sometype"
            );
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
            assert_eq!(
                param_node.param_name.as_text_node_unsafe().as_str(),
                "requiredParamWithNoType"
            );
            assert_eq!(
                param_node
                    .param_description
                    .unwrap()
                    .as_text_node_unsafe()
                    .as_str(),
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
                param_node.param_name.as_text_node_unsafe().as_str(),
                "optionalParamWithTypeAndDescription"
            );
            assert_eq!(
                param_node
                    .param_description
                    .unwrap()
                    .as_text_node_unsafe()
                    .as_str(),
                "This is a cool parameter"
            );
            assert!(!param_node.required)
        } else {
            panic!("Expected a LiquidDocParamNode");
        }
    }

    #[test]
    pub fn test_serialization_round_trip() {
        assert_json_output!(
            "@param {sometype} requiredParamWithSomeType - This is a cool parameter"
        );
    }

    #[test]
    pub fn complex_example_with_many_params() {
        assert_json_output!("@param requiredParamWithNoType
@param {String} paramWithDescription - param with description and `punctation`. This is still a valid param description.
@param {String} paramWithNoDescription
@param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
@param [optionalParameterWithDescription] - optional parameter description
@param {String} [optionalParameterWithType]")
    }
}
