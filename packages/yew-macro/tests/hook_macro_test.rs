#[allow(dead_code)]
#[rustversion::attr(stable(1.56), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/hook_macro/*-pass.rs");
    t.compile_fail("tests/hook_macro/*-fail.rs");
}
