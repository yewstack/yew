crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/html",
    Content::new(vec![
        h1(vec![text("\u{4f7f}\u{7528} html! \u{5b8f}")]),
        p(vec![
            code("html!"),
            text(
                " \u{5b8f}\u{5141}\u{8bb8}\u{4f60}\u{4e3a}\u{7ec4}\u{4ef6}\u{7f16}\u{5199}\\
                 u{58f0}\u{660e}\u{5f0f}\u{7684} HTML \u{548c} \
                 SVG\u{3002}\u{5982}\u{679c}\u{4f60}\u{4f7f}\u{7528}\u{8fc7} React \u{7684} \
                 JSX\u{ff0c}\u{5c06}\u{4f1a}\u{611f}\u{89c9}\u{5230}\u{975e}\u{5e38}\u{719f}\\
                 u{6089}\u{3002}"
            ),
        ]),
        p(vec![bold(vec![text("\u{91cd}\u{8981}\u{63d0}\u{793a}")])]),
        ol(vec![
            li(vec![
                code("html!"),
                text(
                    " \u{5b8f}\u{8c03}\u{7528}\u{4e2d}\u{53ea}\u{80fd}\u{6709}\u{4e00}\u{4e2a}\\
                     u{6839}\u{8282}\u{70b9}"
                ),
            ]),
            li(vec![
                text("\u{7a7a}\u{7684} "),
                code("html! {}"),
                text(
                    " \u{5b8f}\u{8c03}\u{7528}\u{662f}\u{6709}\u{6548}\u{7684}\u{4f46}\u{4e0d}\\
                     u{4f1a}\u{6e32}\u{67d3}\u{4efb}\u{4f55}\u{5185}\u{5bb9}"
                ),
            ]),
            li(vec![
                text(
                    "\u{5e38}\u{91cf}\u{5fc5}\u{987b}\u{59cb}\u{7ec8}\u{88ab}\u{5f15}\u{53f7}\\
                     u{62ec}\u{8d77}\u{6765}\u{5e76}\u{88ab}\u{5305}\u{542b}\u{5728}\u{5927}\\
                     u{62ec}\u{53f7}\u{91cc}\u{ff1a}"
                ),
                code("html! { \"Hello, World\" }"),
            ]),
        ]),
    ])
);
