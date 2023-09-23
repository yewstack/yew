use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew_agent::prelude::*;
use yew_agent::Codec;

/// Example to use a custom codec.
pub struct Postcard;

impl Codec for Postcard {
    fn encode<I>(input: I) -> JsValue
    where
        I: Serialize,
    {
        let buf = postcard::to_vec::<_, 32>(&input).expect("can't serialize a worker message");
        Uint8Array::from(buf.as_slice()).into()
    }

    fn decode<O>(input: JsValue) -> O
    where
        O: for<'de> Deserialize<'de>,
    {
        let data = Uint8Array::from(input).to_vec();
        postcard::from_bytes(&data).expect("can't deserialize a worker message")
    }
}

#[oneshot]
pub async fn FibonacciTask(n: u32) -> u32 {
    fn fib(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    fib(n)
}
