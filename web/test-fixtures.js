#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { benchmarkComparison } from './benchmark.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import parsers
let toLiquidHtmlAST, NodeTypes;
let pestParser = null;

async function loadParsers() {
    try {
        // Load Ohm.js parser
        const ohmModule = await import("@shopify/liquid-html-parser");
        toLiquidHtmlAST = ohmModule.toLiquidHtmlAST;
        NodeTypes = ohmModule.NodeTypes;
        console.log("✅ Ohm.js parser loaded");

        // Try to load Pest.rs WASM parser
        try {
            const wasmModule = await import('./pkg/wasm_liquiddoc_parser.js');
            await wasmModule.default();
            pestParser = wasmModule;
            console.log("✅ Pest.rs WASM parser loaded");
        } catch (wasmError) {
            console.warn("⚠️ Pest.rs WASM parser not available:", wasmError.message);
        }
    } catch (error) {
        console.error("❌ Failed to load parsers:", error);
        process.exit(1);
    }
}

function getFixtureFiles() {
    const fixturesDir = path.join(__dirname, 'fixtures');
    const files = fs.readdirSync(fixturesDir)
        .filter(file => file.endsWith('.liquid'))
        .sort((a, b) => {
            // Extract numbers for proper sorting
            const numA = parseInt(a.match(/\d+/)?.[0] || '0');
            const numB = parseInt(b.match(/\d+/)?.[0] || '0');
            return numA - numB;
        });
    
    return files.map(file => ({
        name: file,
        path: path.join(fixturesDir, file),
        content: fs.readFileSync(path.join(fixturesDir, file), 'utf8').trim()
    }));
}

function testFixture(fixture, iterations = 100) {
    console.log(`\n🧪 Testing ${fixture.name}`);
    console.log(`Content preview: ${fixture.content.substring(0, 60)}${fixture.content.length > 60 ? '...' : ''}`);
    
    const ohmInput = `{% doc %}\n${fixture.content}\n{% enddoc %}`;
    const pestInput = fixture.content;
    
    const benchmarks = [
        ["Ohm.js", () => toLiquidHtmlAST(ohmInput)]
    ];
    
    if (pestParser && pestParser.parse_liquid_doc_pest) {
        benchmarks.push(["Pest.rs", () => pestParser.parse_liquid_doc_pest(pestInput)]);
    }
    
    return benchmarkComparison(benchmarks, iterations);
}

function aggregateResults(allResults) {
    console.log("\n" + "=".repeat(60));
    console.log("📊 AGGREGATE RESULTS ACROSS ALL FIXTURES");
    console.log("=".repeat(60));
    
    const ohmTimes = [];
    const pestTimes = [];
    
    allResults.forEach(results => {
        if (results.length >= 1) ohmTimes.push(results[0].avgTime);
        if (results.length >= 2) pestTimes.push(results[1].avgTime);
    });
    
    if (ohmTimes.length > 0) {
        const ohmAvg = ohmTimes.reduce((a, b) => a + b, 0) / ohmTimes.length;
        console.log(`🔍 Ohm.js average across all fixtures: ${ohmAvg.toFixed(4)} ms`);
    }
    
    if (pestTimes.length > 0) {
        const pestAvg = pestTimes.reduce((a, b) => a + b, 0) / pestTimes.length;
        console.log(`⚡ Pest.rs average across all fixtures: ${pestAvg.toFixed(4)} ms`);
        
        if (ohmTimes.length > 0) {
            const ohmAvg = ohmTimes.reduce((a, b) => a + b, 0) / ohmTimes.length;
            const overallSpeedup = ohmAvg / pestAvg;
            console.log(`🚀 Overall speedup: ${overallSpeedup.toFixed(2)}x`);
        }
    }
}

async function main() {
    const iterations = process.argv[2] ? parseInt(process.argv[2]) : 100;
    
    console.log("🔧 Loading parsers...");
    await loadParsers();
    
    console.log("📁 Discovering fixture files...");
    const fixtures = getFixtureFiles();
    console.log(`Found ${fixtures.length} fixture files:`);
    fixtures.forEach(f => console.log(`  - ${f.name}`));
    
    const allResults = [];
    
    for (const fixture of fixtures) {
        try {
            const results = testFixture(fixture, iterations);
            allResults.push(results);
        } catch (error) {
            console.error(`❌ Error testing ${fixture.name}:`, error.message);
        }
    }
    
    aggregateResults(allResults);
}

main().catch(console.error);
