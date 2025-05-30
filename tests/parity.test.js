import fs from 'fs';
import path from 'path';

import * as wasm from '../web/pkg/wasm_liquiddoc_parser.js';
import { toLiquidHtmlAST } from '@shopify/liquid-html-parser';

import { describe, it, expect, beforeAll } from 'vitest';

describe('Rust WASM LiquidDoc Parser', async () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(
      __dirname,
      '../web/pkg/wasm_liquiddoc_parser_bg.wasm'
    );
    const wasmBytes = fs.readFileSync(wasmPath);
    await wasm.default(wasmBytes);
  });

  const cases = [
    // [('empty', '')],
    // ['@example', '@example simple inline example'],
    [
      'multiple params',
      `@param requiredParamWithNoType
@param {String} paramWithDescription - param with description and punctation. This is still a valid param description.
@param {String} paramWithNoDescription
@param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
@param [optionalParameterWithDescription] - optional parameter description
@param {String} [optionalParameterWithType]
@unsupported this node falls back to a text node`,
    ],
    [
      'multiline example',
      `@example including inline code
This is a valid example
It can have multiple lines`,
    ],
    [
      'multiple @example tags',
      `@example
First Example
@example
Second Example`,
    ],
    [
      '@example and @param',
      `@example
This is a valid example
It can have multiple lines
@param {String} paramWithDescription - param with description`,
    ],
    [
      'multiple @description',
      `@description This is a description
@description This is another description
it can have multiple lines`,
    ],
    [
      '@description, @example, @param',
      `@description This is a description
@example This is an example
@param {String} paramWithDescription - param with description`,
    ],
    [
      'implicit and explicit @description',
      `this is an implicit description
in a header
@description with a description annotation`,
    ],
    [
      '@prompt multiline',
      `@prompt
This is a prompt
It can have multiple lines`,
    ],
    [
      'AI generated block with @prompt and @param',
      `this block was AI generated
@prompt
  First prompt
@param {String} paramName - param description`,
    ],
  ];

  describe('Rust WASM LiquidDoc Parser', () => {
    it.each(cases)('%s', (_, input) => {
      parseLiquid(input);
    });
  });
});

function parseLiquid(input) {
  console.log('INPUT: ', input);

  const pest = wasm.parse_liquid(input).nodes;

  // we don't care about the document, LiquidRawTag, or RawMarkup stuff - just the nodes
  let ohm = toLiquidHtmlAST('{% doc %}' + input + '{% enddoc %}').children[0]
    .body.nodes;

  debugger;

  // for the sake of readability, we are normalizing position to account for the {% doc %} tags
  // ohm = ohm.map((node) => {
  //   const normalizedNode = JSON.parse(JSON.stringify(node));
  //   const adjustPositions = (obj) => {
  //     if (obj && typeof obj === 'object') {
  //       if (obj.position) {
  //         obj.position.start -= 9;
  //         obj.position.end -= 9;
  //       }
  //       Object.values(obj).forEach(adjustPositions);
  //     }
  //   };

  //   adjustPositions(normalizedNode);
  //   return normalizedNode;
  // });

  console.log('OG: ', JSON.stringify(ohm, null, 2));
  console.log('PEST: ', JSON.stringify(pest, null, 2));

  debugger;

  for (let i = 0; i < pest.length; i++) {
    const pestNode = pest[i];
    const ohmNode = ohm[i];

    expect(pestNode.type).toBeDefined();
    expect(ohmNode.type).toBeDefined();
    expect(pestNode.type).toBe(ohmNode.type);

    if (
      pestNode.type === 'LiquidDocParamNode' ||
      pestNode.type === 'LiquidDocExampleNode' ||
      pestNode.type === 'LiquidDocDescriptionNode'
    ) {
      expect(pestNode.name).toBe(ohmNode.name);
    }

    if (
      pestNode.type === 'LiquidDocPromptNode' ||
      pestNode.type === 'TextNode'
    ) {
      expect(pestNode.value).toBe(ohmNode.value);
    }

    // if (pestNode.position && typeof pestNode.position === 'object') {
    //   if (pestNode.position) {
    //     expect(pestNode.position.start).toBe(ohmNode.position.start);
    //     expect(pestNode.position.end).toBe(ohmNode.position.end);
    //   }
    // }
  }
}
