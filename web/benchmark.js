// Simple agnostic benchmark utilities for liquid parsers

export class BenchmarkResult {
    constructor(name, times, totalTime) {
        this.name = name;
        this.times = times;
        this.totalTime = totalTime;
        this.iterations = times.length;
        this.avgTime = totalTime / this.iterations;
        this.throughput = 1000 / this.avgTime; // ops per second
    }

    log() {
        console.log(`✅ ${this.name} Results:`);
        console.log(`   Total time: ${this.totalTime.toFixed(2)} ms`);
        console.log(`   Average time: ${this.avgTime.toFixed(4)} ms`);
        console.log(`   Throughput: ${this.throughput.toFixed(2)} ops/sec`);
    }
}

export function benchmark(name, fn, iterations = 100) {
    console.log(`\n📊 Benchmarking ${name}...`);
    
    // Warm up
    for (let i = 0; i < 5; i++) {
        fn();
    }
    
    const times = [];
    const startTime = performance.now();
    
    for (let i = 0; i < iterations; i++) {
        const iterStart = performance.now();
        fn();
        const iterEnd = performance.now();
        times.push(iterEnd - iterStart);
    }
    
    const endTime = performance.now();
    const totalTime = endTime - startTime;
    
    return new BenchmarkResult(name, times, totalTime);
}

export function compare(results) {
    if (results.length < 2) return;

    console.log("\n🔥 PERFORMANCE COMPARISON:");
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    const [ohm, pest] = results;
    const speedRatio = ohm.avgTime / pest.avgTime;

    console.log(`🚀 Pest.rs is ${speedRatio.toFixed(2)}x FASTER than Ohm.js`);

    console.log(`   Ohm.js avg: ${ohm.avgTime.toFixed(4)} ms`);
    console.log(`   Pest.rs avg: ${pest.avgTime.toFixed(4)} ms`);
    console.log(`   Difference: ${Math.abs(ohm.avgTime - pest.avgTime).toFixed(4)} ms`);
}

export function benchmarkComparison(benchmarks, iterations = 100) {
    console.log("🚀 Starting benchmark comparison...");
    console.log(`Iterations: ${iterations}`);
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    const results = [];
    
    for (const [name, fn] of benchmarks) {
        const result = benchmark(name, fn, iterations);
        result.log();
        results.push(result);
    }
    
    if (results.length > 1) {
        compare(results);
    }
    
    return results;
}