#[allow(dead_code)]
#[cfg(yew_lints)]
#[rustversion::attr(nightly, test)]
fn test_html_lints() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/html_lints/fail.rs");
}
