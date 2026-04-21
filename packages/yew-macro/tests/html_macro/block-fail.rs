use yew::prelude::*;

fn compile_fail() {

    let not_tree = || ();
    html! {
        <div>{ not_tree() }</div>
    };
    html! {
        <div>{ for (0..3).map(|_| not_tree()) }</div>
    };
}

fn main() {}
