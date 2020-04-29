# Futures Example
This example shows off how to make a asynchronous fetch request using web_sys and Yew's futures support.

### How to run:
This example requires rustc v1.39.0 or above to compile due to its use of async/.await syntax.

```sh
wasm-pack build --target web --out-dir ../static/ --out-name wasm && python -m SimpleHTTPServer 8080
```
This will compile the project, bundle up the compiler output and static assets, and start a http server on port 8080 so you can access the example at localhost:8080.

It is expected that you have a setup with wasm-pack and python installed.
