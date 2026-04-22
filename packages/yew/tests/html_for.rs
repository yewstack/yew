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

#[wasm_bindgen_test]
async fn for_break_with_trailing_semi() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..10 {
                    if i > 2 {
                        break;
                    }
                    <span>{i}</span>
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
async fn for_continue_with_trailing_semi() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..6 {
                    if i % 2 == 0 {
                        continue;
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
async fn for_match_arm_break() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..10 {
                    match i {
                        3 => break,
                        _ => <span>{i}</span>,
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
async fn for_match_arm_continue() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..6 {
                    match i % 2 {
                        0 => continue,
                        _ => <span>{i}</span>,
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
async fn for_labeled_break_crosses_macro() {
    // Demonstrates `break 'outer` inside an html! `for` body terminates the
    // enclosing labeled Rust `for` loop, not just the macro's inner iteration.
    //
    // If labels didn't cross the macro boundary, `break 'outer` would break
    // the macro's inner for and `rows_completed` would count every row.
    // With working labeled break, the first inner `break 'outer` aborts the
    // outer Rust `for`, so post-html! increment never runs.
    #[component]
    fn App() -> Html {
        let mut rows_completed = 0;
        'outer: for _row in 0..3 {
            let _ = html! {
                for col in 0..10 {
                    if col >= 1 {
                        break 'outer;
                    }
                    <span>{col}</span>
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
async fn for_labeled_break_no_semi() {
    #[component]
    fn App() -> Html {
        let mut rows_completed = 0;
        'outer: for _row in 0..3 {
            let _ = html! {
                for col in 0..10 {
                    if col >= 1 {
                        break 'outer
                    }
                    <span>{col}</span>
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
async fn for_labeled_continue_no_semi() {
    #[component]
    fn App() -> Html {
        let mut rows_skipped = 0;
        'outer: for row in 0..3 {
            let _ = html! {
                for col in 0..10 {
                    if row == 1 {
                        continue 'outer
                    }
                    <span>{col}</span>
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

    // Row 1's inner `continue 'outer` skips over `rows_skipped += 1;`, so
    // the counter stays at 0 — proving the label crossed the macro boundary.
    assert_eq!(render_and_read::<App>().await, "rows_skipped=0");
}

#[wasm_bindgen_test]
async fn for_return_exits_component() {
    // `return` inside an html! body returns from the enclosing function. The
    // component function therefore yields the inner `html! { <p .../> }`
    // instead of the outer `<div>` fragment, and the "never" span is dead
    // code. Verifies bare `return val;` semantics in a for-body preamble.
    #[component]
    fn App() -> Html {
        html! {
            <div id="original">
                for _ in 0..1 {
                    return html!{ <p id="result">{"returned"}</p> };
                    <span>{"never"}</span>
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "returned");
}

#[wasm_bindgen_test]
async fn for_return_in_unbraced_match_arm() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="original">
                for i in 0..10 {
                    match i {
                        3 => return html!{ <p id="result">{format!("stopped at {i}")}</p> },
                        _ => <span>{i}</span>,
                    }
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "stopped at 3");
}

// `break <html-element/>`, `continue <html-element/>`, and `return <html-element/>`
// in an unbraced match arm are compile errors (see html-match-fail.rs). The
// documented workaround is to wrap the arm body in braces and use `html!(...)`
// to produce the value. The tests below verify the workaround compiles and
// renders correctly.
#[wasm_bindgen_test]
async fn for_return_html_workaround_with_braced_arm() {
    #[component]
    fn App() -> Html {
        html! {
            <div id="original">
                for i in 0..10 {
                    match i {
                        3 => { return html!(<p id="result">{format!("stopped at {i}")}</p>) }
                        _ => <span>{i}</span>,
                    }
                }
            </div>
        }
    }

    assert_eq!(render_and_read::<App>().await, "stopped at 3");
}

#[wasm_bindgen_test]
async fn for_break_workaround_with_braced_arm() {
    // Breaking the loop doesn't need a value; the workaround for users who
    // tried `break <html/>` is just a braced `{ break }` arm.
    #[component]
    fn App() -> Html {
        html! {
            <div id="result">
                for i in 0..10 {
                    match i {
                        3 => { break }
                        _ => <span>{i}</span>,
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
