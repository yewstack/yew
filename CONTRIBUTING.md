## Contribution Guide

### Setup your local development environment

#### Add the wasm target

```bash
rustup target add wasm32-unknown-unknown
```

#### Install [cargo-web]

This is an optional tool that simplifies running the examples:

```bash
cargo install cargo-web
```

> Add `--force` option to ensure you install the latest version.

[cargo-web]: https://github.com/koute/cargo-web

#### Build

```bash
cargo build --target wasm32-unknown-unknown
```

#### Test

##### Web Tests
First, ensure that `wasm-bindgen-cli` is installed.
[Instructions](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#install-the-test-runner)

Additionally a webdriver must be installed locally and configured to be on the
`PATH`. Currently supports `geckodriver`, `chromedriver`, and `safaridriver`,
although more driver support may be added! You can download these at:

* geckodriver - https://github.com/mozilla/geckodriver/releases
* chromedriver - http://chromedriver.chromium.org/downloads
* safaridriver - should be preinstalled on OSX

##### Macro Tests
When adding or updating tests, please make sure you have updated the appropriate `stderr` file, which you can find [here](https://github.com/yewstack/yew/tree/master/tests/macro) for the `html!` macro. These files ensure that macro compilation errors are correct and easy to understand.

To update or generate a new `stderr` file you can run `TRYBUILD=overwrite cargo test --test macro_test` or `TRYBUILD=overwrite cargo test --test derive_props_test`.

##### Running Tests

```bash
./ci/run_tests.sh
```
or

```bash
cargo test --target wasm32-unknown-unknown --features wasm_test
```
