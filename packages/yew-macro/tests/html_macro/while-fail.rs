mod smth {
    const KEY: u32 = 42;
}

fn main() {
    _ = ::yew::html!{while};
    _ = ::yew::html!{while true};
    _ = ::yew::html!{while {} { <div/> }};

    _ = ::yew::html!{while true {
        <div key="duplicate" />
    }};

    _ = ::yew::html!{while true {
        <div key={smth::KEY} />
    }};
}
