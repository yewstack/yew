use yew_html::html;
use yew_html::HtmlTree;

fn tree_block() -> HtmlTree {
    html! {}
}

fn main() {
    html! {
        { tree_block() }
    };

    html! {
        {
            let stmt = tree_block();
            stmt
        }
    };
}
