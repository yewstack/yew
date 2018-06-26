//! Service to handle routing.

use stdweb::web::History;
use stdweb::web::Location;
use stdweb::web::window;
use stdweb::Value;
use stdweb::web::EventListenerHandle;
use stdweb::web::event::PopStateEvent;
use stdweb::web::IEventTarget;
use stdweb::JsSerialize;
use stdweb::unstable::TryFrom;
use yew::callback::Callback;

use std::marker::PhantomData;


/// A service that facilitates manipulation of the browser's URL bar and responding to browser
/// 'forward' and 'back' events.
///
/// The `T` determines what route state can be stored in the route service.
pub struct RouteService<T> {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    phantom_data: PhantomData<T>
}


impl <T> RouteService<T>
    where T: JsSerialize + Clone + TryFrom<Value> + 'static
{
    /// Creates the route service.
    pub fn new() -> RouteService<T> {
        let location = window().location().expect("browser does not support location API");
        RouteService {
            history: window().history(),
            location,
            event_listener: None,
            phantom_data: PhantomData
        }
    }

    /// Registers a callback to the route service.
    /// Callbacks will be called when the History API experiences a change such as
    /// popping a state off of its stack when the forward or back buttons are pressed.
    pub fn register_callback(&mut self, callback: Callback<(String, T)>) {
        self.event_listener = Some(window().add_event_listener(
            move |event: PopStateEvent| {
                let state_value: Value = event.state();

                if let Ok(state) = T::try_from(state_value) {
                    let location: Location = window().location().unwrap();
                    let route: String = Self::get_route_from_location(&location);

                    callback.emit((route.clone(), state.clone()))
                } else {
                    eprintln!("Nothing farther back in history, not calling routing callback.");
                }
            },
        ));
    }


    /// Sets the browser's url bar to contain the provided route,
    /// and creates a history entry that can be navigated via the forward and back buttons.
    /// The route should be a relative path that starts with a '/'.
    /// A state object be stored with the url.
    pub fn set_route(&mut self, route: &str, state: T) {

        self.history.push_state(
            state,
            "",
            Some(route),
        );
    }

    fn get_route_from_location(location: &Location) -> String {
        let path = location.pathname().unwrap();
        let query = location.search().unwrap();
        let fragment = location.hash().unwrap();
        format!("{path}{query}{fragment}",
            path=path,
            query=query,
            fragment=fragment)
    }

    /// Gets the concatenated path, query, and fragment strings
    pub fn get_route(&self) -> String {
        Self::get_route_from_location(&self.location)
    }

    /// Gets the path name of the current url.
    pub fn get_path(&self) -> String {
        self.location.pathname().unwrap()
    }

    /// Gets the query string of the current url.
    pub fn get_query(&self) -> String {
        self.location.search().unwrap()
    }

    /// Gets the fragment of the current url.
    pub fn get_fragment(&self) -> String {
        self.location.hash().unwrap()
    }
}