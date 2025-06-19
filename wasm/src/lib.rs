use wasm_bindgen::prelude::*;
use dbt_extractor::extract_from_source;

#[wasm_bindgen]
pub fn extract_from_source_js(src: &str) -> JsValue {
    let extraction = extract_from_source(src.as_bytes());
    serde_wasm_bindgen::to_value(&extraction).unwrap()
}
