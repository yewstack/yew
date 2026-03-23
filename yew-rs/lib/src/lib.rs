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
