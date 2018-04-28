#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;
extern crate yew;

use yew::prelude::*;
use yew::services::console::ConsoleService;

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::virtual_dom::VNode;

pub struct Model {
    pub value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    None,
    Bulk(Vec<Msg>),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                env.as_mut().log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                env.as_mut().log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, env);
                env.as_mut().log("Bulk action");
            },
            Msg::None => {
                env.as_mut().log("No action");
                return false;
            }
        }
        true
    }
}


const SVG: &str = r#"
<h2>Inline SVG or <i>any</i> HTML:</h2>
<svg height="250" width="500">
  <polygon points="220,10 300,210 170,250 123,234" style="fill:lime;stroke:purple;stroke-width:1" />
    Sorry, your browser does not support inline SVG.
</svg>
"#;

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,

{
    fn view(&self) -> Html<CTX, Self> {
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
