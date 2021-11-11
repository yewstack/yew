use yew::{html, Callback, Component, ComponentLink, MouseEvent, Properties};

use wasm_bindgen::prelude::*;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    /// button id, required to identify the button in JS
    pub id: String,
    /// button label
    #[prop_or_default]
    pub label: String,
    /// an optional callback to be called when the button is clicked
    #[prop_or_default]
    pub callback: Callback<MouseEvent>,
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
    /// allows for keeping a reference to our animation on JS side
    animation: Option<Animate>,
}

pub enum Msg {
    OnMouseEnter,
    OnMouseLeave,
}

impl Component for Button {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            animation: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::OnMouseEnter => {
                self.animation.as_ref().unwrap().enter();
                true
            }
            Msg::OnMouseLeave => {
                self.animation.as_ref().unwrap().leave();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        let onclick = &self.props.callback;
        let onmouseenter = self.link.callback(|_| Msg::OnMouseEnter);
        let onmouseleave = self.link.callback(|_| Msg::OnMouseLeave);
        html! {
          <button
            id={self.props.id.clone()}
            onclick=onclick
            onmouseenter=onmouseenter
            onmouseleave=onmouseleave>{ &self.props.label }</button>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            js_log(format!("Rust: on first render ({})", self.props.id));
            self.animation = Some(Animate::new(self.props.id.clone()));
        }
    }

    fn destroy(&mut self) {
        js_log(format!("Rust: on destroy ({})", self.props.id));
        self.animation.as_ref().unwrap().destroy();
        self.animation = None;
    }
}

/// allows for calling JS from Rust
///
/// see src/button.js
#[wasm_bindgen(module = "/src/button.js")]
extern "C" {
    type Animate;

    #[wasm_bindgen(constructor)]
    fn new(id: String) -> Animate;

    // methods on class

    /// on mouse enter
    #[wasm_bindgen(method)]
    fn enter(this: &Animate);
    #[wasm_bindgen(method)]
    /// on mouse leave
    fn leave(this: &Animate);
    /// on destroy element
    ///
    /// allows for cleaning references on JS side
    #[wasm_bindgen(method)]
    fn destroy(this: &Animate);
}

impl std::fmt::Debug for Animate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animate").field("obj", &self.obj).finish()
    }
}

/// simple console log
#[wasm_bindgen(inline_js = "export function js_log(message) { console.log(message); }")]
extern "C" {
    fn js_log(message: String);
}
