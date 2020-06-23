#![recursion_limit = "512"]

use web_sys::{console, Node};
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender};

const HTML: &str = r#"
<h2>Inline HTML with SVG</h2>
<p>The whole contents of this page is stored as a constant HTML string in
the Rust source code. The code queries the DOM, creates a new element, and
applies this snippet of HTML to the element's innerHTML.</p>
<p>If you look at your browser's console you can see the DOM element (logged to the console).</p>
<svg height="250" width="500">
  <polygon points="220,10 300,210 170,250 123,234" style="fill:lime;stroke:purple;stroke-width:1" />
    Sorry, your browser does not support inline SVG.
</svg>
"#;

pub struct Model {
    pub value: i64,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let js_svg = {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(HTML);
            console::log_1(&div);
            div
        };
        eprintln!("js_svg: {:?}", js_svg);
        let node = Node::from(js_svg);
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }
}
