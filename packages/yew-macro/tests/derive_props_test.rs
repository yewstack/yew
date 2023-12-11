#[allow(dead_code)]
#[rustversion::attr(stable(1.64), test)]
fn derive_props() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_props/pass.rs");
    t.compile_fail("tests/derive_props/fail.rs");
}
