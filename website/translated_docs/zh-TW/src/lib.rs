#[macro_use]
extern crate doc_comment;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_sample_app() {
    doctest!("getting-started/build-a-sample-app.md");
}

#[wasm_bindgen_test]
fn test_optimisations() {
    doctest!("advanced-topics/optimizations.md");
}

#[wasm_bindgen_test]
fn test_properties() {
    doctest!("concepts/components/properties.md");
}