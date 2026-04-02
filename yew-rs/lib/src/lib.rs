pub mod components;
pub mod content;
pub mod styles;

pub use components::examples_gallery::examples_page_content;
pub use components::footer::Footer;
pub use components::layout::Layout;
pub use components::navbar::Navbar;
pub use components::sidebar::{
    docs_sidebar, flatten_sidebar, migration_guides_sidebar, Sidebar, SidebarCategory,
    SidebarEntry, SidebarItem,
};
pub use content::Content;
use implicit_clone::unsync::IArray;
use yew::prelude::*;

pub const LATEST_STABLE: &str = "0.23";

pub fn lang_prefix(lang: &str) -> String {
    if lang.is_empty() {
        String::new()
    } else {
        format!("/{lang}")
    }
}

pub fn strip_lang_prefix<'a>(path: &'a str, lang: &str) -> &'a str {
    if lang.is_empty() {
        path
    } else {
        let prefix = format!("/{lang}");
        path.strip_prefix(&prefix).unwrap_or(path)
    }
}

#[derive(Clone, PartialEq)]
pub struct NavigationContext {
    pub navigate: Callback<(MouseEvent, AttrValue)>,
}

#[derive(Clone, PartialEq)]
pub struct DocContext {
    pub lang: AttrValue,
    pub doc_version: AttrValue,
}

pub fn nav_onclick(
    nav_ctx: &Option<NavigationContext>,
    href: &str,
) -> Option<Callback<MouseEvent>> {
    let nav = nav_ctx.as_ref()?;
    let navigate = nav.navigate.clone();
    let href = AttrValue::from(href.to_owned());
    Some(Callback::from(move |e: MouseEvent| {
        navigate.emit((e, href.clone()));
    }))
}

#[cfg(feature = "csr")]
pub fn set_url_hash(hash: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_hash(hash);
    }
}

#[cfg(feature = "csr")]
#[hook]
pub fn use_clipboard(text: AttrValue) -> (bool, Callback<MouseEvent>) {
    let copied = use_state(|| false);
    let onclick = {
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(window) = web_sys::window() {
                let _ = window.navigator().clipboard().write_text(&text);
                copied.set(true);
                let copied2 = copied.clone();
                gloo::timers::callback::Timeout::new(2000, move || {
                    copied2.set(false);
                })
                .forget();
            }
        })
    };
    (*copied, onclick)
}

pub struct RenderedPage {
    pub url: &'static str,
    pub body: String,
    pub styles: String,
    pub description: String,
}

pub struct PageData {
    pub title: &'static str,
    pub sidebar_path: &'static str,
    pub doc_version: &'static str,
    pub sidebar: IArray<SidebarEntry>,
    pub content: Content,
}

pub const VERSIONS: &[(&str, &str)] = &[
    ("Next", "next"),
    ("0.23", ""),
    ("0.22", "0.22"),
    ("0.21", "0.21"),
    ("0.20", "0.20"),
];

#[cfg(feature = "ssr")]
pub mod ssr_reexports {
    pub use stylist::manager::{render_static, StyleManager};
    pub use stylist::yew::ManagerProvider;
    pub use yew::ServerRenderer;
}

#[macro_export]
macro_rules! page_main {
    ($page:ty) => {
        fn main() {
            yew::Renderer::<$page>::new().render();
        }
    };
}

#[macro_export]
macro_rules! doc_page_impl {
    ($title:expr, $path:expr, $content:expr, $sidebar_fn:path, $doc_version:expr, $lang:expr) => {
        pub const HREF: &str = $path;

        #[allow(unused_imports)]
        use yew::prelude::*;
        #[allow(unused_imports)]
        use $crate::content::*;
        #[allow(unused_imports)]
        use $crate::Layout;

        #[component]
        pub fn Page() -> Html {
            let content: $crate::Content = $content;
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            html! {
                <Layout
                    title={$title}
                    sidebar={$sidebar_fn()}
                    active_sidebar_path={$path}
                    active_nav="Docs"
                    doc_version={$doc_version}
                    lang={$lang}
                    markdown={markdown}
                    toc={toc}
                >
                    { content.to_html() }
                </Layout>
            }
        }
    };
}

#[macro_export]
macro_rules! doc_page_with_content_fn_impl {
    ($title:expr, $path:expr, $content:expr, $sidebar_fn:path, $doc_version:expr, $lang:expr) => {
        pub const HREF: &str = $path;

        #[allow(unused_imports)]
        use yew::prelude::*;
        #[allow(unused_imports)]
        use $crate::content::*;
        #[allow(unused_imports)]
        use $crate::Layout;

        pub fn page_content() -> $crate::Content {
            $content
        }

        #[component]
        pub fn Page() -> Html {
            let content: $crate::Content = page_content();
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            html! {
                <Layout
                    title={$title}
                    sidebar={$sidebar_fn()}
                    active_sidebar_path={$path}
                    active_nav="Docs"
                    doc_version={$doc_version}
                    lang={$lang}
                    markdown={markdown}
                    toc={toc}
                >
                    { content.to_html() }
                </Layout>
            }
        }
    };
}

