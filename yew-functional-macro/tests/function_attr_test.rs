#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/function_attr/*-pass.rs");
    t.compile_fail("tests/function_attr/*-fail.rs");
}
