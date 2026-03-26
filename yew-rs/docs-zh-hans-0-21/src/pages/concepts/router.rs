crate::doc_page!(
    "",
    "/zh-Hans/docs/concepts/router",
    Content::new(vec![
        h1!["Router"],
        p![link![
            "https://crates.io/crates/yew-router",
            "https://crates.io/crates/yew-router"
        ]],
        p![
            "Routers 在单页应用（SPA）中根据 URL 的不同显示不同的页面。当点击一个链接时，Router \
             在本地设置 URL 以指向应用程序中有效的路由，而不是默认请求一个不同的远程资源。然后 \
             Router 检测到此更改后决定要渲染的内容。"
        ],
        h2!["核心元素"],
        h3!["Route"],
        p![
            "包含一个字符串，该字符串表示网址中域名之后的所有内容，还可以选择表示存储在 history \
             api 中的状态。"
        ],
        h3!["RouteService"],
        p!["与浏览器通信以获取和设置路由。"],
        h3!["RouteAgent"],
        p!["拥有一个 RouteService，并用于当路由改变时协调更新，\
            无论更新是来自应用程序自身逻辑还是来自浏览器触发的事件。"],
        h3!["Switch"],
        p![
            code("Switch"),
            " trait 用于在该 trait 的实现者之间转换 ",
            code("Route"),
            "。"
        ],
        h3!["Router"],
        p![
            "Router 组件同 ",
            code("RouterAgent"),
            " 进行通信，并将自动把它从 Agent 那里获得的 Routes 解析为 Switches，并通过 ",
            code("render"),
            " 属性暴露该 Switch，该属性允许指定将生成的 Switch 转换为 ",
            code("HTML"),
            " 的方式。"
        ],
        h2!["如何使用 Router"],
        p![
            "首先，你要创建一个表征你的应用程序所有状态的类型。请注意，虽然这通常是一个枚举，\
             但也支持结构体，并且你可以在内部嵌套实现了 ",
            code("Switch"),
            " trait 的其他项。"
        ],
        p![
            "然后你应该为了你创建的类型派生 ",
            code("Switch"),
            "。对于枚举，每一个成员都必须用 ",
            code("#[to = \"/some/route\"]"),
            " 进行标注，如果你使用结构体，则标注必须出现在结构体声明之外。"
        ],
        p![
            "请注意，由派生宏为 ",
            code("Switch"),
            " 生成的实现，将尝试从头到尾依次创建每个成员，因此，如果任何路由可能与你指定的两个 ",
            code("to"),
            " 标注相匹配，那么第一个会被匹配，第二个将永远不会被尝试。"
        ],
        p![
            "你还可以在 ",
            code("#[to = \"\"]"),
            " 标注中使用 ",
            code("{}"),
            " 的变体来捕获片段。",
            code("{}"),
            " 表示捕获文本直到下一个分隔符（根据上下文可能是\"/\"，\"?\"，\"&\" 或 \"\\#\"）。",
            code("{*}"),
            " 表示捕获文本直到后续字符匹配为止，如果不存在任何字符，则它将匹配任何内容。",
            code("{<number>}"),
            " 表示捕获文本直到遇到指定数目的分隔符为止（例如：",
            code("{2}"),
            " 将一直捕获文本直到遇到两个分隔符为止）。"
        ],
        p![
            "对于具有命名字段的结构体和枚举，你必须在捕获组中指定字段的名称，例如：",
            code("{user_name}"),
            " 或 ",
            code("{*:age}"),
            "。"
        ],
        p![
            "Switch trait 适用于比字符串更结构化的捕获组。你可以指定实现了 ",
            code("Switch"),
            " trait 的任何类型。因此，你可以指定捕获组为 ",
            code("usize"),
            "，并且如果 URL 的捕获部分无法转换为它，则该成员不会被匹配。"
        ]
    ])
);
