#![recursion_limit = "512"]
#[macro_use]
extern crate stdweb;

use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender};

const SVG: &str = r#"
<h2>Inline SVG or <i>any</i> HTML:</h2>
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
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{SVG.to_string()};
            console.log(div);
            return div;
        };
        eprintln!("js_svg: {:?}", js_svg);
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }
}
