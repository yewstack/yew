#[allow(dead_code)]
#[rustversion::attr(stable(1.42), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/derive_props/pass.rs");
    t.compile_fail("tests/derive_props/fail.rs");
}
