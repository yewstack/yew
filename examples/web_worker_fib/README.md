# Web Worker Demo

Calculate fibbonnaci value of a number in the worker thread,
without blocking the main thread.

You can access a live version here:

# Running this example

do `./build.sh && trunk serve`

## notes

The worker is NOT built with [trunk](https://github.com/thedodd/trunk). However, we use `trunk` to serve the app.
Note: multi-threading in yew does not currently build with Trunk, due to issues described in the [multi_thread](/examples/multi_thread/README.md) example.

Instead the example is built with [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) directly.

To build, run `./build.sh`.
You can then serve the build, with `trunk serve`.

# Thanks to

- [insou22](https://github.com/insou22) for writing up the demo.
- [https://github.com/yvt/img2text](https://github.com/yvt/img2text) -- for how to make web workers compile in wasm
