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
For the tests to work one have to ensure that `wasm-bindgen-cli` is installed.
[Instructions](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#install-the-test-runner)

When you add the tests, please make sure you have updated `stderr` appropriate file, which you can find in [here](https://github.com/yewstack/yew/tree/master/tests/macro).
To generate new `stderr` file you should run `./ci/run_tests.sh` and follow instructions.
You can be asked to set env variable `TRYBUILD=overwrite` to automatically overwrite old `stder` file.
Once you have done with it, your PR can be accepted.

Additionally a webdriver must be installed locally and configured to be on the
`PATH`. Currently supports `geckodriver`, `chromedriver`, and `safaridriver`,
although more driver support may be added! You can download these at:

* geckodriver - https://github.com/mozilla/geckodriver/releases
* chromedriver - http://chromedriver.chromium.org/downloads
* safaridriver - should be preinstalled on OSX

```bash
./ci/run_tests.sh
```
or

```bash
cargo test --target wasm32-unknown-unknown --features wasm_test
```
