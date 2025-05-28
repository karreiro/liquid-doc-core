use crate::ast::{LiquidDocParamNode, *};
use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Parser)]
#[grammar = "liquid.pest"]
pub struct LiquidParser;

pub fn visit(builder: &mut LiquidAST, pair: pest::iterators::Pair<Rule>) {
    match pair.as_rule() {
        Rule::ImplicitDescription => {
            let _description_content = pair.into_inner();
            // Process description content
        }
        Rule::LiquidDocNode => {
            let mut content = pair.into_inner();
            let next = content.next().unwrap();
            match next.as_rule() {
                Rule::paramNode => {
                    let node = LiquidDocParamNode::new(&next);

                    builder.add_node(LiquidNode::LiquidDocParamNode(node));
                }
                Rule::exampleNode => {
                    // Process example node
                }
                Rule::descriptionNode => {
                    // Process description node
                }
                Rule::promptNode => {
                    // Process prompt node
                }
                Rule::fallbackNode => {
                    // Process fallback node
                }
                _ => {}
            }
        }
        Rule::TextNode => (), // Process text node
        Rule::EOI => (),
        _ => unreachable!(),
    }
}

pub(crate) fn parse_liquid_string(input: &str) -> Option<LiquidAST> {
    let text = LiquidParser::parse(Rule::LiquidDocNode, input)
        .map_err(|e| println!("Parsing error: {}", e))
        .ok()?;

    let mut ast = LiquidAST::new();
    for pair in text {
        visit(&mut ast, pair);
    }

    Some(ast)
}

#[wasm_bindgen]
pub fn parse_liquid(input: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_liquid_string(input))
        .expect("The LiquidAst was not in the correct format")
}

#[cfg(test)]
mod test {
    // test json serialization and deserialization

    use super::*;
    use crate::ast::LiquidNode;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    // use wasm_bindgen_test::wasm_bindgen_test;
    // use wasm_bindgen_test::wasm_bindgen_test_configure;
    // wasm_bindgen_test_configure!(run_in_browser);
    #[test]
    pub fn test_serialization_round_trip() {
        let input = "@param {sometype} requiredParamWithSomeType - This is a cool parameter";
        let ast = parse_liquid_string(input).unwrap();

        let expected = r#"[
   {
    "type": "LiquidDocParamNode",
    "name": "param",
    "position": {
      "start": 10,
      "end": 80
    },
    "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}",
    "paramName": {
      "type": "TextNode",
      "value": "requiredParamWithSomeType",
      "position": {
        "start": 28,
        "end": 53
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}"
    },
    "paramDescription": {
      "type": "TextNode",
      "value": "This is a cool parameter",
      "position": {
        "start": 56,
        "end": 80
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}"
    },
    "paramType": {
      "type": "TextNode",
      "value": "sometype",
      "position": {
        "start": 18,
        "end": 26
      },
      "source": "{% doc %}\n@param {sometype} requiredParamWithSomeType - This is a cool parameter\n{% enddoc %}"
    },
    "required": true
  }
]"#;

        // prettify json string
        let prettified = serde_json::to_string_pretty(&ast.nodes).unwrap();

        assert_eq!(prettified, expected);
    }
}
