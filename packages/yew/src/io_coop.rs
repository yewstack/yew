//! module that provides io compatibility over browser tasks and other async io tasks (e.g.: tokio)

#[cfg(target_arch = "wasm32")]
mod arch {
    pub use wasm_bindgen_futures::spawn_local;
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use std::future::Future;

    // spawn_local in tokio is more powerful, but we need to adjust the function signature to match
    // wasm_bindgen_futures.
    #[inline(always)]
    pub(crate) fn spawn_local<F>(f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        #[cfg(feature = "tokio")]
        ::tokio::task::spawn_local(f);
        #[cfg(not(feature = "tokio"))]
        {
            let _ = f;
            panic!(
                r#"No scheduler configured for this platform, features related to async can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
            );
        }
    }
}

pub(crate) use arch::*;
