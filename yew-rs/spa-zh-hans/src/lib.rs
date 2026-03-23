use yew::prelude::*;
#[cfg(feature = "csr")]
use yew_router::prelude::*;
use yew_site_lib::PageData;
#[cfg(feature = "csr")]
use yew_site_lib::{Layout, NavigationContext};

#[cfg(feature = "csr")]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/zh-Hans/")]
    Home,
    #[at("/zh-Hans/next/")]
    HomeNext,
    #[at("/zh-Hans/0.22/")]
    HomeV022,
    #[at("/zh-Hans/0.21/")]
    HomeV021,
    #[at("/zh-Hans/0.20/")]
    HomeV020,
    #[at("/zh-Hans/tutorial/")]
    Tutorial,
    #[at("/zh-Hans/next/tutorial/")]
    TutorialNext,
    #[at("/zh-Hans/0.22/tutorial/")]
    TutorialV022,
    #[at("/zh-Hans/0.21/tutorial/")]
    TutorialV021,
    #[at("/zh-Hans/0.20/tutorial/")]
    TutorialV020,
    #[at("/zh-Hans/docs/next/*path")]
    DocsNext { path: String },
    #[at("/zh-Hans/docs/0.22/*path")]
    DocsV022 { path: String },
    #[at("/zh-Hans/docs/0.21/*path")]
    DocsV021 { path: String },
    #[at("/zh-Hans/docs/0.20/*path")]
    DocsV020 { path: String },
    #[at("/zh-Hans/docs/migration-guides/*path")]
    MigrationGuides { path: String },
    #[at("/zh-Hans/docs/*path")]
    DocsStable { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

macro_rules! page_map {
    ($fn_name:ident, $version:expr, $sidebar_fn:path, [
        $( ($path:expr, $title:expr, $spath:expr, $content:expr) ),* $(,)?
    ]) => {
        pub fn $fn_name(path: &str) -> Option<PageData> {
            Some(match path {
                $( $path => PageData {
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

fn sidebar() -> Vec<yew_site_lib::SidebarEntry> {
    yew_site_lib::translate_sidebar(
        yew_site_docs_zh_hans::sidebar_data::docs_sidebar(),
        "zh-Hans",
    )
}

fn sidebar_0_23() -> Vec<yew_site_lib::SidebarEntry> {
    yew_site_lib::translate_sidebar(
        yew_site_docs_zh_hans_0_23::sidebar_data::docs_sidebar(),
        "zh-Hans",
    )
}

fn sidebar_0_22() -> Vec<yew_site_lib::SidebarEntry> {
    yew_site_lib::translate_sidebar(
        yew_site_docs_zh_hans_0_22::sidebar_data::docs_sidebar(),
        "zh-Hans",
    )
}

fn sidebar_0_21() -> Vec<yew_site_lib::SidebarEntry> {
    yew_site_lib::translate_sidebar(
        yew_site_docs_zh_hans_0_21::sidebar_data::docs_sidebar(),
        "zh-Hans",
    )
}

fn sidebar_0_20() -> Vec<yew_site_lib::SidebarEntry> {
    yew_site_lib::translate_sidebar(
        yew_site_docs_zh_hans_0_20::sidebar_data::docs_sidebar(),
        "zh-Hans",
    )
}

page_map!(resolve_next, "Next", sidebar, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_zh_hans::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content()),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_zh_hans::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_zh_hans::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_zh_hans::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_zh_hans::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_zh_hans::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_zh_hans::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_zh_hans::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_zh_hans::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_zh_hans::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_zh_hans::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_zh_hans::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_zh_hans::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_zh_hans::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_zh_hans::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_zh_hans::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_zh_hans::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_zh_hans::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_zh_hans::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_zh_hans::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs_zh_hans::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_zh_hans::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_zh_hans::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_zh_hans::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_zh_hans::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_zh_hans::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_zh_hans::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_zh_hans::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_zh_hans::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_zh_hans::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_zh_hans::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_zh_hans::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_zh_hans::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_zh_hans::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_zh_hans::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs_zh_hans::pages::more::roadmap::page_content()),
]);

page_map!(resolve_stable, "0.23", sidebar_0_23, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_zh_hans::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content_versioned(Some("0.23"))),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_zh_hans::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_zh_hans::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_zh_hans::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_zh_hans::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_zh_hans::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_zh_hans::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_zh_hans::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_zh_hans::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_zh_hans::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_zh_hans::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_zh_hans::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_zh_hans::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_zh_hans::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_zh_hans::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_zh_hans::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_zh_hans::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_zh_hans::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_zh_hans::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_zh_hans::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs_zh_hans::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_zh_hans::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_zh_hans::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_zh_hans::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_zh_hans::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_zh_hans::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_zh_hans::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_zh_hans::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_zh_hans::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_zh_hans::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_zh_hans::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_zh_hans::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_zh_hans::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_zh_hans::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_zh_hans::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs_zh_hans::pages::more::roadmap::page_content()),
]);

page_map!(resolve_v022, "0.22", sidebar_0_22, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_zh_hans::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content_versioned(Some("0.22"))),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_zh_hans::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_zh_hans::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_zh_hans::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_zh_hans::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_zh_hans::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_zh_hans::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_zh_hans::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_zh_hans::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_zh_hans::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_zh_hans::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_zh_hans::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_zh_hans::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_zh_hans::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_zh_hans::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_zh_hans::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_zh_hans::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_zh_hans::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_zh_hans::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_zh_hans::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_zh_hans::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_zh_hans::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs_zh_hans::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_zh_hans::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_zh_hans::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_zh_hans::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_zh_hans::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_zh_hans::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_zh_hans::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_zh_hans::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_zh_hans::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_zh_hans::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_zh_hans::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_zh_hans::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_zh_hans::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_zh_hans::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_zh_hans::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_zh_hans::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs_zh_hans::pages::more::roadmap::page_content()),
]);

