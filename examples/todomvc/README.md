## Yew TodoMVC Demo

This is an implementation of [TodoMVC](http://todomvc.com/) app.

Unlike other implementations, this stores the full state of the model,
including: all entries, entered text and chosen filter.

### How to run:
This example requires rustc v1.39.0 or above to compile due to its use of async/.await syntax.

```sh
wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html
```
This will compile the project, bundle up the compiler output and static assets, and start a http server on port 8080 so you can access the example at [localhost:8080](http://127.0.0.1:8080).

It is expected that you have a setup with [wasm-pack9](https://github.com/rustwasm/wasm-pack) and [miniserve](https://github.com/svenstaro/miniserve) installed.
