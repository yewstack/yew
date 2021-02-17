#[allow(dead_code)]
#[cfg(feature = "lints")]
#[rustversion::attr(nightly, test)]
fn test_html_lints() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/html_lints/fail.rs");
}
