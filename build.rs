use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fixtures_macro.rs");

    let fixtures_dir = Path::new("web/fixtures");
    let entries = fs::read_dir(fixtures_dir)
        .expect("Failed to read fixtures directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension()?.to_str()? == "liquid" {
                path.file_stem()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let macro_content = format!(
        r#"
#[macro_export]
macro_rules! for_each_fixture_file {{
    ($callback:ident) => {{
        {}
    }};
}}
"#,
        entries
            .iter()
            .map(|name| format!("$callback!({:?});", name))
            .collect::<Vec<_>>()
            .join("\n        ")
    );

    fs::write(&dest_path, macro_content).unwrap();

    // Rebuild if fixtures directory changes
    println!("cargo:rerun-if-changed=web/fixtures");
}
