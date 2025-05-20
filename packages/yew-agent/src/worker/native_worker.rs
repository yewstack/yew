use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
pub(crate) use web_sys::Worker as DedicatedWorker;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

use crate::codec::Codec;

pub(crate) trait WorkerSelf {
    type GlobalScope;

    fn worker_self() -> Self::GlobalScope;
}

impl WorkerSelf for DedicatedWorker {
    type GlobalScope = DedicatedWorkerGlobalScope;

    fn worker_self() -> Self::GlobalScope {
        JsValue::from(js_sys::global()).into()
    }
}

pub(crate) trait NativeWorkerExt {
    fn set_on_packed_message<T, CODEC, F>(&self, handler: F)
    where
        T: Serialize + for<'de> Deserialize<'de>,
        CODEC: Codec,
        F: 'static + Fn(T);

    fn post_packed_message<T, CODEC>(&self, data: T)
    where
        T: Serialize + for<'de> Deserialize<'de>,
        CODEC: Codec;
}

macro_rules! worker_ext_impl {
    ($($type:path),+) => {$(
        impl NativeWorkerExt for $type {
            fn set_on_packed_message<T, CODEC, F>(&self, handler: F)
            where
                T: Serialize + for<'de> Deserialize<'de>,
                CODEC: Codec,
                F: 'static + Fn(T)
            {
                let handler = move |message: MessageEvent| {
                    let msg = CODEC::decode(message.data());
                    handler(msg);
                };
                let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(MessageEvent)>).into_js_value();
                self.set_onmessage(Some(closure.as_ref().unchecked_ref()));
            }

            fn post_packed_message<T, CODEC>(&self, data: T)
            where
                T: Serialize + for<'de> Deserialize<'de>,
                CODEC: Codec
            {
                self.post_message(&CODEC::encode(data))
                    .expect_throw("failed to post message");
            }
        }
    )+};
}

worker_ext_impl! {
    DedicatedWorker, DedicatedWorkerGlobalScope
}
