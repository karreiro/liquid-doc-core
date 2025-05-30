import initWasmModule, {
  parse_liquid as parseLiquidDoc,
} from "../node_modules/liquid_doc-wasm/wasm_liquiddoc_parser.js";

let wasmInitialized = false;
let wasmModule = null;
let parseLiquidDoc = null;

/**
 * Dynamically import the WASM module
 */
async function loadWasmModule() {
  if (typeof window !== 'undefined') {
    // Browser environment - use relative import
    const wasmJsUrl = new URL('../wasm/liquiddoc_parser.js', import.meta.url).href;
    return await import(wasmJsUrl);
  } else {
    // Node.js environment - use absolute path
    const path = await import('path');
    const url = await import('url');
    const currentDir = path.dirname(url.fileURLToPath(import.meta.url));
    const wasmJsPath = path.join(currentDir, '../wasm/liquiddoc_parser.js');
    const wasmJsUrl = url.pathToFileURL(wasmJsPath).href;
    return await import(wasmJsUrl);
  }
}

/**
 * Get WASM file location for the current environment
 */
async function getWasmLocation() {
  if (typeof window !== 'undefined') {
    // Browser environment
    return new URL('../wasm/liquiddoc_parser.wasm', import.meta.url).href;
  } else {
    // Node.js environment - read the file as bytes
    const fs = await import('fs/promises');
    const path = await import('path');
    const url = await import('url');

    const currentDir = path.dirname(url.fileURLToPath(import.meta.url));
    const wasmPath = path.join(currentDir, '../wasm/liquiddoc_parser.wasm');

    try {
      const wasmBytes = await fs.readFile(wasmPath);
      return wasmBytes;
    } catch (error) {
      throw new Error(`Could not read WASM file at ${wasmPath}: ${error.message}`);
    }
  }
}

/**
 * Initializes the WebAssembly module.
 * @param {string|Uint8Array} [wasmInput] - Optional WASM file path, URL, or bytes.
 * @returns {Promise<void>} A promise that resolves when initialization is complete.
 */
export async function init(wasmUrl) {
  if (wasmInitialized) {
    console.warn("Wasm module already initialized.");
    return;
  }
  // The initWasmModule function is the default export from the JS glue.
  // It often takes the URL of the .wasm file as an argument.
  // If your wasm-pack target handles wasm loading (e.g. with a bundler),
  // you might not need to pass wasmUrl.
  // For --target web, you usually pass the path to the _bg.wasm file.
  await initWasmModule(wasmUrl); // wasmUrl could be 'pkg/web/liquiddoc_wasm_bg.wasm'
  wasmInitialized = true;
  console.log("Liquid parser Wasm module initialized.");
}

/**
 * Parses a Liquid template string into an AST (Abstract Syntax Tree).
 * Automatically initializes WASM if not already done.
 * @param {string} input The Liquid template string.
 * @returns {Promise<any>} The JavaScript representation of the Liquid AST.
 * @throws {Error} If input is not a string or parsing fails.
 */
export function parseLiquid(input) {
  if (!wasmInitialized) {
    throw new Error("Wasm module not initialized. Call init() first.");
  }
  if (typeof input !== "string") {
    throw new TypeError("Input must be a string.");
  }
  try {
    // Call the imported Wasm function
    return parseLiquidDoc(input);
  } catch (error) {
    // The error from wasm-bindgen might be a JsValue.
    // You might want to process it further or re-throw.
    console.error("Error parsing Liquid:", error);
    throw error; // Re-throw the original error or a custom one
  }
}

// Optional: Export an object with all methods if preferred
// export default {
//   init,
//   parseLiquid
// };

