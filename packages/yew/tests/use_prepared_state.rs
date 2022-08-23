#![cfg(target_arch = "wasm32")]
#![cfg(feature = "hydration")]
#![cfg_attr(nightly_yew, feature(async_closure))]

use std::time::Duration;

mod common;

use common::obtain_result_by_id;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::{Renderer, ServerRenderer};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_prepared_state_works() {
    #[function_component]
    fn Comp() -> HtmlResult {
        let ctr = use_prepared_state!(|_| -> u32 { 12345 }, ())?.unwrap_or_default();

        Ok(html! {
            <div>
                {*ctr}
            </div>
        })
    }

    #[function_component]
    fn App() -> Html {
        html! {
            <Suspense fallback={Html::default()}>
                <div>
                    <Comp />
                </div>
            </Suspense>
        }
    }

    let s = ServerRenderer::<App>::new().render().await;

    assert_eq!(
        s,
        r#"<!--<[use_prepared_state::use_prepared_state_works::{{closure}}::App]>--><!--<[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--<[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--<?>--><div><!--<[use_prepared_state::use_prepared_state_works::{{closure}}::Comp]>--><div>12345</div><script type="application/x-yew-comp-state">ATkwAAAB</script><!--</[use_prepared_state::use_prepared_state_works::{{closure}}::Comp]>--></div><!--</?>--><!--</[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--</[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--</[use_prepared_state::use_prepared_state_works::{{closure}}::App]>-->"#
    );

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::from_millis(100)).await;

    let result = obtain_result_by_id("output");

    // no placeholders, hydration is successful and state 12345 is preserved.
    assert_eq!(result, r#"<div><div>12345</div></div>"#);
}

#[wasm_bindgen_test]
async fn use_prepared_state_with_suspension_works() {
    #[function_component]
    fn Comp() -> HtmlResult {
        let ctr = use_prepared_state!(async move |_| -> u32 { 12345 }, ())?.unwrap_or_default();

        Ok(html! {
            <div>
                {*ctr}
            </div>
        })
    }

    #[function_component]
    fn App() -> Html {
        html! {
            <Suspense fallback={Html::default()}>
                <div>
                    <Comp />
                </div>
            </Suspense>
        }
    }

    let s = ServerRenderer::<App>::new().render().await;

    assert_eq!(
        s,
        r#"<!--<[use_prepared_state::use_prepared_state_with_suspension_works::{{closure}}::App]>--><!--<[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--<[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--<?>--><div><!--<[use_prepared_state::use_prepared_state_with_suspension_works::{{closure}}::Comp]>--><div>12345</div><script type="application/x-yew-comp-state">ATkwAAAB</script><!--</[use_prepared_state::use_prepared_state_with_suspension_works::{{closure}}::Comp]>--></div><!--</?>--><!--</[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--</[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--</[use_prepared_state::use_prepared_state_with_suspension_works::{{closure}}::App]>-->"#
    );

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::from_millis(100)).await;

    let result = obtain_result_by_id("output");

    // no placeholders, hydration is successful and state 12345 is preserved.
    assert_eq!(result, r#"<div><div>12345</div></div>"#);
}
