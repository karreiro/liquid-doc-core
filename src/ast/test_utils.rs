#[macro_export]
macro_rules! assert_json_output {
    ($input:expr $(,)?) => {{
        let ast = $crate::parser::parse_liquid_string($input, Some(10)).unwrap();

        let serialized = serde_json::to_string_pretty(&ast.nodes).unwrap();
        insta::assert_snapshot!(insta::internals::AutoName, serialized, $input);
    }};
}
