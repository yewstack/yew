use std::rc::Rc;

use crate::html::ImplicitClone;

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
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        pub enum ListenerKind {
            $( $kind, )*
            other(std::borrow::Cow<'static, str>),
        }

        impl ListenerKind {
            pub fn type_name(&self) -> std::borrow::Cow<'static, str> {
                match self {
                    Self::other(type_name) => type_name.clone(),
                    $( Self::$kind => stringify!($kind)[2..].into(), )*
                }
            }
        }

        impl AsRef<str> for ListenerKind {
            fn as_ref(&self) -> &str {
                match self {
                    $( Self::$kind => stringify!($kind), )*
                    Self::other(type_name) => type_name.as_ref(),
                }
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

impl ImplicitClone for Listeners {}

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
                            (Some(lhs), Some(rhs)) => {
                                // We are okay with comparisons from different compilation units to
                                // result in false not-equal results. This should only lead in the
                                // worst-case to some unneeded re-renders.
                                #[allow(ambiguous_wide_pointer_comparisons)]
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
