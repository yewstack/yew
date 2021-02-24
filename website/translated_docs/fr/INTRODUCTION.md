---
id: intro
title: Introduction
---

## Qu'est-ce que Yew ?

**Yew** is a modern [Rust](https://www.rust-lang.org/) framework for creating multi-threaded front-end web apps with [WebAssembly](https://webassembly.org/).

- It features a **component-based** framework which makes it easy to create interactive UIs. Developers who have experience with frameworks like [React](https://reactjs.org/) and [Elm](https://elm-lang.org/) should feel quite at home when using Yew.
- It achieves **great performance** by minimizing DOM API calls and by helping developers easily offload processing to the background using web workers.
- It supports **JavaScript interoperability**, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.

### Rejoignez nous 😊

- Vous pouvez reporter des bugs et discuter de fonctionnalités sur la [page d'issues GitHub](https://github.com/yewstack/yew/issues)
- Nous adorons les pull requests. Jetez un œil aux [bonnes premières issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) si vous souhaitez aider !
- Notre [serveur Discord](https://discord.gg/VQck8X4) est très actif et est un excellent endroit pour poser des questions

### Prêt à vous lancer ?

Click the link below to learn how to build your first Yew app and learn from community example projects

[Se lancer](getting-started/project-setup/README.md)

### Pas encore convaincu ?

This project is built on cutting edge technology and is great for developers who like to develop the foundational projects of tomorrow. Here are some reasons why we believe that frameworks like Yew are the future of web development.

#### Attendez, pourquoi WebAssembly ?

WebAssembly *(Wasm)* is a portable low-level language that Rust can compile into. It runs at native speeds in the browser and is interoperable with JavaScript and supported in all major browsers. For ideas on how to get the most out of WebAssembly for your app, check out this list of [use cases](https://webassembly.org/docs/use-cases/).

It should be noted that using Wasm is not (yet) a silver bullet for improving the performance of web apps. As of the present, using DOM APIs from WebAssembly is still slower than calling them directly from JavaScript. This is a temporary issue which the [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) proposal aims to resolve. If you would like to learn more, check out this [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) describing the proposal from Mozilla.

#### Ok, mais pourquoi Rust ?

Rust est rapide comme l'éclair et est fiable grâce à son système de type puissant et son concept de possession. Il est difficile à apprendre mais vaut largement l'effort. Rust a été élu le langage de programmation le plus aimé lors du sondage de Stack Overflow auprès des développeurs pendant 5 années consécutives : [2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted), [2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted), [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages), [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) et [2020](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted).

Rust also helps developers write safer code with its rich type system and ownership model. Say goodbye to hard to track down race condition bugs in JavaScript! In fact, with Rust, most of your bugs will be caught by the compiler before your app even runs. And don't worry, when your app does run into an error, you can still get full stack-traces for your Rust code in the browser console.

#### Alternatives ?

We love to share ideas with other projects and believe we can all help each other reach the full potential of this exciting new technology. If you're not into Yew, you may like the following projects.

- [Percy](https://github.com/chinedufn/percy) - *"A modular toolkit for building isomorphic web apps with Rust + WebAssembly"*
- [Seed](https://github.com/seed-rs/seed) - *"A Rust framework for creating web apps"*
