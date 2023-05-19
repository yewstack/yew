#![cfg(not(target_arch = "wasm32"))]

use yew::prelude::*;

#[tokio::test]
async fn conditional_children_are_omitted() {
    #[derive(Properties, PartialEq)]
    pub struct Props {
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component]
    pub fn Problem(p: &Props) -> Html {
        html! {
            <div class="container">
                {
                    for p.children.iter().enumerate().map(|(i, x)| {
                        html! {
                            <div>
                                <span>{x}</span>
                                <span>{i}</span>
                            </div>
                        }
                    })
                }
            </div>
        }
    }

    #[function_component]
    fn App() -> Html {
        let b = false;
        html! {
            <Problem>
                <span>{ "A" }</span>
                <span>{ "B" }</span>
                if b {
                    <span>{ "C" }</span>
                }
                <span>{ "D" }</span>
            </Problem>
        }
    }

    let mut s = String::new();
    yew::ServerRenderer::<App>::new()
        .render_to_string(&mut s)
        .await;

    assert_eq!(
        s,
        "<!--<[children::conditional_children_are_omitted::{{closure}}::App]>--><!\
         --<[children::conditional_children_are_omitted::{{closure}}::Problem]>--><div \
         class=\"container\"><div><span><span>A</span></span><span>0</span></\
         div><div><span><span>B</span></span><span>1</span></div><div><span><span>D</span></\
         span><span>2</span></div></div><!--</\
         [children::conditional_children_are_omitted::{{closure}}::Problem]>--><!--</\
         [children::conditional_children_are_omitted::{{closure}}::App]>-->"
    );
}

#[tokio::test]
async fn conditional_children_are_omitted_in_non_only_single_node_children_case() {
    #[function_component]
    fn App() -> Html {
        let b = true;
        let a = Some(());
        let chtml = html! {
            <div></div>
        };
        html! {
            <>
                { chtml }
                if a.is_some() || b { <></> }
            </>
        }
    }

    let mut s = String::new();
    yew::ServerRenderer::<App>::new()
        .render_to_string(&mut s)
        .await;

    assert_eq!(
        s,
        "<!--<[children::conditional_children_are_omitted_in_non_only_single_node_children_case::{{closure}}::App]>--><div></div><!--</[children::conditional_children_are_omitted_in_non_only_single_node_children_case::{{closure}}::App]>-->"
    );
}
