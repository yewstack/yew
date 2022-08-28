//! The client-side rendering variant. This is used for client side rendering.

use std::marker::PhantomData;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use super::PreparedStateBase;
use crate::functional::{use_state, Hook, HookContext};
use crate::platform::spawn_local;
use crate::suspense::{Suspension, SuspensionResult};

#[cfg(target_arch = "wasm32")]
async fn decode_base64(s: &str) -> Result<Vec<u8>, JsValue> {
    use gloo::utils::window;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let fetch_promise = window().fetch_with_str(s);

    let content_promise = JsFuture::from(fetch_promise)
        .await
        .and_then(|m| m.dyn_into::<web_sys::Response>())
        .and_then(|m| m.array_buffer())?;

    let content_array = JsFuture::from(content_promise)
        .await
        .as_ref()
        .map(Uint8Array::new)?;

    Ok(content_array.to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
async fn decode_base64(_s: &str) -> Result<Vec<u8>, JsValue> {
    unreachable!("this function is not callable under non-wasm targets!");
}

#[doc(hidden)]
pub fn use_prepared_state<T, D>(deps: D) -> impl Hook<Output = SuspensionResult<Option<Rc<T>>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    struct HookProvider<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        _marker: PhantomData<T>,
        deps: D,
    }

    impl<T, D> Hook for HookProvider<T, D>
    where
        D: Serialize + DeserializeOwned + PartialEq + 'static,
        T: Serialize + DeserializeOwned + 'static,
    {
        type Output = SuspensionResult<Option<Rc<T>>>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let data = use_state(|| {
                let (s, handle) = Suspension::new();
                (
                    SuspensionResult::<(Option<Rc<T>>, Option<Rc<D>>)>::Err(s),
                    Some(handle),
                )
            })
            .run(ctx);

            let state = {
                let data = data.clone();
                ctx.next_prepared_state(move |_re_render, buf| -> PreparedStateBase<T, D> {
                    if let Some(buf) = buf {
                        let buf = format!("data:application/octet-binary;base64,{}", buf);

                        spawn_local(async move {
                            let buf = decode_base64(&buf)
                                .await
                                .expect("failed to deserialize state");

                            let (state, deps) =
                                bincode::deserialize::<(Option<T>, Option<D>)>(&buf)
                                    .map(|(state, deps)| (state.map(Rc::new), deps.map(Rc::new)))
                                    .expect("failed to deserialize state");

                            data.set((Ok((state, deps)), None));
                        });
                    }

                    PreparedStateBase {
                        #[cfg(feature = "ssr")]
                        state: None,
                        #[cfg(feature = "ssr")]
                        deps: None,

                        has_buf: buf.is_some(),
                        _marker: PhantomData,
                    }
                })
            };

            if state.has_buf {
                let (data, deps) = data.0.clone()?;

                if deps.as_deref() == Some(&self.deps) {
                    return Ok(data);
                }
            }

            Ok(None)
        }
    }

    HookProvider::<T, D> {
        _marker: PhantomData,
        deps,
    }
}
