---
title: Testing apps
description: Testing your app
---

&lt;TODO&gt;
:::info
We're working on making it easy to test components, but this is currently a work in progress.

Support for [shallow rendering](https://github.com/yewstack/yew/issues/1413) and a proposal to
[expose the code Yew uses internally for testing components](https://github.com/yewstack/yew/issues/1413)
can be found in the GitHub repository.
:::

## wasm\_bindgen\_test

The Rust Wasm working group maintains a crate called [`wasm_bindgen_test`](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html) which allows you to run tests in a browser in similar fashion to how 
the built-in `#[test]` procedural macro works. More information is given in the [Rust WASM working group's documentation](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html) 
for this module.
