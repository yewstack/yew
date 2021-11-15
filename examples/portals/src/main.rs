use yew::{create_portal, html, Component, Context, Html};

pub struct Model {
    pub style_html: Html,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let document_head = gloo_utils::document()
            .head()
            .expect("head element to be present");
        let style_html = create_portal(
            html! {
                <style>{"p { color: red; }"}</style>
            },
            document_head.into(),
        );
        Self { style_html }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.style_html.clone()}
            <p>{"This paragraph is colored red, and its style is mounted into "}<pre>{"document.head"}</pre>{" with a portal"}</p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
