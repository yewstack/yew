#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use actix_ssr_router::{App, AppProps, LINK_ENDPOINT};
use ssr_e2e_harness::{
    assert_hydrate_home, assert_ssr_hydration_and_client_navigation, output_element,
};
use wasm_bindgen_test::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

const SERVER_BASE: &str = "http://127.0.0.1:8080";

fn make_renderer() -> Renderer<App> {
    Renderer::<App>::with_root_and_props(
        output_element(),
        AppProps {
            endpoint: format!("{SERVER_BASE}{LINK_ENDPOINT}").into(),
        },
    )
}

#[wasm_bindgen_test]
async fn ssr_hydration_and_client_navigation() {
    assert_ssr_hydration_and_client_navigation(make_renderer, SERVER_BASE, LINK_ENDPOINT).await;
}

#[wasm_bindgen_test]
async fn hydrate_home() {
    assert_hydrate_home(make_renderer, SERVER_BASE).await;
}
