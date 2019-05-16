#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/html-block-pass.rs");
    t.compile_fail("tests/html-block-fail.rs");
    t.pass("tests/html-list-pass.rs");
    t.compile_fail("tests/html-list-fail.rs");
}
