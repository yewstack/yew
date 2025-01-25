# Contribution Guide

## Setting up your local development environment

### Add the Wasm target

```bash
rustup target add wasm32-unknown-unknown
```

### Install [cargo-make](https://github.com/sagiegurari/cargo-make)

```bash
cargo install cargo-make
```

You can use the following command to list all available tasks for Yew:

```bash
cargo make --list-all-steps
```

The most important tasks are outlined below.

## Tests

To run all tests, use the following command:

```bash
cargo make test-flow
```

### Browser tests

`cargo make test` will automatically download Geckodriver to a temporary location if it isn't in the PATH.

Because Geckodriver looks for `firefox` in the path, if you use
FireFox Developer Edition, you may get an error, because Developer Editions
binary is called `firefox-developer-edition`.
To fix this, either install the standard version of Firefox or symlink
`firefox` to `firefox-developer-edition`.

### Fetch service tests

The tests for the fetch service require a local [httpbin](https://httpbin.org/) server.
If you have [Docker](https://www.docker.com/) installed,
`cargo make test` will automatically run httpbin in a container for you.

Alternatively, you can set the `HTTPBIN_URL` environment variable to the URL you wish to run tests against.

### Macro tests

When adding or updating tests, please make sure to update the appropriate `stderr` file, which you can find [here](https://github.com/yewstack/yew/tree/master/packages/yew-macro/tests/macro) for the `html!` macro.
These files ensure that macro compilation errors are correct and easy to understand.
These errors can change with each release of the compiler, so they should be generated with the Rust version 1.56
(because some tests make use of const generics which were stabilized in that version).

To update or generate a new `stderr` file you can run `cargo make test-overwrite` in the `yew-macro` directory.

## Linting

The following command checks the code using Rustfmt and Clippy:

```bash
cargo make lint
```

To automatically fix formatting issues, run `cargo +nightly fmt` first.

## Benchmarks

js-framework-benchmark is used as a benchmark for the framework as a whole.
Simply clone [bakape/js-framework-benchmark](https://github.com/bakape/js-framework-benchmark)
and follow the repository's README.

## Writing APIs

When building new APIs, think about what it would be like to use them. Would this API cause confusing and hard to pin error messages? Would this API integrate well with other APIs? Is it intuitive to use this API?

Below, you can find some useful guidance and best practices on how to write APIs. These are only _guidelines_ and while they are helpful and should be followed where possible, in some cases, it may not be possible to do so.

- [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html)

## Website

The source code of our website ([https://yew.rs](https://yew.rs)) is in the [website directory](website).
Most of the times, edits can be done in markdown.

[website/README.md](website/README.md) has more detailed instructions.
