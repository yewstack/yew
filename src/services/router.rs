//! This module contains the implementation of a service for
//! a url router.

use url::Url;

use html::{Component, Env};
use stdweb::Value;
use stdweb::web::{self, IEventTarget};

/// TODO:
/// A handle which helps to cancel the router. Uses removeEventListener
pub struct RouterTask<CTX: 'static, COMP: Component<CTX>> {
    _handle1: web::EventListenerHandle,
    handle2: Value,
    history: web::History,
    route_fn: &'static Fn(RouteInfo) -> COMP::Msg,
}

/// State of the current route.
#[derive(Debug, Clone)]
pub struct RouteInfo {
    /// Url
    pub url: Url,
    /// History state
    pub state: Value,
}

impl RouteInfo {
    /// Initialize the route state using the current window.
    fn new(url: Url, state: Value) -> RouteInfo {
        RouteInfo {
            url: url,
            state: state,
        }
    }
}

fn current_url(window: &web::Window) -> Url {
    // TODO: better error messages around unwraps
    let href = window.location().unwrap().href().unwrap();
    Url::parse(&href).unwrap()
}

impl<'a, CTX: 'a, COMP: Component<CTX>> RouterTask<CTX, COMP> {
    /// Start the Routing Task in the environment.
    ///
    /// Ownership of this Task should typically be put in the `Model`.
    ///
    /// Routing will stop if this Task is dropped.
    pub fn new(
        env: &mut Env<'a, CTX, COMP>,
        route_fn: &'static Fn(RouteInfo) -> COMP::Msg,
    ) -> Self
    {
        let window = web::window();
        let callback = env.send_back(route_fn);

        let callback1 = callback.clone();
        let callback2 = callback;

        let cl_window = window.clone();
        let handle1 = window
            .add_event_listener(move |event: web::event::PopStateEvent| {
                callback1.emit(RouteInfo::new(current_url(&cl_window), event.state()));
            });

        // TODO: koute/stdweb/issues/171
        // self.handle2 = Some(self.window
        //     .add_event_listener(move |_event: web::event::ResourceLoadEvent| {
        //         callback2.emit(RouteInfo::new(Value::Null));
        //     }));

        let cl_window = window.clone();
        let rs_handle = move || {
            callback2.emit(RouteInfo::new(current_url(&cl_window), Value::Null));
        };

        let handle2 = js!{
            var callback = @{rs_handle};
            function listener() {
                callback();
            }
            window.addEventListener("load", listener);
            return {
                callback: callback,
                listener: listener
            };
        };

        RouterTask {
            _handle1: handle1,
            handle2: handle2,
            route_fn: route_fn,
            history: window.history(),
        }
    }

    /// Set the state of the history, including the url.
    ///
    /// This will _not_ trigger the router to change. If a state change is required
    /// it is the user's job to propogate the `Msg`.
    pub fn push_state(&self, state: Value, title: &str, url: Url) -> COMP::Msg {
        self.history.push_state(state.clone(), title, Some(url.as_str()));
        let info = RouteInfo {
            url: url,
            state: state,
        };
        (*self.route_fn)(info)
    }
}

impl<CTX, COMP: Component<CTX>> Drop for RouterTask<CTX, COMP> {
    fn drop(&mut self) {
        js! { @(no_return)
            var handle = @{&self.handle2};
            window.removeEventListener("load", handle.listener);
            handle.callback.drop();
        }
    }
}
