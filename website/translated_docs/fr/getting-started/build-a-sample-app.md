---
title: Créer une application
---

Tout d'abord, créez un nouveau projet cargo:

```bash
cargo new yew-app
```

Ouvrez le répertoire nouvellement créé.

Tout d'abord, ajoutons `yew` tant que dépendances dans le fichier `Cargo.toml`

```toml
[package]
name = "yew-app"
version = "0.1.0"
edition = "2018"

[dependencies]
# you can check the latest version here: https://crates.io/crates/yew
yew = "0.17"
```

Copiez le code suivant dans votre fichier `src/main.rs`

```rust
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` serve de référence à un composant.
    // Il peut être utilisé pour envoyer des messages à ce composant.
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // la valeur a changé, afficher la nouvelle valeur
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Ne devrait que être "true" si les nouveaux props
        // sont différents des props précédents.
        // Ce composant n'a aucun props donc la valeur est toujours "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick={self.link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```

Ce code configure votre `Component` racine, appelé `Model` qui affiche un bouton qui incrémente sa propre valeur vous cliquez dessus. Prenez note de `yew::start_app::<Model>()` dans `main()` qui démarre votre application et la monte sur `<body>` Si vous souhaitez démarrer votre application avec des propriétés dynamiques, vous pouvez utiliser à la place `yew::start_app_with_props::<Model>(..)` .

Enfin, ajoutez un `index.html` dans le dossier racine de votre application:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

## Exécutez votre application

Si vous ne l'avez pas déjà fait, installez [Trunk](https://github.com/thedodd/trunk) :

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

Il ne vous reste plus qu'à exécuter la commande suivante:

```bash
trunk serve
```

Cela démarrera un serveur de développement qui mettra continuellement à jour l'application chaque fois que vous modifiez le code.
