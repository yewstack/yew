use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {
    a: usize,
}

fn compile_fail() {
    yew::props!(Props { ref: NodeRef::default(), key: "key" });
    yew::props!(Props { a: 5, fail: 10 });

    let props = yew::props!(Props { a: 1 });
    yew::props!(Props { a: 1, ..props });

    yew::props!(Props { does_not_exist });
}

fn main() {}
