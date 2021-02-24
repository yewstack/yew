---
title: Modèles de démarrage
---

## `trunk`

- [Minimal Template](https://github.com/yewstack/yew-trunk-minimal-template) - A small application built with Trunk to get you started.

## `wasm-pack`

- [Modèle minimal](https://github.com/yewstack/yew-wasm-pack-minimal) - Utilise `wasm-pack` et `rollup` pour créer votre application et votre propre serveur pour la servir. Pas de cloches ni de sifflets ici.

- [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - Uses `wasm-pack` and the [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) for Webpack to streamline development.

Unlike other tools, `wasm-pack` forces you to use a `lib`, not a `bin` crate, and the entry-point to your program is annotated with a `#[wasm_bindgen(start)]` attribute.

Votre `Cargo.toml` doit également spécifier que le type de votre caisse est un "cdylib".

```toml
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
# You should include "rlib" (the default crate type) otherwise your crate can't be used as a Rust library
# which, among other things, breaks unit testing
crate-type = ["rlib", "cdylib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2"
```

## Autres modèles

- [Parcel Template](https://github.com/spielrs/yew-parcel-template) - Created by a community member and uses [Parcel](https://parceljs.org/)
