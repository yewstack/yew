---
title: Project Setup
sidebar_label: Introduction
description: Set yourself up for success
---

## Overview

Your local development environment will need a couple of tools to compile, build, package and debug your Yew application.


## Install Rust, Rustup and Cargo

To install Rust compiler, Rustup package manager and the Cargo build tool, follow the [official instructions](https://www.rust-lang.org/tools/install).

:::important
The minimum supported Rust version (MSRV) for Yew is `1.45.0`. Older versions can cause unexpected issues accompanied by incomprehensible error messages.
You can check your toolchain version using `rustup show` (under "active toolchain") or alternatively `rustc --version`. To update your toolchain, run `rustup update`.
:::

## Install WebAssembly target

Rust can compile source codes for different "targets" (e.g. different processors). The compilation target for browser-based WebAssembly is called "wasm32-unknown-unknown".  The following command will add this target to your development environment.

`rustup target add wasm32-unknown-unknown`

## Install Trunk

Trunk is the recommended tool for managing deployment and packaging, and will be used throughout the documentation and examples.
See [Wasm Build Tools](./../more/wasm-build-tools.md) for more information on packaging and alternatives.

`cargo install trunk`

## Install wasm-bindgen-cli

Trunk uses the wasm-bindgen-cli to perform deployment and packaging, install using the following command.

`cargo install wasm-bindgen-cli`


## Summary

Now that you have all the tools needed, we can [build a sample application](./build-a-sample-app.md).
