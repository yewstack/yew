#[allow(dead_code)]
#[rustversion::attr(stable(1.60), test)]
fn classes_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/classes_macro/*-pass.rs");
    t.compile_fail("tests/classes_macro/*-fail.rs");
}
