# yew-rs: Yew Documentation Website

## Quick build (English only)

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt
```

Builds English pages only. Run from the workspace root or from `yew-rs/`.

## Full build (all locales)

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --all-locales
```

## Build and serve

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --serve 8080
```

## Tests

`--skip-capture` also saves more time but the SSR tests will fail.
