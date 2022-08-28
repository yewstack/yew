#[allow(dead_code)]
#[rustversion::attr(stable(1.60), test)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/routable_derive/*-pass.rs");
    t.compile_fail("tests/routable_derive/*-fail.rs");
}

fn main() {}
