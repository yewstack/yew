#[allow(dead_code)]
#[rustversion::attr(stable(1.85.0), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/function_component_attr/*-pass.rs");
    t.compile_fail("tests/function_component_attr/*-fail.rs");
}
