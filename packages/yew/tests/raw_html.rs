mod common;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(feature = "tokio")]
use tokio::test;

macro_rules! create_test {
    ($name:ident, $html:expr) => {
        create_test!($name, $html, $html);
    };
    ($name:ident, $html:expr, wrap_div) => {
        create_test!($name, $html, format!("<div>{}</div>", $html));
    };
    ($name:ident, $raw:expr, $expected:expr) => {
        #[test]
        async fn $name() {
            #[function_component]
            fn App() -> Html {
                let raw = Html::from_raw_html(AttrValue::from($raw));
                html! {
                    <div id="raw-container">
                        {raw}
                    </div>
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                use std::time::Duration;

                use yew::platform::time::sleep;

                yew::Renderer::<App>::with_root(
                    gloo::utils::document().get_element_by_id("output").unwrap(),
                )
                .render();

                // wait for render to finish
                sleep(Duration::from_millis(100)).await;

                let e = gloo::utils::document()
                    .get_element_by_id("raw-container")
                    .unwrap();
                assert_eq!(e.inner_html(), $expected);
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let actual = yew::ServerRenderer::<App>::new()
                    .hydratable(false)
                    .render()
                    .await;
                assert_eq!(
                    actual,
                    format!(r#"<div id="raw-container">{}</div>"#, $expected)
                );
            }
        }
    };
}

create_test!(empty_string, "");
create_test!(one_node, "<span>text</span>");
create_test!(
    one_but_nested_node,
    r#"<p>one <a href="https://yew.rs">link</a> more paragraph</p>"#
);
create_test!(
    multi_node,
    r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#,
    wrap_div
);
