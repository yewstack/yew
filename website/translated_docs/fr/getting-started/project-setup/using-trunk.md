---
title: Avec trunk
---

## Installation

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## Utilisation

Consultez ["Créer un exemple d'application"](../build-a-sample-app.md) pour un petit guide sur la façon de créer des applications Yew avec Trunk.

Vous pouvez également le voir en action en regardant nos [exemples](https://github.com/yewstack/yew/tree/master/examples), qui sont tous construits avec Trunk.

Trunk construit votre application sur la base du `index.html` qui sert en quelque sorte de fichier de configuration. Contrairement à `wasm-pack`, cet outil est conçu pour créer des applications web avec Rust. Cela signifie que vous n'avez pas besoin d'ajouter `cdylib` comme cible de bibliothèque et que vous pouvez utiliser la fonction `main` comme point d'entrée.

Pour créer une application Yew simple, vous avez juste besoin d'un `index.html` à la racine de votre projet:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

La CLI Trunk fournit plusieurs commandes utiles, mais pendant le développement, le `trunk serve` est certainement la plus utile. Il exécute un serveur local pour vous et reconstruit automatiquement l'application lorsqu'il détecte des modifications.

Lorsque vous êtes prêt à publier votre application, vous pouvez simplement exécuter `trunk build --release` .

Ce résumé ici ne couvre pas presque toutes les fonctionnalités de Trunk. Consultez son [README](https://github.com/thedodd/trunk) !
