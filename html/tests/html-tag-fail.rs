use yew_html::html;

fn main() {
    html! {
        <div>
    };

    html! {
        <div><div>
    };

    html! {
        </div>
    };

    html! {
        <div><div></div>
    };

    html! {
        <div></div>
        <div></div>
    };

    html! {
        <div></span>
    };

    html! {
        <img /></img>
    };

    html! {
        <div>Invalid</div>
    };

    html! {
        <div></span></div>
    };
}
