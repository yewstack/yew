use yew::prelude::*;
// #[function_component]
// fn App() -> Html {
//     let state = use_state(|| 0);
//
//     let incr_counter = {
//         let state = state.clone();
//         Callback::from(move |_| state.set(*state + 1))
//     };
//
//     let decr_counter = {
//         let state = state.clone();
//         Callback::from(move |_| state.set(*state - 1))
//     };
//
//     html! {
//         <>
//             <p @disabled={false}> {"current count: "} {*state} </p>
//             <button onclick={incr_counter}> {"+"} </button>
//             <button onclick={decr_counter}> {"-"} </button>
//         </>
//     }
// }

fn main() {
    let _d = html! {
            <div class={format!("fail{}", "")}></div>
    };
}
