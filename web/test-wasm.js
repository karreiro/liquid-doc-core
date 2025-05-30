#!/usr/bin/env node

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testWasm() {
  try {
    console.log("🔧 Loading WASM parser...");
    const wasmModule = await import("./pkg/liquiddoc_wasm.js");

    // In Node.js, we need to provide the path to the WASM file
    const wasmPath = path.join(__dirname, "pkg", "liquiddoc_wasm_bg.wasm");
    const wasmBuffer = fs.readFileSync(wasmPath);
    await wasmModule.default(wasmBuffer);

    console.log("✅ WASM parser loaded successfully");

    // Test parsing
    const testInput = "{% doc %}This is a test document{% enddoc %}";
    console.log("🧪 Testing with input:", testInput);

    const result = wasmModule.parse_liquid(testInput);
    console.log("📄 Parse result:", JSON.stringify(result, null, 2));
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

testWasm();

