#[allow(dead_code)]
#[rustversion::attr(stable(1.60), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/hook_attr/*-pass.rs");
    t.compile_fail("tests/hook_attr/*-fail.rs");
}
