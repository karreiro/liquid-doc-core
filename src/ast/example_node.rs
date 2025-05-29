use serde::{Deserialize, Serialize};

use super::{position::Position, LiquidNode, TextNode};

const NODE_NAME: &str = "example";
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocExampleNode {
    pub name: String,
    pub position: Position,
    pub source: String,
    pub content: Box<LiquidNode>,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
}

impl LiquidDocExampleNode {
    pub fn new(content: TextNode, is_inline: bool, position: Position, source: String) -> Self {
        LiquidDocExampleNode {
            content: Box::new(LiquidNode::TextNode(content)),
            is_inline,
            position,
            source,
            name: NODE_NAME.to_string(),
        }
    }

    pub fn from_pair(
        pair: &pest::iterators::Pair<crate::parser::Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        assert!(
            pair.as_rule() == crate::parser::Rule::ExampleNode,
            "Expected a exampleNode, found {:?}",
            pair.as_rule()
        );

        let mut content = TextNode::from_pair(pair, position_offset);
        content.trim_content_start("@example ");

        let position = Position::from_pair(pair, position_offset);
        let source = pair.as_str().to_string();
        LiquidDocExampleNode::new(content, true, position, source)
    }
}

#[cfg(test)]
mod test {
    use crate::{assert_json_output, ast::LiquidNode, parser::parse_liquid_string};

    use pretty_assertions::assert_eq;

    #[test]
    fn parse_inline_example_node() {
        let input = "@example simple inline example\n";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();
        if let LiquidNode::LiquidDocExampleNode(example_node) = node {
            assert_eq!(
                example_node.content.as_text_node_unsafe().as_str(),
                "simple inline example\n"
            );
            assert!(example_node.is_inline);
        } else {
            panic!("Expected a LiquidDocExampleNode");
        }
    }

    #[test]
    pub fn test_serialization_round_trip() {
        assert_json_output!("@example simple inline example\n");
    }

    #[test]
    pub fn complex_example() {
        assert_json_output!("@example
{% render 'resource-card', resource: product, resource_type: 'product', image_width: 300, image_aspect_ratio: '1/1' %}
");
    }
}
