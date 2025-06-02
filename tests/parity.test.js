import fs from 'fs';
import path from 'path';
import { describe, it, expect, beforeAll } from 'vitest';

import * as wasm from '../web/pkg/liquid_doc_wasm.js';
import { toLiquidHtmlAST } from '@shopify/liquid-html-parser';


describe('Rust WASM LiquidDoc Parser', async () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(
      __dirname,
      '../web/pkg/liquid_doc_wasm_bg.wasm'
    );
    const wasmBytes = fs.readFileSync(wasmPath);
    await wasm.default(wasmBytes);
  });

  it('should parse empty doc blocks correctly', () => {
    hasParity('');
  });

  it('should parse inline @example tags', () => {
    hasParity('@example simple inline example');
  });

  it('should parse multiple @param tags with various formats', () => {
    hasParity(`@param requiredParamWithNoType
@param {String} paramWithDescription - param with description and punctuation. This is still a valid param description.
@param {String} paramWithNoDescription
@param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
@param [optionalParameterWithDescription] - optional parameter description
@param {String} [optionalParameterWithType]
@unsupported this node falls back to a text node`);
  });

  it('should parse multiline @example tag with code description', () => {
    hasParity(`@example including inline code
This is a valid example
It can have multiple lines`);
  });

  it('should parse multiple consecutive @example tags', () => {
    hasParity(`@example
First Example
@example
Second Example`);
  });

  it('should parse mixed @example and @param tags with indentation', () => {
    hasParity(`@example
This is a valid example
It can have multiple lines
@param {String} paramWithDescription - param with description`);
  });

  it('should parse multiple @description tags with multiline content', () => {
    hasParity(`@description This is a description
@description This is another description
it can have multiple lines`);
  });

  it('should parse mixed @description, @example, and @param tags', () => {
    hasParity(`@description This is a description
@example This is an example
@param {String} paramWithDescription - param with description`);
  });

  it('should parse both implicit and explicit @description tags in the same block', () => {
    hasParity(`this is an implicit description
in a header
@description with a description annotation`);
  });

  // @prompt doesn't have parity: ohm splits prompts into two text nodes
  // it('should parse multiline @prompt tag', () => {
  //   hasParity(`@prompt
  // This is a prompt
  // It can have multiple lines`);
  // });

  // it('should parse AI generated block with @prompt and @param tags', () => {
  //   hasParity(`this block was AI generated
  // @prompt
  //   First prompt
  // @param {String} paramName - param description`);
  // });
});

function hasParity(input) {
  const pest = wasm.parse_liquid(input).nodes;
  let ohm = toLiquidHtmlAST('{% doc %}' + input + '{% enddoc %}').children[0]
    .body.nodes;

  // normalize position to ignore length of {% doc %} tags
  ohm = ohm.map((node) => normalizePosition(node));

  console.log('INPUT:  ', input);
  console.log('OHM:    ', JSON.stringify(ohm, null, 2));
  console.log('PEST:   ', JSON.stringify(pest, null, 2));

  for (let i = 0; i < ohm.length; i++) {
    const pestNode = pest[i];
    const ohmNode = ohm[i];

    expect(ohmNode.type).toBe(pestNode.type);
    expect(pestNode.position.start).toBe(ohmNode.position.start);
    expect(pestNode.position.end).toBe(ohmNode.position.end);

    if (
      ohmNode.type === 'LiquidDocParamNode' ||
      ohmNode.type === 'LiquidDocExampleNode' ||
      ohmNode.type === 'LiquidDocDescriptionNode'
    ) {
      expect(pestNode.name).toBe(ohmNode.name);
      expect(pestNode.isImplicit).toBe(ohmNode.isImplicit);
      expect(pestNode.isInline).toBe(ohmNode.isInline);
    }

    if (
      ohmNode.type === 'LiquidDocPromptNode' ||
      ohmNode.type === 'TextNode'
    ) {
      expect(pestNode.value).toBe(ohmNode.value);
    }

    if (ohmNode.content) {
      expect(pestNode.content.type).toBe(ohmNode.content.type);
      expect(pestNode.content.value).toBe(ohmNode.content.value);
      expect(pestNode.content.position.start).toBe(ohmNode.content.position.start);
      expect(pestNode.content.position.end).toBe(ohmNode.content.position.end);

      // source doesn't have parity: ohm provides the full source to each node
      // if (ohmNode.content.source) {
      //   const ohmContentSource = ohmNode.content.source
      //     .replace('{% doc %}', '')
      //     .replace('{% enddoc %}', '');
      //   expect(pestNode.content.source).toBe(ohmContentSource);
      // }
    }
  }
}


function normalizePosition(node, offset = 9) {
  const normalizedNode = JSON.parse(JSON.stringify(node));
  const adjustPositions = (obj) => {
    if (obj && typeof obj === 'object') {
      if (obj.position) {
        obj.position.start -= offset;
        obj.position.end -= offset;
      }
      Object.values(obj).forEach(adjustPositions);
    }
  };

  adjustPositions(normalizedNode);
  return normalizedNode;
}
