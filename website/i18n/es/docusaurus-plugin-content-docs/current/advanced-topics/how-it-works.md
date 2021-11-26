---
title: "Como funciona"
description: "Detalles de bajo nivel del framework"
---

# Librerías internas de bajo nivel

## El macro `html!` a fondo

La macro `html!` Convierte el código escrito en una sintaxis personalizada similar a HTML en un código Rust válido. Usar esta macro no es necesario para desarrollar aplicaciones de Yew, pero ssi recomendable. El código generado por esta macro hace uso de la API de la biblioteca pública Yew que puedes usar directamente si lo consideras oportuno. Fíjate que algunos métodos utilizados están indocumentados intencionalmente para evitar un uso indebido accidental. Con cada actualización de `yew-macro`, el código generado será más eficiente y manejará cualquier cambio importante sin muchas (si las hay) modificaciones a la sintaxis de `html!`.

Debido a que la macro `html!` te permite escribir código en un estilo declarativo, el código de diseño de la interfaz de usuario coincide estrechamente con el HTML que se genera en la página. Esto se vuelve cada vez más útil a medida que la aplicación se hace más interactiva y su código base se vuelve más grande. En lugar de escribir manualmente todo el código para manipular el DOM tú mismo, la macro lo maneja por tí.

Usar el macro de `html!` puede parecer magia, pero no tiene nada de misterioso. Si tienes curiosidad por ver como funciona, Prueba a expandir las llamadas a la macro `html!` en tu programa. Existe un comando muy útil que se llama `cargo expand` el cual te permite ver las expansiones de los macros Rust. `cargo expand` no está incluido en `cargo` por defecto, así que necesitarás instalarlo ejecutando `cargo install cargo-expand`, si no lo has hecho todavía.

Ten en cuenta que al ver el código de la macro expandido, es probable que encuentres un código inusualmente detallado. La razón es que el código generado a veces puede chocar con otro código en una aplicación. Para evitar problemas, se respeta la "higiene" de `proc_macro`. Algunos ejemplos incluyen:

1. En lugar de usar `yew :: <module>`, la macro genera `:: yew :: <module>` para asegurarse de que el paquete de Yew está referenciado correctamente. Esta es también la razón por la que se llama a `:: alloc :: vec :: Vec :: new ()` en vez de únicamente a `Vec :: new ()`.
2. Debido a posibles colisiones de nombres de métodos en *traits*, se usa `<Type as Trait>` para asegurarnos de que estamos usando elementos del *trait* correcto.

## Qué es un DOM virtual?

El DOM ("Document Object Model") es una representación del contenido HTML administrado por el navegador para tus páginas web. Un DOM "virtual" es simplemente una copia del DOM que se guarda en la memoria de la aplicación. Gestionar un DOM virtual da como resultado una sobrecarga de memoria más alta, pero permite el procesamiento por lotes y lecturas más rápidas al evitar o retrasar el uso de las API del navegador.

Tener una copia del DOM en la memoria puede ser realmente útil para las bibliotecas que promueven el uso de UI declarativas. En lugar de necesitar un código específico para describir cómo se debe modificar el DOM en respuesta a un evento de usuario, la biblioteca puede usar un enfoque generalizado basado en DOM "diffing". Cuando una componente de Yew se actualiza y quiere cambiar su renderizado, la biblioteca Yew creará una segunda copia del DOM virtual y la comparará directamente con el DOM virtual que se encuentra actualmente en pantalla. El "diff" (o diferencia) entre los dos se puede dividir en actualizaciones incrementales y aplicarse en un lote con la API del navegador. Una vez que se aplican las actualizaciones, la copia del DOM virtual anterior se descarta y se guarda una nueva copia para futuras comprobaciones.

Este algoritmo "diff" se puede optimizar con el tiempo para mejorar el rendimiento de aplicaciones complejas. Dado que las aplicaciones de Yew se ejecutan con WebAssembly, creemos que Yew tiene una ventaja competitiva para adoptar algoritmos más sofisticados en el futuro.

El DOM virtual de Yew no es exactamente idéntico al DOM del navegador. También incluye "listas" y "componentes" para organizar elementos DOM. Una lista puede ser simplemente una secuencia ordenada de elementos, pero puede también ser algo mucho más poderoso. Al anotar cada elemento de la lista con una "clave", los desarrolladores de aplicaciones puede ayudar a Yew a realizar optimizaciones adicionales para garantizar que cuando una lista cambia, la menor cantidad de trabajo se realiza para calcular la actualización de diferencias. Del mismo modo, los componentes proporcionan lógica personalizada para que se indique si es necesario volver a renderizar para optimizar el rendimiento.

## Yew scheduler y event loop en el ámbito de componentes

*Contribuye a la documentación – explica cómo `yew::scheduler` y `yew::html::scope` funcionan en detalle*

## Otras Lecturas
* [Más información sobre macron del libro oficial de Rust](https://doc.rust-lang.org/stable/book/ch19-06-macros.html)
* [Más información acerca de `cargo-expand`](https://github.com/dtolnay/cargo-expand)
* [Documentación de la API de `yew::virtual_dom`](https://docs.rs/yew/*/yew/virtual_dom/index.html)
