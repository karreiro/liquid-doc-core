# liquid-doc-parse

A high-performance Liquid template parser that converts Liquid Doc templates into Abstract Syntax Trees (AST). Built with WebAssembly for optimal performance.

## Installation

```bash
npm install liquid-doc-parse
```

## Usage

The library automatically handles WebAssembly initialization, so you can start parsing immediately:

```javascript
import { parseLiquid } from 'liquid-doc-parse';

async function example() {
  try {
    const template = "description here\n@param requiredParamWithNoType";
    const ast = await parseLiquid(template);
    console.log("Parsed AST:", ast);
  } catch (error) {
    console.error("Error parsing template:", error);
  }
}

example();
```

### CommonJS Usage

```javascript
const { parseLiquid } = require('liquid-doc-parse');

async function example() {
  const ast = await parseLiquid("your liquid template");
  console.log(ast);
}
```

## API

### `parseLiquid(input: string): Promise<any>`

Parses a Liquid template string into an AST.

- **Parameters:**
  - `input` (string): The Liquid template string to parse
- **Returns:** Promise that resolves to the parsed AST
- **Throws:** Error if the input is not a string or if parsing fails

### `init(wasmInput?: string | Uint8Array): Promise<void>`

Manually initialize the WebAssembly module. This is called automatically by `parseLiquid` if not already initialized.

- **Parameters:**
  - `wasmInput` (optional): Custom WASM file path, URL, or bytes
- **Returns:** Promise that resolves when initialization is complete

### `isInitialized(): boolean`

Check if the WASM module is initialized.

- **Returns:** `true` if initialized, `false` otherwise

## Features

- 🚀 **High Performance**: Built with WebAssembly for fast parsing
- 🔄 **Auto-initialization**: No manual setup required
- 🌐 **Universal**: Works in both Node.js and browser environments
- 📦 **Zero Configuration**: WASM binaries are bundled and loaded automatically
- 🔍 **Type Definitions**: Includes TypeScript definitions

## Browser Usage

```html
<script type="module">
  import { parseLiquid } from './node_modules/liquid-doc-parse/dist/index.mjs';

  async function parse() {
    const ast = await parseLiquid("{{ variable }}");
    console.log(ast);
  }

  parse();
</script>
```

## Requirements

- Node.js 14+ (for Node.js usage)
- Modern browsers with WebAssembly support (for browser usage)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
