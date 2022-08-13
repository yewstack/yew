use std::cell::Cell;
use std::panic::PanicInfo;
use std::rc::Rc;

use web_sys::Element;

use crate::app_handle::AppHandle;
use crate::html::BaseComponent;

thread_local! {
    static PANIC_HOOK_IS_SET: Cell<bool> = Cell::new(false);
}

/// Set a custom panic hook.
/// Unless a panic hook is set through this function, Yew will
/// overwrite any existing panic hook when an application is rendered with [Renderer].
#[cfg(feature = "csr")]
pub fn set_custom_panic_hook(hook: Box<dyn Fn(&PanicInfo<'_>) + Sync + Send + 'static>) {
    std::panic::set_hook(hook);
    PANIC_HOOK_IS_SET.with(|hook_is_set| hook_is_set.set(true));
}

fn set_default_panic_hook() {
    if !PANIC_HOOK_IS_SET.with(|hook_is_set| hook_is_set.replace(true)) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}

/// The Yew Renderer.
///
/// This is the main entry point of a Yew application.
#[cfg(feature = "csr")]
#[derive(Debug)]
#[must_use = "Renderer does nothing unless render() is called."]
pub struct Renderer<COMP>
where
    COMP: BaseComponent + 'static,
{
    root: Element,
    props: COMP::Properties,
}

impl<COMP> Default for Renderer<COMP>
where
    COMP: BaseComponent + 'static,
    COMP::Properties: Default,
{
    fn default() -> Self {
        Self::with_props(Default::default())
    }
}

impl<COMP> Renderer<COMP>
where
    COMP: BaseComponent + 'static,
    COMP::Properties: Default,
{
    /// Creates a [Renderer] that renders into the document body with default properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [Renderer] that renders into a custom root with default properties.
    pub fn with_root(root: Element) -> Self {
        Self::with_root_and_props(root, Default::default())
    }
}

impl<COMP> Renderer<COMP>
where
    COMP: BaseComponent + 'static,
{
    /// Creates a [Renderer] that renders into the document body with custom properties.
    pub fn with_props(props: COMP::Properties) -> Self {
        Self::with_root_and_props(
            gloo::utils::document()
                .body()
                .expect("no body node found")
                .into(),
            props,
        )
    }

    /// Creates a [Renderer] that renders into a custom root with custom properties.
    pub fn with_root_and_props(root: Element, props: COMP::Properties) -> Self {
        Self { root, props }
    }

    /// Renders the application.
    pub fn render(self) -> AppHandle<COMP> {
        set_default_panic_hook();
        AppHandle::<COMP>::mount_with_props(self.root, Rc::new(self.props))
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    impl<COMP> Renderer<COMP>
    where
        COMP: BaseComponent + 'static,
    {
        /// Hydrates the application.
        pub fn hydrate(self) -> AppHandle<COMP> {
            set_default_panic_hook();
            AppHandle::<COMP>::hydrate_with_props(self.root, Rc::new(self.props))
        }
    }
}
