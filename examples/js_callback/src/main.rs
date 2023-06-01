use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult};

mod bindings;

static WASM_BINDGEN_SNIPPETS_PATH: OnceCell<String> = OnceCell::new();

#[function_component]
fn Important() -> Html {
    let msg = use_memo((), |_| bindings::hello());
    html! {
        <>
            <h2>{"Important"}</h2>
            <p>{msg}</p>
        </>
    }
}

#[hook]
fn use_do_bye() -> SuspensionResult<String> {
    let path = WASM_BINDGEN_SNIPPETS_PATH
        .get()
        .map(|path| format!("{path}/js/unimp.js"))
        .unwrap();
    let s = use_future(|| async move {
        let promise = bindings::import(&path);
        let module = JsFuture::from(promise).await.unwrap_throw();
        let module = module.unchecked_into::<bindings::UnimpModule>();
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
    let wasm_bindgen_snippets_path = js_sys::global()
        .unchecked_into::<bindings::Window>()
        .wasm_bindgen_snippets_path();
    WASM_BINDGEN_SNIPPETS_PATH
        .set(wasm_bindgen_snippets_path)
        .expect("unreachable");
    yew::Renderer::<App>::new().render();
}
