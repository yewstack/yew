pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["单页应用程序 (SPA) 中的路由器处理根据 URL \
            显示不同的页面。与点击链接时请求不同的远程资源的默认行为不同，路由器会在本地设置 URL \
            以指向应用程序中的有效路由。然后，路由器检测到此更改并决定要渲染的内容。"],
        p![
            "Yew 在 ",
            code("yew-router"),
            " crate 中提供了路由器支持。要开始使用它，请将依赖项添加到您的 ",
            code("Cargo.toml"),
            " 文件中。",
        ],
        code_block(
            "toml",
            r#"yew-router = { git = "https://github.com/yewstack/yew.git" }"#,
        ),
        p![
            "所需的工具均在 ",
            code("yew_router::prelude"),
            " 模块中提供，",
        ],
        h2!["用法"],
        p!["最开始，你需要定义一个 ", code("Route"), "。",],
        p![
            "路由由一个 ",
            code("enum"),
            " 定义，它派生自 ",
            code("Routable"),
            "。这个枚举必须实现 ",
            code("Clone + PartialEq"),
            "。",
        ],
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
        p![
            code("Route"),
            " 与 ",
            code("<Switch />"),
            " 组件配对，后者会找到与浏览器当前 URL 匹配的路径变体，并将其传递给 ",
            code("render"),
            " 回调。然后回调决定要渲染的内容。如果没有路径匹配，路由器会导航到带有 ",
            code("not_found"),
            " 属性的路径。如果没有指定路由，则不会渲染任何内容，并且会在控制台中记录一条消息，\
             说明没有匹配的路由。",
        ],
        p![
            "yew-router 的大多数组件，特别是 ",
            code("<Link />"),
            " 和 ",
            code("<Switch />"),
            "，必须是某个 Router 组件（例如 ",
            code("<BrowserRouter />"),
            "）的（深层）子元素。通常在应用程序中只需要一个 Router，通常由最顶层的 ",
            code("<App />"),
            " 组件立即渲染。Router 注册了一个上下文，这是 Links 和 Switches \
             功能所需的。下面提供了一个示例。",
        ],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "在浏览器环境中使用 ",
                code("yew-router"),
                " 时，强烈推荐使用 ",
                code("<BrowserRouter />"),
                "。您可以在 ",
                link!("https://docs.rs/yew-router/", "API 参考"),
                " 中找到其他路由器类型。",
            ],
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
        h3!["路径段"],
        p![
            "路由还可以使用动态和命名通配符段从路由中提取信息。然后，您可以在 ",
            code("<Switch />"),
            " 内访问帖子的 id，并通过属性将其转发到相应的组件。",
        ],
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
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "您也可以使用普通的 ",
                code("Post"),
                " 变体，而不是 ",
                code("Post {id: String}"),
                "。例如，当 ",
                code("Post"),
                " 与另一个路由器一起渲染时，该字段可能是多余的，\
                 因为另一个路由器可以匹配并处理路径。有关详细信息，请参阅下面的",
                link!("#nested-router", "嵌套路由器"),
                "部分。",
            ],
        ),
        p![
            "请注意，字段必须实现 ",
            code("Clone + PartialEq"),
            " 作为 ",
            code("Route"),
            " 枚举的一部分。它们还必须实现 ",
            code("std::fmt::Display"),
            " 和 ",
            code("std::str::FromStr"),
            " 以进行序列化和反序列化。整数、浮点数和字符串等原始类型已经满足这些要求。",
        ],
        p![
            "当路径的形式匹配，但反序列化失败（根据 ",
            code("FromStr"),
            "）时。路由器将认为路由不匹配，并尝试渲染未找到的路由（或者如果未指定未找到的路由，\
             则渲染空白页面）。",
        ],
        p!["参考以下示例："],
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
        p![
            "当段超过 255 时，",
            code("u8::from_str()"),
            " 将失败并返回 ",
            code("ParseIntError"),
            "，路由器将认为路由不匹配。",
        ],
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "router deserialization failure behavior",
        ),
        p![
            "有关路由语法和如何绑定参数的更多信息，请查看 ",
            link!(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                "route-recognizer",
            ),
            "。",
        ],
        h3!["位置 (Location)"],
        p![
            "路由器通过上下文提供了一个通用的 ",
            code("Location"),
            " 结构，可以用于访问路由信息。它们可以通过钩子或 ",
            code("ctx.link()"),
            " 上的便捷函数来检索。",
        ],
        h3!["导航"],
        p![code("yew_router"), " 提供了一些工具来处理导航。",],
        h4!["链接"],
        p![
            code("<Link />"),
            " 渲染为 ",
            code("<a>"),
            " 元素，",
            code("onclick"),
            " 事件处理程序将调用 ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault",
                "preventDefault",
            ),
            "，并将目标页面推送到历史记录中并渲染所需的页面，这正是单页应用程序所期望的行为。\
             普通锚元素的默认 ",
            code("onclick"),
            " 会重新加载页面。",
        ],
        p![
            code("<Link />"),
            " 组件还会将其子元素传递给 ",
            code("<a>"),
            " 元素。可以将其视为应用内路由的 ",
            code("<a/>"),
            " 替代品。不同之处在于你需要提供 ",
            code("to"),
            " 属性而不是 ",
            code("href"),
            "。示例用法如下：",
        ],
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#,
        ),
        p!["结构体变量也可以正常工作："],
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew!" }</Link<Route>>"#,
        ),
        h4!["导航接口"],
        p![
            "导航器 API 为函数组件和结构组件提供。它们使回调能够更改路由。可以在任一情况下获取 ",
            code("Navigator"),
            " 实例以操作路由。",
        ],
        h5!["函数式组件"],
        p![
            "对于函数组件，当底层导航器提供程序更改时，",
            code("use_navigator"),
            " 钩子会重新渲染组件。 以下是实现一个按钮的示例，该按钮在点击时导航到 ",
            code("Home"),
            " 路由。",
        ],
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
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "这里的示例使用了 ",
                code("Callback::from"),
                "。如果目标路由可以与组件所在的路由相同，或者只是为了安全起见，请使用普通的回调。\
                 例如，考虑在每个页面上都有一个徽标按钮，点击该按钮会返回主页。\
                 在主页上点击该按钮两次会导致代码崩溃，因为第二次点击会推送一个相同的 Home \
                 路由，并且 ",
                code("use_navigator"),
                " 钩子不会触发重新渲染。",
            ],
        ),
        p![
            "如果您想替换当前的位置而不是将新位置推到堆栈上，请使用 ",
            code("navigator.replace()"),
            " 而不是 ",
            code("navigator.push()"),
            "。",
        ],
        p![
            "您可能会注意到 ",
            code("navigator"),
            " 必须移动到回调中，因此不能再次用于其他回调。幸运的是，",
            code("navigator"),
            " 实现了 ",
            code("Clone"),
            "，例如，以下是如何为不同的路由设置多个按钮：",
        ],
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
        h5!["结构体组件"],
        p![
            "对于结构体组件，可以通过 ",
            code("ctx.link().navigator()"),
            " API 获取 ",
            code("Navigator"),
            " 实例。其余部分与函数组件的情况相同。以下是一个渲染单个按钮的视图函数示例。",
        ],
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
        h4!["重定向"],
        p![
            code("yew-router"),
            " 还在 prelude 中提供了一个 ",
            code("<Redirect />"),
            " 组件。它可以用于实现与导航器 API 类似的效果。该组件接受一个 ",
            code("to"),
            " 属性作为目标路由。当渲染 ",
            code("<Redirect/>"),
            " 时，用户将被重定向到属性中指定的路由。以下是一个示例：",
        ],
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
        admonition!(
            AdmonitionType::Tip,
            Some("如何选择 `Redirect` 或 `Navigator`"),
            p![
                "Navigator API 是在回调中操作路由的唯一方法。\n而 ",
                code("<Redirect />"),
                " 可以作为组件中的返回值使用。您可能还希望在其他非组件上下文中使用 ",
                code("<Redirect />"),
                "，例如在",
                link!("#nested-router", "嵌套路由器"),
                "的 switch 函数中。",
            ],
        ),
        h3!["监听变化"],
        h4!["函数式组件"],
        p![
            "您可以使用 ",
            code("use_location"),
            " 和 ",
            code("use_route"),
            " 钩子。当提供的值发生变化时，您的组件将重新渲染。",
        ],
        h4!["结构体组件"],
        p![
            "为了响应路由变化，您可以将回调闭包传递给 ",
            code("ctx.link()"),
            " 的 ",
            code("add_location_listener()"),
            " 方法。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p!["一旦位置监听器被删除，它将被取消注册。请确保将句柄存储在组件状态中。"],
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
        p![
            code("ctx.link().location()"),
            " 和 ",
            code("ctx.link().route::<R>()"),
            " 也可以用于一次性检索位置和路由。",
        ],
        h3!["查询参数"],
        h4!["在导航时指定查询参数"],
        p![
            "为了在导航到新路由时指定查询参数，可以使用 ",
            code("navigator.push_with_query"),
            " 或 ",
            code("navigator.replace_with_query"),
            " 函数。它使用 ",
            code("serde"),
            " 将参数序列化为 URL 的查询字符串，因此任何实现了 ",
            code("Serialize"),
            " 的类型都可以传递。最简单的形式是包含字符串对的 ",
            code("HashMap"),
            "。",
        ],
        h4!["获取当前路由的查询参数"],
        p![
            code("location.query"),
            " 用于获取查询参数。它使用 ",
            code("serde"),
            " 从 URL 的查询字符串中反序列化参数。",
        ],
        h2_id!("nested-router", "嵌套路由器"),
        p!["当应用程序变得更大时，嵌套路由器可能会很有用。考虑以下路由器结构："],
        themed_img(
            "/img/nested-router-light.svg",
            "/img/nested-router-dark.svg",
            "nested router structure",
        ),
        p![
            "嵌套的 ",
            code("SettingsRouter"),
            " 处理所有以 ",
            code("/settings"),
            " 开头的 URL。此外，它会将未匹配的 URL 重定向到主 ",
            code("NotFound"),
            " 路由。因此，",
            code("/settings/gibberish"),
            " 将重定向到 ",
            code("/404"),
            "。",
        ],
        admonition!(
            AdmonitionType::Caution,
            None,
            p!["请注意，该接口仍在开发中，这样写的方式尚未最终确定"],
        ),
        p!["可以使用以下代码实现："],
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
        h3!["基底路径 (Basename)"],
        p![
            "可以使用 ",
            code("yew-router"),
            " 定义基底路径 (Basename)。 基底路径是所有路由的公共前缀。导航器 API 和 ",
            code("<Switch />"),
            " 组件都支持基底路径设置。所有推送的路由都会加上基底路径前缀，所有的 switch \
             都会在尝试将路径解析为 ",
            code("Routable"),
            " 之前去掉基底路径。",
        ],
        p![
            "如果没有为 Router 组件提供基底路径属性，它将使用 HTML 文件中 ",
            code("<base />"),
            " 元素的 href 属性，并在 HTML 文件中没有 ",
            code("<base />"),
            " 元素时回退到 ",
            code("/"),
            "。",
        ],
        h2!["相关示例"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/router",
            "路由",
        )]],
        h2!["接口参考"],
        ul![li![link!("https://docs.rs/yew-router/", "yew-router",)]],
    ])
    .with_description("Yew's official router")
}

crate::doc_page!(
    "路由 (Router)",
    "/zh-Hans/docs/concepts/router",
    page_content()
);
