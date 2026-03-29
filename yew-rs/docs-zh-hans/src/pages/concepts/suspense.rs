pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["占位标签 (Suspense) \
             是一种在等待任务完成前暂停组件渲染的方式，同时显示一个回退（占位符）UI。"],
        p!["它可以用于从服务器获取数据，等待代理完成任务，或执行其他后台异步任务。"],
        p!["在占位标签出现之前，数据获取通常发生在组件渲染之后（渲染时获取）或之前（获取后渲染）。"],
        h3!["边渲染，边下载"],
        p!["占位标签 (Suspense) \
             提供了一种新的方法，允许组件在渲染过程中发起数据请求。当组件发起数据请求时，渲染过程将被暂停，并显示一个回退 \
             UI，直到请求完成。"],
        p!["推荐使用钩子 (Hook) 来使用占位标签。"],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"#,
        ),
        p![
            "在上面的示例中，",
            code("use_user"),
            " 钩子将在加载用户信息时暂停组件渲染，并在加载 ",
            code("user"),
            " 之前显示 ",
            code("Loading..."),
            " 占位符。",
        ],
        p![
            "要定义一个暂停组件渲染的钩子，它需要返回一个 ",
            code("SuspensionResult<T>"),
            "。当组件需要被暂停时，钩子应该返回一个 ",
            code("Err(Suspension)"),
            "，用户应该使用 ",
            code("?"),
            " 解包它，这样它将被转换为 ",
            code("Html"),
            "。",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

struct User {
    name: String,
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // 当用户加载完成时，我们将其作为 Ok(user) 返回。
        Some(m) => Ok(m),
        None => {
            // 当用户仍在加载时，我们创建一个 `Suspension`
            // 并在数据加载完成时调用 `SuspensionHandle::resume`，
            // 组件将自动重新渲染。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}"#,
        ),
        h4!["关于实现暂停钩子 (Hook) 的注意事项"],
        p![
            link!(
                "https://docs.rs/yew/latest/yew/suspense/struct.Suspension.html#method.new",
                code("Suspension::new"),
            ),
            " 返回 2 个值：暂停上下文本身和一个暂停句柄。后者负责在何时重新渲染暂停的组件，它提供了 2 种可互换的方法：",
        ],
        ol![
            li![
                "调用其 ",
                link!(
                    "https://docs.rs/yew/latest/yew/suspense/struct.SuspensionHandle.html#method.resume",
                    code("resume"),
                ),
                " 方法。",
            ],
            li!["丢弃句柄。"],
        ],
        admonition!(
            AdmonitionType::Danger,
            None,
            p![
                "暂停句柄必须存储直到更新组件的时候，即使用新接收的数据；否则，暂停的组件将进入无限重新渲染循环，从而影响性能。\n在上面的示例中，暂停句柄通过移动到闭包中并传递给 ",
                code("on_load_user_complete"),
                " 来保存。\n当虚拟用户加载时，将调用闭包，从而调用 ",
                code("handle.resume()"),
                " 并重新渲染与暂停上下文相关的组件。",
            ],
        ),
        h1!["完整示例"],
        code_block(
            "rust",
            r#"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn load_user() -> Option<User> {
    todo!()  // 略
}

fn on_load_user_complete<F: FnOnce()>(_fn: F) {
    todo!()  // 略
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // 如果用户已加载，则将其作为 Ok(user) 返回。
        Some(m) => Ok(m),
        None => {
            // 当用户仍在加载时，我们创建一个 `Suspension`
            // 并在数据加载完成时调用 `SuspensionHandle::resume`，
            // 组件将自动重新渲染。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"#,
        ),
        h3!["在结构体组件中使用占位标签"],
        p![
            "直接暂停结构体组件是不可能的。然而，您可以使用一个函数组件作为",
            doc_link!(crate::pages::advanced_topics::struct_components::hoc, "高阶组件"),
            "来实现基于占位标签的数据获取。",
        ],
        p![
            "Yew 仓库中的",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/suspense/src/struct_consumer.rs",
                "占位标签示例",
            ),
            "演示了如何使用这个组件。",
        ],
        h2!["相关示例"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/suspense",
            "占位标签",
        )]],
    ])
    .with_description("Suspense for data fetching")
}

crate::doc_page!(
    "占位标签 (Suspense)",
    "/zh-Hans/docs/concepts/suspense",
    page_content()
);
