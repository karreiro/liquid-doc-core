use magnus::{function, prelude::*, Error, Ruby};
use liquid_doc_parser::parse_liquid_string;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

fn parse_liquid(source: String) -> String {
    match parse_liquid_string(&source, None) {
        Some(ast) => {
            // Convert the AST to JSON for Ruby consumption
            match serde_json::to_string_pretty(&ast) {
                Ok(json) => json,
                Err(e) => format!("Error serializing AST: {}", e),
            }
        }
        None => "Failed to parse liquid template".to_string(),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("LiquidDocCore")?;
    module.define_module_function("parse", function!(parse_liquid, 1))?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

