use yew_html::html;

struct NotHtmlTree {}

fn not_tree() -> NotHtmlTree {
    NotHtmlTree {}
}

fn main() {
    html! {
        { not_tree() }
    };

    html! {
        {
            let not_a_tree = not_tree();
            not_a_tree
        }
    };

    html! {
        <>
            { (0..3).map(|_| not_tree()) }
        </>
    };
}
