mod private;
mod public;
mod queue;

pub use private::Private;
pub use public::Public;

use super::*;
use js_sys::{Array, Reflect, Uint8Array};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    Blob, BlobPropertyBag, DedicatedWorkerGlobalScope, MessageEvent, Url, Worker, WorkerOptions,
};

/// Implements rules to register a worker in a separate thread.
pub trait Threaded {
    /// Executes an agent in the current environment.
    /// Uses in `main` function of a worker.
    fn register();
}

/// Message packager, based on serde::Serialize/Deserialize
pub trait Packed {
    /// Pack serializable message into Vec<u8>
    fn pack(&self) -> Vec<u8>;
    /// Unpack deserializable message of byte slice
    fn unpack(data: &[u8]) -> Self;
}

impl<T: Serialize + for<'de> Deserialize<'de>> Packed for T {
    fn pack(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("can't serialize an agent message")
    }

    fn unpack(data: &[u8]) -> Self {
        bincode::deserialize(data).expect("can't deserialize an agent message")
    }
}

/// Serializable messages to worker
#[derive(Serialize, Deserialize, Debug)]
enum ToWorker<T> {
    /// Client is connected
    Connected(HandlerId),
    /// Incoming message to Worker
    ProcessInput(HandlerId, T),
    /// Client is disconnected
    Disconnected(HandlerId),
    /// Worker should be terminated
    Destroy,
}

/// Serializable messages sent by worker to consumer
#[derive(Serialize, Deserialize, Debug)]
enum FromWorker<T> {
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    /// Outgoing message to consumer
    ProcessOutput(HandlerId, T),
}

fn send_to_remote<AGN>(worker: &Worker, msg: ToWorker<AGN::Input>)
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    let msg = msg.pack();
    worker.post_message_vec(msg);
}

fn worker_new(name_of_resource: &str, resource_is_relative: bool, is_module: bool) -> Worker {
    let origin = yew::utils::origin().unwrap();
    let pathname = yew::utils::window().location().pathname().unwrap();

    let prefix = if resource_is_relative {
        // Location pathname always contains initial '/', so unwrap will never fail.
        pathname.revsplit_once('/').unwrap().0
    } else {
        ""
    };
    let script_url = format!("{}{}/{}", origin, prefix, name_of_resource);
    let wasm_url = format!(
        "{}{}/{}",
        origin,
        prefix,
        name_of_resource.replace(".js", "_bg.wasm")
    );
    let array = Array::new();
    array.push(
        &format!(
            r#"importScripts("{}");wasm_bindgen("{}");"#,
            script_url, wasm_url
        )
        .into(),
    );
    let blob = Blob::new_with_str_sequence_and_options(
        &array,
        BlobPropertyBag::new().type_("application/javascript"),
    )
    .unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    if is_module {
        let options = WorkerOptions::new();
        Reflect::set(
            options.as_ref(),
            &JsValue::from_str("type"),
            &JsValue::from_str("module"),
        )
        .unwrap();
        Worker::new_with_options(&url, &options).expect("failed to spawn worker")
    } else {
        Worker::new(&url).expect("failed to spawn worker")
    }
}

fn worker_self() -> DedicatedWorkerGlobalScope {
    JsValue::from(js_sys::global()).into()
}

trait RevSplitOnce {
    /// Splits the string on the last occurrence of the specified delimiter and
    /// returns prefix before delimiter and suffix after delimiter.
    ///
    /// Behaves identically to `rsplit_once` introduced with rust 1.52 but is
    /// available prior to 1.52.
    fn revsplit_once(&self, delimiter: char) -> Option<(&str, &str)>;
}

impl RevSplitOnce for str {
    fn revsplit_once<'a>(&'a self, delimiter: char) -> Option<(&'a str, &'a str)> {
        let components: Vec<&'_ str> = self.rsplitn(2, delimiter).collect();
        assert!(components.len() < 3);
        if components.len() == 2 {
            Some((components[1], components[0]))
        } else {
            None
        }
    }
}

#[test]
fn test_rsplit_once() {
    // These from the docstrings in rsplit_once.
    assert_eq!("cfg".revsplit_once('='), None);
    assert_eq!("cfg=foo".revsplit_once('='), Some(("cfg", "foo")));
    assert_eq!("cfg=foo=bar".revsplit_once('='), Some(("cfg=foo", "bar")));
}

trait WorkerExt {
    fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>));

    fn post_message_vec(&self, data: Vec<u8>);
}

macro_rules! worker_ext_impl {
    ($($type:ident),+) => {$(
        impl WorkerExt for $type {
            fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>)) {
                let handler = move |message: MessageEvent| {
                    let data = Uint8Array::from(message.data()).to_vec();
                    handler(data);
                };
                let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(MessageEvent)>);
                self.set_onmessage(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }

            fn post_message_vec(&self, data: Vec<u8>) {
                self.post_message(&Uint8Array::from(data.as_slice()))
                    .expect("failed to post message");
            }
        }
    )+};
}

worker_ext_impl! {
    Worker, DedicatedWorkerGlobalScope
}
