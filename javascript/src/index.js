import initWasmModule, {
  parse_liquid as parseLiquidDocWasm,
} from "../wasm/liquiddoc_parser.js";

let wasmInitialized = false;
let wasmInitPromise = null;

/**
 * Get WASM data for the current environment
 */
async function getWasmData() {
  if (typeof window !== 'undefined') {
    // Browser environment - return URL
    return new URL('../wasm/liquiddoc_parser.wasm', import.meta.url).href;
  } else {
    // Node.js environment - read file as bytes
    const { readFile } = await import('fs/promises');
    const { dirname, join } = await import('path');
    const { fileURLToPath } = await import('url');

    const currentDir = dirname(fileURLToPath(import.meta.url));
    const wasmPath = join(currentDir, '../wasm/liquiddoc_parser.wasm');

    try {
      const wasmBytes = await readFile(wasmPath);
      return wasmBytes;
    } catch (error) {
      throw new Error(`Could not read WASM file at ${wasmPath}: ${error.message}`);
    }
  }
}

/**
 * Auto-initialize the WebAssembly module
 */
async function autoInit() {
  if (wasmInitialized) {
    return;
  }

  if (wasmInitPromise) {
    return wasmInitPromise;
  }

  wasmInitPromise = (async () => {
    try {
      const wasmData = await getWasmData();
      await initWasmModule({ module_or_path: wasmData });
      wasmInitialized = true;
      console.log("Liquid parser WASM module auto-initialized.");
    } catch (error) {
      console.error("Failed to auto-initialize WASM module:", error);
      throw error;
    }
  })();

  return wasmInitPromise;
}

/**
 * Initializes the WebAssembly module manually (optional).
 * @param {string|Uint8Array} [wasmInput] - Optional WASM file path, URL, or bytes.
 * @returns {Promise<void>} A promise that resolves when initialization is complete.
 */
export async function init(wasmInput) {
  if (wasmInitialized) {
    console.warn("WASM module already initialized.");
    return;
  }

  if (wasmInitPromise) {
    return wasmInitPromise;
  }

  wasmInitPromise = (async () => {
    try {
      const wasmData = wasmInput || await getWasmData();
      await initWasmModule({ module_or_path: wasmData });
      wasmInitialized = true;
      console.log("Liquid parser WASM module initialized.");
    } catch (error) {
      console.error("Failed to initialize WASM module:", error);
      throw error;
    }
  })();

  return wasmInitPromise;
}

/**
 * Parses a Liquid template string into an AST (Abstract Syntax Tree).
 * Automatically initializes WASM if not already done.
 * @param {string} input The Liquid template string.
 * @returns {Promise<any>} The JavaScript representation of the Liquid AST.
 * @throws {Error} If input is not a string or parsing fails.
 */
export async function parseLiquid(input) {
  // Auto-initialize if not already done
  if (!wasmInitialized) {
    await autoInit();
  }

  if (typeof input !== "string") {
    throw new TypeError("Input must be a string.");
  }

  try {
    // Call the imported WASM function
    return parseLiquidDocWasm(input);
  } catch (error) {
    console.error("Error parsing Liquid:", error);
    throw error;
  }
}

// Start auto-initialization immediately when module is imported
autoInit().catch(error => {
  console.warn("Auto-initialization failed, will retry when parseLiquid is called:", error.message);
});

// Optional: Export an object with all methods if preferred
// export default {
//   init,
//   parseLiquid
// };