page_map!(resolve_v021, "0.21", sidebar_0_21, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_zh_hans_0_21::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_zh_hans_0_21::pages::getting_started::build_a_sample_app::page_content()),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_zh_hans_0_21::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_zh_hans_0_21::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_zh_hans_0_21::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_zh_hans_0_21::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_zh_hans_0_21::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_zh_hans_0_21::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_zh_hans_0_21::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_zh_hans_0_21::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_zh_hans_0_21::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_zh_hans_0_21::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_zh_hans_0_21::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_zh_hans_0_21::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_zh_hans_0_21::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_zh_hans_0_21::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_zh_hans_0_21::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_zh_hans_0_21::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs_zh_hans_0_21::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_zh_hans_0_21::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_zh_hans_0_21::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_zh_hans_0_21::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_zh_hans_0_21::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_zh_hans_0_21::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_zh_hans_0_21::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_zh_hans_0_21::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_zh_hans_0_21::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_zh_hans_0_21::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_zh_hans_0_21::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_zh_hans_0_21::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_zh_hans_0_21::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_zh_hans_0_21::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_zh_hans_0_21::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_zh_hans_0_21::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs_zh_hans_0_21::pages::more::roadmap::page_content()),
]);

page_map!(resolve_v020, "0.20", sidebar_0_20, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_zh_hans_0_20::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_zh_hans_0_20::pages::getting_started::build_a_sample_app::page_content()),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_zh_hans_0_20::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_zh_hans_0_20::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_zh_hans_0_20::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_zh_hans_0_20::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_zh_hans_0_20::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_zh_hans_0_20::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_zh_hans_0_20::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_zh_hans_0_20::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_zh_hans_0_20::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_zh_hans_0_20::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_zh_hans_0_20::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_zh_hans_0_20::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_zh_hans_0_20::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_zh_hans_0_20::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_zh_hans_0_20::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_zh_hans_0_20::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs_zh_hans_0_20::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_zh_hans_0_20::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_zh_hans_0_20::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_zh_hans_0_20::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_zh_hans_0_20::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_zh_hans_0_20::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_zh_hans_0_20::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_zh_hans_0_20::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_zh_hans_0_20::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_zh_hans_0_20::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_zh_hans_0_20::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_zh_hans_0_20::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_zh_hans_0_20::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_zh_hans_0_20::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_zh_hans_0_20::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_zh_hans_0_20::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs_zh_hans_0_20::pages::more::roadmap::page_content()),
]);

