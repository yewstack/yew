## Contribution Guide

### Setup your local development environment

#### Add the wasm target

```bash
rustup target add wasm32-unknown-unknown
```

#### Build

```bash
cargo build --target wasm32-unknown-unknown
```

#### Examples

TODO: Add more info 

##### stdweb
In order to run the examples in `./yew-stdweb`, you may wish to install [cargo-web]:

```bash
cargo install cargo-web
```

[cargo-web]: https://github.com/koute/cargo-web

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

Lastly, the fetch tests require a local httpbin server running on port `8000`. We recommend running this with Docker:

```bash
docker run -p 8000:80 kennethreitz/httpbin
```

##### Macro Tests
When adding or updating tests, please make sure you have updated the appropriate `stderr` file, which you can find [here](https://github.com/yewstack/yew/tree/master/yew-macro/tests/macro) for the `html!` macro. These files ensure that macro compilation errors are correct and easy to understand.

To update or generate a new `stderr` file you can run `TRYBUILD=overwrite cargo test --test macro_test` or `TRYBUILD=overwrite cargo test --test derive_props_test` from the `yew-macro` directory.

##### Running Tests

```bash
./ci/run_tests.sh
```
or

```bash
cargo test --target wasm32-unknown-unknown --features wasm_test
```

#### Benchmarks

If you wish to improve the performance of Yew, we request that you prove the improvements of your changes with benchmarks.

1. Fork and clone https://github.com/yewstack/js-framework-benchmark
2. Update `frameworks/yew/Cargo.toml` with your fork of Yew and the branch for your changes
3. Open a new PR with your `Cargo.toml` changes

Feel free to add new benchmark tests if the current benchmark coverage is insufficient!
