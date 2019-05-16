use yew_html::html;
use yew_html::HtmlTree;

fn nested_list() -> HtmlTree {
    html! {
        <></>
    }
}

fn main() {
    html! {
        <>
            { nested_list() }
        </>
    };
}
