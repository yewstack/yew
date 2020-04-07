use yew::prelude::*;

fn compile_pass() {
    let placeholder = "placeholder-text";
    let maxlength = "20";
    let href = "https://yew.rs/docs/";
    let style = "background-color:blue";
    let size = 25;

    html! { <input placeholder= />};
    html! { <input placeholder= maxlength= />};
    html! { <input style="color:red" size= />};
    html! { <a href= >{"Yew"}</a>};
    html! { <div style= >
            <h1>{"Test"}</h1>
            </div>
        };
}

fn main() {}