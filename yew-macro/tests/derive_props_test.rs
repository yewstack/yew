#[allow(dead_code)]
#[rustversion::attr(all(since(1.45), not(any(nightly, beta))), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_props/pass.rs");
    t.compile_fail("tests/derive_props/fail.rs");
}
