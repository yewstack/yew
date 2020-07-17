## Yew TodoMVC Demo

This is an implementation of [TodoMVC](http://todomvc.com/) app.

Unlike other implementations, this stores the full state of the model,
including: all entries, entered text and chosen filter.

### How to run:
This example requires rustc v1.39.0 or above to compile due to its use of the `async`/`await` syntax.

```sh
wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html
```
This will compile the project, bundle the compiler output and static assets together, and start a HTTP server running locally on port 8080 (to access the example you can use your browser to navigate to [localhost:8080](http://127.0.0.1:8080)).

To run this example you'll need to have [wasm-pack](https://github.com/rustwasm/wasm-pack) and [miniserve](https://github.com/svenstaro/miniserve) installed.
