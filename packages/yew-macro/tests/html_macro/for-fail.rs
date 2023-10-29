fn main() {
    _ = ::yew::html!{for x};
    _ = ::yew::html!{for x in};
    _ = ::yew::html!{for x in 0 .. 10};
    _ = ::yew::html!{for (x, y) in 0 .. 10 {
        <span>{x}</span>
    }};
}
