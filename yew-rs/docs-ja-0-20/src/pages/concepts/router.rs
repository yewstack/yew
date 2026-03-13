crate::doc_page!(
    "Router",
    "/ja/docs/concepts/router",
    Content::new(vec![
        p(vec![text(
            "Routers in Single Page Applications (SPA) handle displaying different pages \
             depending on what the URL is. Instead of the default behavior of requesting a \
             different remote resource when a link is clicked, the router instead sets the URL \
             locally to point to a valid route in your application. The router then detects this \
             change and then decides what to render."
        ),]),
        p(vec![
            text("Yew provides router support in the "),
            code("yew-router"),
            text(" crate. To start using it, add the dependency to your "),
            code("Cargo.toml"),
        ]),
        code_block("toml", r#"yew-router = "0.17""#),
        p(vec![
            text("The utilities needed are provided under "),
            code("yew_router::prelude"),
            text(","),
        ]),
        h2(vec![text("Usage")]),
        p(vec![
            text("You start by defining a "),
            code("Route"),
            text("."),
        ]),
        p(vec![
            text("Routes are defined as an "),
            code("enum"),
            text(" which derives "),
            code("Routable"),
            text(". This enum must be "),
            code("Clone + PartialEq"),
            text("."),
        ]),
        code_block(
            "rust",
            r##"use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}"##
        ),
        p(vec![
            text("A "),
            code("Route"),
            text(" is paired with a "),
            code("<Switch />"),
            text(
                " component, which finds the variant whose path matches the browser's current URL \
                 and passes it to the "
            ),
            code("render"),
            text(
                " callback. The callback then decides what to render. In case no path is matched, \
                 the router navigates to the path with "
            ),
            code("not_found"),
            text(
                " attribute. If no route is specified, nothing is rendered, and a message is \
                 logged to console stating that no route was matched."
            ),
        ]),
        p(vec![
            text("Most of yew-router's components, in particular "),
            code("<Link />"),
            text(" and "),
            code("<Switch />"),
            text(", must be (grand-)children of one of the Router components (e.g. "),
            code("<BrowserRouter />"),
            text(
                "). You usually only need a single Router in your app, most often rendered \
                 immediately by your most top-level "
            ),
            code("<App />"),
            text(
                " component. The Router registers a context, which is needed for Links and \
                 Switches to function. An example is shown below."
            ),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("When using "),
                code("yew-router"),
                text(" in browser environment, "),
                code("<BrowserRouter />"),
                text(" is highly recommended. You can find other router flavours in the "),
                link("https://docs.rs/yew-router/", vec![text("API Reference")]),
                text("."),
            ]),]
        ),
        code_block(
            "rust",
            r##"use yew_router::prelude::*;
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

#[function_component(Secure)]
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

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}"##
        ),
        h3(vec![text("Path Segments")]),
        p(vec![
            text(
                "It is also possible to extract information from a route using dynamic and named \
                 wildcard segments. You can then access the post's id inside "
            ),
            code("<Switch />"),
            text(" and forward it to the appropriate component via properties."),
        ]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
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
}"##
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("You can have a normal "),
                code("Post"),
                text(" variant instead of "),
                code("Post {id: String}"),
                text(" too. For example when "),
                code("Post"),
                text(
                    " is rendered with another router, the field can then be redundant as the \
                     other router is able to match and handle the path. See the "
                ),
                link("#nested-router", vec![text("Nested Router")]),
                text(" section below for details"),
            ]),]
        ),
        p(vec![
            text("Note the fields must implement "),
            code("Clone + PartialEq"),
            text(" as part of the "),
            code("Route"),
            text(" enum. They must also implement "),
            code("std::fmt::Display"),
            text(" and "),
            code("std::str::FromStr"),
            text(
                " for serialization and deserialization. Primitive types like integer, float, and \
                 String already satisfy the requirements."
            ),
        ]),
        p(vec![
            text(
                "In case when the form of the path matches, but the deserialization fails (as per "
            ),
            code("FromStr"),
            text(
                "). The router will consider the route as unmatched and try to render the not \
                 found route (or a blank page if the not found route is unspecified)."
            ),
        ]),
        p(vec![text("Consider this example:")]),
        code_block_ignore(
            "rust",
            r##"#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/news/:id")]
    News { id: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
// switch function renders News and id as is. Omitted here."##
        ),
        p(vec![
            text("When the segment goes over 255, "),
            code("u8::from_str()"),
            text(" fails with "),
            code("ParseIntError"),
            text(", the router will then consider the route unmatched."),
        ]),
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "router deserialization failure behavior"
        ),
        p(vec![
            text(
                "For more information about the route syntax and how to bind parameters, check \
                 out "
            ),
            link(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                vec![text("route-recognizer")]
            ),
            text("."),
        ]),
        h3(vec![text("Location")]),
        p(vec![
            text("The router provides a universal "),
            code("Location"),
            text(
                " struct via context which can be used to access routing information. They can be \
                 retrieved by hooks or convenient functions on "
            ),
            code("ctx.link()"),
            text("."),
        ]),
        h3(vec![text("Navigation")]),
        p(vec![
            code("yew_router"),
            text(" provides a handful of tools to work with navigation."),
        ]),
        h4(vec![text("Link")]),
        p(vec![
            text("A "),
            code("<Link />"),
            text(" renders as an "),
            code("<a>"),
            text(" element, the "),
            code("onclick"),
            text(" event handler will call "),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault",
                vec![text("preventDefault")]
            ),
            text(
                ", and push the targeted page to the history and render the desired page, which \
                 is what should be expected from a Single Page App. The default onclick of a \
                 normal anchor element would reload the page."
            ),
        ]),
        p(vec![
            text("The "),
            code("<Link />"),
            text(" component also passes its children to the "),
            code("<a>"),
            text(" element. Consider it a replacement of "),
            code("<a/>"),
            text(" for in-app routes. Except you supply a "),
            code("to"),
            text(" attribute instead of a "),
            code("href"),
            text(". An example usage:"),
        ]),
        code_block_ignore(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#
        ),
        p(vec![text("Struct variants work as expected too:")]),
        code_block_ignore(
            "rust",
            r###"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew v0.19 out now!" }</Link<Route>>"###
        ),
        h4(vec![text("Navigator API")]),
        p(vec![
            text(
                "Navigator API is provided for both function components and struct components. \
                 They enable callbacks to change the route. An "
            ),
            code("Navigator"),
            text(" instance can be obtained in either cases to manipulate the route."),
        ]),
        h5(vec![text("Function Components")]),
        p(vec![
            text("For function components, the "),
            code("use_navigator"),
            text(
                " hook re-renders the component when the underlying navigator provider changes. \
                 Here's how to implement a button that navigates to the "
            ),
            code("Home"),
            text(" route when clicked."),
        ]),
        code_block_ignore(
            "rust",
            r##"#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <button {onclick}>{"Click to go home"}</button>
        </>
    }
}"##
        ),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text("The example here uses "),
                code("Callback::from"),
                text(
                    ". Use a normal callback if the target route can be the same with the route \
                     the component is in, or just to play safe. For example, when you have a logo \
                     button on every page, that goes back to home when clicked, clicking that \
                     button twice on home page causes the code to panic because the second click \
                     pushes an identical Home route and the "
                ),
                code("use_navigator"),
                text(" hook won't trigger a re-render."),
            ]),]
        ),
        p(vec![
            text(
                "If you want to replace the current location instead of pushing a new location \
                 onto the stack, use "
            ),
            code("navigator.replace()"),
            text(" instead of "),
            code("navigator.push()"),
            text("."),
        ]),
        p(vec![
            text("You may notice "),
            code("navigator"),
            text(
                " has to move into the callback, so it can't be used again for other callbacks. \
                 Luckily "
            ),
            code("navigator"),
            text(" implements "),
            code("Clone"),
            text(", here's for example how to have multiple buttons to different routes:"),
        ]),
        code_block_ignore(
            "rust",
            r##"use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavItems)]
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
}"##
        ),
        h5(vec![text("Struct Components")]),
        p(vec![
            text("For struct components, the "),
            code("Navigator"),
            text(" instance can be obtained through the "),
            code("ctx.link().navigator()"),
            text(
                " API. The rest is identical with the function component case. Here's an example \
                 of a view function that renders a single button."
            ),
        ]),
        code_block_ignore(
            "rust",
            r#"fn view(&self, ctx: &Context<Self>) -> Html {
    let navigator = ctx.link().navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html!{
        <button {onclick}>{"Go Home"}</button>
    }
}"#
        ),
        h4(vec![text("Redirect")]),
        p(vec![
            code("yew-router"),
            text(" also provides a "),
            code("<Redirect />"),
            text(
                " component in the prelude. It can be used to achieve similar effects as the \
                 navigator API. The component accepts a "
            ),
            code("to"),
            text(" attribute as the target route. When a "),
            code("<Redirect/>"),
            text(
                " is rendered users will be redirect to the route specified in props. Here is an \
                 example:"
            ),
        ]),
        code_block_ignore(
            "rust",
            r##"#[function_component(SomePage)]
fn some_page() -> Html {
    // made-up hook `use_user`
    let user = match use_user() {
        Some(user) => user,
        // Redirects to the login page when user is `None`.
        None => return html! {
            <Redirect<Route> to={Route::Login}/>
        },
    };
    // ... actual page content.
}"##
        ),
        admonition(
            AdmonitionType::Tip,
            Some("Redirect vs Navigator, which to use"),
            vec![p(vec![
                text("The Navigator API is the only way to manipulate route in callbacks. While "),
                code("<Redirect />"),
                text(" can be used as return values in a component. You might also want to use "),
                code("<Redirect />"),
                text(" in other non-component context, for example in the switch function of a "),
                link("#nested-router", vec![text("Nested Router")]),
                text("."),
            ]),]
        ),
        h3(vec![text("Listening to Changes")]),
        h4(vec![text("Function Components")]),
        p(vec![
            text("You can use "),
            code("use_location"),
            text(" and "),
            code("use_route"),
            text(" hooks. Your components will re-render when provided values change."),
        ]),
        h4(vec![text("Struct Components")]),
        p(vec![
            text("In order to react on route changes, you can pass a callback closure to the "),
            code("add_location_listener()"),
            text(" method of "),
            code("ctx.link()"),
            text("."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![text(
                "The location listener will get unregistered once it is dropped. Make sure to \
                 store the handle inside your component state."
            ),]),]
        ),
        code_block_ignore(
            "rust",
            r#"fn create(ctx: &Context<Self>) -> Self {
    let listener = ctx.link()
        .add_location_listener(ctx.link().callback(
            // handle event
        ))
        .unwrap();
    MyComponent {
        _listener: listener
    }
}"#
        ),
        p(vec![
            code("ctx.link().location()"),
            text(" and "),
            code("ctx.link().route::<R>()"),
            text(" can also be used to retrieve the location and the route once."),
        ]),
        h3(vec![text("Query Parameters")]),
        h4(vec![text("Specifying query parameters when navigating")]),
        p(vec![
            text(
                "In order to specify query parameters when navigating to a new route, use either "
            ),
            code("navigator.push_with_query"),
            text(" or the "),
            code("navigator.replace_with_query"),
            text(" functions. It uses "),
            code("serde"),
            text(
                " to serialize the parameters into query string for the URL so any type that \
                 implements "
            ),
            code("Serialize"),
            text(" can be passed. In its simplest form this is just a "),
            code("HashMap"),
            text(" containing string pairs."),
        ]),
        h4(vec![text("Obtaining query parameters for current route")]),
        p(vec![
            code("location.query"),
            text(" is used to obtain the query parameters. It uses "),
            code("serde"),
            text(" to deserialize the parameters from query string in the URL."),
        ]),
        h2_id("nested-router", vec![text("Nested Router")]),
        p(vec![text(
            "Nested router can be useful when the app grows larger. Consider the following router \
             structure:"
        )]),
        themed_img(
            "/img/nested-router-light.svg",
            "/img/nested-router-dark.svg",
            "nested router structure"
        ),
        p(vec![
            text("The nested "),
            code("SettingsRouter"),
            text(" handles all urls that start with "),
            code("/settings"),
            text(". Additionally, it redirects urls that are not matched to the main "),
            code("NotFound"),
            text(" route. So "),
            code("/settings/gibberish"),
            text(" will redirect to "),
            code("/404"),
            text("."),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![text(
                "Though note that this is still work in progress so the way we do this is not \
                 final"
            )]),]
        ),
        p(vec![text("It can be implemented with the following code:")]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}"##
        ),
        h3(vec![text("Basename")]),
        p(vec![
            text("It's possible to define a basename with "),
            code("yew-router"),
            text(". A basename is a common prefix of all routes. Both the Navigator API and "),
            code("<Switch />"),
            text(
                " component respect basename setting. All pushed routes will be prefixed with the \
                 basename and all switches will strip the basename before trying to parse the \
                 path into a "
            ),
            code("Routable"),
            text("."),
        ]),
        p(vec![
            text(
                "If a basename prop is not supplied to the Router component, it will use the href \
                 attribute of the "
            ),
            code("<base />"),
            text(" element in your html file and fallback to "),
            code("/"),
            text(" if no "),
            code("<base />"),
            text(" presents in the html file."),
        ]),
        h2(vec![text("Relevant examples")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/router",
            vec![text("Router")]
        )]),]),
        h2(vec![text("API Reference")]),
        ul(vec![li(vec![link(
            "https://docs.rs/yew-router/",
            vec![text("yew-router")]
        )]),]),
    ])
);
