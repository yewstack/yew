use yew::prelude::*;
#[cfg(feature = "csr")]
use yew_router::prelude::*;
#[cfg(feature = "csr")]
use yew_site_lib::{Layout, NavigationContext};

#[cfg(feature = "csr")]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/next/")]
    HomeNext,
    #[at("/0.22/")]
    HomeV022,
    #[at("/0.21/")]
    HomeV021,
    #[at("/0.20/")]
    HomeV020,
    #[at("/tutorial/")]
    Tutorial,
    #[at("/next/tutorial/")]
    TutorialNext,
    #[at("/0.22/tutorial/")]
    TutorialV022,
    #[at("/0.21/tutorial/")]
    TutorialV021,
    #[at("/0.20/tutorial/")]
    TutorialV020,
    #[at("/docs/next/*path")]
    DocsNext { path: String },
    #[at("/docs/0.22/*path")]
    DocsV022 { path: String },
    #[at("/docs/0.21/*path")]
    DocsV021 { path: String },
    #[at("/docs/0.20/*path")]
    DocsV020 { path: String },
    #[at("/docs/migration-guides/*path")]
    MigrationGuides { path: String },
    #[at("/docs/*path")]
    DocsStable { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

yew_site_lib::spa_version_resolver!(
    resolve_next,
    "Next",
    yew_site_docs::sidebar_data::docs_sidebar,
    yew_site_docs,
    yew_site_docs::pages::getting_started::build_a_sample_app::page_content()
);
yew_site_lib::spa_version_resolver!(
    resolve_stable,
    "0.23",
    yew_site_docs_0_23::sidebar_data::docs_sidebar,
    yew_site_docs,
    yew_site_docs::pages::getting_started::build_a_sample_app::page_content_versioned(Some("0.23"))
);
yew_site_lib::spa_version_resolver!(
    resolve_v022,
    "0.22",
    yew_site_docs_0_22::sidebar_data::docs_sidebar,
    yew_site_docs,
    yew_site_docs::pages::getting_started::build_a_sample_app::page_content_versioned(Some("0.22"))
);

yew_site_lib::page_map!(resolve_v021, "0.21", yew_site_docs_0_21::sidebar_data::docs_sidebar, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_0_21::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_0_21::pages::getting_started::build_a_sample_app::page_content()),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_0_21::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_0_21::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_0_21::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_0_21::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_0_21::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_0_21::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_0_21::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_0_21::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_0_21::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_0_21::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_0_21::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_0_21::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_0_21::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_0_21::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_0_21::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_0_21::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_0_21::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_0_21::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_0_21::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_0_21::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_0_21::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_0_21::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_0_21::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_0_21::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_0_21::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_0_21::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_0_21::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_0_21::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_0_21::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_0_21::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs::pages::more::roadmap::page_content()),
]);

