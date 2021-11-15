use wasm_bindgen::JsCast;
use web_sys::{Element, ShadowRootInit, ShadowRootMode};
use yew::{create_portal, html, Children, Component, Context, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ShadowDOMProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct ShadowDOMHost {
    host_ref: NodeRef,
    inner_host: Option<Element>,
}

impl Component for ShadowDOMHost {
    type Message = ();
    type Properties = ShadowDOMProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            host_ref: NodeRef::default(),
            inner_host: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let shadow_root = self
                .host_ref
                .get()
                .expect("rendered host")
                .unchecked_into::<Element>()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
                .expect("installing shadow root succeeds");
            let inner_host = gloo_utils::document()
                .create_element("div")
                .expect("can create inner wrapper");
            shadow_root
                .append_child(&inner_host)
                .expect("can attach inner host");
            self.inner_host = Some(inner_host);
            ctx.link().send_message(());
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let contents = if let Some(ref inner_host) = self.inner_host {
            create_portal(
                html! {
                    {for ctx.props().children.iter()}
                },
                inner_host.clone(),
            )
        } else {
            html! { <></> }
        };
        html! {
            <div ref={self.host_ref.clone()}>
                {contents}
            </div>
        }
    }
}

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
            <ShadowDOMHost>
                <p>{"This paragraph is rendered in a shadow dom and thus not affected by the surrounding styling context"}</p>
            </ShadowDOMHost>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
