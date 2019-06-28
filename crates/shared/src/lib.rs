#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate anymap;
extern crate bincode;
extern crate slab;
#[macro_use]
extern crate stdweb;

pub mod agent;
pub mod app;
pub mod callback;
pub mod html;
pub mod scheduler;
pub mod virtual_dom;

/// The module that contains all events available in the framework.
pub mod events {
    pub use html::{ChangeData, InputData};

    pub use stdweb::web::event::{
        BlurEvent, ClickEvent, ContextMenuEvent, DoubleClickEvent, DragDropEvent, DragEndEvent,
        DragEnterEvent, DragEvent, DragExitEvent, DragLeaveEvent, DragOverEvent, DragStartEvent,
        FocusEvent, GotPointerCaptureEvent, IKeyboardEvent, IMouseEvent, IPointerEvent,
        KeyDownEvent, KeyPressEvent, KeyUpEvent, LostPointerCaptureEvent, MouseDownEvent,
        MouseEnterEvent, MouseLeaveEvent, MouseMoveEvent, MouseOutEvent, MouseOverEvent,
        MouseUpEvent, MouseWheelEvent, PointerCancelEvent, PointerDownEvent, PointerEnterEvent,
        PointerLeaveEvent, PointerMoveEvent, PointerOutEvent, PointerOverEvent, PointerUpEvent,
        ScrollEvent, SubmitEvent,
    };
}

pub mod prelude {
    pub use html::{Component, ComponentLink, Href, Html, Renderable, ShouldRender};

    pub use app::App;

    pub use callback::Callback;

    pub use agent::{Bridge, Bridged, Threaded};

    pub use events::*;

    /// Prelude module for creating worker.
    pub mod worker {
        pub use agent::{
            Agent, AgentLink, Bridge, Bridged, Context, Global, HandlerId, Job, Private, Public,
            Transferable,
        };
    }
}

pub use self::prelude::*;
