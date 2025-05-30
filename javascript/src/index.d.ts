/**
 * Initializes the WebAssembly module.
 * @param wasmUrl - Optional URL to the .wasm file.
 * @returns A promise that resolves when initialization is complete.
 */
export function init(wasmUrl?: string): Promise<void>;

/**
 * Parses a Liquid template string into an AST (Abstract Syntax Tree).
 * Automatically initializes WASM if not already done.
 * @param input - The Liquid template string.
 * @returns The JavaScript representation of the Liquid AST.
 * @throws Error if input is not a string or parsing fails.
 */
export function parseLiquid(input: string): Promise<any>;

/**
 * Check if the WASM module is initialized
 * @returns True if initialized, false otherwise
 */
export function isInitialized(): boolean;

/**
 * Alias for init function
 */
export function initWasm(wasmUrl?: string): Promise<void>;

declare const _default: {
    init: typeof init;
    parseLiquid: typeof parseLiquid;
    isInitialized: typeof isInitialized;
};

export default _default;