yew_site_lib::page_map!(resolve_v020, "0.20", yew_site_docs_0_20::sidebar_data::docs_sidebar, [
    ("getting-started", "Getting Started", "/docs/getting-started", yew_site_docs_0_20::pages::getting_started::introduction::page_content()),
    ("getting-started/build-a-sample-app", "Build a sample app", "/docs/getting-started/build-a-sample-app", yew_site_docs_0_20::pages::getting_started::build_a_sample_app::page_content()),
    ("getting-started/editor-setup", "Editor setup", "/docs/getting-started/editor-setup", yew_site_docs_0_20::pages::getting_started::editor_setup::page_content()),
    ("getting-started/examples", "Examples", "/docs/getting-started/examples", yew_site_docs_0_20::pages::getting_started::examples::page_content()),
    ("concepts/basic-web-technologies/html", "HTML with html!", "/docs/concepts/basic-web-technologies/html", yew_site_docs_0_20::pages::concepts::basic_web_technologies::html::page_content()),
    ("concepts/basic-web-technologies/css", "CSS with classes!", "/docs/concepts/basic-web-technologies/css", yew_site_docs_0_20::pages::concepts::basic_web_technologies::css::page_content()),
    ("concepts/basic-web-technologies/js", "JS with RS", "/docs/concepts/basic-web-technologies/js", yew_site_docs_0_20::pages::concepts::basic_web_technologies::js::page_content()),
    ("concepts/basic-web-technologies/wasm-bindgen", "wasm-bindgen", "/docs/concepts/basic-web-technologies/wasm-bindgen", yew_site_docs_0_20::pages::concepts::basic_web_technologies::wasm_bindgen::page_content()),
    ("concepts/basic-web-technologies/web-sys", "web-sys", "/docs/concepts/basic-web-technologies/web-sys", yew_site_docs_0_20::pages::concepts::basic_web_technologies::web_sys::page_content()),
    ("concepts/function-components", "Function Components", "/docs/concepts/function-components", yew_site_docs_0_20::pages::concepts::function_components::introduction::page_content()),
    ("concepts/function-components/properties", "Properties", "/docs/concepts/function-components/properties", yew_site_docs_0_20::pages::concepts::function_components::properties::page_content()),
    ("concepts/function-components/callbacks", "Callbacks", "/docs/concepts/function-components/callbacks", yew_site_docs_0_20::pages::concepts::function_components::callbacks::page_content()),
    ("concepts/function-components/children", "Children", "/docs/concepts/function-components/children", yew_site_docs_0_20::pages::concepts::function_components::children::page_content()),
    ("concepts/function-components/pure-components", "Pure Components", "/docs/concepts/function-components/pure-components", yew_site_docs_0_20::pages::concepts::function_components::pure_components::page_content()),
    ("concepts/function-components/hooks", "Hooks", "/docs/concepts/function-components/hooks", yew_site_docs_0_20::pages::concepts::function_components::hooks::introduction::page_content()),
    ("concepts/function-components/hooks/custom-hooks", "Custom Hooks", "/docs/concepts/function-components/hooks/custom-hooks", yew_site_docs_0_20::pages::concepts::function_components::hooks::custom_hooks::page_content()),
    ("concepts/function-components/node-refs", "Node Refs", "/docs/concepts/function-components/node-refs", yew_site_docs_0_20::pages::concepts::function_components::node_refs::page_content()),
    ("concepts/function-components/state", "State", "/docs/concepts/function-components/state", yew_site_docs_0_20::pages::concepts::function_components::state::page_content()),
    ("concepts/function-components/communication", "Communication between components", "/docs/concepts/function-components/communication", yew_site_docs_0_20::pages::concepts::function_components::communication::page_content()),
    ("concepts/function-components/generics", "Generic Components", "/docs/concepts/function-components/generics", yew_site_docs_0_20::pages::concepts::function_components::generics::page_content()),
    ("concepts/html", "HTML", "/docs/concepts/html", yew_site_docs_0_20::pages::concepts::html::introduction::page_content()),
    ("concepts/html/components", "Components", "/docs/concepts/html/components", yew_site_docs_0_20::pages::concepts::html::components::page_content()),
    ("concepts/html/elements", "Elements", "/docs/concepts/html/elements", yew_site_docs_0_20::pages::concepts::html::elements::page_content()),
    ("concepts/html/events", "Events", "/docs/concepts/html/events", yew_site_docs_0_20::pages::concepts::html::events::page_content()),
    ("concepts/html/classes", "Classes", "/docs/concepts/html/classes", yew_site_docs_0_20::pages::concepts::html::classes::page_content()),
    ("concepts/html/fragments", "Fragments", "/docs/concepts/html/fragments", yew_site_docs_0_20::pages::concepts::html::fragments::page_content()),
    ("concepts/html/lists", "Lists", "/docs/concepts/html/lists", yew_site_docs_0_20::pages::concepts::html::lists::page_content()),
    ("concepts/html/literals-and-expressions", "Literals and Expressions", "/docs/concepts/html/literals-and-expressions", yew_site_docs_0_20::pages::concepts::html::literals_and_expressions::page_content()),
    ("concepts/html/conditional-rendering", "Conditional Rendering", "/docs/concepts/html/conditional-rendering", yew_site_docs::pages::concepts::html::conditional_rendering::page_content()),
    ("concepts/agents", "Agents", "/docs/concepts/agents", yew_site_docs_0_20::pages::concepts::agents::page_content()),
    ("concepts/contexts", "Contexts", "/docs/concepts/contexts", yew_site_docs_0_20::pages::concepts::contexts::page_content()),
    ("concepts/router", "Router", "/docs/concepts/router", yew_site_docs_0_20::pages::concepts::router::page_content()),
    ("concepts/suspense", "Suspense", "/docs/concepts/suspense", yew_site_docs_0_20::pages::concepts::suspense::page_content()),
    ("advanced-topics/how-it-works", "How it works", "/docs/advanced-topics/how-it-works", yew_site_docs_0_20::pages::advanced_topics::how_it_works::page_content()),
    ("advanced-topics/struct-components", "Introduction", "/docs/advanced-topics/struct-components", yew_site_docs_0_20::pages::advanced_topics::struct_components::introduction::page_content()),
    ("advanced-topics/struct-components/hoc", "Higher Order Components", "/docs/advanced-topics/struct-components/hoc", yew_site_docs_0_20::pages::advanced_topics::struct_components::hoc::page_content()),
    ("advanced-topics/struct-components/lifecycle", "Lifecycle", "/docs/advanced-topics/struct-components/lifecycle", yew_site_docs_0_20::pages::advanced_topics::struct_components::lifecycle::page_content()),
    ("advanced-topics/struct-components/scope", "Scope", "/docs/advanced-topics/struct-components/scope", yew_site_docs_0_20::pages::advanced_topics::struct_components::scope::page_content()),
    ("advanced-topics/struct-components/callbacks", "Callbacks", "/docs/advanced-topics/struct-components/callbacks", yew_site_docs_0_20::pages::advanced_topics::struct_components::callbacks::page_content()),
    ("advanced-topics/struct-components/properties", "Properties", "/docs/advanced-topics/struct-components/properties", yew_site_docs_0_20::pages::advanced_topics::struct_components::properties::page_content()),
    ("advanced-topics/struct-components/refs", "Refs", "/docs/advanced-topics/struct-components/refs", yew_site_docs_0_20::pages::advanced_topics::struct_components::refs::page_content()),
    ("advanced-topics/children", "Children", "/docs/advanced-topics/children", yew_site_docs_0_20::pages::advanced_topics::children::page_content()),
    ("advanced-topics/optimizations", "Optimizations & Best Practices", "/docs/advanced-topics/optimizations", yew_site_docs_0_20::pages::advanced_topics::optimizations::page_content()),
    ("advanced-topics/portals", "Portals", "/docs/advanced-topics/portals", yew_site_docs_0_20::pages::advanced_topics::portals::page_content()),
    ("advanced-topics/server-side-rendering", "Server-side Rendering", "/docs/advanced-topics/server-side-rendering", yew_site_docs_0_20::pages::advanced_topics::server_side_rendering::page_content()),
    ("advanced-topics/immutable", "Immutable Types", "/docs/advanced-topics/immutable", yew_site_docs_0_20::pages::advanced_topics::immutable::page_content()),
    ("more/debugging", "Debugging", "/docs/more/debugging", yew_site_docs_0_21::pages::more::debugging::page_content()),
    ("more/deployment", "Deployment", "/docs/more/deployment", yew_site_docs_0_20::pages::more::deployment::page_content()),
    ("more/css", "CSS", "/docs/more/css", yew_site_docs_0_20::pages::more::css::page_content()),
    ("more/testing", "Testing apps", "/docs/more/testing", yew_site_docs_0_20::pages::more::testing::page_content()),
    ("more/roadmap", "Roadmap", "/docs/more/roadmap", yew_site_docs::pages::more::roadmap::page_content()),
]);

