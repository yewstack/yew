---
title: Avec trunk
---

## Installation

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## Utilisation

Check out ["Build a sample app"](../build-a-sample-app.md) for a short guide on how to build Yew apps with Trunk.

Vous pouvez également le voir en action en regardant nos [exemples](https://github.com/yewstack/yew/tree/master/examples), qui sont tous construits avec Trunk.

Trunk construit votre application sur la base du `index.html` qui sert en quelque sorte de fichier de configuration. Contrairement à `wasm-pack`, cet outil est conçu pour créer des applications web avec Rust. Cela signifie que vous n'avez pas besoin d'ajouter `cdylib` comme cible de bibliothèque et que vous pouvez utiliser la fonction `main` comme point d'entrée.

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

The Trunk CLI provides several useful commands but during development `trunk serve` is certainly the most useful one. It runs a local server for you and automatically rebuilds the app when it detects changes.

When you're ready to release your app, you can just run `trunk build --release`.

Ce résumé ici ne couvre pas presque toutes les fonctionnalités de Trunk. Consultez son [README](https://github.com/thedodd/trunk) !
