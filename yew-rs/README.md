# yew-rs: Yew Documentation Website

## Build and preview a subset of pages

```sh
cargo r -p yew-site-ssg -- \
  --page /docs/getting-started \
  --skip-wasm-opt \
  --skip-capture \
  --jobs 16 \
  --serve 8080
```

This builds only pages matching `/docs/getting-started`, skips wasm-opt (faster), and serves on `http://localhost:8080`.

## Interactive page picker

```sh
cargo r -p yew-site-ssg -- --skip-wasm-opt --skip-capture --jobs 16 --serve 8080
```

Without `--page`, the SSG shows an interactive fuzzy-searchable picker. Type to filter, space to toggle pages, enter to confirm. Press esc to build all pages.

In non-interactive environments (CI, piped stdin), it builds all pages automatically.

## Tests

`--skip-capture` saves time but the SSR tests will fail.
