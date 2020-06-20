## Contribution Guide

### Settting up your local development environment

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

#### Running the tests

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

If you wish to improve the performance of Yew, we ask you to prove the improvements of your changes through benchmarking.

1. Fork and clone https://github.com/yewstack/js-framework-benchmark
2. Update `frameworks/yew/Cargo.toml` with your fork of Yew and the branch for your changes
3. Open a new PR with your `Cargo.toml` changes

Feel free to add new benchmark tests if the current benchmark coverage is insufficient!

#### Pre commit hooks

A Git hook is a script configured to run at a certain point in a Git workflow – for example every time a commit is added.

Yew has Git hooks for this repository which you can (optionally) use. These enable you to catch problems before you commit them.

Currently the hooks will run `cargo fmt` and `cargo check` which ensures that your contributions both compile and eliminates errors telling you that your code isn't correctly formatted. They also run `clippy` which is a Rust linter which finds and reports code which could be improved.

To set up the Git hooks:
1. install [pre-commit](https://pre-commit.com/)
2. ensure you are in a directory which is a clone of the Yew git repo and run `pre-commit install`

The hooks have been installed!

You can manually run the hooks at any time using `pre-commit run`.

### Writing APIs
When building new APIs, think about what it would be like to use them. Would this API cause confusing and hard to pin error mesages? Would this API integrate well with other APIs? Is it intuitive to use this API?

There are many resources which provide good guidance on how to write APIs, a few of which are given below. These are only *guidlines* and while they are useful and should be followed where possible, in some cases it may not be possible to do so.
* [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html)
