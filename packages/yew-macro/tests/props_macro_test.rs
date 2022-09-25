#[allow(dead_code)]
#[rustversion::attr(stable(1.60), test)]
fn props_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/props_macro/*-pass.rs");
    t.compile_fail("tests/props_macro/*-fail.rs");
}

#[test]
fn props_order() {
    yew::html!( <div ref={yew::NodeRef::default()} />);

    #[derive(Default)]
    struct Gen {
        state: usize,
    }
    impl Gen {
        fn next(&mut self) -> usize {
            self.state += 1;
            self.state
        }
    }

    #[derive(yew::Properties, PartialEq)]
    struct Props {
        my_first: usize,
        second: usize,
        last: usize,
    }

    let mut g = Gen::default();
    let props = yew::props!(Props {
        my_first: g.next(),
        second: g.next(),
        last: g.next(),
    });


    assert_eq!(props.my_first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}
