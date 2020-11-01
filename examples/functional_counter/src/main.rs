use js_sys::Date;
use yew::services::ConsoleService;
use yew_functional::{use_state, functional_component};
use yew::{Html, html, Callback, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct RenderedAtProps {
    pub time: Date,
}

#[functional_component(RenderedAt)]
pub fn rendered_at(props: RenderedAtProps) -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
            { String::from(props.time.to_string()) }
        </p>
    }
}

#[functional_component(App)]
fn app() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    let (counter_one, set_counter_one) = (counter.clone(), set_counter.clone());
    let inc_on_click = move |e: yew::MouseEvent| -> () {
        ConsoleService::log("plus one");
        set_counter_one(*counter_one + 1);
    };

    let (counter_two, set_counter_two) = (counter.clone(), set_counter.clone());
    let dec_on_click = move |e: yew::MouseEvent| -> () {
        ConsoleService::log("minus one");
        set_counter_two(*counter_two - 1);
    };

    html! {<>
        <nav class="menu">
            <button onclick=Callback::from(inc_on_click)>
                { "Increment" }
            </button>

            <button onclick=Callback::from(dec_on_click)>
                { "Decrement" }
            </button>
        </nav>
        <p>
            <b>{ "Current value: " }</b>
            { counter }
        </p>
        <RenderedAt time=Date::new_0() />
    </>}
}

fn main() {
    yew::start_app::<App>();
}
