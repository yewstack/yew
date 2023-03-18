# Js Callback Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fjs_callback)](https://examples.yew.rs/js_callback)

## Concepts

The example uses wasm-bindgen to import functionality from Javascript.
To learn more about the subject, refer to ["The `wasm-binden` Guide"](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html).

This example also demonstrates how to delay the loading of the snippet using Suspense.

### Serving JS files 

JS files can be served when they're present in `dist` directory. There are two ways to copy these files:
1. Use [JS Snippets](https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html). 
2. Use trunk to copy the file and import it manually

### Using JS Snippets

This example uses this approach. `wasm-bindgen` handles copying the files with this approach. 
All you have to do is define the `extern` block. The files are copied to `dist/snippets/<bin-name>-<hash>/` directory.

If the file is to be loaded with the initial load, you can simply use the JS imports, as shown we `imp.js`.

If you would like to lazy-load the JS module, you need to use the `trunk`'s `post_build` hook 
from [`trunk_post_build.rs`](trunk_post_build.rs) access the snippets' directory path at runtime and use in 
[`import()` for dynamic imports](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#dynamic_imports)


### Copying file with trunk

This approach is only needed if the JS module is to be lazy-loaded. It allows us to skip the step where we 
provide the snippets' directory path to the app. Instead, the file is copied at a known location that can 
easily be referenced im `import()` statement.

## Improvements

This example is a purely technical demonstration and lacks an actual purpose.
The best way to improve this example would be to incorporate this concept into a small application.

- Do something more complex in the Javascript code to demonstrate more of `wasm-bindgen`'s capabilities.
- Improve the presentation of the example with CSS.

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```