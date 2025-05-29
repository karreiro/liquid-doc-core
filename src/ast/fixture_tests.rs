// In your test file
include!(concat!(env!("OUT_DIR"), "/fixtures_macro.rs"));

macro_rules! generate_fixture_test {
    ($fixture_name:expr) => {
        paste::paste! {
            #[test]
            fn [<test_fixture_ $fixture_name>]() {
                let content = include_str!(concat!("../../web/fixtures/", $fixture_name, ".liquid"));
                // Your test logic here
                crate::assert_json_output!(content);
            }
        }
    };
}

// This will generate all the tests
for_each_fixture_file!(generate_fixture_test);
