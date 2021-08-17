#[allow(dead_code)]
#[rustversion::attr(stable(1.51), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/custom_event/custom_event_pass.rs");
    t.compile_fail("tests/custom_event/custom_event_fail.rs");
}
