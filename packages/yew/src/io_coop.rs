//! module that provides io compatibility over browser tasks and other async io tasks (e.g.: tokio)

#[cfg(target_arch = "wasm32")]
mod io_wasm_bindgen {
    pub use wasm_bindgen_futures::spawn_local;
}

#[cfg(target_arch = "wasm32")]
pub(crate) use io_wasm_bindgen::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
mod io_tokio {
    use std::future::Future;

    // spawn_local in tokio is more powerful, but we need to adjust the function signature to match
    // wasm_bindgen_futures.
    #[inline(always)]
    pub(crate) fn spawn_local<F>(f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        tokio::task::spawn_local(f);
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
pub(crate) use io_tokio::*;
