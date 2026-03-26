pub mod components;
pub mod content;
pub mod styles;

pub use components::footer::Footer;
pub use components::layout::Layout;
pub use components::navbar::Navbar;
pub use components::sidebar::{
    flatten_sidebar, migration_guides_sidebar, Sidebar, SidebarCategory, SidebarEntry, SidebarItem,
};
pub use content::Content;
use yew::prelude::*;

pub const LATEST_STABLE: &str = "0.23";

#[derive(Clone, PartialEq)]
pub struct NavigationContext {
    pub navigate: Callback<(MouseEvent, AttrValue)>,
}

pub struct PageData {
    pub title: &'static str,
    pub sidebar_path: &'static str,
    pub doc_version: &'static str,
    pub sidebar: Vec<SidebarEntry>,
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

pub fn translate_sidebar(sidebar: Vec<SidebarEntry>, lang: &str) -> Vec<SidebarEntry> {
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
        .into_iter()
        .map(|e| translate_entry(e, lang))
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
            html! {
                <ManagerProvider manager={props.manager.clone()}>
                    <$crate::Layout
                        title={props.title}
                        sidebar={props.sidebar.clone()}
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

        let (writer, reader) = render_static();
        let body = ServerRenderer::<__SpaWrapper>::with_props(move || {
            let mgr = StyleManager::builder().writer(writer).build().unwrap();
            __SpaProps {
                manager: mgr,
                title: $title,
                sidebar: $sidebar,
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

        ($url, body, styles)
    }};
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! render_page {
    ($url:expr, $page:ty) => {{
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

        ($url, body, styles)
    }};
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
        fn sidebar() -> Vec<$crate::SidebarEntry> {
            $crate::translate_sidebar($next::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_23() -> Vec<$crate::SidebarEntry> {
            $crate::translate_sidebar($v023::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_22() -> Vec<$crate::SidebarEntry> {
            $crate::translate_sidebar($v022::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_21() -> Vec<$crate::SidebarEntry> {
            $crate::translate_sidebar($v021::sidebar_data::docs_sidebar(), $lang)
        }
        fn sidebar_0_20() -> Vec<$crate::SidebarEntry> {
            $crate::translate_sidebar($v020::sidebar_data::docs_sidebar(), $lang)
        }
    };
}

#[macro_export]
macro_rules! spa_version_resolver {
    ($fn_name:ident, $version:expr, $sidebar_fn:path, $doc:ident, $bsa_content:expr) => {
        pub fn $fn_name(path: &str) -> Option<$crate::PageData> {
            Some(match path {
                "getting-started" => $crate::PageData { title: "Getting Started", sidebar_path: "/docs/getting-started", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::getting_started::introduction::page_content() },
                "getting-started/build-a-sample-app" => $crate::PageData { title: "Build a sample app", sidebar_path: "/docs/getting-started/build-a-sample-app", doc_version: $version, sidebar: $sidebar_fn(), content: $bsa_content },
                "getting-started/editor-setup" => $crate::PageData { title: "Editor setup", sidebar_path: "/docs/getting-started/editor-setup", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::getting_started::editor_setup::page_content() },
                "getting-started/examples" => $crate::PageData { title: "Examples", sidebar_path: "/docs/getting-started/examples", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::getting_started::examples::page_content() },
                "concepts/basic-web-technologies/html" => $crate::PageData { title: "HTML with html!", sidebar_path: "/docs/concepts/basic-web-technologies/html", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::basic_web_technologies::html::page_content() },
                "concepts/basic-web-technologies/css" => $crate::PageData { title: "CSS with classes!", sidebar_path: "/docs/concepts/basic-web-technologies/css", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::basic_web_technologies::css::page_content() },
                "concepts/basic-web-technologies/js" => $crate::PageData { title: "JS with RS", sidebar_path: "/docs/concepts/basic-web-technologies/js", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::basic_web_technologies::js::page_content() },
                "concepts/basic-web-technologies/wasm-bindgen" => $crate::PageData { title: "wasm-bindgen", sidebar_path: "/docs/concepts/basic-web-technologies/wasm-bindgen", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::basic_web_technologies::wasm_bindgen::page_content() },
                "concepts/basic-web-technologies/web-sys" => $crate::PageData { title: "web-sys", sidebar_path: "/docs/concepts/basic-web-technologies/web-sys", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::basic_web_technologies::web_sys::page_content() },
                "concepts/function-components" => $crate::PageData { title: "Function Components", sidebar_path: "/docs/concepts/function-components", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::introduction::page_content() },
                "concepts/function-components/properties" => $crate::PageData { title: "Properties", sidebar_path: "/docs/concepts/function-components/properties", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::properties::page_content() },
                "concepts/function-components/callbacks" => $crate::PageData { title: "Callbacks", sidebar_path: "/docs/concepts/function-components/callbacks", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::callbacks::page_content() },
                "concepts/function-components/children" => $crate::PageData { title: "Children", sidebar_path: "/docs/concepts/function-components/children", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::children::page_content() },
                "concepts/function-components/pure-components" => $crate::PageData { title: "Pure Components", sidebar_path: "/docs/concepts/function-components/pure-components", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::pure_components::page_content() },
                "concepts/function-components/hooks" => $crate::PageData { title: "Hooks", sidebar_path: "/docs/concepts/function-components/hooks", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::hooks::introduction::page_content() },
                "concepts/function-components/hooks/custom-hooks" => $crate::PageData { title: "Custom Hooks", sidebar_path: "/docs/concepts/function-components/hooks/custom-hooks", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::hooks::custom_hooks::page_content() },
                "concepts/function-components/node-refs" => $crate::PageData { title: "Node Refs", sidebar_path: "/docs/concepts/function-components/node-refs", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::node_refs::page_content() },
                "concepts/function-components/state" => $crate::PageData { title: "State", sidebar_path: "/docs/concepts/function-components/state", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::state::page_content() },
                "concepts/function-components/communication" => $crate::PageData { title: "Communication between components", sidebar_path: "/docs/concepts/function-components/communication", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::communication::page_content() },
                "concepts/function-components/generics" => $crate::PageData { title: "Generic Components", sidebar_path: "/docs/concepts/function-components/generics", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::function_components::generics::page_content() },
                "concepts/html" => $crate::PageData { title: "HTML", sidebar_path: "/docs/concepts/html", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::introduction::page_content() },
                "concepts/html/components" => $crate::PageData { title: "Components", sidebar_path: "/docs/concepts/html/components", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::components::page_content() },
                "concepts/html/elements" => $crate::PageData { title: "Elements", sidebar_path: "/docs/concepts/html/elements", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::elements::page_content() },
                "concepts/html/events" => $crate::PageData { title: "Events", sidebar_path: "/docs/concepts/html/events", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::events::page_content() },
                "concepts/html/classes" => $crate::PageData { title: "Classes", sidebar_path: "/docs/concepts/html/classes", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::classes::page_content() },
                "concepts/html/fragments" => $crate::PageData { title: "Fragments", sidebar_path: "/docs/concepts/html/fragments", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::fragments::page_content() },
                "concepts/html/lists" => $crate::PageData { title: "Lists", sidebar_path: "/docs/concepts/html/lists", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::lists::page_content() },
                "concepts/html/literals-and-expressions" => $crate::PageData { title: "Literals and Expressions", sidebar_path: "/docs/concepts/html/literals-and-expressions", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::literals_and_expressions::page_content() },
                "concepts/html/conditional-rendering" => $crate::PageData { title: "Conditional Rendering", sidebar_path: "/docs/concepts/html/conditional-rendering", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::html::conditional_rendering::page_content() },
                "concepts/agents" => $crate::PageData { title: "Agents", sidebar_path: "/docs/concepts/agents", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::agents::page_content() },
                "concepts/contexts" => $crate::PageData { title: "Contexts", sidebar_path: "/docs/concepts/contexts", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::contexts::page_content() },
                "concepts/router" => $crate::PageData { title: "Router", sidebar_path: "/docs/concepts/router", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::router::page_content() },
                "concepts/suspense" => $crate::PageData { title: "Suspense", sidebar_path: "/docs/concepts/suspense", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::concepts::suspense::page_content() },
                "advanced-topics/how-it-works" => $crate::PageData { title: "How it works", sidebar_path: "/docs/advanced-topics/how-it-works", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::how_it_works::page_content() },
                "advanced-topics/struct-components" => $crate::PageData { title: "Introduction", sidebar_path: "/docs/advanced-topics/struct-components", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::introduction::page_content() },
                "advanced-topics/struct-components/hoc" => $crate::PageData { title: "Higher Order Components", sidebar_path: "/docs/advanced-topics/struct-components/hoc", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::hoc::page_content() },
                "advanced-topics/struct-components/lifecycle" => $crate::PageData { title: "Lifecycle", sidebar_path: "/docs/advanced-topics/struct-components/lifecycle", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::lifecycle::page_content() },
                "advanced-topics/struct-components/scope" => $crate::PageData { title: "Scope", sidebar_path: "/docs/advanced-topics/struct-components/scope", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::scope::page_content() },
                "advanced-topics/struct-components/callbacks" => $crate::PageData { title: "Callbacks", sidebar_path: "/docs/advanced-topics/struct-components/callbacks", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::callbacks::page_content() },
                "advanced-topics/struct-components/properties" => $crate::PageData { title: "Properties", sidebar_path: "/docs/advanced-topics/struct-components/properties", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::properties::page_content() },
                "advanced-topics/struct-components/refs" => $crate::PageData { title: "Refs", sidebar_path: "/docs/advanced-topics/struct-components/refs", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::struct_components::refs::page_content() },
                "advanced-topics/children" => $crate::PageData { title: "Children", sidebar_path: "/docs/advanced-topics/children", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::children::page_content() },
                "advanced-topics/optimizations" => $crate::PageData { title: "Optimizations & Best Practices", sidebar_path: "/docs/advanced-topics/optimizations", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::optimizations::page_content() },
                "advanced-topics/portals" => $crate::PageData { title: "Portals", sidebar_path: "/docs/advanced-topics/portals", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::portals::page_content() },
                "advanced-topics/server-side-rendering" => $crate::PageData { title: "Server-side Rendering", sidebar_path: "/docs/advanced-topics/server-side-rendering", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::server_side_rendering::page_content() },
                "advanced-topics/immutable" => $crate::PageData { title: "Immutable Types", sidebar_path: "/docs/advanced-topics/immutable", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::advanced_topics::immutable::page_content() },
                "more/debugging" => $crate::PageData { title: "Debugging", sidebar_path: "/docs/more/debugging", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::more::debugging::page_content() },
                "more/deployment" => $crate::PageData { title: "Deployment", sidebar_path: "/docs/more/deployment", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::more::deployment::page_content() },
                "more/css" => $crate::PageData { title: "CSS", sidebar_path: "/docs/more/css", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::more::css::page_content() },
                "more/testing" => $crate::PageData { title: "Testing apps", sidebar_path: "/docs/more/testing", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::more::testing::page_content() },
                "more/roadmap" => $crate::PageData { title: "Roadmap", sidebar_path: "/docs/more/roadmap", doc_version: $version, sidebar: $sidebar_fn(), content: $doc::pages::more::roadmap::page_content() },
                _ => return None,
            })
        }
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
                    if let Some(r) = Route::recognize(href.as_str()) {
                        if !matches!(r, Route::NotFound) {
                            e.prevent_default();
                            navigator.push(&r);
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

#[macro_export]
macro_rules! __ssr_doc_pages {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr) => {{
        let page = $resolve_fn("getting-started").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/getting-started"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("getting-started").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("getting-started/build-a-sample-app").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/getting-started/build-a-sample-app"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("getting-started/build-a-sample-app")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("getting-started/editor-setup").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/getting-started/editor-setup"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("getting-started/editor-setup").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("getting-started/examples").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/getting-started/examples"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("getting-started/examples").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/basic-web-technologies/html").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/basic-web-technologies/html"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/basic-web-technologies/html")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/basic-web-technologies/css").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/basic-web-technologies/css"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/basic-web-technologies/css")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/basic-web-technologies/js").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/basic-web-technologies/js"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/basic-web-technologies/js")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/basic-web-technologies/wasm-bindgen").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/basic-web-technologies/wasm-bindgen"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/basic-web-technologies/wasm-bindgen")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/basic-web-technologies/web-sys").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/basic-web-technologies/web-sys"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/basic-web-technologies/web-sys")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/properties").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/properties"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/properties")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/callbacks").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/callbacks"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/callbacks")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/children").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/children"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/children")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/pure-components").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/pure-components"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/pure-components")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/hooks").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/hooks"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/hooks")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/hooks/custom-hooks").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!(
                $url_prefix,
                "/concepts/function-components/hooks/custom-hooks"
            ),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/hooks/custom-hooks")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/node-refs").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/node-refs"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/node-refs")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/state").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/state"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/state")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/communication").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/communication"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/communication")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/function-components/generics").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/function-components/generics"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/function-components/generics")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/html").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/components").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/components"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/components").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/elements").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/elements"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/elements").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/events").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/events"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/events").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/classes").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/classes"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/classes").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/fragments").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/fragments"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/fragments").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/lists").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/lists"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/lists").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/literals-and-expressions").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/literals-and-expressions"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/literals-and-expressions")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/html/conditional-rendering").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/html/conditional-rendering"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/html/conditional-rendering")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("concepts/agents").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/agents"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/agents").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/contexts").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/contexts"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/contexts").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/router").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/router"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/router").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("concepts/suspense").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/concepts/suspense"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("concepts/suspense").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/how-it-works").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/how-it-works"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/how-it-works").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/hoc").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/hoc"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/hoc")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/lifecycle").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/lifecycle"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/lifecycle")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/scope").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/scope"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/scope")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/callbacks").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/callbacks"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/callbacks")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/properties").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/properties"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/properties")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/struct-components/refs").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/struct-components/refs"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/struct-components/refs")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/children").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/children"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/children").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/optimizations").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/optimizations"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/optimizations")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/portals").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/portals"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/portals").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/server-side-rendering").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/server-side-rendering"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/server-side-rendering")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("advanced-topics/immutable").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/advanced-topics/immutable"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("advanced-topics/immutable").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("more/debugging").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/more/debugging"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("more/debugging").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("more/deployment").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/more/deployment"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("more/deployment").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("more/css").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/more/css"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("more/css").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("more/testing").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/more/testing"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("more/testing").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("more/roadmap").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/more/roadmap"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("more/roadmap").unwrap().content
        ));
    }};
}

