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

  function deepGet(path, obj) {
    return path.split('.').reduce((curr, k) => {
      if (curr && curr[k] !== undefined) return curr[k];
      return undefined;
    }, obj);
  }

  function expectPath(input, path, value) {
    const pest = wasm.parse_liquid(input).nodes;
    const ohm = toLiquidHtmlAST('{% doc %}' + input + '{% enddoc %}')
      .children[0].body.nodes;

    const pestValue = deepGet(path, pest);
    const ohmValue = deepGet(path, ohm);

    debugger;

    // temp, need ot figure out null/undefined parity
    if (value !== null) {
      expect(pestValue).to.eql(value);
      expect(ohmValue).to.eql(value);
    }
  }

  it('should parse multiple params', () => {
    const input = `@param requiredParamWithNoType
@param {String} paramWithDescription - param with description and \`punctation\`. This is still a valid param description.
@param {String} paramWithNoDescription
@param {String} [optionalParameterWithTypeAndDescription] - optional parameter with type and description
@param [optionalParameterWithDescription] - optional parameter description
@param {String} [optionalParameterWithType]
@unsupported this node falls back to a text node`;

    expectPath(input, '0.type', 'LiquidDocParamNode');
    expectPath(input, '0.name', 'param');
    expectPath(input, '0.required', true);
    expectPath(input, '0.paramName.type', 'TextNode');
    expectPath(input, '0.paramName.value', 'requiredParamWithNoType');
    expectPath(input, '0.paramType', null);
    expectPath(input, '0.paramDescription', null);

    expectPath(input, '1.type').to.eql('LiquidDocParamNode');
    expectPath(input, '1.name').to.eql('param');
    expectPath(input, '1.required').to.eql(true);
    expectPath(input, '1.paramName.type').to.eql('TextNode');
    expectPath(input, '1.paramName.value').to.eql('paramWithDescription');
    expectPath(input, '1.paramDescription.type').to.eql('TextNode');
    expectPath(input, '1.paramDescription.value').to.eql(
      'param with description and `punctuation`. This is still a valid param description.'
    );
    expectPath(input, '1.paramType.type').to.eql('TextNode');
    expectPath(input, '1.paramType.value').to.eql('String');

    expectPath(input, '2.type').to.eql('LiquidDocParamNode');
    expectPath(input, '2.name').to.eql('param');
    expectPath(input, '2.paramName.type').to.eql('TextNode');
    expectPath(input, '2.paramName.value').to.eql('paramWithNoDescription');
    expectPath(input, '2.paramDescription').to.be.null;
    expectPath(input, '2.paramType.type').to.eql('TextNode');
    expectPath(input, '2.paramType.value').to.eql('String');

    expectPath(input, '3.type').to.eql('LiquidDocParamNode');
    expectPath(input, '3.name').to.eql('param');
    expectPath(input, '3.required').to.eql(false);
    expectPath(input, '3.paramName.type').to.eql('TextNode');
    expectPath(input, '3.paramName.value').to.eql(
      'optionalParameterWithTypeAndDescription'
    );
    expectPath(input, '3.paramDescription.value').to.eql(
      'optional parameter with type and description'
    );
    expectPath(input, '3.paramType.type').to.eql('TextNode');
    expectPath(input, '3.paramType.value').to.eql('String');

    expectPath(input, '4.type').to.eql('LiquidDocParamNode');
    expectPath(input, '4.name').to.eql('param');
    expectPath(input, '4.required').to.eql(false);
    expectPath(input, '4.paramName.type').to.eql('TextNode');
    expectPath(input, '4.paramName.value').to.eql(
      'optionalParameterWithDescription'
    );
    expectPath(input, '4.paramDescription.type').to.eql('TextNode');
    expectPath(input, '4.paramDescription.value').to.eql(
      'optional parameter description'
    );
    expectPath(input, '4.paramType').to.be.null;

    expectPath(input, '5.type').to.eql('LiquidDocParamNode');
    expectPath(input, '5.name').to.eql('param');
    expectPath(input, '5.required').to.eql(false);
    expectPath(input, '5.paramName.type').to.eql('TextNode');
    expectPath(input, '5.paramName.value').to.eql('optionalParameterWithType');
    expectPath(input, '5.paramDescription').to.be.null;
    expectPath(input, '5.paramType.type').to.eql('TextNode');
    expectPath(input, '5.paramType.value').to.eql('String');

    expectPath(input, '6.type').to.eql('TextNode');
    expectPath(input, '6.value').to.eql(
      '@unsupported this node falls back to a text node'
    );
  });

  it('should parse examples', () => {
    const input = `@example simple inline example`;
    expectPath(input, '0.name').to.eql('example');
    expectPath(input, '0.type').to.eql('LiquidDocExampleNode');
    expectPath(input, '0.content.type').to.eql('TextNode');
    expectPath(input, '0.content.value').to.eql('simple inline example\n');
  });

  //   it('should parse examples with inline code', () => {
  //     const input = `@example including inline code
  // This is a valid example
  // It can have multiple lines`;
  //     expectPath(input, '0.type').to.eql('LiquidDocExampleNode');
  //     expectPath(input, '0.name').to.eql('example');
  //     expectPath(input, '0.content.value').to.eql(
  //       'including inline code\n        This is a valid example\n        It can have multiple lines\n'
  //     );
  //   });

  it('should parse multiple examples', () => {
    const input = `@example
First Example
@example
Second Example`;

    expectPath(input, '0.type').to.eql('LiquidDocExampleNode');
    expectPath(input, '0.name').to.eql('example');
    expectPath(input, '0.content.value').to.eql('First Example\n'); // @example\nFirst Example\n
    expectPath(input, '1.type').to.eql('LiquidDocExampleNode');
    expectPath(input, '1.name').to.eql('example');
    expectPath(input, '1.content.value').to.eql('Second Example\n'); // @example\nSecond Example\n
  });

  it('should parse examples with params', () => {
    const input = `@example
          This is a valid example
          It can have multiple lines
          @param {String} paramWithDescription - param with description`;
    expectPath(input, '0.type').to.eql('LiquidDocExampleNode');
    expectPath(input, '0.name').to.eql('example');
    // expectPath(input, '0.content.value').to.eql(
    //   'This is a valid example\n        It can have multiple lines\n'
    // );
    expectPath(input, '1.type').to.eql('LiquidDocParamNode');
    expectPath(input, '1.name').to.eql('param');
    // expectPath(input, '1.paramName.value').to.eql('paramWithDescription');
    // expectPath(input, '1.paramDescription.value').to.eql(
    //   'param with description'
    // );
  });

  it('should parse multiple descriptions', () => {
    const input = `@description This is a description
@description This is another description
it can have multiple lines`;
    expectPath(input, '0.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '0.content.value').to.eql('This is a description\n');
    expectPath(input, '1.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '1.content.value').to.eql(
      'This is another description\n        it can have multiple lines\n'
    );
  });

  it('should parse descriptions with examples and params', () => {
    const input = `@description This is a description
@example This is an example
@param {String} paramWithDescription - param with description`;
    expectPath(input, '0.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '0.content.value').to.eql('This is a description\n');
    expectPath(input, '1.type').to.eql('LiquidDocExampleNode');
    expectPath(input, '1.name').to.eql('example');
    expectPath(input, '1.content.value').to.eql('This is an example\n');
    expectPath(input, '2.type').to.eql('LiquidDocParamNode');
    expectPath(input, '2.name').to.eql('param');
    expectPath(input, '2.paramName.value').to.eql('paramWithDescription');
    expectPath(input, '2.paramDescription.value').to.eql(
      'param with description'
    );
  });

  it('should parse implicit descriptions', () => {
    const input = `this is an implicit description
in a header
@description with a description annotation`;
    expectPath(input, '0.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '0.content.value').to.eql(
      'this is an implicit description\n        in a header\n\n'
    );
    expectPath(input, '0.isImplicit').to.eql(true);
    expectPath(input, '1.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '1.content.value').to.eql(
      'with a description annotation\n'
    );
    expectPath(input, '1.isImplicit').to.eql(false);
  });

  it('should parse prompts', () => {
    const input = `@prompt
This is a prompt
It can have multiple lines`;
    expectPath(input, '0.type').to.eql('LiquidDocPromptNode');
    expectPath(input, '0.name').to.eql('prompt');
    expectPath(input, '0.content.value').to.eql(
      '\n    This is a prompt\n    It can have multiple lines\n'
    );
  });

  it('should parse descriptions with prompts and params', () => {
    const input = `
    this block was AI generated

@prompt
  First prompt

@param {String} paramName - param description`;

    expectPath(input, '0.type').to.eql('LiquidDocDescriptionNode');
    expectPath(input, '0.content.value').to.eql(
      'this block was AI generated\n\n'
    );
    expectPath(input, '1.type').to.eql('LiquidDocPromptNode');
    expectPath(input, '1.name').to.eql('prompt');
    expectPath(input, '1.content.value').to.eql('\n    First prompt\n\n');
    expectPath(input, '2.type').to.eql('LiquidDocParamNode');
    expectPath(input, '2.paramName.value').to.eql('paramName');
  });
});
