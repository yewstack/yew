#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::scheduler;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

async fn render_and_read<C: BaseComponent<Properties = ()>>() -> String {
    yew::Renderer::<C>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();
    scheduler::flush().await;
    obtain_result()
}

#[wasm_bindgen_test]
async fn for_break_emits_prefix() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..10 {
                    if i > 5 {
                        break
                    }
                    <span>{i}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span>"
    );
}

#[wasm_bindgen_test]
async fn for_continue_skips_matching() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..6 {
                    if i % 2 == 0 {
                        continue
                    }
                    <span>{i}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>1</span><span>3</span><span>5</span>"
    );
}

#[wasm_bindgen_test]
async fn for_break_and_continue_together() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..100 {
                    if i >= 8 {
                        break
                    }
                    if i % 3 == 0 {
                        continue
                    }
                    <span>{i}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>1</span><span>2</span><span>4</span><span>5</span><span>7</span>"
    );
}
