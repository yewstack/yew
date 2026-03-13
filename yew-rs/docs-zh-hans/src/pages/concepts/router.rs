pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text(
            "单页应用程序 (SPA) 中的路由器处理根据 URL \
             显示不同的页面。与点击链接时请求不同的远程资源的默认行为不同，路由器会在本地设置 URL \
             以指向应用程序中的有效路由。然后，路由器检测到此更改并决定要渲染的内容。",
        )]),
        p(vec![
            text("Yew 在 "),
            code("yew-router"),
            text(" crate 中提供了路由器支持。要开始使用它，请将依赖项添加到您的 "),
            code("Cargo.toml"),
            text(" 文件中。"),
        ]),
        code_block(
            "toml",
            r#"yew-router = { git = "https://github.com/yewstack/yew.git" }"#,
        ),
        p(vec![
            text("所需的工具均在 "),
            code("yew_router::prelude"),
            text(" 模块中提供，"),
        ]),
        h2(vec![text("用法")]),
        p(vec![
            text("最开始，你需要定义一个 "),
            code("Route"),
            text("。"),
        ]),
        p(vec![
            text("路由由一个 "),
            code("enum"),
            text(" 定义，它派生自 "),
            code("Routable"),
            text("。这个枚举必须实现 "),
            code("Clone + PartialEq"),
            text("。"),
        ]),
        code_block(
            "rust",
            r#"use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}"#,
        ),
        p(vec![
            code("Route"),
            text(" 与 "),
            code("<Switch />"),
            text(" 组件配对，后者会找到与浏览器当前 URL 匹配的路径变体，并将其传递给 "),
            code("render"),
            text(" 回调。然后回调决定要渲染的内容。如果没有路径匹配，路由器会导航到带有 "),
            code("not_found"),
            text(
                " 属性的路径。如果没有指定路由，则不会渲染任何内容，并且会在控制台中记录一条消息，\
                 说明没有匹配的路由。",
            ),
        ]),
        p(vec![
            text("yew-router 的大多数组件，特别是 "),
            code("<Link />"),
            text(" 和 "),
            code("<Switch />"),
            text("，必须是某个 Router 组件（例如 "),
            code("<BrowserRouter />"),
            text("）的（深层）子元素。通常在应用程序中只需要一个 Router，通常由最顶层的 "),
            code("<App />"),
            text(
                " 组件立即渲染。Router 注册了一个上下文，这是 Links 和 Switches \
                 功能所需的。下面提供了一个示例。",
            ),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("在浏览器环境中使用 "),
                code("yew-router"),
                text(" 时，强烈推荐使用 "),
                code("<BrowserRouter />"),
                text("。您可以在 "),
                link("https://docs.rs/yew-router/", vec![text("API 参考")]),
                text(" 中找到其他路由器类型。"),
            ])],
        ),
        code_block(
            "rust",
            r#"use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}"#,
        ),
        h3(vec![text("路径段")]),
        p(vec![
            text("路由还可以使用动态和命名通配符段从路由中提取信息。然后，您可以在 "),
            code("<Switch />"),
            text(" 内访问帖子的 id，并通过属性将其转发到相应的组件。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/*path")]
    Misc { path: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Post { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
        Route::Misc { path } => html! {<p>{format!("Matched some other path: {}", path)}</p>},
    }
}"#,
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("您也可以使用普通的 "),
                code("Post"),
                text(" 变体，而不是 "),
                code("Post {id: String}"),
                text("。例如，当 "),
                code("Post"),
                text(
                    " 与另一个路由器一起渲染时，该字段可能是多余的，\
                     因为另一个路由器可以匹配并处理路径。有关详细信息，请参阅下面的",
                ),
                link("#nested-router", vec![text("嵌套路由器")]),
                text("部分。"),
            ])],
        ),
        p(vec![
            text("请注意，字段必须实现 "),
            code("Clone + PartialEq"),
            text(" 作为 "),
            code("Route"),
            text(" 枚举的一部分。它们还必须实现 "),
            code("std::fmt::Display"),
            text(" 和 "),
            code("std::str::FromStr"),
            text(" 以进行序列化和反序列化。整数、浮点数和字符串等原始类型已经满足这些要求。"),
        ]),
        p(vec![
            text("当路径的形式匹配，但反序列化失败（根据 "),
            code("FromStr"),
            text(
                "）时。路由器将认为路由不匹配，\
                 并尝试渲染未找到的路由（或者如果未指定未找到的路由，则渲染空白页面）。",
            ),
        ]),
        p(vec![text("参考以下示例：")]),
        code_block(
            "rust",
            r#"#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/news/:id")]
    News { id: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
// 切换函数会渲染 News 和 id。这里省略了。"#,
        ),
        p(vec![
            text("当段超过 255 时，"),
            code("u8::from_str()"),
            text(" 将失败并返回 "),
            code("ParseIntError"),
            text("，路由器将认为路由不匹配。"),
        ]),
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "router deserialization failure behavior",
        ),
        p(vec![
            text("有关路由语法和如何绑定参数的更多信息，请查看 "),
            link(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                vec![text("route-recognizer")],
            ),
            text("。"),
        ]),
        h3(vec![text("位置 (Location)")]),
        p(vec![
            text("路由器通过上下文提供了一个通用的 "),
            code("Location"),
            text(" 结构，可以用于访问路由信息。它们可以通过钩子或 "),
            code("ctx.link()"),
            text(" 上的便捷函数来检索。"),
        ]),
        h3(vec![text("导航")]),
        p(vec![
            code("yew_router"),
            text(" 提供了一些工具来处理导航。"),
        ]),
        h4(vec![text("链接")]),
        p(vec![
            code("<Link />"),
            text(" 渲染为 "),
            code("<a>"),
            text(" 元素，"),
            code("onclick"),
            text(" 事件处理程序将调用 "),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault",
                vec![text("preventDefault")],
            ),
            text(
                "，并将目标页面推送到历史记录中并渲染所需的页面，这正是单页应用程序所期望的行为。\
                 普通锚元素的默认 ",
            ),
            code("onclick"),
            text(" 会重新加载页面。"),
        ]),
        p(vec![
            code("<Link />"),
            text(" 组件还会将其子元素传递给 "),
            code("<a>"),
            text(" 元素。可以将其视为应用内路由的 "),
            code("<a/>"),
            text(" 替代品。不同之处在于你需要提供 "),
            code("to"),
            text(" 属性而不是 "),
            code("href"),
            text("。示例用法如下："),
        ]),
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#,
        ),
        p(vec![text("结构体变量也可以正常工作：")]),
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew!" }</Link<Route>>"#,
        ),
        h4(vec![text("导航接口")]),
        p(vec![
            text(
                "导航器 API 为函数组件和结构组件提供。它们使回调能够更改路由。\
                 可以在任一情况下获取 ",
            ),
            code("Navigator"),
            text(" 实例以操作路由。"),
        ]),
        h5(vec![text("函数式组件")]),
        p(vec![
            text("对于函数组件，当底层导航器提供程序更改时，"),
            code("use_navigator"),
            text(" 钩子会重新渲染组件。 以下是实现一个按钮的示例，该按钮在点击时导航到 "),
            code("Home"),
            text(" 路由。"),
        ]),
        code_block(
            "rust",
            r#"#[component(MyComponent)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <button {onclick}>{"Click to go home"}</button>
        </>
    }
}"#,
        ),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("这里的示例使用了 "),
                code("Callback::from"),
                text(
                    "。如果目标路由可以与组件所在的路由相同，或者只是为了安全起见，\
                     请使用普通的回调。例如，考虑在每个页面上都有一个徽标按钮，\
                     点击该按钮会返回主页。在主页上点击该按钮两次会导致代码崩溃，\
                     因为第二次点击会推送一个相同的 Home 路由，并且 ",
                ),
                code("use_navigator"),
                text(" 钩子不会触发重新渲染。"),
            ])],
        ),
        p(vec![
            text("如果您想替换当前的位置而不是将新位置推到堆栈上，请使用 "),
            code("navigator.replace()"),
            text(" 而不是 "),
            code("navigator.push()"),
            text("。"),
        ]),
        p(vec![
            text("您可能会注意到 "),
            code("navigator"),
            text(" 必须移动到回调中，因此不能再次用于其他回调。幸运的是，"),
            code("navigator"),
            text(" 实现了 "),
            code("Clone"),
            text("，例如，以下是如何为不同的路由设置多个按钮："),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_router::prelude::*;

#[component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    let go_to_first_post_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Post { id: "first-post".to_string() }));
        html! {
            <button {onclick}>{"click to go the first post"}</button>
        }
    };

    let go_to_secure_button = {
        let onclick = Callback::from(move |_| navigator.push(&Route::Secure));
        html! {
            <button {onclick}>{"click to go to secure"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_first_post_button}
            {go_to_secure_button}
        </>
    }
}"#,
        ),
        h5(vec![text("结构体组件")]),
        p(vec![
            text("对于结构体组件，可以通过 "),
            code("ctx.link().navigator()"),
            text(" API 获取 "),
            code("Navigator"),
            text(" 实例。其余部分与函数组件的情况相同。以下是一个渲染单个按钮的视图函数示例。"),
        ]),
        code_block(
            "rust",
            r#"fn view(&self, ctx: &Context<Self>) -> Html {
    let navigator = ctx.link().navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html!{
        <button {onclick}>{"Go Home"}</button>
    }
}"#,
        ),
        h4(vec![text("重定向")]),
        p(vec![
            code("yew-router"),
            text(" 还在 prelude 中提供了一个 "),
            code("<Redirect />"),
            text(" 组件。它可以用于实现与导航器 API 类似的效果。该组件接受一个 "),
            code("to"),
            text(" 属性作为目标路由。当渲染 "),
            code("<Redirect/>"),
            text(" 时，用户将被重定向到属性中指定的路由。以下是一个示例："),
        ]),
        code_block(
            "rust",
            r#"#[component(SomePage)]
fn some_page() -> Html {
    // 建立对 `use_user` 的钩子
    let user = match use_user() {
        Some(user) => user,
        // 当用户为 `None` 时重定向到登录页面
        None => return html! {
            <Redirect<Route> to={Route::Login}/>
        },
    };
    // ... 实际页面内容
}"#,
        ),
        admonition(
            AdmonitionType::Tip,
            Some("如何选择 `Redirect` 或 `Navigator`"),
            vec![p(vec![
                text("Navigator API 是在回调中操作路由的唯一方法。\n而 "),
                code("<Redirect />"),
                text(" 可以作为组件中的返回值使用。您可能还希望在其他非组件上下文中使用 "),
                code("<Redirect />"),
                text("，例如在"),
                link("#nested-router", vec![text("嵌套路由器")]),
                text("的 switch 函数中。"),
            ])],
        ),
        h3(vec![text("监听变化")]),
        h4(vec![text("函数式组件")]),
        p(vec![
            text("您可以使用 "),
            code("use_location"),
            text(" 和 "),
            code("use_route"),
            text(" 钩子。当提供的值发生变化时，您的组件将重新渲染。"),
        ]),
        h4(vec![text("结构体组件")]),
        p(vec![
            text("为了响应路由变化，您可以将回调闭包传递给 "),
            code("ctx.link()"),
            text(" 的 "),
            code("add_location_listener()"),
            text(" 方法。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text(
                "一旦位置监听器被删除，它将被取消注册。请确保将句柄存储在组件状态中。",
            )])],
        ),
        code_block(
            "rust",
            r#"fn create(ctx: &Context<Self>) -> Self {
    let listener = ctx.link()
        .add_location_listener(ctx.link().callback(
            // 处理事件
        ))
        .unwrap();
    MyComponent {
        _listener: listener
    }
}"#,
        ),
        p(vec![
            code("ctx.link().location()"),
            text(" 和 "),
            code("ctx.link().route::<R>()"),
            text(" 也可以用于一次性检索位置和路由。"),
        ]),
        h3(vec![text("查询参数")]),
        h4(vec![text("在导航时指定查询参数")]),
        p(vec![
            text("为了在导航到新路由时指定查询参数，可以使用 "),
            code("navigator.push_with_query"),
            text(" 或 "),
            code("navigator.replace_with_query"),
            text(" 函数。它使用 "),
            code("serde"),
            text(" 将参数序列化为 URL 的查询字符串，因此任何实现了 "),
            code("Serialize"),
            text(" 的类型都可以传递。最简单的形式是包含字符串对的 "),
            code("HashMap"),
            text("。"),
        ]),
        h4(vec![text("获取当前路由的查询参数")]),
        p(vec![
            code("location.query"),
            text(" 用于获取查询参数。它使用 "),
            code("serde"),
            text(" 从 URL 的查询字符串中反序列化参数。"),
        ]),
        h2_id("nested-router", vec![text("嵌套路由器")]),
        p(vec![text(
            "当应用程序变得更大时，嵌套路由器可能会很有用。考虑以下路由器结构：",
        )]),
        themed_img(
            "/img/nested-router-light.svg",
            "/img/nested-router-dark.svg",
            "nested router structure",
        ),
        p(vec![
            text("嵌套的 "),
            code("SettingsRouter"),
            text(" 处理所有以 "),
            code("/settings"),
            text(" 开头的 URL。此外，它会将未匹配的 URL 重定向到主 "),
            code("NotFound"),
            text(" 路由。因此，"),
            code("/settings/gibberish"),
            text(" 将重定向到 "),
            code("/404"),
            text("。"),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![text(
                "请注意，该接口仍在开发中，这样写的方式尚未最终确定",
            )])],
        ),
        p(vec![text("可以使用以下代码实现：")]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew_router::prelude::*;
use gloo::utils::window;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/news")]
    News,
    #[at("/contact")]
    Contact,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings")]
    Profile,
    #[at("/settings/friends")]
    Friends,
    #[at("/settings/theme")]
    Theme,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<h1>{"Home"}</h1>},
        MainRoute::News => html! {<h1>{"News"}</h1>},
        MainRoute::Contact => html! {<h1>{"Contact"}</h1>},
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<h1>{"Profile"}</h1>},
        SettingsRoute::Friends => html! {<h1>{"Friends"}</h1>},
        SettingsRoute::Theme => html! {<h1>{"Theme"}</h1>},
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

#[component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}"#,
        ),
        h3(vec![text("基底路径 (Basename)")]),
        p(vec![
            text("可以使用 "),
            code("yew-router"),
            text(" 定义基底路径 (Basename)。 基底路径是所有路由的公共前缀。导航器 API 和 "),
            code("<Switch />"),
            text(
                " 组件都支持基底路径设置。所有推送的路由都会加上基底路径前缀，所有的 switch \
                 都会在尝试将路径解析为 ",
            ),
            code("Routable"),
            text(" 之前去掉基底路径。"),
        ]),
        p(vec![
            text("如果没有为 Router 组件提供基底路径属性，它将使用 HTML 文件中 "),
            code("<base />"),
            text(" 元素的 href 属性，并在 HTML 文件中没有 "),
            code("<base />"),
            text(" 元素时回退到 "),
            code("/"),
            text("。"),
        ]),
        h2(vec![text("相关示例")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/router",
            vec![text("路由")],
        )])]),
        h2(vec![text("接口参考")]),
        ul(vec![li(vec![link(
            "https://docs.rs/yew-router/",
            vec![text("yew-router")],
        )])]),
    ])
}

crate::doc_page!(
    "路由 (Router)",
    "/zh-Hans/docs/concepts/router",
    page_content()
);
