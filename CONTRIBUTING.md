# Contribution Guide

## Settting up your local development environment

### Add the wasm target

```bash
rustup target add wasm32-unknown-unknown
```

### Build

```bash
cargo build --target wasm32-unknown-unknown
```

#### stdweb

In order to run the examples in `./yew-stdweb`, you may wish to install [cargo-web](https://github.com/koute/cargo-web):

```bash
cargo install cargo-web
```

## Tests

Yew has unit tests which can be run with `cargo test`. However, a large portion of the tests need to be run in a browser.
This section will guide you through the process of running these tests locally.

### Integration Tests

First, ensure that `wasm-pack` is installed.
[Instructions](https://rustwasm.github.io/wasm-pack/installer/)

`wasm-pack` automatically takes care of installing the correct `wasm-bindgen` version and downloading a webdriver for the browser.

The following command is all you need to run the tests:

```bash
wasm-pack test --firefox --headless -- --features wasm_test
```

You can replace `--firefox` with `--chrome` or `--safari` if you want to run the tests in a different browser. Currently Yew's CI tests use Firefox.

Make sure to run the command in the directory of the crate you wish to test (e.g. `yew` or `yewtil`).
If you run the command in the repository root you will get an error like this:

```bash
Error: failed to parse manifest: yew/Cargo.toml
Caused by: missing field `package`
```

#### Manually installing a webdriver

`wasm-pack` automatically downloads the selected webdriver for you.
If desired, you can download a WebDriver manually. For instructions, please consult the documentation of the webdriver you wish to install.

You might want to pass the argument `--mode no-install` to `wasm-pack` to make sure it doesn't try to install something.
If the webdriver binary is in the path you can use the same command as above to run the tests.
If not, you need to tell `wasm-pack test` where to find it using the `--geckodriver`, `--chromedriver`, or `--safaridriver` option.
Run `wasm-pack test --help` to learn more.

#### Running all tests

You can use the `/ci/run_tests.sh` script to run all tests. This is the same script that is used by Yew's CI to run the whole test suite.
The script currently always runs the tests in Firefox.

#### Fetch service tests

This only applies to the `yew` (and `yew-stdweb`) crate.

The tests for the fetch service require a local httpbin server. We recommend running this with [Docker](https://www.docker.com/):

```bash
docker run -p 8000:80 kennethreitz/httpbin
```

Before running the tests you need to set the `HTTPBIN_URL` environment variable to the url of your httpbin instance.
If you used the previous command to start the server the value should be "http://localhost:8000" (It's important that you don't add a trailing slash).

```shell
# Unix-like
export HTTPBIN_URL="http://localhost:8000"

# Windows
set HTTPBIN_URL=http://localhost:8000
```

You also need to activate the `httpbin_test` feature in order for the tests to run:

```bash
wasm-pack test --firefox --headless -- --features wasm_test,httpbin_test
```

If you're using the `/ci/run_tests.sh` script you only need to set the environment variable. The script will automatically add the feature.

### Macro tests

When adding or updating tests, please make sure you have updated the appropriate `stderr` file, which you can find [here](https://github.com/yewstack/yew/tree/master/yew-macro/tests/macro) for the `html!` macro. These files ensure that macro compilation errors are correct and easy to understand.

To update or generate a new `stderr` file you can run `TRYBUILD=overwrite cargo test --test macro_test` or `TRYBUILD=overwrite cargo test --test derive_props_test` from the `yew-macro` directory.

## Benchmarks

If you wish to improve the performance of Yew, we ask you to prove the improvements of your changes through benchmarking.

1. Fork and clone [yewstack/js-framework-benchmark](https://github.com/yewstack/js-framework-benchmark)
2. Update `frameworks/yew/Cargo.toml` with your fork of Yew and the branch for your changes
3. Open a new PR with your `Cargo.toml` changes

Feel free to add new benchmark tests if the current benchmark coverage is insufficient!

## Writing APIs

When building new APIs, think about what it would be like to use them. Would this API cause confusing and hard to pin error mesages? Would this API integrate well with other APIs? Is it intuitive to use this API?

There are many resources which provide good guidance on how to write APIs, a few of which are given below. These are only _guidelines_ and while they are useful and should be followed where possible, in some cases it may not be possible to do so.

- [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html)
