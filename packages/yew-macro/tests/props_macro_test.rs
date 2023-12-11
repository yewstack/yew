#[allow(dead_code)]
#[rustversion::attr(stable(1.64), test)]
fn props_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/props_macro/*-pass.rs");
    t.compile_fail("tests/props_macro/*-fail.rs");
}

#[test]
fn props_order() {
    #[derive(yew::Properties, PartialEq)]
    struct Props {
        first: usize,
        second: usize,
        last: usize,
    }

    let mut g = 1..=3;
    let props = yew::props!(Props {
        first: g.next().unwrap(),
        second: g.next().unwrap(),
        last: g.next().unwrap()
    });

    assert_eq!(props.first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}
