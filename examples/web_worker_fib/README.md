# Web Worker Fib

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fweb_worker_fib)](https://examples.yew.rs/web_worker_fib)

Calculate fibrillation value of a number in the worker thread, without blocking the main thread.

## Concepts

The example illustrates how to use `yew-agent` to send tasks to a worker thread in a Yew application.

## Thanks to

- [insou22](https://github.com/insou22) for writing up the demo.
- [https://github.com/yvt/img2text](https://github.com/yvt/img2text) -- for how to make web workers compile in wasm

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```