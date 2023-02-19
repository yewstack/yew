use yew::prelude::*;
use gloo::timers::callback::Interval;

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

#[function_component(Clock)]
fn clock() -> Html {
    let time = use_state(|| get_current_time());

    {
        let time = time.clone();
        use_effect_with_deps(|_|{
            Interval::new(1000, move || time.set(get_current_time())).forget();
        },());
    }
    html!(
        <div id="time">{ time.as_str() }</div>    
    )
}


#[function_component]
fn App() -> Html {
    html!(
        <>
            <div id="buttons">
                <button>{ "Start Timeout" }</button>
                <button>{ "Start Interval" }</button>
                <button>{ "Cancel" }</button>
            </div>
            <div id="wrapper">
                <Clock />
            </div>
        </>
    )
}


fn main() {
    yew::Renderer::<App>::new().render();
}
