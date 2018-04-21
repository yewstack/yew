
#[macro_use]
extern crate yew;

mod forum_router;
mod button;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::route::*;

use yew::html::Renderable;

use button::Button;

use yew::services::route::Router;

use forum_router::Route as ForumRoute;


pub struct Context {
    routing: RouteService
}

struct Model {
    route: Route
}

#[derive(Clone, Debug)]
enum Route {
    Forums(ForumRoute),
    PageNotFoundRoute
}


enum Msg {
    Navigate(Route),
}

impl From<RouteResult> for Msg {
    fn from( result: RouteResult) -> Self {
        match result {
            Ok(mut route_info) => {
               Msg::Navigate(Route::from_route_main(&mut route_info))
            }
            Err(e) => {
                eprintln!("Couldn't route: '{:?}'", e);
                Msg::Navigate(Route::PageNotFoundRoute)
            }
        }
    }
}


impl Router for Route {
    // For the top level case, this _MUST_ return Some.
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        Some(Self::from_route_main(route))
    }
    fn to_route(&self) -> RouteInfo {
        match *self {
            // You can add RouteInfos together to combine paths in logical order.
            // The fragment and query of the rhs take precedence over any fragment or query set by the lhs.
            Route::Forums(ref forum_route)=> RouteInfo::parse("/forums").unwrap() + forum_route.to_route(),
            Route::PageNotFoundRoute => RouteInfo::parse("/PageNotFound").unwrap(),
        }
    }
}

impl MainRouter for Route {
    fn from_route_main(route: &mut RouteInfo) -> Self {
        if let Some(RouteSection::Node{segment}) = route.next() {
            match segment.as_str() {
                "forums" => {
                    // If the child can't be resolved, redirect to the right page here.
                    if let Some(child) = ForumRoute::from_route(route) { // Pass the route info to the child for it to figure itself out.
                        Route::Forums(child)
                    } else {
                        Route::PageNotFoundRoute
                    }
                },
                _ => Route::PageNotFoundRoute
            }
        } else {
            Route::PageNotFoundRoute
        }
    }
}


impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let callback = context.send_back(|route_result: RouteResult| {
            Msg::from(route_result)
        });
         // When the user presses the back or forward button, an event will file and cause the callback to fire
        context.routing.register_router(callback);


        let route: Route = Route::from_route_main(&mut context.routing.get_current_route_info());
        context.routing.replace_url(route.clone()); // sets the url to be dependent on what the route_info was resolved to

        Model {
            route
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => {
                println!("Main route: Navigating");
                context.routing.set_route(route.clone());
                self.route = route;
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                {"This could be some html that will be on every page, like a header."}
                <Button: title="GoToForums", onsignal=|_| Msg::Navigate(Route::Forums(ForumRoute::ForumsList) ) ,/>
                <div>
                    {self.route.view()}
                </div>
            </div>
        }
    }
}


impl Renderable<Context, Model> for Route {
    fn view(&self) -> Html<Context, Model> {
        match *self {
            Route::Forums(ref forum_route) => {
                html! {
                    <>
                        {forum_route.view()}
                    </>
                }
            }
            Route::PageNotFoundRoute => {
                html! {
                    <div>
                        {"Page not found"}
                    </div>
                }
            }
        }
    }
}


fn main() {
    yew::initialize();
    let context = Context {
        routing: RouteService::new()
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}