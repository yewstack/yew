mod smth {
    const KEY: u32 = 42;
}

fn main() {
    _ = ::yew::html!{for x};
    _ = ::yew::html!{for x in};
    _ = ::yew::html!{for x in 0 .. 10};
    _ = ::yew::html!{for (x, y) in 0 .. 10 {
        <span>{x}</span>
    }};

    _ = ::yew::html!{for _ in 0 .. 10 {
        <span>{break}</span>
    }};

    _ = ::yew::html!{for _ in 0 .. 10 {
        <div key="duplicate" />
    }};

    _ = ::yew::html!{for _ in 0 .. 10 {
        <div key={smth::KEY} />
    }};
}
