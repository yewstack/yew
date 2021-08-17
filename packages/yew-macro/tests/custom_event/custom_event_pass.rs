// Inside a module as public custom events can be used outside the module they are defined in.
mod events {
    use yew::{custom_event, web_sys::Event};

    // NewType with matching event type and ident - i.e. "custard" can be custard
    #[custom_event(custard)]
    pub struct CustardEvent(Event);

    #[custom_event(pbj = "peanut butter jelly")]
    pub struct PeanutButterJellyEvent(Event);
}

use yew::custom_event;

// Expand on a custom event
#[custom_event(dessert)]
struct DessertEvent(events::CustardEvent);

fn main() {
    use events::*;
    use yew::StaticEvent;
    assert_eq!("custard", custard::event_name());
    assert_eq!("custard", CustardEvent::event_name());

    assert_eq!("peanut butter jelly", pbj::event_name());
    assert_eq!("peanut butter jelly", PeanutButterJellyEvent::event_name());

    assert_eq!("dessert", dessert::event_name());
    assert_eq!("dessert", DessertEvent::event_name());
}
