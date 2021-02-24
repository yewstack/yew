---
title: Composants fonctionnels
sidebar_label: introduction
description: Introduction aux fonctions composants
---

:::warning
Nous travaillons toujours sur les composants de fonction et les Hooks. Ils ne sont pas encore tout à fait prêts à être utilisés.
Si vous souhaitez aider, jetez un œil au [tableau de projet](https://github.com/yewstack/yew/projects/3) pour une liste des choses qui doivent encore être faites.
:::

Les composants fonctionnels sont une version simplifiée des composants normaux. Ils consistent en une seule fonction qui reçoit les props et détermine ce qui doit être rendu en renvoyant `Html` . Fondamentalement, c'est un composant qui a été réduit à la seule méthode de `view` . En soi, cela serait assez limitatif car vous ne pouvez créer que des composants purs, mais c'est là que les Hooks entrent en jeu. Les Hooks permettent aux composants de fonction d'utiliser l'état et d'autres fonctionnalités Yew sans implémenter le trait `Component` .

## Introduction aux fonctions composants

La manière la plus simple de créer un function composant est d'ajouter l'attribut [`#[function_component]`](function-components/attribute.md) à une fonction.

```rust
#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! { "Hello world" }
}
```

### Sous le capot

Les fonctions composants se composent de deux parties. Tout d'abord, le trait `FunctionProvider` qui est comparable au trait `Component` mais qui n'a qu'une seule méthode appelée `run` . La deuxième partie est la structure `FunctionComponent` qui entoure le type `FunctionProvider` et le transforme en un `Component` réel. L'attribut `#[function_component]` implémente essentiellement `FunctionProvider` pour vous et l'expose encapsulé dans `FunctionComponent` .

### Hooks

Les Hooks sont simplement des fonctions qui vous permettent de s'inscire à l'état et / ou au cycle de vie des composants et d'exécuter des actions. Yew est livré avec quelques crochets prédéfinis. Tu peux aussi créer des Hooks personalisés.

#### Hooks prédéfinis

Yew est livré avec les Hooks prédéfinis suivants:

- [`use_state`](function-components/pre-defined-hooks.md#use_state)
- [`use_ref`](function-components/pre-defined-hooks.md#use_ref)
- [`use_reducer`](function-components/pre-defined-hooks.md#use_reducer)
- [`use_reducer_with_init`](function-components/pre-defined-hooks.md#use_reducer_with_init)
- [`use_effect`](function-components/pre-defined-hooks.md#use_effect)
- [`use_effect_with_deps`](function-components/pre-defined-hooks.md#use_effect_with_deps)

#### Crochets personnalisés

Dans certains cas, vous souhaitez définir vos propres Hooks pour des raisons quelconques. Yew vous permet de définir vos propres Hooks, ce qui vous permet d'extraire votre logique potentiellement avec l'état du composant dans des fonctions réutilisables. Consultez la section [Définition des Hooks personnalisés](function-components/custom-hooks.md#defining-custom-hooks) pour plus d'informations.
