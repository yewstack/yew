---
title: "Introducción"
slug: /
---

## ¿Qué es Yew?

**Yew** es un framework moderno de [Rust](https://www.rust-lang.org/) para la creación de aplicaciones
web frontend multi-hilo usando [WebAssembly](https://webassembly.org/).

* Cuenta con un framework **basado en componentes** el cual facilita la creación de interfaces de usuario interactivas.
Los desarrolladores con experiencia en frameworks como [React](https://reactjs.org/) y
[Elm](https://elm-lang.org/) se sentirán como en casa usando Yew.
* Logra un **gran desempeño** al minimizar las llamadas a la API del DOM y ayudando a los desarrolladores a
delegar fácilmente el procesamiento a hilos en segundo plano usando web workers.
* Soporta **interoperabilidad JavaScript**, permitiendo a los desarrolladores aprovechar los paquetes NPM
y la integración con aplicaciones JavaScript existentes.

### Únete 😊

* Puedes reportar bugs y discutir features en la [página de issues de GitHub](https://github.com/yewstack/yew/issues)
* Amamos los pull requests. ¡Revisa los [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) 
si te gustaría ayudar!
* Nuestro [chat de Discord](https://discord.gg/VQck8X4) está muy activo y es un gran lugar
para realizar preguntas

### ¿Listo para empezar?

Da clic en el enlace de abajo para aprender cómo construir tu primer applicación Yew y aprender de los
proyectos de ejemplo construidos por la comunidad

[Comenzando](./getting-started/project-setup.md)

### ¿Aún no estás convencido?

Este proyecto está construido con tecnología punta y es genial para desarrolladores que quieren crear 
los proyectos fundacionales del mañana. Pensamos que la velocidad y fiabilidad de las tecnologías en
las que Yew está construido se volverán el estándar para aplicaciones web rápidas y resilientes
del futuro.

#### Espera, ¿por qué WebAssembly?

WebAssembly _\(Wasm\)_ es un lenguaje portable de bajo nivel en el que Rust puede compilar. Se ejecuta
a velocidades nativas en el navegador y es interoperable con JavaScript y soportado en la gran mayoría de navegadores
modernos. Para ideas de cómo obtener el mayor provecho de WebAssembly en tu aplicación, revisa esta lista de
[casos de uso](https://webassembly.org/docs/use-cases/).

Cabe señalar que usar Wasm no es (aún) una solución a prueba de fallos para mejorar el rendimiento de
las aplicaciones web. Por ahora, el usar las APIs DOM de WebAssembly aún es más lento que llamarlas
directamente desde JavaScript. Este es un problema temporal el cual se pretende resolver con
[WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md).
Si quieres aprender más, revisa este [excelente artículo](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) (de Mozilla) 
el cual describe la propuesta.

#### De acuerdo, pero ¿por qué Rust?

Rust es increíblemente rápido y confiable con su rico sistema de tipos y su modelo de propiedad. Tiene una gran 
curva de aprendizaje pero bien vale el esfuerzo. Rust ha sido votado como el lenguaje de programación 
más amado en la encuesta a desarrolladores de Stack Overflow por cinco años consecutivos:
[2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted), 
[2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted), 
[2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages), 
[2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) 
y [2020](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted).

Rust también ayuda a los desarrolladores a escribir código más seguro con su rico sistema de tipos y modelo de propiedad. ¡Dile 
adiós a los bugs de condiciones de carrera difíciles de rastrear en JavaScript! De hecho, con Rust, la mayoría de tus 
bugs serán detectados por el compilador aún antes que tú aplicación se ejecute. Y no te preocupes, cuando se encuentre un 
error en tú aplicación, todavía puedes obtener rastros completos de la pila de errores para tú código Rust en la consola del navegador.

#### ¿Alternativas?

Amamos compartir ideas con otros proyectos y creemos que todos podemos ayudarnos entre sí para alcanzar 
nuestro máximo potencial de esta excitante y nueva tecnología. Si lo tuyo no es Yew, te podrían gustar 
los siguientes proyectos:

* [Percy](https://github.com/chinedufn/percy) - _"Una caja de herramientas modular para construir
aplicaciones web isomórficas con Rust + WebAssembly"_
* [Seed](https://github.com/seed-rs/seed) - _"Un framework de Rust para crear aplicaciones web"_