page_map!(resolve_migration, "", sidebar, [
    ("yew/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew/from-0-19-0-to-0-20-0", yew_site_docs::pages::migration_guides::yew::from_0_19_0_to_0_20_0::page_content()),
    ("yew/from-0-20-0-to-0-21-0", "From 0.20.0 to 0.21.0", "/docs/migration-guides/yew/from-0-20-0-to-0-21-0", yew_site_docs::pages::migration_guides::yew::from_0_20_0_to_0_21_0::page_content()),
    ("yew/from-0-21-0-to-0-22-0", "From 0.21.0 to 0.22.0", "/docs/migration-guides/yew/from-0-21-0-to-0-22-0", yew_site_docs::pages::migration_guides::yew::from_0_21_0_to_0_22_0::page_content()),
    ("yew/from-0-22-0-to-0-23-0", "From 0.22.0 to 0.23.0", "/docs/migration-guides/yew/from-0-22-0-to-0-23-0", yew_site_docs_zh_hans::pages::migration_guides::yew::from_0_22_0_to_0_23_0::page_content()),
    ("yew-agent/from-0-0-0-to-0-1-0", "From 0.0.0 to 0.1.0", "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_0_0_to_0_1_0::page_content()),
    ("yew-agent/from-0-1-0-to-0-2-0", "From 0.1.0 to 0.2.0", "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_1_0_to_0_2_0::page_content()),
    ("yew-agent/from-0-3-0-to-0-4-0", "From 0.3.0 to 0.4.0", "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_3_0_to_0_4_0::page_content()),
    ("yew-agent/from-0-4-0-to-0-5-0", "From 0.4.0 to 0.5.0", "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0", yew_site_docs_zh_hans::pages::migration_guides::yew_agent::from_0_4_0_to_0_5_0::page_content()),
    ("yew-router/from-0-15-0-to-0-16-0", "From 0.15.0 to 0.16.0", "/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0", yew_site_docs_zh_hans::pages::migration_guides::yew_router::from_0_15_0_to_0_16_0::page_content()),
    ("yew-router/from-0-16-0-to-0-17-0", "From 0.16.0 to 0.17.0", "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0", yew_site_docs::pages::migration_guides::yew_router::from_0_16_0_to_0_17_0::page_content()),
    ("yew-router/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0", yew_site_docs_zh_hans::pages::migration_guides::yew_router::from_0_19_0_to_0_20_0::page_content()),
]);

#[cfg(feature = "csr")]
pub fn resolve_page(route: &Route) -> Option<PageData> {
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
fn resolve_tutorial(route: &Route) -> Option<(yew_site_lib::Content, &'static str)> {
    match route {
        Route::Tutorial => Some((
            yew_site_docs_zh_hans::pages::tutorial::page_content_versioned(Some("0.23")),
            "0.23",
        )),
        Route::TutorialNext => Some((
            yew_site_docs_zh_hans::pages::tutorial::page_content(),
            "Next",
        )),
        Route::TutorialV022 => Some((
            yew_site_docs_zh_hans::pages::tutorial::page_content_versioned(Some("0.22")),
            "0.22",
        )),
        Route::TutorialV021 => Some((
            yew_site_docs_zh_hans_0_21::pages::tutorial::page_content(),
            "0.21",
        )),
        Route::TutorialV020 => Some((
            yew_site_docs_zh_hans_0_20::pages::tutorial::page_content(),
            "0.20",
        )),
        _ => None,
    }
}

#[cfg(feature = "csr")]
fn resolve_home(route: &Route) -> Option<(&'static str, &'static str, &'static str)> {
    match route {
        Route::Home => Some(("zh-Hans", "", "0.23")),
        Route::HomeNext => Some(("zh-Hans", "next", "Next")),
        Route::HomeV022 => Some(("zh-Hans", "0.22", "0.22")),
        Route::HomeV021 => Some(("zh-Hans", "0.21", "0.21")),
        Route::HomeV020 => Some(("zh-Hans", "0.20", "0.20")),
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
                    lang="zh-Hans"
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
                    lang="zh-Hans"
                    markdown={markdown}
                    toc={toc}
                >
                    { content.to_html() }
                </Layout>
            </ContextProvider<NavigationContext>>
        }
    } else if let Some((locale, version_slug, doc_version)) = resolve_home(&route) {
        let lang_prefix = format!("/{locale}");
        let current_path = if version_slug.is_empty() {
            format!("{lang_prefix}/")
        } else {
            format!("{lang_prefix}/{version_slug}/")
        };
        html! {
            <ContextProvider<NavigationContext> context={nav_ctx}>
                <Layout
                    title=""
                    full_width=true
                    lang="zh-Hans"
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
                <Layout title="Page Not Found" active_nav="Docs" lang="zh-Hans">
                    <h1>{"404 - Page Not Found"}</h1>
                </Layout>
            </ContextProvider<NavigationContext>>
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn render_pages() -> Vec<(&'static str, String, String)> {
    let mut pages = Vec::new();

    macro_rules! ssr_page {
        ($url:expr, $page_fn:ident, $path:expr) => {{
            let page = $page_fn($path).unwrap();
            pages.push(yew_site_lib::render_spa_page!(
                $url,
                page.title,
                page.sidebar,
                page.sidebar_path,
                page.doc_version,
                "zh-Hans",
                $page_fn($path).unwrap().content
            ));
        }};
    }

    // Next version
    ssr_page!(
        "/zh-Hans/docs/next/getting-started",
        resolve_next,
        "getting-started"
    );
    ssr_page!(
        "/zh-Hans/docs/next/getting-started/build-a-sample-app",
        resolve_next,
        "getting-started/build-a-sample-app"
    );
    ssr_page!(
        "/zh-Hans/docs/next/getting-started/editor-setup",
        resolve_next,
        "getting-started/editor-setup"
    );
    ssr_page!(
        "/zh-Hans/docs/next/getting-started/examples",
        resolve_next,
        "getting-started/examples"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/basic-web-technologies/html",
        resolve_next,
        "concepts/basic-web-technologies/html"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/basic-web-technologies/css",
        resolve_next,
        "concepts/basic-web-technologies/css"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/basic-web-technologies/js",
        resolve_next,
        "concepts/basic-web-technologies/js"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/basic-web-technologies/wasm-bindgen",
        resolve_next,
        "concepts/basic-web-technologies/wasm-bindgen"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/basic-web-technologies/web-sys",
        resolve_next,
        "concepts/basic-web-technologies/web-sys"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components",
        resolve_next,
        "concepts/function-components"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/properties",
        resolve_next,
        "concepts/function-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/callbacks",
        resolve_next,
        "concepts/function-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/children",
        resolve_next,
        "concepts/function-components/children"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/pure-components",
        resolve_next,
        "concepts/function-components/pure-components"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/hooks",
        resolve_next,
        "concepts/function-components/hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/hooks/custom-hooks",
        resolve_next,
        "concepts/function-components/hooks/custom-hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/node-refs",
        resolve_next,
        "concepts/function-components/node-refs"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/state",
        resolve_next,
        "concepts/function-components/state"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/communication",
        resolve_next,
        "concepts/function-components/communication"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/function-components/generics",
        resolve_next,
        "concepts/function-components/generics"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html",
        resolve_next,
        "concepts/html"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/components",
        resolve_next,
        "concepts/html/components"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/elements",
        resolve_next,
        "concepts/html/elements"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/events",
        resolve_next,
        "concepts/html/events"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/classes",
        resolve_next,
        "concepts/html/classes"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/fragments",
        resolve_next,
        "concepts/html/fragments"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/lists",
        resolve_next,
        "concepts/html/lists"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/literals-and-expressions",
        resolve_next,
        "concepts/html/literals-and-expressions"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/html/conditional-rendering",
        resolve_next,
        "concepts/html/conditional-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/agents",
        resolve_next,
        "concepts/agents"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/contexts",
        resolve_next,
        "concepts/contexts"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/router",
        resolve_next,
        "concepts/router"
    );
    ssr_page!(
        "/zh-Hans/docs/next/concepts/suspense",
        resolve_next,
        "concepts/suspense"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/how-it-works",
        resolve_next,
        "advanced-topics/how-it-works"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components",
        resolve_next,
        "advanced-topics/struct-components"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/hoc",
        resolve_next,
        "advanced-topics/struct-components/hoc"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/lifecycle",
        resolve_next,
        "advanced-topics/struct-components/lifecycle"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/scope",
        resolve_next,
        "advanced-topics/struct-components/scope"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/callbacks",
        resolve_next,
        "advanced-topics/struct-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/properties",
        resolve_next,
        "advanced-topics/struct-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/struct-components/refs",
        resolve_next,
        "advanced-topics/struct-components/refs"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/children",
        resolve_next,
        "advanced-topics/children"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/optimizations",
        resolve_next,
        "advanced-topics/optimizations"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/portals",
        resolve_next,
        "advanced-topics/portals"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/server-side-rendering",
        resolve_next,
        "advanced-topics/server-side-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/next/advanced-topics/immutable",
        resolve_next,
        "advanced-topics/immutable"
    );
    ssr_page!(
        "/zh-Hans/docs/next/more/debugging",
        resolve_next,
        "more/debugging"
    );
    ssr_page!(
        "/zh-Hans/docs/next/more/deployment",
        resolve_next,
        "more/deployment"
    );
    ssr_page!("/zh-Hans/docs/next/more/css", resolve_next, "more/css");
    ssr_page!(
        "/zh-Hans/docs/next/more/testing",
        resolve_next,
        "more/testing"
    );
    ssr_page!(
        "/zh-Hans/docs/next/more/roadmap",
        resolve_next,
        "more/roadmap"
    );
    {
        pages.push(yew_site_lib::render_spa_page!(
            "/zh-Hans/next/tutorial",
            "Tutorial",
            yew_site_docs_zh_hans::sidebar_data::docs_sidebar(),
            "/tutorial",
            "Next",
            "zh-Hans",
            yew_site_docs_zh_hans::pages::tutorial::page_content()
        ));
    }

    // Stable (0.23)
    ssr_page!(
        "/zh-Hans/docs/getting-started",
        resolve_stable,
        "getting-started"
    );
    ssr_page!(
        "/zh-Hans/docs/getting-started/build-a-sample-app",
        resolve_stable,
        "getting-started/build-a-sample-app"
    );
    ssr_page!(
        "/zh-Hans/docs/getting-started/editor-setup",
        resolve_stable,
        "getting-started/editor-setup"
    );
    ssr_page!(
        "/zh-Hans/docs/getting-started/examples",
        resolve_stable,
        "getting-started/examples"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/basic-web-technologies/html",
        resolve_stable,
        "concepts/basic-web-technologies/html"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/basic-web-technologies/css",
        resolve_stable,
        "concepts/basic-web-technologies/css"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/basic-web-technologies/js",
        resolve_stable,
        "concepts/basic-web-technologies/js"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/basic-web-technologies/wasm-bindgen",
        resolve_stable,
        "concepts/basic-web-technologies/wasm-bindgen"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
        resolve_stable,
        "concepts/basic-web-technologies/web-sys"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components",
        resolve_stable,
        "concepts/function-components"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/properties",
        resolve_stable,
        "concepts/function-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/callbacks",
        resolve_stable,
        "concepts/function-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/children",
        resolve_stable,
        "concepts/function-components/children"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/pure-components",
        resolve_stable,
        "concepts/function-components/pure-components"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/hooks",
        resolve_stable,
        "concepts/function-components/hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/hooks/custom-hooks",
        resolve_stable,
        "concepts/function-components/hooks/custom-hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/node-refs",
        resolve_stable,
        "concepts/function-components/node-refs"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/state",
        resolve_stable,
        "concepts/function-components/state"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/communication",
        resolve_stable,
        "concepts/function-components/communication"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/function-components/generics",
        resolve_stable,
        "concepts/function-components/generics"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html",
        resolve_stable,
        "concepts/html"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/components",
        resolve_stable,
        "concepts/html/components"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/elements",
        resolve_stable,
        "concepts/html/elements"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/events",
        resolve_stable,
        "concepts/html/events"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/classes",
        resolve_stable,
        "concepts/html/classes"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/fragments",
        resolve_stable,
        "concepts/html/fragments"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/lists",
        resolve_stable,
        "concepts/html/lists"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/literals-and-expressions",
        resolve_stable,
        "concepts/html/literals-and-expressions"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/html/conditional-rendering",
        resolve_stable,
        "concepts/html/conditional-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/agents",
        resolve_stable,
        "concepts/agents"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/contexts",
        resolve_stable,
        "concepts/contexts"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/router",
        resolve_stable,
        "concepts/router"
    );
    ssr_page!(
        "/zh-Hans/docs/concepts/suspense",
        resolve_stable,
        "concepts/suspense"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/how-it-works",
        resolve_stable,
        "advanced-topics/how-it-works"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components",
        resolve_stable,
        "advanced-topics/struct-components"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/hoc",
        resolve_stable,
        "advanced-topics/struct-components/hoc"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/lifecycle",
        resolve_stable,
        "advanced-topics/struct-components/lifecycle"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/scope",
        resolve_stable,
        "advanced-topics/struct-components/scope"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/callbacks",
        resolve_stable,
        "advanced-topics/struct-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/properties",
        resolve_stable,
        "advanced-topics/struct-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/struct-components/refs",
        resolve_stable,
        "advanced-topics/struct-components/refs"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/children",
        resolve_stable,
        "advanced-topics/children"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/optimizations",
        resolve_stable,
        "advanced-topics/optimizations"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/portals",
        resolve_stable,
        "advanced-topics/portals"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/server-side-rendering",
        resolve_stable,
        "advanced-topics/server-side-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/advanced-topics/immutable",
        resolve_stable,
        "advanced-topics/immutable"
    );
    ssr_page!(
        "/zh-Hans/docs/more/debugging",
        resolve_stable,
        "more/debugging"
    );
    ssr_page!(
        "/zh-Hans/docs/more/deployment",
        resolve_stable,
        "more/deployment"
    );
    ssr_page!("/zh-Hans/docs/more/css", resolve_stable, "more/css");
    ssr_page!("/zh-Hans/docs/more/testing", resolve_stable, "more/testing");
    ssr_page!("/zh-Hans/docs/more/roadmap", resolve_stable, "more/roadmap");
    {
        pages.push(yew_site_lib::render_spa_page!(
            "/zh-Hans/tutorial",
            "Tutorial",
            yew_site_docs_zh_hans_0_23::sidebar_data::docs_sidebar(),
            "/tutorial",
            "0.23",
            "zh-Hans",
            yew_site_docs_zh_hans::pages::tutorial::page_content_versioned(Some("0.23"))
        ));
    }

    // 0.22
    ssr_page!(
        "/zh-Hans/docs/0.22/getting-started",
        resolve_v022,
        "getting-started"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/getting-started/build-a-sample-app",
        resolve_v022,
        "getting-started/build-a-sample-app"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/getting-started/editor-setup",
        resolve_v022,
        "getting-started/editor-setup"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/getting-started/examples",
        resolve_v022,
        "getting-started/examples"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/basic-web-technologies/html",
        resolve_v022,
        "concepts/basic-web-technologies/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/basic-web-technologies/css",
        resolve_v022,
        "concepts/basic-web-technologies/css"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/basic-web-technologies/js",
        resolve_v022,
        "concepts/basic-web-technologies/js"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/basic-web-technologies/wasm-bindgen",
        resolve_v022,
        "concepts/basic-web-technologies/wasm-bindgen"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/basic-web-technologies/web-sys",
        resolve_v022,
        "concepts/basic-web-technologies/web-sys"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components",
        resolve_v022,
        "concepts/function-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/properties",
        resolve_v022,
        "concepts/function-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/callbacks",
        resolve_v022,
        "concepts/function-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/children",
        resolve_v022,
        "concepts/function-components/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/pure-components",
        resolve_v022,
        "concepts/function-components/pure-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/hooks",
        resolve_v022,
        "concepts/function-components/hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/hooks/custom-hooks",
        resolve_v022,
        "concepts/function-components/hooks/custom-hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/node-refs",
        resolve_v022,
        "concepts/function-components/node-refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/state",
        resolve_v022,
        "concepts/function-components/state"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/communication",
        resolve_v022,
        "concepts/function-components/communication"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/function-components/generics",
        resolve_v022,
        "concepts/function-components/generics"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html",
        resolve_v022,
        "concepts/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/components",
        resolve_v022,
        "concepts/html/components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/elements",
        resolve_v022,
        "concepts/html/elements"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/events",
        resolve_v022,
        "concepts/html/events"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/classes",
        resolve_v022,
        "concepts/html/classes"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/fragments",
        resolve_v022,
        "concepts/html/fragments"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/lists",
        resolve_v022,
        "concepts/html/lists"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/literals-and-expressions",
        resolve_v022,
        "concepts/html/literals-and-expressions"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/html/conditional-rendering",
        resolve_v022,
        "concepts/html/conditional-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/agents",
        resolve_v022,
        "concepts/agents"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/contexts",
        resolve_v022,
        "concepts/contexts"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/router",
        resolve_v022,
        "concepts/router"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/concepts/suspense",
        resolve_v022,
        "concepts/suspense"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/how-it-works",
        resolve_v022,
        "advanced-topics/how-it-works"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components",
        resolve_v022,
        "advanced-topics/struct-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/hoc",
        resolve_v022,
        "advanced-topics/struct-components/hoc"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/lifecycle",
        resolve_v022,
        "advanced-topics/struct-components/lifecycle"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/scope",
        resolve_v022,
        "advanced-topics/struct-components/scope"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/callbacks",
        resolve_v022,
        "advanced-topics/struct-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/properties",
        resolve_v022,
        "advanced-topics/struct-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/struct-components/refs",
        resolve_v022,
        "advanced-topics/struct-components/refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/children",
        resolve_v022,
        "advanced-topics/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/optimizations",
        resolve_v022,
        "advanced-topics/optimizations"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/portals",
        resolve_v022,
        "advanced-topics/portals"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/server-side-rendering",
        resolve_v022,
        "advanced-topics/server-side-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/advanced-topics/immutable",
        resolve_v022,
        "advanced-topics/immutable"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/more/debugging",
        resolve_v022,
        "more/debugging"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/more/deployment",
        resolve_v022,
        "more/deployment"
    );
    ssr_page!("/zh-Hans/docs/0.22/more/css", resolve_v022, "more/css");
    ssr_page!(
        "/zh-Hans/docs/0.22/more/testing",
        resolve_v022,
        "more/testing"
    );
    ssr_page!(
        "/zh-Hans/docs/0.22/more/roadmap",
        resolve_v022,
        "more/roadmap"
    );
    {
        pages.push(yew_site_lib::render_spa_page!(
            "/zh-Hans/0.22/tutorial",
            "Tutorial",
            yew_site_docs_zh_hans_0_22::sidebar_data::docs_sidebar(),
            "/tutorial",
            "0.22",
            "zh-Hans",
            yew_site_docs_zh_hans::pages::tutorial::page_content_versioned(Some("0.22"))
        ));
    }

    // 0.21
    ssr_page!(
        "/zh-Hans/docs/0.21/getting-started",
        resolve_v021,
        "getting-started"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/getting-started/build-a-sample-app",
        resolve_v021,
        "getting-started/build-a-sample-app"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/getting-started/editor-setup",
        resolve_v021,
        "getting-started/editor-setup"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/getting-started/examples",
        resolve_v021,
        "getting-started/examples"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/basic-web-technologies/html",
        resolve_v021,
        "concepts/basic-web-technologies/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/basic-web-technologies/css",
        resolve_v021,
        "concepts/basic-web-technologies/css"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/basic-web-technologies/js",
        resolve_v021,
        "concepts/basic-web-technologies/js"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/basic-web-technologies/wasm-bindgen",
        resolve_v021,
        "concepts/basic-web-technologies/wasm-bindgen"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/basic-web-technologies/web-sys",
        resolve_v021,
        "concepts/basic-web-technologies/web-sys"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components",
        resolve_v021,
        "concepts/function-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/properties",
        resolve_v021,
        "concepts/function-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/callbacks",
        resolve_v021,
        "concepts/function-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/children",
        resolve_v021,
        "concepts/function-components/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/pure-components",
        resolve_v021,
        "concepts/function-components/pure-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/hooks",
        resolve_v021,
        "concepts/function-components/hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/hooks/custom-hooks",
        resolve_v021,
        "concepts/function-components/hooks/custom-hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/node-refs",
        resolve_v021,
        "concepts/function-components/node-refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/state",
        resolve_v021,
        "concepts/function-components/state"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/communication",
        resolve_v021,
        "concepts/function-components/communication"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/function-components/generics",
        resolve_v021,
        "concepts/function-components/generics"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html",
        resolve_v021,
        "concepts/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/components",
        resolve_v021,
        "concepts/html/components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/elements",
        resolve_v021,
        "concepts/html/elements"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/events",
        resolve_v021,
        "concepts/html/events"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/classes",
        resolve_v021,
        "concepts/html/classes"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/fragments",
        resolve_v021,
        "concepts/html/fragments"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/lists",
        resolve_v021,
        "concepts/html/lists"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/literals-and-expressions",
        resolve_v021,
        "concepts/html/literals-and-expressions"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/html/conditional-rendering",
        resolve_v021,
        "concepts/html/conditional-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/agents",
        resolve_v021,
        "concepts/agents"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/contexts",
        resolve_v021,
        "concepts/contexts"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/router",
        resolve_v021,
        "concepts/router"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/concepts/suspense",
        resolve_v021,
        "concepts/suspense"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/how-it-works",
        resolve_v021,
        "advanced-topics/how-it-works"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components",
        resolve_v021,
        "advanced-topics/struct-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/hoc",
        resolve_v021,
        "advanced-topics/struct-components/hoc"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/lifecycle",
        resolve_v021,
        "advanced-topics/struct-components/lifecycle"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/scope",
        resolve_v021,
        "advanced-topics/struct-components/scope"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/callbacks",
        resolve_v021,
        "advanced-topics/struct-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/properties",
        resolve_v021,
        "advanced-topics/struct-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/struct-components/refs",
        resolve_v021,
        "advanced-topics/struct-components/refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/children",
        resolve_v021,
        "advanced-topics/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/optimizations",
        resolve_v021,
        "advanced-topics/optimizations"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/portals",
        resolve_v021,
        "advanced-topics/portals"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/server-side-rendering",
        resolve_v021,
        "advanced-topics/server-side-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/advanced-topics/immutable",
        resolve_v021,
        "advanced-topics/immutable"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/more/debugging",
        resolve_v021,
        "more/debugging"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/more/deployment",
        resolve_v021,
        "more/deployment"
    );
    ssr_page!("/zh-Hans/docs/0.21/more/css", resolve_v021, "more/css");
    ssr_page!(
        "/zh-Hans/docs/0.21/more/testing",
        resolve_v021,
        "more/testing"
    );
    ssr_page!(
        "/zh-Hans/docs/0.21/more/roadmap",
        resolve_v021,
        "more/roadmap"
    );
    {
        pages.push(yew_site_lib::render_spa_page!(
            "/zh-Hans/0.21/tutorial",
            "Tutorial",
            yew_site_docs_zh_hans_0_21::sidebar_data::docs_sidebar(),
            "/tutorial",
            "0.21",
            "zh-Hans",
            yew_site_docs_zh_hans_0_21::pages::tutorial::page_content()
        ));
    }

    // 0.20
    ssr_page!(
        "/zh-Hans/docs/0.20/getting-started",
        resolve_v020,
        "getting-started"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/getting-started/build-a-sample-app",
        resolve_v020,
        "getting-started/build-a-sample-app"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/getting-started/editor-setup",
        resolve_v020,
        "getting-started/editor-setup"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/getting-started/examples",
        resolve_v020,
        "getting-started/examples"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/basic-web-technologies/html",
        resolve_v020,
        "concepts/basic-web-technologies/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/basic-web-technologies/css",
        resolve_v020,
        "concepts/basic-web-technologies/css"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/basic-web-technologies/js",
        resolve_v020,
        "concepts/basic-web-technologies/js"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/basic-web-technologies/wasm-bindgen",
        resolve_v020,
        "concepts/basic-web-technologies/wasm-bindgen"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/basic-web-technologies/web-sys",
        resolve_v020,
        "concepts/basic-web-technologies/web-sys"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components",
        resolve_v020,
        "concepts/function-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/properties",
        resolve_v020,
        "concepts/function-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/callbacks",
        resolve_v020,
        "concepts/function-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/children",
        resolve_v020,
        "concepts/function-components/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/pure-components",
        resolve_v020,
        "concepts/function-components/pure-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/hooks",
        resolve_v020,
        "concepts/function-components/hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/hooks/custom-hooks",
        resolve_v020,
        "concepts/function-components/hooks/custom-hooks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/node-refs",
        resolve_v020,
        "concepts/function-components/node-refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/state",
        resolve_v020,
        "concepts/function-components/state"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/communication",
        resolve_v020,
        "concepts/function-components/communication"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/function-components/generics",
        resolve_v020,
        "concepts/function-components/generics"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html",
        resolve_v020,
        "concepts/html"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/components",
        resolve_v020,
        "concepts/html/components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/elements",
        resolve_v020,
        "concepts/html/elements"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/events",
        resolve_v020,
        "concepts/html/events"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/classes",
        resolve_v020,
        "concepts/html/classes"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/fragments",
        resolve_v020,
        "concepts/html/fragments"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/lists",
        resolve_v020,
        "concepts/html/lists"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/literals-and-expressions",
        resolve_v020,
        "concepts/html/literals-and-expressions"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/html/conditional-rendering",
        resolve_v020,
        "concepts/html/conditional-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/agents",
        resolve_v020,
        "concepts/agents"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/contexts",
        resolve_v020,
        "concepts/contexts"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/router",
        resolve_v020,
        "concepts/router"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/concepts/suspense",
        resolve_v020,
        "concepts/suspense"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/how-it-works",
        resolve_v020,
        "advanced-topics/how-it-works"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components",
        resolve_v020,
        "advanced-topics/struct-components"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/hoc",
        resolve_v020,
        "advanced-topics/struct-components/hoc"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/lifecycle",
        resolve_v020,
        "advanced-topics/struct-components/lifecycle"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/scope",
        resolve_v020,
        "advanced-topics/struct-components/scope"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/callbacks",
        resolve_v020,
        "advanced-topics/struct-components/callbacks"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/properties",
        resolve_v020,
        "advanced-topics/struct-components/properties"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/struct-components/refs",
        resolve_v020,
        "advanced-topics/struct-components/refs"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/children",
        resolve_v020,
        "advanced-topics/children"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/optimizations",
        resolve_v020,
        "advanced-topics/optimizations"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/portals",
        resolve_v020,
        "advanced-topics/portals"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/server-side-rendering",
        resolve_v020,
        "advanced-topics/server-side-rendering"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/advanced-topics/immutable",
        resolve_v020,
        "advanced-topics/immutable"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/more/debugging",
        resolve_v020,
        "more/debugging"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/more/deployment",
        resolve_v020,
        "more/deployment"
    );
    ssr_page!("/zh-Hans/docs/0.20/more/css", resolve_v020, "more/css");
    ssr_page!(
        "/zh-Hans/docs/0.20/more/testing",
        resolve_v020,
        "more/testing"
    );
    ssr_page!(
        "/zh-Hans/docs/0.20/more/roadmap",
        resolve_v020,
        "more/roadmap"
    );
    {
        pages.push(yew_site_lib::render_spa_page!(
            "/zh-Hans/0.20/tutorial",
            "Tutorial",
            yew_site_docs_zh_hans_0_20::sidebar_data::docs_sidebar(),
            "/tutorial",
            "0.20",
            "zh-Hans",
            yew_site_docs_zh_hans_0_20::pages::tutorial::page_content()
        ));
    }

    // Migration guides
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew/from-0-19-0-to-0-20-0",
        resolve_migration,
        "yew/from-0-19-0-to-0-20-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew/from-0-20-0-to-0-21-0",
        resolve_migration,
        "yew/from-0-20-0-to-0-21-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew/from-0-21-0-to-0-22-0",
        resolve_migration,
        "yew/from-0-21-0-to-0-22-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        resolve_migration,
        "yew/from-0-22-0-to-0-23-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0",
        resolve_migration,
        "yew-agent/from-0-0-0-to-0-1-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
        resolve_migration,
        "yew-agent/from-0-1-0-to-0-2-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0",
        resolve_migration,
        "yew-agent/from-0-3-0-to-0-4-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
        resolve_migration,
        "yew-agent/from-0-4-0-to-0-5-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0",
        resolve_migration,
        "yew-router/from-0-15-0-to-0-16-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0",
        resolve_migration,
        "yew-router/from-0-16-0-to-0-17-0"
    );
    ssr_page!(
        "/zh-Hans/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
        resolve_migration,
        "yew-router/from-0-19-0-to-0-20-0"
    );

    pages.push(yew_site_lib::render_page!(
        "/zh-Hans/",
        yew_site_home::PageZhHans
    ));
    pages.push(yew_site_lib::render_page!(
        "/zh-Hans/next/",
        yew_site_home::PageZhHansNext
    ));
    pages.push(yew_site_lib::render_page!(
        "/zh-Hans/0.22/",
        yew_site_home::PageZhHansV022
    ));
    pages.push(yew_site_lib::render_page!(
        "/zh-Hans/0.21/",
        yew_site_home::PageZhHansV021
    ));
    pages.push(yew_site_lib::render_page!(
        "/zh-Hans/0.20/",
        yew_site_home::PageZhHansV020
    ));
    pages
}
