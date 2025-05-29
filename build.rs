// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("test_files.rs");

    let dir_path = "web/fixtures";
    let entries = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("liquid")) {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Generate multiple useful macros
    let macro_definition = format!(
        r#"
// Macro to iterate over each test file name
#[macro_export]
macro_rules! for_each_fixture_file {{
    ($macro:ident) => {{
        {}
    }}
}}

// Macro that expands to an array of all fixture file names
#[macro_export]
macro_rules! fixture_file_names {{
    () => {{
        &[{}]
    }}
}}

// Macro that expands to an array of all fixture file paths
#[macro_export]
macro_rules! fixture_file_paths {{
    () => {{
        &[{}]
    }}
}}
"#,
        entries
            .iter()
            .map(|name| format!("        $macro!({:?});", name))
            .collect::<Vec<_>>()
            .join("\n"),
        entries
            .iter()
            .map(|name| format!("{:?}", name))
            .collect::<Vec<_>>()
            .join(", "),
        entries
            .iter()
            .map(|name| format!("{:?}", format!("web/fixtures/{}.liquid", name)))
            .collect::<Vec<_>>()
            .join(", ")
    );

    fs::write(&dest_path, macro_definition).unwrap();

    // Rebuild if the directory changes
    println!("cargo:rerun-if-changed={}", dir_path);
}
