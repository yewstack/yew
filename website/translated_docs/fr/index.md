---
id: intro
title: Introduction
---

## Qu'est-ce que Yew ?

**Yew** est une bibliothèque [Rust](https://www.rust-lang.org/) moderne pour la création des applications web parallèles avec [WebAssembly](https://webassembly.org/).

- Dispose d'une bibliothèque **basé sur les composants** qui rend facile la création d'interfaces interactives. Ceux qui ont de l’expérience avec des bibliothèques comme [React](https://reactjs.org/) et [Elm](https://elm-lang.org/) sentiront chez-eux en utilisant Yew.
- Réalise de **grandes performances ** en réduisant les appels à l'API DOM et en aidant les développeurs à décharger facilement la charge de travail en arrière plan grâce à l'utilisation des web workers.
- Supporte **l'interopérabilité JavaScript**, permettant aux développeurs d'exploiter les packages NPM, et de s'intégrer aux applications JavaScript existantes.

### Rejoignez nous 😊

- Vous pouvez reporter des bugs et discuter de fonctionnalités sur la [page d'issues GitHub](https://github.com/yewstack/yew/issues)
- Nous adorons les pull requests. Jetez un œil aux [bonnes premières issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) si vous souhaitez aider !
- Notre [serveur Discord](https://discord.gg/VQck8X4) est très actif et est un excellent endroit pour poser des questions

### Prêt à vous lancer ?

Cliquez sur le lien si dessous pour apprendre à créer votre première application Yew et pour voir les projets de la communauté.

[Se lancer](getting-started/project-setup/README.md)

### Pas encore convaincu ?

Ce projet est basé sur une technologie de pointe et est parfait pour les développeurs qui aiment développer les projets fondateurs de demain. Voici quelques raisons pourquoi nous pensons que les bibliothèques comme Yew sont le futur du développement web.

#### Attendez, pourquoi WebAssembly ?

WebAssembly *(Wasm)* est un langage de bas niveau portable vers lequel Rust peut se compiler. Il s'exécute à une vitesse native dans le navigateur, est interopérable avec JavaScript, et est supporté par tous les navigateurs web principaux. Pour savoir comment tirer le meilleur de WebAssembly dans votre application, regardez la liste des [cas d'utilisation](https://webassembly.org/docs/use-cases/).

Il est à noter que l'utilisation de Wasm n'est pas (encore) une solution miracle pour améliorer la performance d'applications web. Pour le moment, l'utilisation d'API DOM depuis WebAssembly est toujours plus lente que directement depuis JavaScript. C'est un problème temporaire que la proposition de [types d'interface WebAssembly](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) vise à résoudre. Si vous souhaitez en savoir plus, consultez cet [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) décrivant la proposition de Mozilla.

#### Ok, mais pourquoi Rust ?

Rust est rapide comme l'éclair et est fiable grâce à son système de type puissant et son concept de possession. Il est difficile à apprendre mais vaut largement l'effort. Rust a été élu le langage de programmation le plus aimé lors du sondage de Stack Overflow auprès des développeurs pendant 5 années consécutives : [2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted), [2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted), [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages), [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) et [2020](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted).

Rust aide également les développeurs à écrire du code plus sûr grâce à son système de type complet et son concept de possession. Dîtes au revoir aux bugs de situation de compétition compliqués à dénicher en JavaScript ! En effet, avec Rust, la plupart de vos bugs seront détectés par le compilateur avant même que votre application soit lancée. Et ne vous inquiétez pas, si votre application rencontre une erreur, vous pourrez toujours obtenir la trace de la pile complète de votre code Rust dans la console du navigateur.

#### Alternatives ?

Nous aimons partager des idées avec d'autres projets, et pensons que nous pouvons tous nous entraider pour atteindre le plein potentiel de cette nouvelle technologie. Si Yew ne vous dis rien, vous pourriez considérer les projets suivants.

- [Percy](https://github.com/chinedufn/percy) - *"A modular toolkit for building isomorphic web apps with Rust + WebAssembly"*
- [Seed](https://github.com/seed-rs/seed) - *"A Rust framework for creating web apps"*
