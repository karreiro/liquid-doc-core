/// Generates a JSON snapshot of the AST for the given Liquid input.
/// If the output changes between test runs, the test will fail.
/// To compare and accept/reject changes, use the `cargo insta review` on the command line.
#[macro_export]
macro_rules! assert_json_output {
    ($input:expr $(,)?) => {{
        let ast = $crate::parser::parse_liquid_string($input, Some(10)).unwrap();

        let serialized = serde_json::to_string_pretty(&ast.nodes).unwrap();
        insta::assert_snapshot!(insta::internals::AutoName, serialized, $input);
    }};
}
