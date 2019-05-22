use yew_html::html;

fn main() {
    html! { "valid" "invalid" };

    html! {
        <span>{ "valid" "invalid" }</span>
    };

    // unsupported literals
    html! { b'a' };
    html! { b"str" };
    html! { 1111111111111111111111111111111111111111111111111111111111111111111111111111 };
}
