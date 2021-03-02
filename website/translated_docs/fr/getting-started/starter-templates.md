---
title: Modèles de démarrage
---

## `trunk`

- [Modèle minimal](https://github.com/yewstack/yew-trunk-minimal-template) - Une application construite avec Trunk pour vous aider à démarrer.

## `wasm-pack`

- [Modèle minimal](https://github.com/yewstack/yew-wasm-pack-minimal) - Utilise `wasm-pack` et `rollup` pour créer votre application et votre propre serveur pour la servir. Pas de cloches ni de sifflets ici.

- [Webpack Template](https://github.com/yewstack/yew-wasm-pack-template) - Utilise `wasm-pack` et le [`wasm-pack-plugin`](https://github.com/wasm-tool/wasm-pack-plugin) pour Webpack pour simplifier le développement.

Contrairement à d'autres outils, `wasm-pack` vous oblige à utiliser une cible `lib` et non pas `bin`, et le point d'entrée de votre programme doit être annoté avec un `#[wasm_bindgen(start)]` .

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

- [Modèle pour Parcel](https://github.com/spielrs/yew-parcel-template) - Créé par un membre de la communauté et utilise [Parcel](https://parceljs.org/)
