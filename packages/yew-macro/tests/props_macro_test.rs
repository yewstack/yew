#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn props_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/props_macro/*-pass.rs");
    t.compile_fail("tests/props_macro/*-fail.rs");
}
