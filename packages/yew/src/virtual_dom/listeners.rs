use std::rc::Rc;

/// The [Listener] trait is an universal implementation of an event listener
/// which is used to bind Rust-listener to JS-listener (DOM).
pub trait Listener {
    /// Returns the name of the event
    fn kind(&self) -> ListenerKind;

    /// Handles an event firing
    fn handle(&self, event: web_sys::Event);

    /// Makes the event listener passive. See
    /// [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener).
    fn passive(&self) -> bool;
}

impl std::fmt::Debug for dyn Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Listener {{ kind: {}, passive: {:?} }}",
            self.kind().as_ref(),
            self.passive(),
        )
    }
}

macro_rules! gen_listener_kinds {
    ($($kind:ident)*) => {
        /// Supported kinds of DOM event listeners
        // Using instead of strings to optimise registry collection performance by simplifying
        // hashmap hash calculation.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        pub enum ListenerKind {
            $( $kind, )*
        }

        impl ListenerKind {
            /// Get the case-sensitive string representing the event type
            pub fn event_type(&self) -> &'static str {
                match self {
                    $( ListenerKind::$kind => stringify!($kind), )*
                }
            }
        }

        impl AsRef<str> for ListenerKind {
            fn as_ref(&self) -> &str {
                self.event_type()
            }
        }
    };
}

gen_listener_kinds! {
    onabort
    onauxclick
    onblur
    oncancel
    oncanplay
    oncanplaythrough
    onchange
    onclick
    onclose
    oncontextmenu
    oncuechange
    ondblclick
    ondrag
    ondragend
    ondragenter
    ondragexit
    ondragleave
    ondragover
    ondragstart
    ondrop
    ondurationchange
    onemptied
    onended
    onerror
    onfocus
    onfocusin
    onfocusout
    onformdata
    oninput
    oninvalid
    onkeydown
    onkeypress
    onkeyup
    onload
    onloadeddata
    onloadedmetadata
    onloadstart
    onmousedown
    onmouseenter
    onmouseleave
    onmousemove
    onmouseout
    onmouseover
    onmouseup
    onpause
    onplay
    onplaying
    onprogress
    onratechange
    onreset
    onresize
    onscroll
    onsecuritypolicyviolation
    onseeked
    onseeking
    onselect
    onslotchange
    onstalled
    onsubmit
    onsuspend
    ontimeupdate
    ontoggle
    onvolumechange
    onwaiting
    onwheel
    oncopy
    oncut
    onpaste
    onanimationcancel
    onanimationend
    onanimationiteration
    onanimationstart
    ongotpointercapture
    onloadend
    onlostpointercapture
    onpointercancel
    onpointerdown
    onpointerenter
    onpointerleave
    onpointerlockchange
    onpointerlockerror
    onpointermove
    onpointerout
    onpointerover
    onpointerup
    onselectionchange
    onselectstart
    onshow
    ontouchcancel
    ontouchend
    ontouchmove
    ontouchstart
    ontransitioncancel
    ontransitionend
    ontransitionrun
    ontransitionstart
}

/// A list of event listeners
#[derive(Debug)]
pub enum Listeners {
    /// No listeners registered or pending.
    /// Distinct from `Pending` with an empty slice to avoid an allocation.
    None,

    /// Not yet added to the element or registry
    Pending(Box<[Option<Rc<dyn Listener>>]>),
}

impl PartialEq for Listeners {
    fn eq(&self, rhs: &Self) -> bool {
        use Listeners::*;

        match (self, rhs) {
            (None, None) => true,
            (Pending(lhs), Pending(rhs)) => {
                if lhs.len() != rhs.len() {
                    false
                } else {
                    use std::option::Option::None;

                    lhs.iter()
                        .zip(rhs.iter())
                        .all(|(lhs, rhs)| match (lhs, rhs) {
                            (Some(lhs), Some(rhs)) =>
                            {
                                #[allow(clippy::vtable_address_comparisons)]
                                Rc::ptr_eq(lhs, rhs)
                            }
                            (None, None) => true,
                            _ => false,
                        })
                }
            }
            (None, Pending(pending)) | (Pending(pending), None) => pending.len() == 0,
        }
    }
}

impl Clone for Listeners {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Pending(v) => Self::Pending(v.clone()),
        }
    }
}

impl Default for Listeners {
    fn default() -> Self {
        Self::None
    }
}
