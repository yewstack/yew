use std::collections::HashMap;
use yew::prelude::*;
use yew_functional::*;
use yew_router::prelude::*;
use yew_router::{RenderFn, Routable};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/no/:id")]
    No { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

/*
impl Routes {
    const HOME: &'static str = "/";
    const NO: &'static str = "/no/:id";
    const NOT_FOUND: &'static str = "/404";
}

impl Routable for Routes {
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self> {
        match path {
            // `at` attribute will be used
            Self::HOME => Some(Self::Home),
            Self::NOT_FOUND => Some(Self::NotFound),
            Self::NO => Some(Self::No(params.get("id")?.parse().ok()?)),
            _ => None,
        }
    }

    fn to_route(&self) -> String {
        match self {
            Routes::Home => "/".to_string(),
            Routes::No(id) => format!("/no/{}", id),
            Routes::NotFound => "/404".to_string(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
*/

#[function_component(Home)]
fn home() -> Html {
    let onclick2 = Callback::from(|_| {
        weblog::console_log!(format!("{:#?}", RouterService::query()));
    });

    html! {
        <>
            <h1>{"Hello World"}</h1>
            <Link<Routes> route=Routes::No { id: 2 }>{"Click me no"}</Link<Routes>>
            <br />
            <button onclick=onclick2>{"Click me query"}</button>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct NoProps {
    id: u32,
}

#[function_component(No)]
fn no(props: &NoProps) -> Html {
    let onclick2 = Callback::from(|_| {
        RouterService::push(Routes::No { id: 5 }, {
            let mut map = HashMap::new();
            map.insert("fuck", "yes".to_string());
            Some(map)
        })
    });

    html! {
        <>
            <h1>{ {props.id} }</h1>
            <button onclick=onclick2>{"Click me home"}</button>
        </>
    }
}

#[function_component(Main)]
fn app() -> Html {
    let switch_dyn: RenderFn<Routes> = RenderFn::new(switch);

    html! {
        <Router<Routes> not_found_route="/404" render=switch_dyn>
        </Router<Routes>>
    }
}

fn main() {
    yew::start_app::<Main>()
}

fn switch(routes: Routes) -> Html {
    weblog::console_log!("macthed", format!("{:?}", routes));
    match routes {
        Routes::Home => html! { <Home /> },
        Routes::No { id } => html! { <No id=id /> },
        Routes::NotFound => html! {<h1>{"404"}</h1>},
    }
}
