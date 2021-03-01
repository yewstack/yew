use yew::prelude::*;

fn compile_fail() {
    html! {
        <>
            { () }
        </>
    };

    html! {
        <>123</>
    }

    let not_tree = || ();
    html! {
        <div>{ not_tree() }</div>
    };
    html! {
        <>{ for (0..3).map(|_| not_tree()) }</>
    };
}

fn main() {}
