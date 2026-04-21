#[allow(dead_code)]
#[rustversion::attr(stable(1.85.0), test)]
fn test_html_deprecation() {
    let t = trybuild::TestCases::new();
    t.pass("tests/html_deprecation/pass.rs");
    t.compile_fail("tests/html_deprecation/fail.rs");
}
