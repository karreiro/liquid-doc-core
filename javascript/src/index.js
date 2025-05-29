import initWasmModule, { parse_liquid as parseLiquidDoc } from '../node_modules/wasm-liquiddoc-parser/wasm_liquiddoc_parser.js';

let wasmInitialized = false;

/**
 * Initializes the WebAssembly module.
 * This must be called once before any other parser functions can be used.
 * @param {string} [wasmUrl] - Optional URL to the .wasm file.
 *                             If not provided, it defaults to 'liquid_engine_bg.wasm'
 *                             relative to the generated JS glue file.
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
    await initWasmModule(wasmUrl); // wasmUrl could be 'pkg/web/wasm_liquiddoc_parser_bg.wasm'
    wasmInitialized = true;
    console.log("Liquid parser Wasm module initialized.");
}

/**
 * Parses a Liquid template string into an AST (Abstract Syntax Tree).
 * @param {string} input The Liquid template string.
 * @returns {any} The JavaScript representation of the Liquid AST.
 * @throws {Error} If the Wasm module is not initialized or if input is not a string.
 */
export function parseLiquid(input) {
    if (!wasmInitialized) {
        throw new Error("Wasm module not initialized. Call init() first.");
    }
    if (typeof input !== 'string') {
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