#[macro_export]
macro_rules! __ssr_migration_pages {
    ($pages:ident, $url_prefix:expr, $resolve_fn:ident, $lang:expr) => {{
        let page = $resolve_fn("yew/from-0-19-0-to-0-20-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew/from-0-19-0-to-0-20-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew/from-0-19-0-to-0-20-0").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("yew/from-0-20-0-to-0-21-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew/from-0-20-0-to-0-21-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew/from-0-20-0-to-0-21-0").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("yew/from-0-21-0-to-0-22-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew/from-0-21-0-to-0-22-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew/from-0-21-0-to-0-22-0").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("yew/from-0-22-0-to-0-23-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew/from-0-22-0-to-0-23-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew/from-0-22-0-to-0-23-0").unwrap().content
        ));
    }
    {
        let page = $resolve_fn("yew-agent/from-0-0-0-to-0-1-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-agent/from-0-0-0-to-0-1-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-agent/from-0-0-0-to-0-1-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-agent/from-0-1-0-to-0-2-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-agent/from-0-1-0-to-0-2-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-agent/from-0-1-0-to-0-2-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-agent/from-0-3-0-to-0-4-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-agent/from-0-3-0-to-0-4-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-agent/from-0-3-0-to-0-4-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-agent/from-0-4-0-to-0-5-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-agent/from-0-4-0-to-0-5-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-agent/from-0-4-0-to-0-5-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-router/from-0-15-0-to-0-16-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-router/from-0-15-0-to-0-16-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-router/from-0-15-0-to-0-16-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-router/from-0-16-0-to-0-17-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-router/from-0-16-0-to-0-17-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-router/from-0-16-0-to-0-17-0")
                .unwrap()
                .content
        ));
    }
    {
        let page = $resolve_fn("yew-router/from-0-19-0-to-0-20-0").unwrap();
        $pages.push($crate::render_spa_page!(
            concat!($url_prefix, "/yew-router/from-0-19-0-to-0-20-0"),
            page.title,
            page.sidebar,
            page.sidebar_path,
            page.doc_version,
            $lang,
            $resolve_fn("yew-router/from-0-19-0-to-0-20-0")
                .unwrap()
                .content
        ));
    }};
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
        pub async fn render_pages() -> Vec<(&'static str, String, String)> {
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

            pages.push($crate::render_page!(concat!($url_prefix, "/"), $home));
            pages.push($crate::render_page!(
                concat!($url_prefix, "/next/"),
                $home_next
            ));
            pages.push($crate::render_page!(
                concat!($url_prefix, "/0.22/"),
                $home_v022
            ));
            pages.push($crate::render_page!(
                concat!($url_prefix, "/0.21/"),
                $home_v021
            ));
            pages.push($crate::render_page!(
                concat!($url_prefix, "/0.20/"),
                $home_v020
            ));

            pages
        }
    };
}
