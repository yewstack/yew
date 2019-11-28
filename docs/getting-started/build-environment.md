# Build Environment

## Build Tools

As shown in [Getting Started](./), using `cargo-web` is the quickest way to get up and running. Unfortunately `cargo-web` requires multiple compile passes and therefore is not as fast as other approaches. The most popular alternative is called `wasm-pack`. Check out the [Starter Templates](starter-templates.md) to get up and running quickly.

### `cargo-web`

Cargo web is a cargo subcommand for building client web apps. It makes building and deploying web applications. Read more [here](https://github.com/koute/cargo-web).

**Install**

```bash
cargo install cargo-web
```

#### Build

```bash
cargo web build
```

#### Run

```bash
cargo web start
```

#### Supported Targets

* `wasm32-unknown-unknown`
* `wasm32-unknown-emscripten`
* `asmjs-unknown-emscripten`

### `wasm-pack`

This tool was created by the Rust / Wasm Working Group and is the most actively developed tool for building WebAssembly applications. It supports building to a Node.JS package and has an accompanying Webpack plugin for easy integration with an existing JavaScript application. Find more information [here](https://rustwasm.github.io/docs/wasm-pack/introduction.html).

{% hint style="info" %}
Note that your crate-type will need to be `cdylib`when using `wasm-pack`
{% endhint %}

**Install**

```bash
cargo install wasm-pack
```

#### Build

This command will produce a bundle in the `./pkg` directory with your app's compiled WebAssembly along with a JavaScript wrapper which can be used to start your application.

```bash
wasm-pack build
```

#### Bundle

For more information on Rollup visit this [guide](https://rollupjs.org/guide/en/#quick-start)

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

#### Serve

Feel free to use your preferred server. Here we use a simple python server to serve to [http://\[::1\]:8000](http://[::1]:8000).

```bash
python -m SimpleHTTPServer 8080
```

#### Supported Targets

* `wasm32-unknown-unknown`