yew_site_lib::page_map!(
    resolve_migration,
    "",
    yew_site_docs::sidebar_data::docs_sidebar,
    [
        (
            "yew/from-0-19-0-to-0-20-0",
            "From 0.19.0 to 0.20.0",
            "/docs/migration-guides/yew/from-0-19-0-to-0-20-0",
            yew_site_docs::pages::migration_guides::yew::from_0_19_0_to_0_20_0::page_content()
        ),
        (
            "yew/from-0-20-0-to-0-21-0",
            "From 0.20.0 to 0.21.0",
            "/docs/migration-guides/yew/from-0-20-0-to-0-21-0",
            yew_site_docs::pages::migration_guides::yew::from_0_20_0_to_0_21_0::page_content()
        ),
        (
            "yew/from-0-21-0-to-0-22-0",
            "From 0.21.0 to 0.22.0",
            "/docs/migration-guides/yew/from-0-21-0-to-0-22-0",
            yew_site_docs::pages::migration_guides::yew::from_0_21_0_to_0_22_0::page_content()
        ),
        (
            "yew/from-0-22-0-to-0-23-0",
            "From 0.22.0 to 0.23.0",
            "/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
            yew_site_docs::pages::migration_guides::yew::from_0_22_0_to_0_23_0::page_content()
        ),
        (
            "yew-agent/from-0-0-0-to-0-1-0",
            "From 0.0.0 to 0.1.0",
            "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0",
            yew_site_docs::pages::migration_guides::yew_agent::from_0_0_0_to_0_1_0::page_content()
        ),
        (
            "yew-agent/from-0-1-0-to-0-2-0",
            "From 0.1.0 to 0.2.0",
            "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0",
            yew_site_docs::pages::migration_guides::yew_agent::from_0_1_0_to_0_2_0::page_content()
        ),
        (
            "yew-agent/from-0-3-0-to-0-4-0",
            "From 0.3.0 to 0.4.0",
            "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0",
            yew_site_docs::pages::migration_guides::yew_agent::from_0_3_0_to_0_4_0::page_content()
        ),
        (
            "yew-agent/from-0-4-0-to-0-5-0",
            "From 0.4.0 to 0.5.0",
            "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
            yew_site_docs::pages::migration_guides::yew_agent::from_0_4_0_to_0_5_0::page_content()
        ),
        (
            "yew-router/from-0-15-0-to-0-16-0",
            "From 0.15.0 to 0.16.0",
            "/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0",
            yew_site_docs::pages::migration_guides::yew_router::from_0_15_0_to_0_16_0::page_content(
            )
        ),
        (
            "yew-router/from-0-16-0-to-0-17-0",
            "From 0.16.0 to 0.17.0",
            "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0",
            yew_site_docs::pages::migration_guides::yew_router::from_0_16_0_to_0_17_0::page_content(
            )
        ),
        (
            "yew-router/from-0-19-0-to-0-20-0",
            "From 0.19.0 to 0.20.0",
            "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
            yew_site_docs::pages::migration_guides::yew_router::from_0_19_0_to_0_20_0::page_content(
            )
        ),
    ]
);

yew_site_lib::spa_csr_boilerplate!(
    "",
    "en",
    "",
    yew_site_docs,
    yew_site_docs_0_21,
    yew_site_docs_0_20
);

yew_site_lib::spa_ssr_render_pages!(
    "", "",
    yew_site_docs, yew_site_docs_0_21, yew_site_docs_0_20,
    tutorial_sidebar_next: vec![],
    tutorial_sidebar_stable: vec![],
    tutorial_sidebar_0_22: vec![],
    tutorial_sidebar_0_21: vec![],
    tutorial_sidebar_0_20: vec![],
    home_pages: [yew_site_home::Page, yew_site_home::PageNext, yew_site_home::PageV022, yew_site_home::PageV021, yew_site_home::PageV020]
);
