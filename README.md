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
gem install sinatra puma rackup

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
