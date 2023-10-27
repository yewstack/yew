use yew::prelude::*;

fn main() {
    divan::main();
}

#[function_component]
fn Stuff(_: &()) -> Html {
    html! {
        <p>{"A custom component"}</p>
    }
}

#[divan::bench(sample_size = 10000000)]
fn vnode_clone(bencher: divan::Bencher) {
    let html = html! {
        <div class={classes!("hello-world")}>
            <span>{"Hello"}</span>
            <strong style="color:red">{"World"}</strong>
            <Stuff />
            <Suspense fallback={html!("Loading...")}>
                <Stuff />
            </Suspense>
        </div>
    };

    bencher.bench_local(move || {
        let _ = divan::black_box(html.clone());
    });
}
