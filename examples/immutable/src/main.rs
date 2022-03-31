mod array;
mod map;
mod string;

use self::array::*;
use self::map::*;
use self::string::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "IString Example" }</h1>
            <StringExample />
            <hr/>
            <h1>{ "IArray Example" }</h1>
            <ArrayExample />
            <hr/>
            <h1>{ "IMap Example" }</h1>
            <MapExample />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
