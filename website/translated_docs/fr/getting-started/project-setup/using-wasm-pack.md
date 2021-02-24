---
title: Avec wasm-pack
---

Cet outil a été créé par le groupe de travail Rust / Wasm pour créer des applications en WebAssembly. Il prend en charge le code d'empaquetage dans les `npm` et est accompagné d'un [plugin Webpack](https://github.com/wasm-tool/wasm-pack-plugin) pour une intégration facile avec les applications JavaScript existantes. Plus d'informations sont données dans [la documentation de `wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/introduction.html) .

::: note `wasm-pack` nécessite que vous définissiez explicitement le cible `cdylib`:

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

:::

## Installation

```bash
cargo install wasm-pack
```

## Build

Cette commande produira un bundle dans le dossier `./pkg` avec le WebAssembly compilé de votre application ainsi qu'un wrapper JavaScript qui peut être utilisé pour démarrer votre application.

```bash
wasm-pack build --target web
```

## Bundle

For more information on rollup.js visit this [guide](https://rollupjs.org/guide/en/#quick-start).

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

When using a bundler like rollup.js you can omit `--target web`.

## Serve

Feel free to use your preferred server. Here we use a simple Python server to serve the built app.

```bash
python -m http.server 8000
```

Si vous n'avez pas installé Python, vous pouvez installer et utiliser le paquet [`simple-http-server`](https://github.com/TheWaWaR/simple-http-server).
