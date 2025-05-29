use crate::parser::parse_liquid_string;
use std::fs;

// Include the generated constants and macros from the build script
include!(concat!(env!("OUT_DIR"), "/test_files.rs"));

// Re-export the generated constants for easy access from other modules
pub use {FIXTURE_NAMES, FIXTURE_PATHS};

/// Macro to generate a test function for a specific fixture file
macro_rules! generate_fixture_test {
    ($fixture_name:expr) => {
        paste::paste! {
            #[test]
            fn [<test_fixture_ $fixture_name>]() {
                let fixture_path = concat!("web/fixtures/", $fixture_name, ".liquid");
                let content = fs::read_to_string(fixture_path)
                    .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", fixture_path));

                // Test that the file can be parsed without errors
                let result = parse_liquid_string(&content, None);
                assert!(result.is_some(), "Failed to parse fixture: {}", $fixture_name);

                let ast = result.unwrap();

                // Basic validation that we got some nodes
                assert!(!ast.nodes.is_empty(), "No nodes found in fixture: {}", $fixture_name);

                // You can add more specific assertions here based on what you expect
                println!("✅ Successfully parsed {} with {} nodes", $fixture_name, ast.nodes.len());
            }
        }
    };
}

/// Macro to generate a benchmarking test for a specific fixture file
macro_rules! generate_fixture_benchmark {
    ($fixture_name:expr) => {
        paste::paste! {
            #[cfg(test)]
            fn [<bench_fixture_ $fixture_name>]() {
                let fixture_path = concat!("web/fixtures/", $fixture_name, ".liquid");
                let content = fs::read_to_string(fixture_path)
                    .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", fixture_path));

                let start = std::time::Instant::now();
                let iterations = 1000;

                for _ in 0..iterations {
                    let _result = parse_liquid_string(&content, None);
                }

                let duration = start.elapsed();
                let avg_time = duration.as_nanos() as f64 / iterations as f64;

                println!("📊 Benchmark {}: {:.2} ns/parse", $fixture_name, avg_time);
            }
        }
    };
}

/// Macro to generate a JSON serialization test for a specific fixture file
macro_rules! generate_fixture_json_test {
    ($fixture_name:expr) => {
        paste::paste! {
            #[test]
            fn [<test_fixture_ $fixture_name _json_roundtrip>]() {
                let fixture_path = concat!("web/fixtures/", $fixture_name, ".liquid");
                let content = fs::read_to_string(fixture_path)
                    .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", fixture_path));

                let ast = parse_liquid_string(&content, None)
                    .expect(&format!("Failed to parse fixture: {}", $fixture_name));

                // Test JSON serialization
                let json = serde_json::to_string(&ast)
                    .expect(&format!("Failed to serialize fixture {} to JSON", $fixture_name));

                // Test JSON deserialization
                let _deserialized: crate::ast::LiquidAST = serde_json::from_str(&json)
                    .expect(&format!("Failed to deserialize fixture {} from JSON", $fixture_name));

                println!("✅ JSON roundtrip successful for {}", $fixture_name);
            }
        }
    };
}

// Generate all fixture tests using the macro from build.rs
for_each_fixture_file!(generate_fixture_test);

// Generate all JSON roundtrip tests
for_each_fixture_file!(generate_fixture_json_test);

// Generate all benchmark tests
for_each_fixture_file!(generate_fixture_benchmark);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_file_names_macro() {
        let names = fixture_file_names!();
        assert!(!names.is_empty(), "Should have fixture file names");

        // Check that we have all 9 fixtures
        assert_eq!(names.len(), 9, "Should have 9 fixture files");

        // Check that all names start with "fixture"
        for name in names {
            assert!(
                name.starts_with("fixture"),
                "All fixture names should start with 'fixture': {}",
                name
            );
        }

        println!("📋 Available fixtures: {:?}", names);
    }

    #[test]
    fn test_fixture_file_paths_macro() {
        let paths = fixture_file_paths!();
        assert!(!paths.is_empty(), "Should have fixture file paths");

        // Check that all paths exist
        for path in paths {
            assert!(
                fs::metadata(path).is_ok(),
                "Fixture file should exist: {}",
                path
            );
        }

        println!("📁 Fixture paths verified: {:?}", paths);
    }

    #[test]
    fn test_all_fixtures_can_be_parsed() {
        let paths = fixture_file_paths!();
        let mut successful_parses = 0;

        for path in paths {
            let content = fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Failed to read fixture file: {}", path));

            if let Some(ast) = parse_liquid_string(&content, None) {
                successful_parses += 1;
                println!("✅ Parsed {}: {} nodes", path, ast.nodes.len());
            } else {
                println!("❌ Failed to parse {}", path);
            }
        }

        assert_eq!(
            successful_parses,
            paths.len(),
            "All fixtures should parse successfully"
        );
    }

    /// Helper function to run benchmarks on all fixtures
    #[test]
    fn benchmark_all_fixtures() {
        println!("\n🚀 Running benchmarks on all fixtures...");

        // Use the procedurally generated benchmark functions
        bench_fixture_fixture1();
        bench_fixture_fixture2();
        bench_fixture_fixture3();
        bench_fixture_fixture4();
        bench_fixture_fixture5();
        bench_fixture_fixture6();
        bench_fixture_fixture7();
        bench_fixture_fixture8();
        bench_fixture_fixture9();
    }
}
