# yew-rs: Yew Documentation Website

## Quick build

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --skip-capture
```

Builds the English pages, skips injecting the index.html with captured content. The fastest command to use in development.

## Quick build and serve

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --skip-capture --serve 8080
```
