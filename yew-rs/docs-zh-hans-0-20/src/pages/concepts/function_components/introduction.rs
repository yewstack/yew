crate::doc_page!(
    "\u{51fd}\u{6570}\u{5f0f}\u{7ec4}\u{4ef6}",
    "/zh-Hans/docs/concepts/function-components",
    Content::new(vec![
        p(vec![
            text(
                "\u{51fd}\u{6570}\u{5f0f}\u{7ec4}\u{4ef6}\u{662f}\u{666e}\u{901a}\u{7ec4}\u{4ef6}\\
                 \
                 u{7684}\u{7b80}\u{5316}\u{7248}\u{3002}\u{5b83}\u{4eec}\u{7531}\u{4e00}\u{4e2a}\\
                 u{63a5}\u{6536} props \
                 \u{7684}\u{51fd}\u{6570}\u{7ec4}\u{6210}\u{ff0c}\u{5e76}\u{901a}\u{8fc7}\u{8fd4}\\
                 \
                 u{56de}"
            ),
            code("Html"),
            text(
                "\u{6765}\u{786e}\u{5b9a}\u{5e94}\u{8be5}\u{5448}\u{73b0}\u{4ec0}\u{4e48}\u{3002}\\
                 \
                 u{57fa}\u{672c}\u{4e0a}\u{ff0c}\u{5b83}\u{662f}\u{4e00}\u{4e2a}\u{7b80}\u{5316}\\
                 u{4e3a}"
            ),
            code("view"),
            text(
                "\u{65b9}\u{6cd5}\u{7684}\u{7ec4}\u{4ef6}\u{3002}\u{5c31}\u{5176}\u{672c}\u{8eab}\\
                 \
                 u{800c}\u{8a00}\u{ff0c}\u{8fd9}\u{5c06}\u{662f}\u{76f8}\u{5f53}\u{6709}\u{9650}\\
                 u{7684}\u{ff0c}\u{56e0}\u{4e3a}\u{60a8}\u{53ea}\u{80fd}\u{521b}\u{5efa}\u{7eaf}\\
                 u{7ec4}\u{4ef6}\u{ff0c}\u{800c}\u{8fd9}\u{5c31}\u{662f} Hook \
                 \u{5927}\u{5c55}\u{8eab}\u{624b}\u{7684}\u{5730}\u{65b9}\u{3002}Hook \
                 \u{5141}\u{8bb8}\u{51fd}\u{6570}\u{7ec4}\u{4ef6}\u{65e0}\u{9700}\u{5b9e}\u{73b0}"
            ),
            code("Component"),
            text(
                " trait\u{ff0c}\u{5c31}\u{53ef}\u{4ee5}\u{4f7f}\u{7528}\u{72b6}\u{6001}\\
                 u{ff08}state\u{ff09}\u{548c}\u{5176}\u{4ed6} Yew \u{529f}\u{80fd}\u{3002}"
            ),
        ]),
        h2(vec![text(
            "\u{521b}\u{5efa}\u{51fd}\u{6570}\u{5f0f}\u{7ec4}\u{4ef6}"
        )]),
        p(vec![
            text(
                "\u{521b}\u{5efa}\u{51fd}\u{6570}\u{5f0f}\u{7ec4}\u{4ef6}\u{7684}\u{6700}\u{7b80}\\
                 \
                 u{5355}\u{65b9}\u{6cd5}\u{662f}\u{5728}\u{51fd}\u{6570}\u{524d}\u{6dfb}\u{52a0}"
            ),
            code("#[function_component]"),
            text("\u{5c5e}\u{6027}\u{3002}"),
        ]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#
        ),
        h3(vec![text("\u{66f4}\u{591a}\u{7ec6}\u{8282}")]),
        p(vec![
            text(
                "\u{51fd}\u{6570}\u{5f0f}\u{7ec4}\u{4ef6}\u{7531}\u{4e24}\u{90e8}\u{5206}\u{7ec4}\\
                 \
                 u{6210}\u{3002}\u{9996}\u{5148}\u{ff0c} "
            ),
            code("FunctionProvider"),
            text(" trait \u{4e0e}"),
            code("Component"),
            text(
                " trait \u{5dee}\u{4e0d}\u{591a}\u{ff0c}\u{4f46}\u{5b83}\u{53ea}\u{6709}\u{4e00}\\
                 u{4e2a}\u{540d}\u{4e3a}"
            ),
            code("run"),
            text("\u{65b9}\u{6cd5}\u{3002}\u{4e4b}\u{540e}\u{662f}"),
            code("FunctionComponent"),
            text("\u{7ed3}\u{6784}\u{4f53}\u{ff0c}\u{5b83}\u{5c01}\u{88c5}\u{4e86}"),
            code("FunctionProvider"),
            text(
                "\u{7c7b}\u{578b}\u{5e76}\u{5c06}\u{5176}\u{8f6c}\u{6362}\u{4e3a}\u{5b9e}\u{9645}\\
                 \
                 u{7684}"
            ),
            code("Component"),
            text(" \u{3002} "),
            code("#[function_component]"),
            text("\u{5c5e}\u{6027}\u{672c}\u{8d28}\u{4e0a}\u{53ea}\u{662f}"),
            code("FunctionProvider"),
            text("\u{5e76}\u{5c06}\u{5176}\u{66b4}\u{9732}\u{5728}"),
            code("FunctionComponent"),
            text(" \u{3002}"),
        ]),
        h3(vec![text("\u{94a9}\u{5b50}\u{ff08}Hooks\u{ff09}")]),
        p(vec![text(
            "\u{94a9}\u{5b50}\u{ff08}Hooks\u{ff09}\u{5c31}\u{662f}\u{8ba9}\u{60a8}\u{201c}\\
             u{94a9}\u{4f4f}\u{201d}\u{7ec4}\u{4ef6}\u{7684}\u{72b6}\u{6001}\u{ff08}state\u{ff09}\\
             u{548c}/\u{6216}\u{751f}\u{547d}\u{5468}\u{671f}\u{5e76}\u{6267}\u{884c}\u{64cd}\\
             u{4f5c}\u{7684}\u{51fd}\u{6570}\u{3002} \u{9664}\u{4e86} Yew \
             \u{81ea}\u{5e26}\u{7684}\u{4e00}\u{4e9b}\u{9884}\u{5b9a}\u{4e49}\u{7684} \
             Hook\u{3002}\u{60a8}\u{4e5f}\u{53ef}\u{4ee5}\u{521b}\u{5efa}\u{81ea}\u{5df1}\u{7684}\\
             \
             u{3002}"
        ),]),
    ])
);
