#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/functional_macro/*pass");
    t.compile_fail("tests/functional_macro/*fail.rs");
}
