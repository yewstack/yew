# [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon) Demo

Calculate the sum from 1 to n in parallel from a background WASM thread using
[`rayon`](https://github.com/rayon-rs/rayon).

This example makes use of
[`SharedArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)
for shared-memory concurrency. Because the main/UI thread is never allowed to
block, the state is (lazily) initialised globally behind a `Mutex`; the worker
thread which initialises the `rayon` threadpool and dispatches to it is then
responsible for updating and modifying the state, after which it notifies the
main thread to trigger a re-render. This should be vastly more efficient than
the alternative approach of message-passing via
[`serde`](https://github.com/serde-rs/serde)
[`bincode`](https://github.com/bincode-org/bincode), which is the default for
`yew::agent`; additionally, your application state need not implement `serde`'s
`Serialize` and `Deserialize`, nor be clonable - it just needs to be
thread-safe.

`SharedArrayBuffer` is [available in most modern
browsers](https://caniuse.com/sharedarraybuffer) when the following CORS
headers are set (`serve.py` does this):

```
Cross-Origin-Opener-Policy: 'same-origin'
Cross-Origin-Embedder-Policy: 'require-corp'
```

This also demonstrates how to use [module
workers](https://web.dev/module-workers/), which is necessitated by
`wasm-bindgen-rayon` requiring `wasm-pack --target web`. A polyfill is used to
provide support for this in Firefox
([compatibility](https://caniuse.com/mdn-api_worker_worker_ecmascript_modules),
and this must be served from the same host because of the CORS headers required
to enable `SharedArrayBuffer`.

# Usage

* `make` to build;
* `make serve` to host on http://localhost:8000.

# Thanks

- [SpanishPear](https://github.com/SpanishPear) for [the
PR](https://github.com/yewstack/yew/pull/2087) upon which this demo
is based.
