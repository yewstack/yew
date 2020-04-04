#[allow(dead_code)]
#[rustversion::attr(stable(1.42), test)]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro/html-block-pass.rs");
    t.compile_fail("tests/macro/html-block-fail.rs");

    t.pass("tests/macro/html-component-pass.rs");
    t.compile_fail("tests/macro/html-component-fail.rs");
    t.compile_fail("tests/macro/html-component-fail-unimplemented.rs");

    t.pass("tests/macro/html-iterable-pass.rs");
    t.compile_fail("tests/macro/html-iterable-fail.rs");

    t.pass("tests/macro/html-list-pass.rs");
    t.compile_fail("tests/macro/html-list-fail.rs");

    t.pass("tests/macro/html-node-pass.rs");
    t.compile_fail("tests/macro/html-node-fail.rs");

    t.pass("tests/macro/html-tag-pass.rs");
    t.compile_fail("tests/macro/html-tag-fail.rs");
}
