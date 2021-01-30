---
title: Using trunk
---

## Install

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## Usage

Check out ["Build a sample app"](../build-a-sample-app.md) for a short guide on how to build Yew apps with Trunk.

You can also see it in action by looking at our [examples](https://github.com/yewstack/yew/tree/master/examples),
all of which are built with Trunk.

Trunk builds your app based on the `index.html` file which serves as a config file of sorts.
Unlike `wasm-pack`, this tool is actually designed to build apps.
This means you don't need to add `cdylib` as a library target and you can use the `main` function
as an entry point.

To build a simple Yew app you just need an `index.html` file at the root of your project:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

The Trunk CLI provides several useful commands but during development `trunk serve` is certainly the most useful one.
It runs a local server for you and automatically rebuilds the app when it detects changes.

When you're ready to release your app, you can just run `trunk build --release`.

This summary here doesn't nearly cover all of Trunk's features,
be sure to check out the [README](https://github.com/thedodd/trunk)!
