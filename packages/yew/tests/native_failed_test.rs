#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
#[rustversion::attr(stable(1.56), test)]
fn native_failed_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failed_tests/*-fail.rs");
}
