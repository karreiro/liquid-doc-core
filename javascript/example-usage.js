// Example of how to use the published package
import { parseLiquid } from 'liquid-doc-parse';

async function example() {
  try {
    // No need to call init() - it happens automatically!
    const template = "description here\n@param requiredParamWithNoType";
    const ast = await parseLiquid(template);
    console.log("Parsed AST:", ast);
  } catch (error) {
    console.error("Error:", error);
  }
}

example();
