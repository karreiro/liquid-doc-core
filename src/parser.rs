use crate::ast::{LiquidDocParamNode, *};
use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

pub fn visit(
    ast: &mut LiquidAST,
    pair: pest::iterators::Pair<Rule>,
    position_offset: Option<usize>,
) {
    match pair.as_rule() {
        Rule::Document => {
            // The Document rule is the root of the AST, so we can just ignore it
            for inner_pair in pair.into_inner() {
                visit(ast, inner_pair, position_offset);
            }
        }
        Rule::ImplicitDescription => {
            let node = LiquidDocDescriptionNode::implicit(&pair, position_offset);
            if !node.content.is_empty() {
                ast.add_node(LiquidNode::LiquidDocDescriptionNode(node));
            }
        }
        Rule::LiquidDocNode => {
            let mut content = pair.into_inner();
            let next = content.next().unwrap();
            match next.as_rule() {
                Rule::paramNode => {
                    let node = LiquidDocParamNode::new(&next, position_offset);

                    ast.add_node(LiquidNode::LiquidDocParamNode(node));
                }
                Rule::exampleNode => {
                    let node = LiquidDocExampleNode::from_pair(&next, position_offset);
                    ast.add_node(LiquidNode::LiquidDocExampleNode(node));
                }
                Rule::descriptionNode => {
                    let node = LiquidDocDescriptionNode::explicit(&next, position_offset);
                    ast.add_node(LiquidNode::LiquidDocDescriptionNode(node));
                }
                Rule::promptNode => {
                    // Process prompt node
                    unimplemented!("Prompt nodes are not yet implemented");
                }
                Rule::fallbackNode => {
                    // Process fallback node
                    unimplemented!("Fallback nodes are not yet implemented");
                }
                _ => unreachable!("Unexpected rule in LiquidDocNode: {:?}", next.as_rule()),
            }
        }
        Rule::TextNode => {
            let text_node = TextNode::from_pair(&pair, position_offset);
            if !text_node.is_empty() {
                ast.add_node(LiquidNode::TextNode(text_node));
            }
        }
        _ => unimplemented!("Handle rule: {:?}", pair.as_rule()),
    }
}

pub(crate) fn parse_liquid_string(
    input: &str,
    position_offset: Option<usize>,
) -> Option<LiquidAST> {
    let text = LiquidParser::parse(Rule::Document, input)
        .map_err(|e| println!("Parsing error: {}", e))
        .ok()?;

    let mut ast = LiquidAST::new();
    for pair in text {
        visit(&mut ast, pair, position_offset);
    }

    Some(ast)
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_liquid_string(input, None))
        .expect("The LiquidAst was not in the correct format")
}

#[cfg(test)]
mod test {
    // test json serialization and deserialization

    use super::*;
    use pretty_assertions::assert_eq;
    // use wasm_bindgen_test::wasm_bindgen_test;
    // use wasm_bindgen_test::wasm_bindgen_test_configure;
    // wasm_bindgen_test_configure!(run_in_browser);
    #[test]
    pub fn test_serialization_round_trip() {
        let input = "@param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input, Some(10)).unwrap();

        let expected = r#"[
  {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 10,
      "end": 80
    },
    "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "required": true,
    "paramType": {
      "value": "sometype",
      "position": {
        "start": 18,
        "end": 26
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramName": {
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 28,
        "end": 53
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramDescription": {
      "value": "This is a cool parameter",
      "position": {
        "start": 56,
        "end": 80
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    }
  }
]"#;

        // prettify json string
        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_serialization_round_trip_with_description() {
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
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "isImplicit": true,
    "isInline": true,
    "position": {
      "start": 10,
      "end": 16
    },
    "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "name": "description"
  },
  {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 16,
      "end": 86
    },
    "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "required": true,
    "paramType": {
      "value": "sometype",
      "position": {
        "start": 23,
        "end": 33
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramName": {
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 34,
        "end": 59
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramDescription": {
      "value": "This is a cool parameter",
      "position": {
        "start": 62,
        "end": 86
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
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
        "start": 10,
        "end": 29
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "isImplicit": false,
    "isInline": true,
    "position": {
      "start": 10,
      "end": 29
    },
    "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "name": "description"
  },
  {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 29,
      "end": 99
    },
    "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "required": true,
    "paramType": {
      "value": "sometype",
      "position": {
        "start": 36,
        "end": 46
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramName": {
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 47,
        "end": 72
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    },
    "paramDescription": {
      "value": "This is a cool parameter",
      "position": {
        "start": 75,
        "end": 99
      },
      "source": "{% doc %}\nkdkd\n\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
      "type": "TextNode"
    }
  }
]"#;

        // prettify json string
        let actual = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
