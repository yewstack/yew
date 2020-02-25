## Contribution Guide

### Setup your local development environment

#### Add the wasm target

```bash
rustup add target wasm32-unknown-unknown
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
For the tests to work one have to ensure that `wasm-bindgen-cli` is installed.
[Instructions](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#install-the-test-runner)

Additionally a webdriver must be installed locally and configured to be on the
`PATH`. Currently supports `geckodriver`, `chromedriver`, and `safaridriver`,
although more driver support may be added! You can download these at:

* geckodriver - https://github.com/mozilla/geckodriver/releases
* chromedriver - http://chromedriver.chromium.org/downloads
* safaridriver - should be preinstalled on OSX

```bash
./ci/run_tests.sh
```
