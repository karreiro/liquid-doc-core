use clap::Parser;
use liquid_doc_parser::parse_liquid_string;
use serde_json;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "liquid-doc-parser")]
#[command(about = "A CLI for parsing Liquid documentation")]
struct Cli {
    /// Input file path. If not specified, reads from stdin
    #[arg(long, short)]
    input: Option<String>,

    /// Output format
    #[arg(long, short, default_value = "json")]
    format: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read input from file or stdin
    let input_content = match cli.input {
        Some(file_path) => fs::read_to_string(file_path)?,
        None => {
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            content
        }
    };

    // Parse the liquid content
    match parse_liquid_string(&input_content, None) {
        Some(result) => {
            match cli.format.as_str() {
                "json" => {
                    // Convert to JSON for display
                    match serde_json::to_string_pretty(&result) {
                        Ok(json) => println!("{}", json),
                        Err(e) => {
                            eprintln!("Error serializing result to JSON: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                "debug" => {
                    println!("{:#?}", result);
                }
                _ => {
                    eprintln!("Unsupported format: {}", cli.format);
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("Error parsing liquid content");
            std::process::exit(1);
        }
    }

    Ok(())
}
