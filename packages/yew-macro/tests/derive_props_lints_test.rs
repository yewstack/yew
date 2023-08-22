#[allow(dead_code)]
#[cfg(yew_lints)]
#[rustversion::attr(nightly, test)]
fn test_derive_props_lints() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive_props_lints/fail.rs");
}