fn translate_label(label: &'static str, lang: &str) -> &'static str {
    match (label, lang) {
        // -- categories --
        ("Getting Started", "ja") => "はじめる",
        ("Getting Started", "zh-Hans") => "从零开始",
        ("Getting Started", "zh-Hant") => "從零開始",
        ("Concepts", "ja") => "コンセプト",
        ("Concepts", "zh-Hans") => "核心概念",
        ("Concepts", "zh-Hant") => "核心觀念",
        ("Basic Web Technologies", "ja") => "Web 技術の基本",
        ("Basic Web Technologies", "zh-Hans") => "Yew 中的基本 Web 技术",
        ("Basic Web Technologies", "zh-Hant") => "基本 Web 技術",
        ("Components", "ja") => "コンポーネント",
        ("Components", "zh-Hans") => "函数式组件",
        ("Components", "zh-Hant") => "函式元件",
        ("Hooks", "ja") => "フック",
        ("Hooks", "zh-Hans") => "钩子",
        ("Hooks", "zh-Hant") => "Hook",
        ("Advanced Topics", "ja") => "応用トピック",
        ("Advanced Topics", "zh-Hans") => "高级主题",
        ("Advanced Topics", "zh-Hant") => "進階主題",
        ("Struct Components", "ja") => "構造体コンポーネント",
        ("Struct Components", "zh-Hans") => "结构化组件",
        ("Struct Components", "zh-Hant") => "結構體元件",
        ("More", "ja") => "その他",
        ("More", "zh-Hans") => "更多",
        ("More", "zh-Hant") => "更多",
        ("Migration Guides", "ja") => "移行ガイド",
        ("Migration Guides", "zh-Hans") => "迁移指南",
        ("Migration Guides", "zh-Hant") => "移轉指南",
        // -- items --
        ("Build a Sample App", "ja") => "サンプルアプリを作る",
        ("Build a Sample App", "zh-Hans") => "构建示例应用",
        ("Build a Sample App", "zh-Hant") => "建構範例應用",
        ("Examples", "ja") => "サンプル集",
        ("Examples", "zh-Hans") => "示例",
        ("Examples", "zh-Hant") => "範例",
        ("Editor Setup", "ja") => "エディタ設定",
        ("Editor Setup", "zh-Hans") => "编辑器设置",
        ("Editor Setup", "zh-Hant") => "編輯器設定",
        ("HTML", _) => "HTML",
        ("CSS", _) => "CSS",
        ("JavaScript", "ja") => "JavaScript",
        ("Properties", "ja") => "プロパティ",
        ("Properties", "zh-Hans") => "属性",
        ("Properties", "zh-Hant") => "屬性",
        ("Callbacks", "ja") => "コールバック",
        ("Callbacks", "zh-Hans") => "回调",
        ("Callbacks", "zh-Hant") => "回呼",
        ("Children", "ja") => "子要素",
        ("Children", "zh-Hans") => "子组件",
        ("Children", "zh-Hant") => "子元件",
        ("Pure Components", "ja") => "純粋コンポーネント",
        ("Pure Components", "zh-Hans") => "纯组件",
        ("Pure Components", "zh-Hant") => "純元件",
        ("Custom Hooks", "ja") => "カスタムフック",
        ("Custom Hooks", "zh-Hans") => "自定义钩子",
        ("Custom Hooks", "zh-Hant") => "自訂 Hook",
        ("Node Refs", "ja") => "ノード参照",
        ("Node Refs", "zh-Hans") => "节点引用",
        ("Node Refs", "zh-Hant") => "節點參考",
        ("State", "ja") => "ステート",
        ("State", "zh-Hans") => "状态",
        ("State", "zh-Hant") => "狀態",
        ("Communication", "ja") => "コンポーネント間通信",
        ("Communication", "zh-Hans") => "组件通信",
        ("Communication", "zh-Hant") => "元件通訊",
        ("Generics", "ja") => "ジェネリクス",
        ("Generics", "zh-Hans") => "泛型",
        ("Generics", "zh-Hant") => "泛型",
        ("Elements", "ja") => "要素",
        ("Elements", "zh-Hans") => "元素",
        ("Elements", "zh-Hant") => "元素",
        ("Events", "ja") => "イベント",
        ("Events", "zh-Hans") => "事件",
        ("Events", "zh-Hant") => "事件",
        ("Classes", "ja") => "クラス",
        ("Classes", "zh-Hans") => "类",
        ("Classes", "zh-Hant") => "類別",
        ("Fragments", "ja") => "フラグメント",
        ("Fragments", "zh-Hans") => "片段",
        ("Fragments", "zh-Hant") => "片段",
        ("Lists", "ja") => "リスト",
        ("Lists", "zh-Hans") => "列表",
        ("Lists", "zh-Hant") => "列表",
        ("Literals and Expressions", "ja") => "リテラルと式",
        ("Literals and Expressions", "zh-Hans") => "字面量与表达式",
        ("Literals and Expressions", "zh-Hant") => "字面量與表達式",
        ("Conditional Rendering", "ja") => "条件付きレンダリング",
        ("Conditional Rendering", "zh-Hans") => "条件渲染",
        ("Conditional Rendering", "zh-Hant") => "條件渲染",
        ("Agents", "ja") => "エージェント",
        ("Agents", "zh-Hans") => "代理",
        ("Agents", "zh-Hant") => "代理",
        ("Contexts", "ja") => "コンテキスト",
        ("Contexts", "zh-Hans") => "上下文",
        ("Contexts", "zh-Hant") => "上下文",
        ("Router", "ja") => "ルーター",
        ("Router", "zh-Hans") => "路由",
        ("Router", "zh-Hant") => "路由",
        ("Suspense", "ja") => "サスペンス",
        ("Suspense", "zh-Hans") => "悬念",
        ("Suspense", "zh-Hant") => "Suspense",
        ("How It Works", "ja") => "仕組み",
        ("How It Works", "zh-Hans") => "工作原理",
        ("How It Works", "zh-Hant") => "運作原理",
        ("Higher Order Components", "ja") => "高階コンポーネント",
        ("Higher Order Components", "zh-Hans") => "高阶组件",
        ("Higher Order Components", "zh-Hant") => "高階元件",
        ("Lifecycle", "ja") => "ライフサイクル",
        ("Lifecycle", "zh-Hans") => "生命周期",
        ("Lifecycle", "zh-Hant") => "生命週期",
        ("Scope", "ja") => "スコープ",
        ("Scope", "zh-Hans") => "作用域",
        ("Scope", "zh-Hant") => "作用域",
        ("Refs", "ja") => "参照",
        ("Refs", "zh-Hans") => "引用",
        ("Refs", "zh-Hant") => "參考",
        ("Optimizations", "ja") => "最適化",
        ("Optimizations", "zh-Hans") => "优化",
        ("Optimizations", "zh-Hant") => "最佳化",
        ("Portals", "ja") => "ポータル",
        ("Portals", "zh-Hans") => "传送门",
        ("Portals", "zh-Hant") => "Portal",
        ("Server-Side Rendering", "ja") => "サーバーサイドレンダリング",
        ("Server-Side Rendering", "zh-Hans") => "服务端渲染",
        ("Server-Side Rendering", "zh-Hant") => "伺服器端渲染",
        ("Immutable", "ja") => "イミュータブル",
        ("Immutable", "zh-Hans") => "不可变数据",
        ("Immutable", "zh-Hant") => "不可變資料",
        ("Debugging", "ja") => "デバッグ",
        ("Debugging", "zh-Hans") => "调试",
        ("Debugging", "zh-Hant") => "除錯",
        ("Deployment", "ja") => "デプロイ",
        ("Deployment", "zh-Hans") => "部署",
        ("Deployment", "zh-Hant") => "部署",
        ("Testing", "ja") => "テスト",
        ("Testing", "zh-Hans") => "测试",
        ("Testing", "zh-Hant") => "測試",
        ("Roadmap", "ja") => "ロードマップ",
        ("Roadmap", "zh-Hans") => "路线图",
        ("Roadmap", "zh-Hant") => "路線圖",
        _ => label,
    }
}

pub fn translate_sidebar(sidebar: IArray<SidebarEntry>, lang: &str) -> IArray<SidebarEntry> {
    if lang.is_empty() || lang == "en" {
        return sidebar;
    }
    fn translate_entry(entry: SidebarEntry, lang: &str) -> SidebarEntry {
        match entry {
            SidebarEntry::Item(item) => SidebarEntry::Item(SidebarItem {
                label: translate_label(item.label, lang),
                href: item.href,
            }),
            SidebarEntry::Category(cat) => SidebarEntry::Category(SidebarCategory {
                label: translate_label(cat.label, lang),
                link: cat.link,
                items: cat
                    .items
                    .into_iter()
                    .map(|e| translate_entry(e, lang))
                    .collect(),
            }),
        }
    }
    sidebar
        .iter()
        .map(|e| translate_entry(e.clone(), lang))
        .collect()
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! render_spa_page {
    ($url:expr, $title:expr, $sidebar:expr, $sidebar_path:expr, $doc_version:expr, $lang:expr, $content_expr:expr) => {{
        use ::yew::prelude::*;
        use $crate::ssr_reexports::{render_static, ManagerProvider, ServerRenderer, StyleManager};

        #[derive(Properties, PartialEq)]
        struct __SpaProps {
            manager: StyleManager,
            title: &'static str,
            sidebar: Vec<$crate::SidebarEntry>,
            sidebar_path: &'static str,
            doc_version: &'static str,
            lang: &'static str,
        }

        fn __make_content() -> $crate::Content {
            $content_expr
        }

        #[component]
        fn __SpaWrapper(props: &__SpaProps) -> Html {
            let content = __make_content();
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            let sidebar: ::implicit_clone::unsync::IArray<$crate::SidebarEntry> =
                props.sidebar.clone().into();
            html! {
                <ManagerProvider manager={props.manager.clone()}>
                    <$crate::Layout
                        title={props.title}
                        sidebar={sidebar}
                        active_sidebar_path={props.sidebar_path}
                        active_nav="Docs"
                        doc_version={props.doc_version}
                        lang={props.lang}
                        markdown={markdown}
                        toc={toc}
                    >
                        { content.to_html() }
                    </$crate::Layout>
                </ManagerProvider>
            }
        }

        let description = __make_content().description_text();

        let (writer, reader) = render_static();
        let __sidebar_vec: Vec<$crate::SidebarEntry> = ::std::ops::Deref::deref(&$sidebar).to_vec();
        let body = ServerRenderer::<__SpaWrapper>::with_props(move || {
            let mgr = StyleManager::builder().writer(writer).build().unwrap();
            __SpaProps {
                manager: mgr,
                title: $title,
                sidebar: __sidebar_vec,
                sidebar_path: $sidebar_path,
                doc_version: $doc_version,
                lang: $lang,
            }
        })
        .hydratable(false)
        .render()
        .await;

        let style_data = reader.read_style_data();
        let mut styles = String::new();
        style_data.write_static_markup(&mut styles).unwrap();

        $crate::RenderedPage {
            url: $url,
            body,
            styles,
            description,
        }
    }};
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! render_page {
    ($url:expr, $page:ty) => {
        $crate::render_page!($url, $page, "")
    };
    ($url:expr, $page:ty, $desc:expr) => {{
        use ::yew::prelude::*;
        use $crate::ssr_reexports::{render_static, ManagerProvider, ServerRenderer, StyleManager};

        #[derive(Properties, PartialEq)]
        struct __SsrWrapperProps {
            manager: StyleManager,
        }

        #[component]
        fn __SsrWrapper(props: &__SsrWrapperProps) -> Html {
            html! {
                <ManagerProvider manager={props.manager.clone()}>
                    <$page />
                </ManagerProvider>
            }
        }

        let (writer, reader) = render_static();
        let body = ServerRenderer::<__SsrWrapper>::with_props(move || {
            let mgr = StyleManager::builder().writer(writer).build().unwrap();
            __SsrWrapperProps { manager: mgr }
        })
        .hydratable(false)
        .render()
        .await;

        let style_data = reader.read_style_data();
        let mut styles = String::new();
        style_data.write_static_markup(&mut styles).unwrap();

        $crate::RenderedPage {
            url: $url,
            body,
            styles,
            description: ($desc).to_string(),
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __doc_page_list {
    ($callback:path, $($ctx:tt)*) => {
        $callback! { $($ctx)*
            ;
            ("getting-started", "Getting Started", "/docs/getting-started", getting_started :: introduction),
            ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", getting_started :: build_a_sample_app),
            ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", getting_started :: editor_setup),
            ("getting-started/examples", "Examples", "/docs/getting-started/examples", getting_started :: examples),
            ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", concepts :: basic_web_technologies :: html),
            ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", concepts :: basic_web_technologies :: css),
            ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", concepts :: basic_web_technologies :: js),
            ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", concepts :: basic_web_technologies :: wasm_bindgen),
            ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", concepts :: basic_web_technologies :: web_sys),
            ("concepts/function-components", "Function Components", "/docs/concepts/function-components", concepts :: function_components :: introduction),
            ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", concepts :: function_components :: properties),
            ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", concepts :: function_components :: callbacks),
            ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", concepts :: function_components :: children),
            ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", concepts :: function_components :: pure_components),
            ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", concepts :: function_components :: hooks :: introduction),
            ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", concepts :: function_components :: hooks :: custom_hooks),
            ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", concepts :: function_components :: node_refs),
            ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", concepts :: function_components :: state),
            ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", concepts :: function_components :: communication),
            ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", concepts :: function_components :: generics),
            ("concepts/html", "HTML", "/docs/concepts/html", concepts :: html :: introduction),
            ("concepts/html/components", "Components", "/docs/concepts/html/components", concepts :: html :: components),
            ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", concepts :: html :: elements),
            ("concepts/html/events", "Events", "/docs/concepts/html/events", concepts :: html :: events),
            ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", concepts :: html :: classes),
            ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", concepts :: html :: fragments),
            ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", concepts :: html :: lists),
            ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", concepts :: html :: literals_and_expressions),
            ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", concepts :: html :: conditional_rendering),
            ("concepts/agents", "Agents", "/docs/concepts/agents", concepts :: agents),
            ("concepts/contexts", "Contexts", "/docs/concepts/contexts", concepts :: contexts),
            ("concepts/router", "Router", "/docs/concepts/router", concepts :: router),
            ("concepts/suspense", "Suspense", "/docs/concepts/suspense", concepts :: suspense),
            ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", advanced_topics :: how_it_works),
            ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", advanced_topics :: struct_components :: introduction),
            ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", advanced_topics :: struct_components :: hoc),
            ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", advanced_topics :: struct_components :: lifecycle),
            ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", advanced_topics :: struct_components :: scope),
            ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", advanced_topics :: struct_components :: callbacks),
            ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", advanced_topics :: struct_components :: properties),
            ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", advanced_topics :: struct_components :: refs),
            ("advanced-topics/children", "Children", "/docs/advanced-topics/children", advanced_topics :: children),
            ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", advanced_topics :: optimizations),
            ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", advanced_topics :: portals),
            ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", advanced_topics :: server_side_rendering),
            ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", advanced_topics :: immutable),
            ("more/debugging", "Debugging", "/docs/more/debugging", more :: debugging),
            ("more/deployment", "Deployment", "/docs/more/deployment", more :: deployment),
            ("more/css", "CSS", "/docs/more/css", more :: css),
            ("more/testing", "Testing apps", "/docs/more/testing", more :: testing),
            ("more/roadmap", "Roadmap", "/docs/more/roadmap", more :: roadmap),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __migration_page_list {
    ($callback:path, $($ctx:tt)*) => {
        $callback! { $($ctx)*
            ;
            ("yew/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew/from-0-19-0-to-0-20-0", migration_guides :: yew :: from_0_19_0_to_0_20_0),
            ("yew/from-0-20-0-to-0-21-0", "From 0.20.0 to 0.21.0", "/docs/migration-guides/yew/from-0-20-0-to-0-21-0", migration_guides :: yew :: from_0_20_0_to_0_21_0),
            ("yew/from-0-21-0-to-0-22-0", "From 0.21.0 to 0.22.0", "/docs/migration-guides/yew/from-0-21-0-to-0-22-0", migration_guides :: yew :: from_0_21_0_to_0_22_0),
            ("yew/from-0-22-0-to-0-23-0", "From 0.22.0 to 0.23.0", "/docs/migration-guides/yew/from-0-22-0-to-0-23-0", migration_guides :: yew :: from_0_22_0_to_0_23_0),
            ("yew-agent/from-0-0-0-to-0-1-0", "From 0.0.0 to 0.1.0", "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0", migration_guides :: yew_agent :: from_0_0_0_to_0_1_0),
            ("yew-agent/from-0-1-0-to-0-2-0", "From 0.1.0 to 0.2.0", "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0", migration_guides :: yew_agent :: from_0_1_0_to_0_2_0),
            ("yew-agent/from-0-3-0-to-0-4-0", "From 0.3.0 to 0.4.0", "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0", migration_guides :: yew_agent :: from_0_3_0_to_0_4_0),
            ("yew-agent/from-0-4-0-to-0-5-0", "From 0.4.0 to 0.5.0", "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0", migration_guides :: yew_agent :: from_0_4_0_to_0_5_0),
            ("yew-router/from-0-15-0-to-0-16-0", "From 0.15.0 to 0.16.0", "/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0", migration_guides :: yew_router :: from_0_15_0_to_0_16_0),
            ("yew-router/from-0-16-0-to-0-17-0", "From 0.16.0 to 0.17.0", "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0", migration_guides :: yew_router :: from_0_16_0_to_0_17_0),
            ("yew-router/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0", migration_guides :: yew_router :: from_0_19_0_to_0_20_0),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __doc_render_pages_impl {
    ($pages:ident, $url_prefix:expr ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        $($pages.push($crate::render_page!(concat!($url_prefix, "/", $slug), pages::$($mod_seg)::+::Page));)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __doc_render_migration_pages_impl {
    ($pages:ident, $mg_prefix:expr ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        $($pages.push($crate::render_page!(concat!($mg_prefix, "/", $slug), pages::$($mod_seg)::+::Page));)*
    };
}

#[macro_export]
macro_rules! doc_render_pages {
    ($url_prefix:expr $(, with_migration_guides: $mg_prefix:expr)?) => {
        #[cfg(feature = "ssr")]
        pub async fn render_pages() -> Vec<$crate::RenderedPage> {
            let mut pages = Vec::new();
            $crate::__doc_page_list!($crate::__doc_render_pages_impl, pages, $url_prefix);
            $($crate::__migration_page_list!($crate::__doc_render_migration_pages_impl, pages, $mg_prefix);)?
            pages.push($crate::render_page!(concat!($url_prefix, "/tutorial"), pages::tutorial::Page));
            pages
        }
    };
}

#[macro_export]
macro_rules! page_map {
    ($fn_name:ident, $version:expr, $sidebar_fn:path, [
        $( ($path:expr, $title:expr, $spath:expr, $content:expr) ),* $(,)?
    ]) => {
        pub fn $fn_name(path: &str) -> Option<$crate::PageData> {
            Some(match path {
                $( $path => $crate::PageData {
                    title: $title,
                    sidebar_path: $spath,
                    doc_version: $version,
                    sidebar: $sidebar_fn(),
                    content: $content,
                }, )*
                _ => return None,
            })
        }
    };
}

#[macro_export]
macro_rules! spa_sidebar_fns {
    ($lang:expr, $next:ident, $v023:ident, $v022:ident, $v021:ident, $v020:ident) => {
        fn sidebar() -> ::implicit_clone::unsync::IArray<$crate::SidebarEntry> {
            $crate::translate_sidebar($next::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_23() -> ::implicit_clone::unsync::IArray<$crate::SidebarEntry> {
            $crate::translate_sidebar($v023::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_22() -> ::implicit_clone::unsync::IArray<$crate::SidebarEntry> {
            $crate::translate_sidebar($v022::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_21() -> ::implicit_clone::unsync::IArray<$crate::SidebarEntry> {
            $crate::translate_sidebar($v021::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_20() -> ::implicit_clone::unsync::IArray<$crate::SidebarEntry> {
            $crate::translate_sidebar($v020::sidebar_data::docs_sidebar(), $lang)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __spa_version_resolver_impl {
    ($fn_name:ident, $version:expr, $sidebar_fn:path, $doc:ident ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        pub fn $fn_name(path: &str) -> Option<$crate::PageData> {
            Some(match path {
                $($slug => $crate::PageData {
                    title: $title,
                    sidebar_path: $spath,
                    doc_version: $version,
                    sidebar: $sidebar_fn(),
                    content: $doc::pages::$($mod_seg)::+::page_content(),
                },)*
                _ => return None,
            })
        }
    };
}

#[macro_export]
macro_rules! spa_version_resolver {
    ($fn_name:ident, $version:expr, $sidebar_fn:path, $doc:ident) => {
        $crate::__doc_page_list!{$crate::__spa_version_resolver_impl, $fn_name, $version, $sidebar_fn, $doc}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __spa_migration_resolver_impl {
    ($fn_name:ident, $sidebar_fn:path, $doc:ident ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        pub fn $fn_name(path: &str) -> Option<$crate::PageData> {
            Some(match path {
                $($slug => $crate::PageData {
                    title: $title,
                    sidebar_path: $spath,
                    doc_version: "",
                    sidebar: $sidebar_fn(),
                    content: $doc::pages::$($mod_seg)::+::page_content(),
                },)*
                _ => return None,
            })
        }
    };
}

#[macro_export]
macro_rules! spa_migration_resolver {
    ($fn_name:ident, $sidebar_fn:path, $doc:ident) => {
        $crate::__migration_page_list!{$crate::__spa_migration_resolver_impl, $fn_name, $sidebar_fn, $doc}
    };
}

#[macro_export]
macro_rules! spa_csr_boilerplate {
    ($lang:expr, $locale:expr, $url_prefix:expr, $docs_next:ident, $docs_0_21:ident, $docs_0_20:ident) => {
        #[cfg(feature = "csr")]
        pub fn resolve_page(route: &Route) -> Option<$crate::PageData> {
            match route {
                Route::DocsNext { path } => resolve_next(path),
                Route::DocsStable { path } => resolve_stable(path),
                Route::DocsV022 { path } => resolve_v022(path),
                Route::DocsV021 { path } => resolve_v021(path),
                Route::DocsV020 { path } => resolve_v020(path),
                Route::MigrationGuides { path } => resolve_migration(path),
                Route::Home
                | Route::HomeNext
                | Route::HomeV022
                | Route::HomeV021
                | Route::HomeV020
                | Route::Tutorial
                | Route::TutorialNext
                | Route::TutorialV022
                | Route::TutorialV021
                | Route::TutorialV020
                | Route::NotFound => None,
            }
        }

        #[cfg(feature = "csr")]
        fn resolve_tutorial(route: &Route) -> Option<($crate::Content, &'static str)> {
            match route {
                Route::Tutorial => Some((
                    $docs_next::pages::tutorial::page_content_versioned(Some("0.23")),
                    "0.23",
                )),
                Route::TutorialNext => Some(($docs_next::pages::tutorial::page_content(), "Next")),
                Route::TutorialV022 => Some((
                    $docs_next::pages::tutorial::page_content_versioned(Some("0.22")),
                    "0.22",
                )),
                Route::TutorialV021 => Some(($docs_0_21::pages::tutorial::page_content(), "0.21")),
                Route::TutorialV020 => Some(($docs_0_20::pages::tutorial::page_content(), "0.20")),
                _ => None,
            }
        }

        #[cfg(feature = "csr")]
        fn resolve_home(route: &Route) -> Option<(&'static str, &'static str, &'static str)> {
            match route {
                Route::Home => Some(($locale, "", "0.23")),
                Route::HomeNext => Some(($locale, "next", "Next")),
                Route::HomeV022 => Some(($locale, "0.22", "0.22")),
                Route::HomeV021 => Some(($locale, "0.21", "0.21")),
                Route::HomeV020 => Some(($locale, "0.20", "0.20")),
                _ => None,
            }
        }

        #[cfg(feature = "csr")]
        #[component]
        pub fn App() -> Html {
            html! {
                <BrowserRouter>
                    <AppInner />
                </BrowserRouter>
            }
        }

        #[cfg(feature = "csr")]
        #[component]
        fn AppInner() -> Html {
            let route = use_route::<Route>().unwrap_or(Route::NotFound);
            let navigator = use_navigator().unwrap();

            let nav_callback = {
                let navigator = navigator.clone();
                Callback::from(move |(e, href): (MouseEvent, AttrValue)| {
                    let (path, fragment) = match href.find('#') {
                        Some(i) => (&href[..i], &href[i + 1..]),
                        None => (href.as_str(), ""),
                    };
                    if let Some(r) = Route::recognize(path) {
                        if !matches!(r, Route::NotFound) {
                            e.prevent_default();
                            navigator.push(&r);
                            if !fragment.is_empty() {
                                $crate::set_url_hash(&format!("#{fragment}"));
                            }
                        }
                    }
                })
            };
            let nav_ctx = NavigationContext {
                navigate: nav_callback,
            };

            if let Some(page) = resolve_page(&route) {
                let content = page.content;
                let toc = content.toc_entries();
                let markdown = content.to_markdown();
                html! {
                    <ContextProvider<NavigationContext> context={nav_ctx}>
                        <Layout
                            title={page.title}
                            sidebar={page.sidebar}
                            active_sidebar_path={page.sidebar_path}
                            active_nav="Docs"
                            doc_version={page.doc_version}
                            lang={$lang}
                            markdown={markdown}
                            toc={toc}
                        >
                            { content.to_html() }
                        </Layout>
                    </ContextProvider<NavigationContext>>
                }
            } else if let Some((content, doc_version)) = resolve_tutorial(&route) {
                let toc = content.toc_entries();
                let markdown = content.to_markdown();
                html! {
                    <ContextProvider<NavigationContext> context={nav_ctx}>
                        <Layout
                            title="Tutorial"
                            active_nav="Tutorial"
                            active_sidebar_path="/tutorial"
                            doc_version={doc_version}
                            lang={$lang}
                            markdown={markdown}
                            toc={toc}
                        >
                            { content.to_html() }
                        </Layout>
                    </ContextProvider<NavigationContext>>
                }
            } else if let Some((locale, version_slug, doc_version)) = resolve_home(&route) {
                let current_path = if version_slug.is_empty() {
                    format!("{}/", $url_prefix)
                } else {
                    format!("{}/{version_slug}/", $url_prefix)
                };
                html! {
                    <ContextProvider<NavigationContext> context={nav_ctx}>
                        <Layout
                            title=""
                            full_width=true
                            lang={$lang}
                            doc_version={doc_version}
                            active_sidebar_path={current_path}
                        >
                            { yew_site_home::home_html(locale, version_slug) }
                        </Layout>
                    </ContextProvider<NavigationContext>>
                }
            } else {
                html! {
                    <ContextProvider<NavigationContext> context={nav_ctx}>
                        <Layout title="Page Not Found" active_nav="Docs" lang={$lang}>
                            <h1>{"404 - Page Not Found"}</h1>
                        </Layout>
                    </ContextProvider<NavigationContext>>
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ssr_doc_pages_impl {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        $({
            let page = $resolve_fn($slug).unwrap();
            $pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/", $slug),
                page.title, page.sidebar, page.sidebar_path, page.doc_version, $lang,
                $resolve_fn($slug).unwrap().content
            ));
        })*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ssr_migration_pages_impl {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr ; $( ($slug:expr, $title:expr, $spath:expr, $($mod_seg:ident)::+) ),* $(,)?) => {
        $({
            let page = $resolve_fn($slug).unwrap();
            $pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/", $slug),
                page.title, page.sidebar, page.sidebar_path, page.doc_version, $lang,
                $resolve_fn($slug).unwrap().content
            ));
        })*
    };
}

#[macro_export]
macro_rules! __ssr_doc_pages {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr) => {
        $crate::__doc_page_list! {$crate::__ssr_doc_pages_impl, $pages, $url_prefix, $resolve_fn, $lang}
    };
}

#[macro_export]
macro_rules! __ssr_migration_pages {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr) => {
        $crate::__migration_page_list!{$crate::__ssr_migration_pages_impl, $pages, $url_prefix, $resolve_fn, $lang}
    };
}

#[macro_export]
macro_rules! spa_ssr_render_pages {
    (
        $lang:expr, $url_prefix:expr,
        $docs_next:ident, $docs_0_21:ident, $docs_0_20:ident,
        tutorial_sidebar_next: $tutorial_sidebar_next:expr,
        tutorial_sidebar_stable: $tutorial_sidebar_stable:expr,
        tutorial_sidebar_0_22: $tutorial_sidebar_0_22:expr,
        tutorial_sidebar_0_21: $tutorial_sidebar_0_21:expr,
        tutorial_sidebar_0_20: $tutorial_sidebar_0_20:expr,
        home_pages: [$home:ty, $home_next:ty, $home_v022:ty, $home_v021:ty, $home_v020:ty]
    ) => {
        #[cfg(feature = "ssr")]
        pub async fn render_pages() -> Vec<$crate::RenderedPage> {
            let mut pages = Vec::new();

            $crate::__ssr_doc_pages!(
                pages,
                concat!($url_prefix, "/docs/next"),
                resolve_next,
                $lang
            );
            pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/next/tutorial"),
                "Tutorial",
                $tutorial_sidebar_next,
                "/tutorial",
                "Next",
                $lang,
                $docs_next::pages::tutorial::page_content()
            ));

            $crate::__ssr_doc_pages!(pages, concat!($url_prefix, "/docs"), resolve_stable, $lang);
            pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/tutorial"),
                "Tutorial",
                $tutorial_sidebar_stable,
                "/tutorial",
                "0.23",
                $lang,
                $docs_next::pages::tutorial::page_content_versioned(Some("0.23"))
            ));

            $crate::__ssr_doc_pages!(
                pages,
                concat!($url_prefix, "/docs/0.22"),
                resolve_v022,
                $lang
            );
            pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/0.22/tutorial"),
                "Tutorial",
                $tutorial_sidebar_0_22,
                "/tutorial",
                "0.22",
                $lang,
                $docs_next::pages::tutorial::page_content_versioned(Some("0.22"))
            ));

            $crate::__ssr_doc_pages!(
                pages,
                concat!($url_prefix, "/docs/0.21"),
                resolve_v021,
                $lang
            );
            pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/0.21/tutorial"),
                "Tutorial",
                $tutorial_sidebar_0_21,
                "/tutorial",
                "0.21",
                $lang,
                $docs_0_21::pages::tutorial::page_content()
            ));

            $crate::__ssr_doc_pages!(
                pages,
                concat!($url_prefix, "/docs/0.20"),
                resolve_v020,
                $lang
            );
            pages.push($crate::render_spa_page!(
                concat!($url_prefix, "/0.20/tutorial"),
                "Tutorial",
                $tutorial_sidebar_0_20,
                "/tutorial",
                "0.20",
                $lang,
                $docs_0_20::pages::tutorial::page_content()
            ));

            $crate::__ssr_migration_pages!(
                pages,
                concat!($url_prefix, "/docs/migration-guides"),
                resolve_migration,
                $lang
            );

            {
                let __home_desc = yew_site_home::home_description($lang);
                pages.push($crate::render_page!(
                    concat!($url_prefix, "/"),
                    $home,
                    __home_desc
                ));
                pages.push($crate::render_page!(
                    concat!($url_prefix, "/next/"),
                    $home_next,
                    __home_desc
                ));
                pages.push($crate::render_page!(
                    concat!($url_prefix, "/0.22/"),
                    $home_v022,
                    __home_desc
                ));
                pages.push($crate::render_page!(
                    concat!($url_prefix, "/0.21/"),
                    $home_v021,
                    __home_desc
                ));
                pages.push($crate::render_page!(
                    concat!($url_prefix, "/0.20/"),
                    $home_v020,
                    __home_desc
                ));
            }

            pages
        }
    };
}
