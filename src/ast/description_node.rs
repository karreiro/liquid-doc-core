use serde::{Deserialize, Serialize};

use super::{position::Position, TextNode};

//   {
//     "type": "LiquidDocDescriptionNode",
//     "name": "description",
//     "position": {
//       "start": 10,
//       "end": 16
//     },
//     "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
//     "content": {
//       "type": "TextNode",
//       "value": "kdkd\n\n",
//       "position": {
//         "start": 10,
//         "end": 16
//       },
//       "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}"
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

        let content = TextNode::from_pair(pair, position_offset);
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
        assert_eq!(description_node.content.value, "@description kdkd\n");
    }
}
