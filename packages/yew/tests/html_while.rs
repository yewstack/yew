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
async fn while_iterates_until_false() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 5 {
                    let current = { let c = i; i += 1; c };
                    <span>{current}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span><span>3</span><span>4</span>"
    );
}

#[wasm_bindgen_test]
async fn while_break_exits_early() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 100 {
                    let current = { let c = i; i += 1; c };
                    if current > 5 {
                        break
                    }
                    <span>{current}</span>
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
async fn while_continue_skips_matching() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 6 {
                    let current = { let c = i; i += 1; c };
                    if current % 2 == 0 {
                        continue
                    }
                    <span>{current}</span>
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
async fn while_let_iterates_option_some() {
    #[component]
    fn App() -> Html {
        let mut it = (0..4).into_iter();
        html! {
            <div id="result">
                while let Some(v) = it.next() {
                    <span>{v}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span><span>3</span>"
    );
}

#[wasm_bindgen_test]
async fn while_let_break_and_continue_together() {
    #[component]
    fn App() -> Html {
        let mut it = (0..100).into_iter();
        html! {
            <div id="result">
                while let Some(v) = it.next() {
                    if v >= 8 {
                        break
                    }
                    if v % 3 == 0 {
                        continue
                    }
                    <span>{v}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>1</span><span>2</span><span>4</span><span>5</span><span>7</span>"
    );
}

#[wasm_bindgen_test]
async fn while_break_with_trailing_semi() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 100 {
                    let current = { let c = i; i += 1; c };
                    if current > 2 {
                        break;
                    }
                    <span>{current}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span>"
    );
}

#[wasm_bindgen_test]
async fn while_continue_with_trailing_semi() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 6 {
                    let current = { let c = i; i += 1; c };
                    if current % 2 == 0 {
                        continue;
                    }
                    <span>{current}</span>
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
async fn while_let_match_arm_break() {
    #[component]
    fn App() -> Html {
        let mut it = (0..10).into_iter();
        html! {
            <div id="result">
                while let Some(v) = it.next() {
                    match v {
                        3 => break,
                        _ => <span>{v}</span>,
                    }
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span>"
    );
}

#[wasm_bindgen_test]
async fn while_let_match_arm_continue() {
    #[component]
    fn App() -> Html {
        let mut it = (0..6).into_iter();
        html! {
            <div id="result">
                while let Some(v) = it.next() {
                    match v % 2 {
                        0 => continue,
                        _ => <span>{v}</span>,
                    }
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
async fn while_expr_stmt_preamble_increments() {
    #[component]
    fn App() -> Html {
        let mut i: i32 = 0;
        html! {
            <div id="result">
                while i < 4 {
                    let current = i;
                    i += 1;
                    <span>{current}</span>
                }
            </div>
        }
    }

    assert_eq!(
        render_and_read::<App>().await,
        "<span>0</span><span>1</span><span>2</span><span>3</span>"
    );
}

#[wasm_bindgen_test]
async fn while_labeled_break_no_semi() {
    #[component]
    fn App() -> Html {
        let mut rows_completed = 0;
        'outer: for _row in 0..3 {
            let mut i: i32 = 0;
            let _ = html! {
                while i < 10 {
                    let current = i;
                    i += 1;
                    if current >= 1 {
                        break 'outer
                    }
                    <span>{current}</span>
                }
            };
            rows_completed += 1;
        }
        html! {
            <div id="result">{ format!("rows_completed={rows_completed}") }</div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "rows_completed=0");
}

#[wasm_bindgen_test]
async fn while_labeled_continue_no_semi() {
    #[component]
    fn App() -> Html {
        let mut rows_skipped = 0;
        'outer: for row in 0..3 {
            let mut i: i32 = 0;
            let _ = html! {
                while i < 2 {
                    let current = i;
                    i += 1;
                    if row == 1 {
                        continue 'outer
                    }
                    <span>{current}</span>
                }
            };
            if row == 1 {
                rows_skipped += 1;
            }
        }
        html! {
            <div id="result">{ format!("rows_skipped={rows_skipped}") }</div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "rows_skipped=0");
}

#[wasm_bindgen_test]
async fn while_return_exits_component() {
    #[component]
    fn App() -> Html {
        let mut it = (0..1).into_iter();
        html! {
            <div id="original">
                while let Some(_) = it.next() {
                    return html!{ <p id="result">{"returned"}</p> };
                    <span>{"never"}</span>
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "returned");
}

#[wasm_bindgen_test]
async fn while_return_in_unbraced_match_arm() {
    #[component]
    fn App() -> Html {
        let mut it = (0..10).into_iter();
        html! {
            <div id="original">
                while let Some(v) = it.next() {
                    match v {
                        3 => return html!{ <p id="result">{format!("stopped at {v}")}</p> },
                        _ => <span>{v}</span>,
                    }
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "stopped at 3");
}

// Counterpart to `for_return_html_workaround_with_braced_arm`: verifies the
// braced-arm workaround works under `while let` too.
#[wasm_bindgen_test]
async fn while_return_html_workaround_with_braced_arm() {
    #[component]
    fn App() -> Html {
        let mut it = (0..10).into_iter();
        html! {
            <div id="original">
                while let Some(v) = it.next() {
                    match v {
                        3 => { return html!(<p id="result">{format!("stopped at {v}")}</p>) }
                        _ => <span>{v}</span>,
                    }
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "stopped at 3");
}
