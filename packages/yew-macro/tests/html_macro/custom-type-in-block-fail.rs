// Custom types with From<T> for VNode or Display do not automatically work
// in html! blocks, because blocks require IntoPropValue<VNode>, not Into<VNode>.
//
// A blanket `impl<T: Into<VNode>> IntoPropValue<VNode> for T` would fix this
// but conflicts with the identity impl `impl<T> IntoPropValue<T> for T` when
// T = VNode. Resolving this overlap requires the `specialization` feature
// (rust-lang/rust#31844), which is still unstable.
//
// Workaround: call `.into()` explicitly or implement IntoPropValue<VNode>.

struct Print {
    text: ::std::string::String,
}

impl ::std::convert::From<Print> for ::yew::virtual_dom::VNode {
    fn from(val: Print) -> Self {
        ::yew::html! {{"Hello "}{val.text}}
    }
}

struct Ratio(::std::primitive::f64);

impl ::std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::write!(f, "{:.2}", self.0)
    }
}

enum Label {
    Text(::std::string::String),
    Number(::std::primitive::i32),
}

impl ::std::convert::From<Label> for ::yew::virtual_dom::VNode {
    fn from(val: Label) -> Self {
        match val {
            Label::Text(t) => ::yew::html! { <span>{t}</span> },
            Label::Number(n) => ::yew::html! { <span>{n}</span> },
        }
    }
}

fn main() {
    let text = Print {
        text: "World".into(),
    };
    let _ = ::yew::html! { <div>{text}</div> };

    let setback = Ratio(3.14);
    let _ = ::yew::html! { <div>{setback}</div> };

    let label = Label::Text("hello".into());
    let _ = ::yew::html! { <div>{label}</div> };
}
