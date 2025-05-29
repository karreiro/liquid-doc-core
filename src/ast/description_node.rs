use serde::{Deserialize, Serialize};

use super::{position::Position, TextNode};

//   {
//     "type": "LiquidDocDescriptionNode",
//     "name": "description",
//     "position": {
//       "start": 10,
//       "end": 16
//     },
//     "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
//     "content": {
//       "type": "TextNode",
//       "value": "kdkd\n\n",
//       "position": {
//         "start": 10,
//         "end": 16
//       },
//       "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n"
//     },
//     "isImplicit": true,
//     "isInline": true
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LiquidDocDescriptionNode {
    pub content: TextNode,
    #[serde(rename = "isImplicit")]
    pub is_implicit: bool,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
    pub position: Position,
    pub source: String,
    pub name: String,
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
    use crate::{ast::LiquidNode, parser::parse_liquid_string};
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
        let input = "@description kdkd\n";
        let ast = parse_liquid_string(input, Some(10)).unwrap();

        let expected = r#"[
  {
    "type": "LiquidDocDescriptionNode",
    "content": {
      "value": "kdkd\n",
      "position": {
        "start": 23,
        "end": 28
      },
      "source": "@description kdkd\n",
      "type": "TextNode"
    },
    "isImplicit": false,
    "isInline": true,
    "position": {
      "start": 10,
      "end": 28
    },
    "source": "@description kdkd\n",
    "name": "description"
  }
]"#;

        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_serialization_round_trip_with_implicit_description() {
        let input = "kdkd

@param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input, Some(10)).unwrap();

        let expected = r#"[
  {
    "type": "LiquidDocDescriptionNode",
    "content": {
      "value": "kdkd\n\n",
      "position": {
        "start": 10,
        "end": 16
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "isImplicit": true,
    "isInline": true,
    "position": {
      "start": 10,
      "end": 16
    },
    "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
    "name": "description"
  },
  {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 16,
      "end": 86
    },
    "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
    "required": true,
    "paramType": {
      "value": "sometype",
      "position": {
        "start": 24,
        "end": 32
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "paramName": {
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 34,
        "end": 59
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "paramDescription": {
      "value": "This is a cool parameter",
      "position": {
        "start": 62,
        "end": 86
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    }
  }
]"#;

        // prettify json string
        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_serialization_round_trip_with_explicit_description() {
        let input = "@description kdkd

@param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input, Some(10)).unwrap();

        let expected = r#"[
  {
    "type": "LiquidDocDescriptionNode",
    "content": {
      "value": "@description kdkd\n\n",
      "position": {
        "start": 23,
        "end": 29
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "isImplicit": false,
    "isInline": true,
    "position": {
      "start": 10,
      "end": 29
    },
    "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
    "name": "description"
  },
  {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 29,
      "end": 99
    },
    "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
    "required": true,
    "paramType": {
      "value": "sometype",
      "position": {
        "start": 36,
        "end": 46
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "paramName": {
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 47,
        "end": 72
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    },
    "paramDescription": {
      "value": "This is a cool parameter",
      "position": {
        "start": 75,
        "end": 99
      },
      "source": "kdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n",
      "type": "TextNode"
    }
  }
]"#;

        // prettify json string
        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
