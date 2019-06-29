pub mod agent;
pub mod app;
pub mod callback;
pub mod html;
pub mod scheduler;
pub mod virtual_dom;

/// The module that contains all events available in the framework.
pub mod events {
    pub use crate::html::{ChangeData, InputData};

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
    pub use crate::agent::{Bridge, Bridged, Threaded};
    pub use crate::app::App;
    pub use crate::callback::Callback;
    pub use crate::events::*;
    pub use crate::html::{Component, ComponentLink, Href, Html, Renderable, ShouldRender};

    /// Prelude module for creating worker.
    pub mod worker {
        pub use crate::agent::{
            Agent, AgentLink, Bridge, Bridged, Context, Global, HandlerId, Job, Private, Public,
            Transferable,
        };
    }
}

pub use self::prelude::*;
