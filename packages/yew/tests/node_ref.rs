#![cfg(target_arch = "wasm32")]

mod common;

use yew::prelude::*;
use wasm_bindgen_test::*;
use yew::platform::sync::mpsc;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(PartialEq, Clone, Properties)]
struct CountProps {
    count: u32,
}

#[function_component]
fn Count(props: &CountProps) -> Html {
    html! {
        <>
            <h2>{ "Current Count:"}</h2>
            <p>{props.count}</p>
        </>
    }
}

#[derive(Clone, Properties)]
struct AppProps {
    node_ref: NodeRef,
    tx: mpsc::Sender<()>,
}

impl PartialEq for AppProps {
    fn eq(&self, _other: &Self) -> bool { false }
}

#[function_component]
fn App(props: &AppProps) -> Html {
    let state = use_state(|| 0);
    let node_ref = props.node_ref.clone();

    {
        let node_ref = node_ref.clone();
        let tx = props.tx.clone();
        use_effect(move || {
            gloo::console::log!("effect", node_ref.get());
            yew::platform::spawn_local(async move {
                tx.send(()).await.unwrap();
            });
            || {}
        })
    }

    html!(
        <>
        <Count count={*state} ref={node_ref} />
        </>
    )
}


#[wasm_bindgen_test]
async fn component_has_no_node_ref() {
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let node_ref = NodeRef::default();
    let props = AppProps { node_ref: node_ref.clone(), tx };
    yew::Renderer::<App>::with_root_and_props(common::output_element(), props).render();
    rx.recv().await.unwrap();
    assert!(node_ref.get().is_none());
}
