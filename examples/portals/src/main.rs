use wasm_bindgen::JsCast;
use web_sys::{Element, ShadowRootInit, ShadowRootMode};
use yew::{create_portal, html, Component, Context, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ShadowDOMProps {
    #[prop_or_default]
    pub children: Html,
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
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .expect("installing shadow root succeeds");
            let inner_host = gloo::utils::document()
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
            create_portal(ctx.props().children.clone(), inner_host.clone())
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

pub struct App {
    style_html: Html,
    title_element: Element,
    counter: u32,
}

pub enum AppMessage {
    IncreaseCounter,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let document_head = gloo::utils::document()
            .head()
            .expect("head element to be present");
        let title_element = document_head
            .query_selector("title")
            .expect("to find a title element")
            .expect("to find a title element");
        title_element.set_text_content(None); // Clear the title element
        let style_html = create_portal(
            html! {
                <style>{"p { color: red; }"}</style>
            },
            document_head.into(),
        );
        Self {
            style_html,
            title_element,
            counter: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::IncreaseCounter => self.counter += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| AppMessage::IncreaseCounter);
        let title = create_portal(
            html! {
                if self.counter > 0 {
                    {format!("Clicked {} times", self.counter)}
                } else {
                    {"Yew • Portals"}
                }
            },
            self.title_element.clone(),
        );
        html! {
            <>
            {self.style_html.clone()}
            {title}
            <p>{"This paragraph is colored red, and its style is mounted into "}<pre>{"document.head"}</pre>{" with a portal"}</p>
            <div>
                <ShadowDOMHost>
                    <p>{"This paragraph is rendered in a shadow dom and thus not affected by the surrounding styling context"}</p>
                    <span>{"Buttons clicked inside the shadow dom work fine."}</span>
                    <button {onclick}>{"Click me!"}</button>
                </ShadowDOMHost>
                <p>{format!("The button has been clicked {} times. This is also reflected in the title of the tab!", self.counter)}</p>
            </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
