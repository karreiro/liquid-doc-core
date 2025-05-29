use serde::{Deserialize, Serialize};

use super::{position::Position, TextNode};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocExampleNode {
    pub content: TextNode,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
    pub position: Position,
    pub source: String,
    pub name: String,
}

impl LiquidDocExampleNode {
    pub fn new(content: TextNode, is_inline: bool, position: Position, source: String) -> Self {
        LiquidDocExampleNode {
            content,
            is_inline,
            position,
            source,
            name: "example".to_string(), // The node name is always "example"
        }
    }

    pub fn from_pair(
        pair: &pest::iterators::Pair<crate::parser::Rule>,
        position_offset: Option<usize>,
    ) -> Self {
        assert!(
            pair.as_rule() == crate::parser::Rule::exampleNode,
            "Expected a exampleNode, found {:?}",
            pair.as_rule()
        );

        let mut content = TextNode::from_pair(pair, position_offset);
        content.value = content.value.trim_start_matches("@example ").to_string();
        content.position.shift_start(9); // Adjust for the "@example " prefix

        let position = Position::from_pair(pair, position_offset);
        let source = pair.as_str().to_string();
        LiquidDocExampleNode::new(content, true, position, source)
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::LiquidNode, parser::parse_liquid_string};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_optional_param_with_type_and_description() {
        let input = "@example simple inline example\n";
        let result = parse_liquid_string(input, Some(10));

        assert!(result.is_some());
        let node = result.unwrap().head();
        if let LiquidNode::LiquidDocExampleNode(example_node) = node {
            assert_eq!(example_node.content.as_str(), "simple inline example\n");
            assert!(example_node.is_inline);
        } else {
            panic!("Expected a LiquidDocExampleNode");
        }
    }

    #[test]
    pub fn test_serialization_round_trip() {
        let input = "@example simple inline example\n";
        let ast = parse_liquid_string(input, Some(10)).unwrap();

        let expected = r#"[
  {
    "type": "LiquidDocExampleNode",
    "content": {
      "value": "simple inline example\n",
      "position": {
        "start": 19,
        "end": 41
      },
      "source": "{% doc %}\n@example simple inline example\n{% enddoc %}",
      "type": "TextNode"
    },
    "isInline": true,
    "position": {
      "start": 10,
      "end": 41
    },
    "source": "{% doc %}\n@example simple inline example\n{% enddoc %}",
    "name": "example"
  }
]"#;

        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
