# Futures Example
This example shows off how to make a asynchronous fetch request using web_sys and Yew's futures support.

Because this example uses features not allowed by cargo web, it cannot be included in the showcase, and must be built with a different toolchain instead.

### How to run:
This example requires rustc v1.39.0 or above to compile due to its use of async/.await syntax.

```sh
wasm-pack build --target web --out-dir ../static/ --out-name wasm -- --features (web_sys|std_web) && python -m SimpleHTTPServer 8080
```
This will compile the project, bundle up the compiler output and static assets, and start a http server on port 8080 so you can access the example at localhost:8080.

It is expected that you have a setup with wasm-pack and python installed.
