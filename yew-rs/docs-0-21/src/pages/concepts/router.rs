crate::doc_page!(
    "Router",
    "/docs/concepts/router",
    Content::new(vec![
        p![
            "Routers in Single Page Applications (SPA) handle displaying different pages \
             depending on what the URL is. Instead of the default behavior of requesting a \
             different remote resource when a link is clicked, the router instead sets the URL \
             locally to point to a valid route in your application. The router then detects this \
             change and then decides what to render.",
        ],
        p![
            "Yew provides router support in the ",
            code("yew-router"),
            " crate. To start using it, add the dependency to your ",
            code("Cargo.toml"),
        ],
        code_block(
            "toml",
            r#"yew-router = { git = "https://github.com/yewstack/yew.git" }"#
        ),
        p![
            "The utilities needed are provided under ",
            code("yew_router::prelude"),
            ",",
        ],
        h2!["Usage"],
        p!["You start by defining a ", code("Route"), ".",],
        p![
            "Routes are defined as an ",
            code("enum"),
            " which derives ",
            code("Routable"),
            ". This enum must be ",
            code("Clone + PartialEq"),
            ".",
        ],
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
        p![
            "A ",
            code("Route"),
            " is paired with a ",
            code("<Switch />"),
            " component, which finds the variant whose path matches the browser's current URL and \
             passes it to the ",
            code("render"),
            " callback. The callback then decides what to render. In case no path is matched, the \
             router navigates to the path with ",
            code("not_found"),
            " attribute. If no route is specified, nothing is rendered, and a message is logged \
             to the console stating that no route was matched.",
        ],
        p![
            "Most of yew-router's components, in particular ",
            code("<Link />"),
            " and ",
            code("<Switch />"),
            ", must be (grand-)children of one of the Router components (e.g. ",
            code("<BrowserRouter />"),
            "). You usually only need a single Router in your app, most often rendered \
             immediately by your most top-level ",
            code("<App />"),
            " component. The Router registers a context, which is needed for Links and Switches \
             to function. An example is shown below.",
        ],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "When using ",
                code("yew-router"),
                " in a browser environment, ",
                code("<BrowserRouter />"),
                " is highly recommended. You can find other router flavors in the ",
                link!("https://docs.rs/yew-router/", "API Reference"),
                ".",
            ]
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
        h3!["Path Segments"],
        p![
            "It is also possible to extract information from a route using dynamic and named \
             wildcard segments. You can then access the post's id inside ",
            code("<Switch />"),
            " and forward it to the appropriate component via properties.",
        ],
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
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "You can have a normal ",
                code("Post"),
                " variant instead of ",
                code("Post {id: String}"),
                " too. For example, when ",
                code("Post"),
                " is rendered with another router, the field can then be redundant as the other \
                 router can match and handle the path. See the ",
                link!("#nested-router", "Nested Router"),
                " section below for details",
            ]
        ),
        p![
            "Note the fields must implement ",
            code("Clone + PartialEq"),
            " as part of the ",
            code("Route"),
            " enum. They must also implement ",
            code("std::fmt::Display"),
            " and ",
            code("std::str::FromStr"),
            " for serialization and deserialization. Primitive types like integer, float, and \
             String already satisfy the requirements.",
        ],
        p![
            "In case when the form of the path matches, but the deserialization fails (as per ",
            code("FromStr"),
            "). The router will consider the route as unmatched and try to render the not found \
             route (or a blank page if the not found route is unspecified).",
        ],
        p!["Consider this example:"],
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
        p![
            "When the segment goes over 255, ",
            code("u8::from_str()"),
            " fails with ",
            code("ParseIntError"),
            ", the router will then consider the route unmatched.",
        ],
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "router deserialization failure behavior"
        ),
        p![
            "For more information about the route syntax and how to bind parameters, check out ",
            link!(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                "route-recognizer"
            ),
            ".",
        ],
        h3!["Location"],
        p![
            "The router provides a universal ",
            code("Location"),
            " struct via context which can be used to access routing information. They can be \
             retrieved by hooks or convenient functions on ",
            code("ctx.link()"),
            ".",
        ],
        h3!["Navigation"],
        p![
            code("yew_router"),
            " provides a handful of tools to work with navigation.",
        ],
        h4!["Link"],
        p![
            "A ",
            code("<Link />"),
            " renders as an ",
            code("<a>"),
            " element, the ",
            code("onclick"),
            " event handler will call ",
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault",
                "preventDefault"
            ),
            ", and push the targeted page to the history and render the desired page, which is \
             what should be expected from a Single Page App. The default ",
            code("onclick"),
            " of a normal anchor element would reload the page.",
        ],
        p![
            "The ",
            code("<Link />"),
            " component also passes its children to the ",
            code("<a>"),
            " element. Consider it a replacement of ",
            code("<a/>"),
            " for in-app routes. Except you supply a ",
            code("to"),
            " attribute instead of a ",
            code("href"),
            ". An example usage:",
        ],
        code_block_ignore(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#
        ),
        p!["Struct variants work as expected too:"],
        code_block_ignore(
            "rust",
            r#"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew v0.19 out now!" }</Link<Route>>"#
        ),
        h4!["Navigator API"],
        p![
            "Navigator API is provided for both function components and struct components. They \
             enable callbacks to change the route. A ",
            code("Navigator"),
            " instance can be obtained in either case to manipulate the route.",
        ],
        h5!["Function Components"],
        p![
            "For function components, the ",
            code("use_navigator"),
            " hook re-renders the component when the underlying navigator provider changes. Here \
             is how to implement a button that navigates to the ",
            code("Home"),
            " route when clicked.",
        ],
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
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "The example here uses ",
                code("Callback::from"),
                ". Use a normal callback if the target route can be the same as the route the \
                 component is in, or just to play safe. For example, consider a logo button on \
                 every page that goes back to the home page when clicked. Clicking that button \
                 twice on the home page causes the code to panic because the second click pushes \
                 an identical Home route and the ",
                code("use_navigator"),
                " hook will not trigger a re-render.",
            ]
        ),
        p![
            "If you want to replace the current location instead of pushing a new location onto \
             the stack, use ",
            code("navigator.replace()"),
            " instead of ",
            code("navigator.push()"),
            ".",
        ],
        p![
            "You may notice ",
            code("navigator"),
            " has to move into the callback, so it cannot be used again for other callbacks. \
             Luckily ",
            code("navigator"),
            " implements ",
            code("Clone"),
            ", here is for example how to have multiple buttons for different routes:",
        ],
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
        h5!["Struct Components"],
        p![
            "For struct components, the ",
            code("Navigator"),
            " instance can be obtained through the ",
            code("ctx.link().navigator()"),
            " API. The rest is identical to the function component case. Here is an example of a \
             view function that renders a single button.",
        ],
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
        h4!["Redirect"],
        p![
            code("yew-router"),
            " also provides a ",
            code("<Redirect />"),
            " component in the prelude. It can be used to achieve similar effects as the \
             navigator API. The component accepts a ",
            code("to"),
            " attribute as the target route. When a ",
            code("<Redirect/>"),
            " is rendered users will be redirected to the route specified in props. Here is an \
             example:",
        ],
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
        admonition!(
            AdmonitionType::Tip,
            Some("Redirect vs Navigator, which to use"),
            p![
                "The Navigator API is the only way to manipulate route in callbacks. While ",
                code("<Redirect />"),
                " can be used as return values in a component. You might also want to use ",
                code("<Redirect />"),
                " in another non-component context, for example in the switch function of a ",
                link!("#nested-router", "Nested Router"),
                ".",
            ]
        ),
        h3!["Listening to Changes"],
        h4!["Function Components"],
        p![
            "You can use ",
            code("use_location"),
            " and ",
            code("use_route"),
            " hooks. Your components will re-render when provided values change.",
        ],
        h4!["Struct Components"],
        p![
            "In order to react on route changes, you can pass a callback closure to the ",
            code("add_location_listener()"),
            " method of ",
            code("ctx.link()"),
            ".",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "The location listener will get unregistered once it is dropped. Make sure to \
                 store the handle inside your component state.",
            ]
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
        p![
            code("ctx.link().location()"),
            " and ",
            code("ctx.link().route::<R>()"),
            " can also be used to retrieve the location and the route once.",
        ],
        h3!["Query Parameters"],
        h4!["Specifying query parameters when navigating"],
        p![
            "In order to specify query parameters when navigating to a new route, use either ",
            code("navigator.push_with_query"),
            " or the ",
            code("navigator.replace_with_query"),
            " functions. It uses ",
            code("serde"),
            " to serialize the parameters into a query string for the URL so any type that \
             implements ",
            code("Serialize"),
            " can be passed. In its simplest form, this is just a ",
            code("HashMap"),
            " containing string pairs.",
        ],
        h4!["Obtaining query parameters for the current route"],
        p![
            code("location.query"),
            " is used to obtain the query parameters. It uses ",
            code("serde"),
            " to deserialize the parameters from the query string in the URL.",
        ],
        h2_id!("nested-router", "Nested Router"),
        p![
            "Nested router can be useful when the app grows larger. Consider the following router \
             structure:"
        ],
        themed_img(
            "/img/nested-router-light.svg",
            "/img/nested-router-dark.svg",
            "nested router structure"
        ),
        p![
            "The nested ",
            code("SettingsRouter"),
            " handles all URLs that start with ",
            code("/settings"),
            ". Additionally, it redirects URLs that are not matched to the main ",
            code("NotFound"),
            " route. So ",
            code("/settings/gibberish"),
            " will redirect to ",
            code("/404"),
            ".",
        ],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "Though note that this is still a work in progress so the way we do this is not \
                 final"
            ]
        ),
        p!["It can be implemented with the following code:"],
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
        h3!["Basename"],
        p![
            "It's possible to define a basename with ",
            code("yew-router"),
            ". A basename is a common prefix of all routes. Both the Navigator API and ",
            code("<Switch />"),
            " component respect basename setting. All pushed routes will be prefixed with the \
             basename and all switches will strip the basename before trying to parse the path \
             into a ",
            code("Routable"),
            ".",
        ],
        p![
            "If a basename prop is not supplied to the Router component, it will use the href \
             attribute of the ",
            code("<base />"),
            " element in your HTML file and fallback to ",
            code("/"),
            " if no ",
            code("<base />"),
            " is present in the HTML file.",
        ],
        h2!["Relevant examples"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/router",
            "Router"
        )],],
        h2!["API Reference"],
        ul![li![link!("https://docs.rs/yew-router/", "yew-router")],],
    ])
    .with_description("Yew's official router")
);
