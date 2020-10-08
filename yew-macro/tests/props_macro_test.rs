#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn tests() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/props_macro/props-fail.rs");

    t.pass("tests/props_macro/resolve-prop-pass.rs");
    t.compile_fail("tests/props_macro/resolve-prop-fail.rs");
}
