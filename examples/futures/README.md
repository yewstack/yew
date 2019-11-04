# Futures Example
This example shows off how to make a asynchronous fetch request using web_sys and Yew's futures support.

Because this example uses features not allowed by cargo web, it cannot be included in the showcase, and must be built with a different toolchain instead.

### How to run:
```sh
rustup run nightly $HOME/.cargo/bin/wasm-pack build --target web && rollup ./main.js --format iife --file ./pkg/bundle.js && python -m SimpleHTTPServer 8080
```
This will compile the project using nightly rust, bundle up the compiler output and static assets, and start a http server on port 8080 so you can access the example at localhost:8080.

It is expected that you have a setup with wasm-pack, rollup, and python installed.
Since wasm-pack doesn't have a nightly version at the moment, you have to hack around that by using `rustup run nightly`.
