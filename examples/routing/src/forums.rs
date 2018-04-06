
use yew::prelude::*;
use Context;
use yew::services::route::RouteInfo;
use yew::services::route::PathSegment;

use button::Button;


pub struct Forums {
    route: Route
}

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    CatForum,
    DogForum,
    ForumsList
}

pub enum Msg {
    Navigate(Route)
}

impl Default for Route {
    fn default() -> Self {
        Route::ForumsList
    }
}

impl From<RouteInfo> for Route {
    fn from(route_info: RouteInfo) -> Self {
       if let Some(second_segment) = route_info.path_segments.get(1).map(String::as_str) {
           match second_segment {
               "cat" => return Route::CatForum,
               "dog" => return Route::DogForum,
               _ => return Route::ForumsList
           }
       }
        Route::ForumsList
    }
}

impl Into<PathSegment> for Route {
    fn into(self) -> PathSegment {
        match self {
            Route::CatForum => "cat".into(), // TODO, I would like a try_into().unwrap() pattern here instead
            Route::DogForum => "dog".into(),
            Route::ForumsList => "".into()
        }
    }
}


#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub route: Route
}

impl Component<Context> for Forums {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Forums {
            route: props.route
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => {

                println!("Forums: Navigating");
                // This will inform the Model component that the url has changed, and will cause it to
                // update its route field, and therefore, this component's props.
                context.routing.call_link(super::Route::Forums(route));
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        println!("change() called in Forums with route");
        self.route = props.route;
        true
    }
}

impl Renderable<Context, Forums> for Forums {
    fn view(&self) -> Html<Context, Self> {
        match self.route {
            Route::CatForum => {
                html! {
                    // Conceptually, these could also be components to which routing props can be passed
                    <>
                        {"I'm the forum for talking about cats"}
                    </>
                }
            }
            Route::DogForum => {
                html! {
                    <>
                        {"I'm the forum for talking about dogs"}
                    </>
                }
            }
            Route::ForumsList => {
                html!{
                    <>
                        <div>
                            <Button: title="Dog forum", onsignal=|_| Msg::Navigate(Route::DogForum) ,/>
                        </div>
                        <div>
                            <Button: title="Cat forum", onsignal=|_| Msg::Navigate(Route::CatForum) ,/>
                        </div>
                    </>
                }
            }
        }
    }
}
