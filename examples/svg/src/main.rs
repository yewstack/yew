#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::console::ConsoleService;

use stdweb::web::Date;

use stdweb::web::{Element, Node};
use stdweb::unstable::TryFrom;
use yew::virtual_dom::{VNode, VElement};

struct Context {
    console: ConsoleService,
}

struct Model {
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
    None,
    Bulk(Vec<Msg>),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Model {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, context);
                context.console.log("Bulk action");
            },
            Msg::None => {
                context.console.log("No action");
                return false;
            }
        }
        true
    }
}


const SVG: &str = r#"
<svg width="400" height="110">
  <rect width="300" height="100" style="fill:rgb(0,0,255);stroke-width:3;stroke:rgb(0,0,0)" />
  Sorry, your browser does not support inline SVG.
</svg>
"#;

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{SVG.to_string()};
            console.log(div);
            return div;
        };
        eprintln!("js_svg: {:?}", js_svg);
        let element = Element::try_from(js_svg).expect("convert js_svg");
        let velement = VElement::from_element("Element".into(), element);
        let svg = VNode::VElement(velement);
        eprintln!("svg: {:?}", svg);
        // html!{
        //     <h1>{"See inner html?:"}</h1>
        //     <div innerHtml=SVG,></div>
        //     <div>{ SVG }</div>
        //     <div>{ svg }</div>
        // }
        svg
    }
}

fn main() {
    yew::initialize();

    let context = Context {
        console: ConsoleService,
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
