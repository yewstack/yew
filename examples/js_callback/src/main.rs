use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult};

mod bindings;

#[function_component]
fn Important() -> Html {
    let msg = use_memo(|_| bindings::hello(), ());
    html! {
        <>
            <h2>{"Important"}</h2>
            <p>{msg}</p>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
    type Module;

    #[wasm_bindgen(method)]
    fn bye(this: &Module) -> String;
}

#[hook]
fn use_do_bye() -> SuspensionResult<String> {
    let s = use_future(|| async move {
        // Run `trunk build`, look in the `dist/snippets/` directory for the file name.
        // this must be the absolute url. it is the restriction of dynamic imports in JS
        let promise =
            bindings::import("/js_callback/snippets/js_callback-12fde9d6e52a7cb5/js/unimp.js");
        let module = JsFuture::from(promise).await.unwrap_throw();
        let module = module.unchecked_into::<Module>();
        module.bye()
    })?;
    Ok((*s).clone())
}

#[function_component]
fn UnImportant() -> HtmlResult {
    let msg = use_do_bye()?;
    Ok(html! {
        <>
            <h2>{"Unimportant"}</h2>
            <p>{msg}</p>
        </>
    })
}

#[function_component]
fn App() -> Html {
    let showing_unimportant = use_state(|| false);

    let show_unimportant = {
        let showing_unimportant = showing_unimportant.clone();
        move |_| showing_unimportant.set(true)
    };
    let fallback = html! {"fallback"};
    html! {
        <main>
            <Important />
            <button onclick={show_unimportant}>{"load unimportant data"}</button>
            <Suspense {fallback}>
                if *showing_unimportant {
                    <UnImportant />
                }
            </Suspense>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
