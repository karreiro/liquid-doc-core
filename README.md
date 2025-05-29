# WebAssembly LiquidDoc Parser

A Rust implementation of the LiquidDoc parser compiled to WebAssembly, for parsing documentation with special annotations.

## Features

- 🦀 Rust LiquidDoc parser compiled to WebAssembly
- 📚 Full LiquidDoc grammar support (@param, @example, @description, @prompt)
- 🌳 Complete AST generation with JavaScript object returns
- 🖥️  CLI parser for terminal testing
- 💎 Ruby Sinatra web server
- 📱 Minimal HTML interface

## LiquidDoc Syntax

LiquidDoc supports documentation with special annotations:

```
This is an implicit description.

@param {string} name - Parameter description
@param [optional] - Optional parameter
@example
someFunction("example")
@description
Detailed description of functionality
@prompt
System prompt for AI models
```

## Quick Start

```bash
# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
bundle

# Build and run
./run.sh
```

## Usage

### CLI Version
```bash
cargo run --bin cli
```

### Web Version
```bash
./build.sh
ruby app.rb
# Visit http://localhost:4567
```

## AST Structure

The parser returns JavaScript objects representing the LiquidDoc AST:

```javascript
{
  "input": "...",
  "success": true,
  "node_count": 5,
  "ast": {
    "type": "LiquidDoc",
    "implicit_description": "This function does something.",
    "nodes": [
      {
        "type": "ParamNode",
        "param_type": "string",
        "name": "input",
        "is_optional": false,
        "description": "The input parameter"
      },
      {
        "type": "ExampleNode", 
        "content": "myFunction(\"test\")"
      }
    ]
  }
}
```

## Dependencies

- **Rust**: Core language with Pest parser
- **Pest**: Parser generator for LiquidDoc grammar
- **wasm-pack**: WebAssembly build tool  
- **Ruby**: Sinatra web framework
- **serde**: JSON serialization for WASM ↔ JS objects

Happy documenting! 📚🚀


## Snapshot testing
To test the json output from the parser, we use snapshot testing via [cargo-insta](https://insta.rs/docs/cli/).
To test something you just need to do something like this:

```rust
#[test]
pub fn test_serialization_round_trip() {
    assert_json_output!("@example simple inline example\n");
}
```

This will create a file in the `snapshots` folder. You can use the command `cargo insta review` to see the difference if a change you make alters the snapshots. This will allow you to simply accept or reject changes as applicable.

To make this even nicer, you can install the `insta snapshots` extension to show an even richer diff when comparing snapshot changes.
