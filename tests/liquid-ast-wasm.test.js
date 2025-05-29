import fs from 'fs';
import path from 'path';

import * as wasm from '../web/pkg/wasm_liquiddoc_parser.js';

import { describe, it, expect, beforeAll } from 'vitest';

describe('Rust WASM LiquidDoc Parser', () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(
      __dirname,
      '../web/pkg/wasm_liquiddoc_parser_bg.wasm'
    );
    const wasmBytes = fs.readFileSync(wasmPath);
    await wasm.default(wasmBytes);
  });

  it('should add numbers', () => {
    const result = wasm.add_numbers(1, 2);
    console.log('add_numbers:', result);
    expect(result).toBeDefined();
  });

  it('should parse doc tags', () => {
    let ast = wasm.parse_liquid('@example hello');
    expect(ast).toBeDefined();
  });
});
