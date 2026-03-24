# yew-rs: Yew Documentation Website

## Quick build (English only)

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --jobs 16
```

Builds English pages only. Run from the workspace root or from `yew-rs/`.

## Full build (all locales)

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --jobs 16 --all-locales
```

## Build and serve

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --jobs 16 --serve 8080
```

## Tests

`--skip-capture` saves time but the SSR tests will fail.
