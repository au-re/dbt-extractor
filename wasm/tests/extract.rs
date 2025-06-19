use dbt_extractor_wasm::extract_from_source_js;
use dbt_extractor::{Extraction, DbtRef};
use serde_wasm_bindgen::from_value;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn extract_from_source_js_returns_expected() {
    let js_val = extract_from_source_js("{{ ref('my_table') }}");
    let result: Extraction = from_value(js_val).unwrap();
    assert_eq!(
        result,
        Extraction::populate(
            Some(vec![DbtRef {
                name: "my_table".to_string(),
                package: None,
                version: None,
            }]),
            None,
            None,
        )
    );
}
