use yew_html::html;

fn main() {
    html! { "" };
    html! { 'a' };
    html! { "hello" };
    html! { 42 };
    html! { 1.234 };
    html! { true };

    html! {
        <span>{ "" }</span>
    };

    html! {
        <span>{ 'a' }</span>
    };

    html! {
        <span>{ "hello" }</span>
    };

    html! {
        <span>{ 42 }</span>
    };

    html! {
        <span>{ 1.234 }</span>
    };

    html! {
        <span>{ true }</span>
    };
}
