---
id: cargo-web
title: Using cargo-web
---

Cargo web is a cargo subcommand for building client web apps. It makes building and deploying web 
applications incredibly easy. It is also the only toolchain that supports Emscripten targets. Read 
more [here](https://github.com/koute/cargo-web).

**Install**

```bash
cargo install cargo-web
```

## Build

```bash
cargo web build
```

## Run

```bash
cargo web start
```

## Supported Targets

* `wasm32-unknown-unknown`
* `wasm32-unknown-emscripten`
* `asmjs-unknown-emscripten`

:::note
For `*-emscripten` targets, you'll need to install the Emscripten SDK
:::
