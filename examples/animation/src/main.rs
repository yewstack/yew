use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;

mod button;
use button::Button;

pub struct App {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    view: Msg,
}

impl Default for Props {
    fn default() -> Self {
        Self { view: Msg::Home }
    }
}

#[derive(Clone)]
pub enum Msg {
    Home,
    Detail,
}

impl Component for App {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn view(&self) -> Html {
        let more_details = self.link.callback(|_| Msg::Detail);
        let go_back = self.link.callback(|_| Msg::Home);
        match self.props.view {
            Msg::Home => html! {
              <>
                <p>{"Did you know that you can animate your Yew component ?"}</p>
                <Button id="btn-more-details" callback=more_details label="See more details" />
              </>
            },
            Msg::Detail => html! {
              <>
                <p>{"Well, a little bit of JavaScript and a pinch of wasm-bindgen in your Rust and you're good to go !"}</p>
                <p>{"Here please note that if both views contains the exact same components (e.g. both a <p> and a <Button>), the instances are recycled (as can be seen from console logs, the id is the same even when switching views)"}</p>
                <Button id="btn-go-back" callback=go_back label="Go back" />
              </>
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Home => {
                self.props.view = Msg::Home;
                true
            }
            Msg::Detail => {
                self.props.view = Msg::Detail;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

fn main() {
    set_panic_hook();
    yew::start_app::<App>();
}
