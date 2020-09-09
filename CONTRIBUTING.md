# Contribution Guide

## Setting up your local development environment

### Add the WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### Install [cargo-make](https://github.com/sagiegurari/cargo-make)

```bash
cargo install cargo-make
```

#### stdweb

To run the examples in `./yew-stdweb`, you may wish to install [cargo-web](https://github.com/koute/cargo-web):

```bash
cargo install cargo-web
```

## Tests

To run all tests, use the following command:

```bash
cargo make tests
```

### Browser tests

`cargo make tests` will automatically download Geckodriver to a temporary location if it isn't in the PATH.

### Fetch service tests

The tests for the fetch service require a local [httpbin](https://httpbin.org/) server.
If you have [Docker](https://www.docker.com/) installed,
`cargo make tests` will automatically run httpbin in a container for you.

Alternatively, you can manually run an httpbin instance however you want and set the `HTTPBIN_URL` environment variable to the URL.

Please note that the public httpbin instance can't be used for these tests.

### Macro tests

When adding or updating tests, please make sure to update the appropriate `stderr` file, which you can find [here](https://github.com/yewstack/yew/tree/master/yew-macro/tests/macro) for the `html!` macro.
These files ensure that macro compilation errors are correct and easy to understand.
These errors can change with each release of the compiler so they should be generated with the MSRV (currently 1.45).

To update or generate a new `stderr` file you can run `TRYBUILD=overwrite cargo +1.45.2 test` in the `yew-macro` directory.

## Linting

The following command checks the code using Rustfmt and Clippy:

```bash
cargo make lint
```

## Benchmarks

If you wish to improve the performance of Yew, we ask you to prove the improvements of your changes through benchmarking.

1. Fork and clone [yewstack/js-framework-benchmark](https://github.com/yewstack/js-framework-benchmark)
2. Update `frameworks/yew/Cargo.toml` with your fork of Yew and the branch for your changes
3. Open a new PR with your `Cargo.toml` changes

Feel free to add new benchmark tests if the current benchmark coverage is insufficient!

## Writing APIs

When building new APIs, think about what it would be like to use them. Would this API cause confusing and hard to pin error mesages? Would this API integrate well with other APIs? Is it intuitive to use this API?

Below, you can find some useful guidance and best practices on how to write APIs. These are only _guidelines_ and while they are helpful and should be followed where possible, in some cases, it may not be possible to do so.

- [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html)
