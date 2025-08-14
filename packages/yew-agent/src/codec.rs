//! Submodule providing the `Codec` trait and its default implementation using `bincode`.

use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Message Encoding and Decoding Format
pub trait Codec {
    /// Encode an input to JsValue
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize;

    /// Decode a message to a type
    fn decode<O>(input: JsValue) -> O
    where
        O: for<'de> Deserialize<'de>;
}

/// Default message encoding with [bincode].
#[derive(Debug)]
pub struct Bincode;

impl Codec for Bincode {
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize,
    {
        let buf = bincode::serde::encode_to_vec(&input, bincode::config::standard())
            .expect("can't serialize an worker message");
        Uint8Array::from(buf.as_slice()).into()
    }

    fn decode<O>(input: JsValue) -> O
    where
        O: for<'de> Deserialize<'de>,
    {
        let data = Uint8Array::from(input).to_vec();
        let (result, _) = bincode::serde::decode_from_slice(&data, bincode::config::standard())
            .expect("can't deserialize an worker message");
        result
    }
}
