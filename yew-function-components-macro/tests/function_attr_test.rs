#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/function_macro/*pass");
    t.compile_fail("tests/function_macro/*fail.rs");
}
