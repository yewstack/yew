crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/router",
    Content::new(vec![
        h1(vec![text("Router")]),
        p(vec![link(
            "https://crates.io/crates/yew-router",
            vec![text("https://crates.io/crates/yew-router")]
        )]),
        p(vec![text(
            "Routers \u{5728}\u{5355}\u{9875}\u{5e94}\u{7528}\u{ff08}SPA\u{ff09}\u{4e2d}\u{6839}\\
             u{636e} URL \
             \u{7684}\u{4e0d}\u{540c}\u{663e}\u{793a}\u{4e0d}\u{540c}\u{7684}\u{9875}\u{9762}\\
             u{3002}\u{5f53}\u{70b9}\u{51fb}\u{4e00}\u{4e2a}\u{94fe}\u{63a5}\u{65f6}\\
             u{ff0c}Router \u{5728}\u{672c}\u{5730}\u{8bbe}\u{7f6e} URL \
             \u{4ee5}\u{6307}\u{5411}\u{5e94}\u{7528}\u{7a0b}\u{5e8f}\u{4e2d}\u{6709}\u{6548}\\
             u{7684}\u{8def}\u{7531}\u{ff0c}\u{800c}\u{4e0d}\u{662f}\u{9ed8}\u{8ba4}\u{8bf7}\\
             u{6c42}\u{4e00}\u{4e2a}\u{4e0d}\u{540c}\u{7684}\u{8fdc}\u{7a0b}\u{8d44}\u{6e90}\\
             u{3002}\u{7136}\u{540e} Router \
             \u{68c0}\u{6d4b}\u{5230}\u{6b64}\u{66f4}\u{6539}\u{540e}\u{51b3}\u{5b9a}\u{8981}\\
             u{6e32}\u{67d3}\u{7684}\u{5185}\u{5bb9}\u{3002}"
        )]),
        h2(vec![text("\u{6838}\u{5fc3}\u{5143}\u{7d20}")]),
        h3(vec![text("Route")]),
        p(vec![text(
            "\u{5305}\u{542b}\u{4e00}\u{4e2a}\u{5b57}\u{7b26}\u{4e32}\u{ff0c}\u{8be5}\u{5b57}\\
             u{7b26}\u{4e32}\u{8868}\u{793a}\u{7f51}\u{5740}\u{4e2d}\u{57df}\u{540d}\u{4e4b}\\
             u{540e}\u{7684}\u{6240}\u{6709}\u{5185}\u{5bb9}\u{ff0c}\u{8fd8}\u{53ef}\u{4ee5}\\
             u{9009}\u{62e9}\u{8868}\u{793a}\u{5b58}\u{50a8}\u{5728} history api \
             \u{4e2d}\u{7684}\u{72b6}\u{6001}\u{3002}"
        )]),
        h3(vec![text("RouteService")]),
        p(vec![text(
            "\u{4e0e}\u{6d4f}\u{89c8}\u{5668}\u{901a}\u{4fe1}\u{4ee5}\u{83b7}\u{53d6}\u{548c}\\
             u{8bbe}\u{7f6e}\u{8def}\u{7531}\u{3002}"
        )]),
        h3(vec![text("RouteAgent")]),
        p(vec![text(
            "\u{62e5}\u{6709}\u{4e00}\u{4e2a} \
             RouteService\u{ff0c}\u{5e76}\u{7528}\u{4e8e}\u{5f53}\u{8def}\u{7531}\u{6539}\u{53d8}\\
             \
             u{65f6}\u{534f}\u{8c03}\u{66f4}\u{65b0}\u{ff0c}\u{65e0}\u{8bba}\u{66f4}\u{65b0}\\
             u{662f}\u{6765}\u{81ea}\u{5e94}\u{7528}\u{7a0b}\u{5e8f}\u{81ea}\u{8eab}\u{903b}\\
             u{8f91}\u{8fd8}\u{662f}\u{6765}\u{81ea}\u{6d4f}\u{89c8}\u{5668}\u{89e6}\u{53d1}\\
             u{7684}\u{4e8b}\u{4ef6}\u{3002}"
        )]),
        h3(vec![text("Switch")]),
        p(vec![
            code("Switch"),
            text(
                " trait \u{7528}\u{4e8e}\u{5728}\u{8be5} trait \
                 \u{7684}\u{5b9e}\u{73b0}\u{8005}\u{4e4b}\u{95f4}\u{8f6c}\u{6362} "
            ),
            code("Route"),
            text("\u{3002}"),
        ]),
        h3(vec![text("Router")]),
        p(vec![
            text("Router \u{7ec4}\u{4ef6}\u{540c} "),
            code("RouterAgent"),
            text(
                " \u{8fdb}\u{884c}\u{901a}\u{4fe1}\u{ff0c}\u{5e76}\u{81ea}\u{52a8}\u{628a}\\
                 u{5b83}\u{4ece} Agent \u{90a3}\u{91cc}\u{83b7}\u{5f97}\u{7684} Routes \
                 \u{89e3}\u{6790}\u{4e3a} Switches\u{ff0c}\u{5e76}\u{901a}\u{8fc7} "
            ),
            code("render"),
            text(
                " \u{5c5e}\u{6027}\u{66b4}\u{9732}\u{8be5} \
                 Switch\u{ff0c}\u{8be5}\u{5c5e}\u{6027}\u{5141}\u{8bb8}\u{6307}\u{5b9a}\u{5c06}\\
                 u{751f}\u{6210}\u{7684} Switch \u{8f6c}\u{6362}\u{4e3a} "
            ),
            code("HTML"),
            text(" \u{7684}\u{65b9}\u{5f0f}\u{3002}"),
        ]),
        h2(vec![text("\u{5982}\u{4f55}\u{4f7f}\u{7528} Router")]),
        p(vec![
            text(
                "\u{9996}\u{5148}\u{ff0c}\u{4f60}\u{8981}\u{521b}\u{5efa}\u{4e00}\u{4e2a}\u{8868}\\
                 \
                 u{5f81}\u{4f60}\u{7684}\u{5e94}\u{7528}\u{7a0b}\u{5e8f}\u{6240}\u{6709}\u{72b6}\\
                 u{6001}\u{7684}\u{7c7b}\u{578b}\u{3002}\u{8bf7}\u{6ce8}\u{610f}\u{ff0c}\u{867d}\\
                 u{7136}\u{8fd9}\u{901a}\u{5e38}\u{662f}\u{4e00}\u{4e2a}\u{679a}\u{4e3e}\u{ff0c}\\
                 u{4f46}\u{4e5f}\u{652f}\u{6301}\u{7ed3}\u{6784}\u{4f53}\u{ff0c}\u{5e76}\u{4e14}\\
                 u{4f60}\u{53ef}\u{4ee5}\u{5728}\u{5185}\u{90e8}\u{5d4c}\u{5957}\u{5b9e}\u{73b0}\\
                 u{4e86} "
            ),
            code("Switch"),
            text(" trait \u{7684}\u{5176}\u{4ed6}\u{9879}\u{3002}"),
        ]),
        p(vec![
            text(
                "\u{7136}\u{540e}\u{4f60}\u{5e94}\u{8be5}\u{4e3a}\u{4e86}\u{4f60}\u{521b}\u{5efa}\\
                 \
                 u{7684}\u{7c7b}\u{578b}\u{6d3e}\u{751f} "
            ),
            code("Switch"),
            text(
                "\u{3002}\u{5bf9}\u{4e8e}\u{679a}\u{4e3e}\u{ff0c}\u{6bcf}\u{4e00}\u{4e2a}\u{6210}\\
                 \
                 u{5458}\u{90fd}\u{5fc5}\u{987b}\u{7528} "
            ),
            code("#[to = \"/some/route\"]"),
            text(
                " \u{8fdb}\u{884c}\u{6807}\u{6ce8}\u{ff0c}\u{5982}\u{679c}\u{4f60}\u{4f7f}\\
                 u{7528}\u{7ed3}\u{6784}\u{4f53}\u{ff0c}\u{5219}\u{6807}\u{6ce8}\u{5fc5}\u{987b}\\
                 u{51fa}\u{73b0}\u{5728}\u{7ed3}\u{6784}\u{4f53}\u{58f0}\u{660e}\u{4e4b}\u{5916}\\
                 u{3002}"
            ),
        ]),
        p(vec![
            text("\u{8bf7}\u{6ce8}\u{610f}\u{ff0c}\u{7531}\u{6d3e}\u{751f}\u{5b8f}\u{4e3a} "),
            code("Switch"),
            text(
                " \u{751f}\u{6210}\u{7684}\u{5b9e}\u{73b0}\u{ff0c}\u{5c06}\u{5c1d}\u{8bd5}\\
                 u{4ece}\u{5934}\u{5230}\u{5c3e}\u{4f9d}\u{6b21}\u{521b}\u{5efa}\u{6bcf}\u{4e2a}\\
                 u{6210}\u{5458}\u{ff0c}\u{56e0}\u{6b64}\u{ff0c}\u{5982}\u{679c}\u{4efb}\u{4f55}\\
                 u{8def}\u{7531}\u{53ef}\u{80fd}\u{4e0e}\u{4f60}\u{6307}\u{5b9a}\u{7684}\u{4e24}\\
                 u{4e2a} "
            ),
            code("to"),
            text(
                " \u{6807}\u{6ce8}\u{76f8}\u{5339}\u{914d}\u{ff0c}\u{90a3}\u{4e48}\u{7b2c}\\
                 u{4e00}\u{4e2a}\u{4f1a}\u{88ab}\u{5339}\u{914d}\u{ff0c}\u{7b2c}\u{4e8c}\u{4e2a}\\
                 u{5c06}\u{6c38}\u{8fdc}\u{4e0d}\u{4f1a}\u{88ab}\u{5c1d}\u{8bd5}\u{3002}"
            ),
        ]),
        p(vec![
            text("\u{4f60}\u{8fd8}\u{53ef}\u{4ee5}\u{5728} "),
            code("#[to = \"\"]"),
            text(" \u{6807}\u{6ce8}\u{4e2d}\u{4f7f}\u{7528} "),
            code("{}"),
            text(" \u{7684}\u{53d8}\u{4f53}\u{6765}\u{6355}\u{83b7}\u{7247}\u{6bb5}\u{3002}"),
            code("{}"),
            text(
                " \u{8868}\u{793a}\u{6355}\u{83b7}\u{6587}\u{672c}\u{76f4}\u{5230}\u{4e0b}\\
                 u{4e00}\u{4e2a}\u{5206}\u{9694}\u{7b26}\u{ff08}\u{6839}\u{636e}\u{4e0a}\u{4e0b}\\
                 u{6587}\u{53ef}\u{80fd}\u{662f}\"/\"\u{ff0c}\"?\"\u{ff0c}\"&\" \u{6216} \
                 \"\\#\"\u{ff09}\u{3002}"
            ),
            code("{*}"),
            text(
                " \u{8868}\u{793a}\u{6355}\u{83b7}\u{6587}\u{672c}\u{76f4}\u{5230}\u{540e}\\
                 u{7eed}\u{5b57}\u{7b26}\u{5339}\u{914d}\u{4e3a}\u{6b62}\u{ff0c}\u{5982}\u{679c}\\
                 u{4e0d}\u{5b58}\u{5728}\u{4efb}\u{4f55}\u{5b57}\u{7b26}\u{ff0c}\u{5219}\u{5b83}\\
                 u{5c06}\u{5339}\u{914d}\u{4efb}\u{4f55}\u{5185}\u{5bb9}\u{3002}"
            ),
            code("{<number>}"),
            text(
                " \u{8868}\u{793a}\u{6355}\u{83b7}\u{6587}\u{672c}\u{76f4}\u{5230}\u{9047}\\
                 u{5230}\u{6307}\u{5b9a}\u{6570}\u{76ee}\u{7684}\u{5206}\u{9694}\u{7b26}\u{4e3a}\\
                 u{6b62}\u{ff08}\u{4f8b}\u{5982}\u{ff1a}"
            ),
            code("{2}"),
            text(
                " \u{5c06}\u{4e00}\u{76f4}\u{6355}\u{83b7}\u{6587}\u{672c}\u{76f4}\u{5230}\\
                 u{9047}\u{5230}\u{4e24}\u{4e2a}\u{5206}\u{9694}\u{7b26}\u{4e3a}\u{6b62}\u{ff09}\\
                 u{3002}"
            ),
        ]),
        p(vec![
            text(
                "\u{5bf9}\u{4e8e}\u{5177}\u{6709}\u{547d}\u{540d}\u{5b57}\u{6bb5}\u{7684}\u{7ed3}\\
                 \
                 u{6784}\u{4f53}\u{548c}\u{679a}\u{4e3e}\u{ff0c}\u{4f60}\u{5fc5}\u{987b}\u{5728}\\
                 u{6355}\u{83b7}\u{7ec4}\u{4e2d}\u{6307}\u{5b9a}\u{5b57}\u{6bb5}\u{7684}\u{540d}\\
                 u{79f0}\u{ff0c}\u{4f8b}\u{5982}\u{ff1a}"
            ),
            code("{user_name}"),
            text(" \u{6216} "),
            code("{*:age}"),
            text("\u{3002}"),
        ]),
        p(vec![
            text(
                "Switch trait \
                 \u{9002}\u{7528}\u{4e8e}\u{6bd4}\u{5b57}\u{7b26}\u{4e32}\u{66f4}\u{7ed3}\u{6784}\\
                 \
                 u{5316}\u{7684}\u{6355}\u{83b7}\u{7ec4}\u{3002}\u{4f60}\u{53ef}\u{4ee5}\u{6307}\\
                 u{5b9a}\u{5b9e}\u{73b0}\u{4e86} "
            ),
            code("Switch"),
            text(
                " trait \u{7684}\u{4efb}\u{4f55}\u{7c7b}\u{578b}\u{3002}\u{56e0}\u{6b64}\u{ff0c}\\
                 u{4f60}\u{53ef}\u{4ee5}\u{6307}\u{5b9a}\u{6355}\u{83b7}\u{7ec4}\u{4e3a} "
            ),
            code("usize"),
            text(
                "\u{ff0c}\u{5e76}\u{4e14}\u{5982}\u{679c} URL \
                 \u{7684}\u{6355}\u{83b7}\u{90e8}\u{5206}\u{65e0}\u{6cd5}\u{8f6c}\u{6362}\u{4e3a}\\
                 \
                 u{5b83}\u{ff0c}\u{5219}\u{8be5}\u{6210}\u{5458}\u{4e0d}\u{4f1a}\u{88ab}\u{5339}\\
                 u{914d}\u{3002}"
            ),
        ]),
    ])
);
