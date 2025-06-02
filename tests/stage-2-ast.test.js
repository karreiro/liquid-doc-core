import fs from 'fs';
import path from 'path';

import * as wasm from '../web/pkg/liquid_doc_wasm.js';
import { toLiquidHtmlAST } from '@shopify/liquid-html-parser';

import { describe, it, expect, beforeAll } from 'vitest';

// based off https://github.com/Shopify/theme-tools/blob/main/packages/liquid-html-parser/src/stage-2-ast.spec.ts
describe('liquid-html-parser parity with wasm.parse_liquid', async () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(
      __dirname,
      '../web/pkg/liquid_doc_wasm_bg.wasm'
    );
    const wasmBytes = fs.readFileSync(wasmPath);
    await wasm.default(wasmBytes);
  });


  it('should parse multiple params', () => {
    const input = `        @param requiredParamWithNoType
        @param {String} paramWithDescription - param with description and \`punctuation\`. This is still a valid param description.
        @param {String} paramWithNoDescription
        @param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
        @param [optionalParameterWithDescription] - optional parameter description
        @param {String} [optionalParameterWithType]
        @unsupported this node falls back to a text node`;

    expect(input).toHaveParity('0.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('0.name', 'param');
    expect(input).toHaveParity('0.required', true);
    expect(input).toHaveParity('0.paramName.type', 'TextNode');
    expect(input).toHaveParity('0.paramName.value', 'requiredParamWithNoType');
    // pest returns undefined, ohm returns null
    // expect(input).toHaveParity('0.paramType', null);
    // expect(input).toHaveParity('0.paramDescription', null);

    expect(input).toHaveParity('1.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('1.name', 'param');
    expect(input).toHaveParity('1.required', true);
    expect(input).toHaveParity('1.paramName.type', 'TextNode');
    expect(input).toHaveParity('1.paramName.value', 'paramWithDescription');
    expect(input).toHaveParity('1.paramDescription.type', 'TextNode');
    expect(input).toHaveParity(
      '1.paramDescription.value',
      'param with description and `punctuation`. This is still a valid param description.'
    );
    expect(input).toHaveParity('1.paramType.type', 'TextNode');
    expect(input).toHaveParity('1.paramType.value', 'String');

    expect(input).toHaveParity('2.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('2.name', 'param');
    expect(input).toHaveParity('2.paramName.type', 'TextNode');
    expect(input).toHaveParity('2.paramName.value', 'paramWithNoDescription');
    // expect(input).toHaveParity('2.paramDescription', null);
    expect(input).toHaveParity('2.paramType.type', 'TextNode');
    expect(input).toHaveParity('2.paramType.value', 'String');

    expect(input).toHaveParity('3.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('3.name', 'param');
    expect(input).toHaveParity('3.required', false);
    expect(input).toHaveParity('3.paramName.type', 'TextNode');
    expect(input).toHaveParity(
      '3.paramName.value',
      'optionalParameterWithTypeAndDescription'
    );
    expect(input).toHaveParity(
      '3.paramDescription.value',
      'optional parameter with type and description'
    );
    expect(input).toHaveParity('3.paramType.type', 'TextNode');
    expect(input).toHaveParity('3.paramType.value', 'String');

    expect(input).toHaveParity('4.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('4.name', 'param');
    expect(input).toHaveParity('4.required', false);
    expect(input).toHaveParity('4.paramName.type', 'TextNode');
    expect(input).toHaveParity('4.paramName.value', 'optionalParameterWithDescription');
    expect(input).toHaveParity('4.paramDescription.type', 'TextNode');
    expect(input).toHaveParity(
      '4.paramDescription.value',
      'optional parameter description'
    );
    // expect(input).toHaveParity('4.paramType', null);

    expect(input).toHaveParity('5.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('5.name', 'param');
    expect(input).toHaveParity('5.required', false);
    expect(input).toHaveParity('5.paramName.type', 'TextNode');
    expect(input).toHaveParity('5.paramName.value', 'optionalParameterWithType');
    // expect(input).toHaveParity('5.paramDescription', null);
    expect(input).toHaveParity('5.paramType.type', 'TextNode');
    expect(input).toHaveParity('5.paramType.value', 'String');

    expect(input).toHaveParity('6.type', 'TextNode');
    expect(input).toHaveParity(
      '6.value',
      '@unsupported this node falls back to a text node'
    );
  });

  it('should parse examples', () => {
    const input = `@example simple inline example`;
    
    expect(input).toHaveParity('0.name', 'example');
    expect(input).toHaveParity('0.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('0.content.type', 'TextNode');
    expect(input).toHaveParity('0.content.value', 'simple inline example');
  });

  it('should parse examples with inline code', () => {
    const input = `        @example including inline code
        This is a valid example
        It can have multiple lines`;
    expect(input).toHaveParity('0.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('0.name', 'example');
    expect(input).toHaveParity(
      '0.content.value',
      'including inline code\n        This is a valid example\n        It can have multiple lines'
    );
  });

  it('should parse multiple descriptions', () => {
    const input = `        @description This is a description
        @description This is another description
        it can have multiple lines`;
    expect(input).toHaveParity('0.type', 'LiquidDocDescriptionNode');
    expect(input).toHaveParity('0.content.value', 'This is a description\n');
    expect(input).toHaveParity('1.type', 'LiquidDocDescriptionNode');
    expect(input).toHaveParity(
      '1.content.value',
      'This is another description\n        it can have multiple lines'
    );
  });

  it('should parse descriptions with examples and params', () => {
    const input = `@description This is a description
@example This is an example
@param {String} paramWithDescription - param with description`;
    expect(input).toHaveParity('0.type', 'LiquidDocDescriptionNode');
    expect(input).toHaveParity('0.content.value', 'This is a description\n');
    expect(input).toHaveParity('1.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('1.name', 'example');
    expect(input).toHaveParity('1.content.value', 'This is an example\n');
    expect(input).toHaveParity('2.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('2.name', 'param');
    expect(input).toHaveParity('2.paramName.value', 'paramWithDescription');
    expect(input).toHaveParity('2.paramDescription.value', 'param with description');
  });


  it('should parse multiple examples', () => {
    const input = `@example
First Example
@example
Second Example`;

    expect(input).toHaveParity('0.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('0.name', 'example');
    expect(input).toHaveParity('0.content.value', 'First Example\n');
    expect(input).toHaveParity('1.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('1.name', 'example');
    expect(input).toHaveParity('1.content.value', 'Second Example');
  });

  it('should parse implicit descriptions', () => {
    const input = `this is an implicit description
in a header

@description with a description annotation`;
    expect(input).toHaveParity('0.type', 'LiquidDocDescriptionNode');
    expect(input).toHaveParity(
      '0.content.value',
      'this is an implicit description\nin a header\n\n'
    );
    expect(input).toHaveParity('0.isImplicit', true);
    expect(input).toHaveParity('1.type', 'LiquidDocDescriptionNode');
    expect(input).toHaveParity('1.content.value', 'with a description annotation');
    expect(input).toHaveParity('1.isImplicit', false);
  });


  it('should parse examples with params', () => {
    const input = `@example
This is a valid example
It can have multiple lines
@param {String} paramWithDescription - param with description`;
    expect(input).toHaveParity('0.type', 'LiquidDocExampleNode');
    expect(input).toHaveParity('0.name', 'example');
    expect(input).toHaveParity(
      '0.content.value',
      'This is a valid example\nIt can have multiple lines\n'
    );
    expect(input).toHaveParity('1.type', 'LiquidDocParamNode');
    expect(input).toHaveParity('1.name', 'param');
    expect(input).toHaveParity('1.paramName.value', 'paramWithDescription');
    expect(input).toHaveParity('1.paramDescription.value', 'param with description');
  });

// @prompt nodes are treated as text nodes
//   it('should parse prompts', () => {
//     const input = `  @prompt
// This is a prompt
// It can have multiple lines`;
//     expect(input).toHaveParity('0.type', 'LiquidDocPromptNode');
//     expect(input).toHaveParity('0.name', 'prompt')
//     expect(input).toHaveParity(
//       '0.content.value',
//       '\n    This is a prompt\n    It can have multiple lines\n'
//     );
//   });

// @prompt nodes are treated as text nodes
//   it('should parse descriptions with prompts and params', () => {
//     const input = `this block was AI generated

// @prompt
//   First prompt

// @param {String} paramName - param description`;

//     expect(input).toHaveParity('0.type', 'LiquidDocDescriptionNode');
//     expect(input).toHaveParity('0.content.value', 'this block was AI generated\n\n');


//     expect(input).toHaveParity('1.type', 'LiquidDocPromptNode');
//     expect(input).toHaveParity('1.name', 'prompt');
//     expect(input).toHaveParity('1.content.value', '\n    First prompt\n\n');
//     expect(input).toHaveParity('2.type', 'LiquidDocParamNode');
//     expect(input).toHaveParity('2.paramName.value', 'paramName');
//   });
});

function deepGet(path, obj) {
  return path.split('.').reduce((curr, k) => {
    if (curr && curr[k] !== undefined) return curr[k];
    return undefined;
  }, obj);
}

expect.extend({
  toHaveParity(input, path, expectedValue) {
    const pest = wasm.parse_liquid(input).nodes;
    let ohm = toLiquidHtmlAST('{% doc %}' + input + '{% enddoc %}').children[0]
      .body.nodes;
    
    const pestValue = deepGet(path, pest);
    const ohmValue = deepGet(path, ohm);
    
    const pestEqualsExpected = pestValue === expectedValue;
    const ohmEqualsExpected = ohmValue === expectedValue;
    const equalsExpected = pestEqualsExpected && ohmEqualsExpected;
    
    return {
      pass: equalsExpected,
      message: () => {
        if (!equalsExpected) {
          return `Expected both parsers to have ${path} = ${expectedValue}\n` +
                 `  PEST: ${pestValue}\n` +
                 `  OHM:  ${ohmValue}`;
        } else if (!pestEqualsExpected) {
          return `PEST parser mismatch at ${path}\n` +
                 `  Expected: ${expectedValue}\n` +
                 `  Got:      ${pestValue}`;
        } else if (!ohmEqualsExpected) {
          return `OHM parser mismatch at ${path}\n` +
                 `  Expected: ${expectedValue}\n` +
                 `  Got:      ${ohmValue}`;
        }
      }
    };
  }
});

