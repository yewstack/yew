---
título: "Optimizaciones y mejores prácticas"
sidebar_label: Optimizaciones
description: "Haz que tu aplicación sea más rápida"
---

## Usando punteros inteligentes de forma efectiva

**Nota: si no estás seguro de algunos de los términos utilizados en esta
sección, el Libro de Rust tiene un útil
[capítulo sobre punteros inteligentes](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).**

En un esfuerzo por evitar clonar grandes cantidades de datos para crear props
cuando se vuelve a renderizar, podemos usar para clonar sólo una referencia a
los datos en lugar de los datos mismos. Si pasas referencias a los datos
relevantes en tus props y componentes hijos en lugar de los datos reales puedes
evitar clonar cualquier dato hasta que necesites modificarlo en el componente
hijo, donde puede utilizar `Rc::make_mut` para clonar y obtener una referencia
mutable a los datos que desea modificar.

Esto aporta más ventajas en `Component::changed` a la hora de averiguar si los
cambios de props requieren que el componente se vuelva a renderizar. Esto se
debe a que en lugar de comparar el valor de los datos de los
punteros subyacentes (es decir, la posición en la memoria de la máquina donde se
almacenan los datos) pueden seguir siendo comparados, el valor
de los datos a los que apuntan debe ser el mismo. Tenga en
cuenta que lo contrario podría no ser cierto. Incluso si dos direcciones de
punteros difieren, los datos subyacentes pueden ser los mismos - en
este caso debe comparar los datos subyacentes.

Para hacer esta comparación tendrá que usar `Rc::ptr_eq` en lugar de usar
simplemente `PartialEq` (que se automáticamente cuando se comparan datos
utilizando el operador de igualdad `==`). La documentación de Rust tiene
[más detalles sobre `Rc::ptr_eq`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq).

Esta optimización es más útil para los tipos de datos que no implementan `Copy`.
Si puedes copiar tus datos de forma eficiente, entonces no vale la pena ponerlos
detrás de un puntero inteligente. Para estructuras que pueden tener muchos
datos, como `Vec`s, `HashMap`s, y `String`s, el uso de punteros inteligentes es
probable que traiga mejoras en el rendimiento.

Esta optimización funciona mejor si los valores nunca son actualizados por los
hijos, e incluso mejor, si si los padres rara vez los actualizan. Esto hace que
`Rc<_>s` sea una buena opción para envolver los valores de las propiedades en
los componentes puros.

Sin embargo, hay que tener en cuenta que a menos que necesites clonar los datos
tú mismo en el componente hijo, esta optimización no sólo es inútil, sino que
además añade un coste innecesario de recuento de referencias. Los objetos en Yew
ya están contados por referencia y no se producen clones de datos internamente.

## Funciones de la vista

Por razones de legibilidad del código, a menudo tiene sentido migrar secciones
de `html!` a sus propias funciones. Esto no sólo hace que su código sea más
legible porque reduce la cantidad de indentación presente, sino que también
fomenta buenos patrones de diseño - particularmente en torno a la construcción
de especialmente en lo que se refiere a la construcción de aplicaciones
componibles, ya que estas funciones pueden ser llamadas en múltiples lugares, lo
que reduce la cantidad de código que hay que escribir. cantidad de código que
tiene que ser escrito.

## Componentes puros

Los componentes puros son componentes que no mutan su estado, sólo muestran el
contenido y propagando los mensajes a los componentes normales y mutables. Se
diferencian de las funciones de vista en que pueden usarse desde la macro
`html!` usando la sintaxis del componente \(`<AlgúnComponentePuro />`) en lugar
de la sintaxis de expresión \(`{alguna_función_de_vista()}`), y que, dependiendo
de su implementación, pueden ser memoized (esto significa que una vez que una
función se llama su valor se "guarda" de modo que si se llama con los mismos
argumentos más de una vez no tiene que volver a calcular su valor y puede
simplemente devolver el valor guardado de la primera llamada a la función), lo
que evita que se vuelvan a idénticos. Yew compara los props internamente y así
la UI sólo se re renderiza si los props cambian.

## Reducir el tiempo de compilación usando espacios de trabajo

Posiblemente, el mayor inconveniente de usar Yew es el largo tiempo que se tarda
en compilar las aplicaciones de Yew. El tiempo El tiempo que se tarda en
compilar un proyecto parece estar relacionado con la cantidad de código que se
pasa a la macro `html!`. Esto no suele ser un problema para los proyectos
pequeños, pero para las aplicaciones más grandes tiene sentido dividir el código
en varias cajas. tiene sentido dividir el código en varias cajas para minimizar
la cantidad de trabajo que el compilador tiene que hacer para cada cambio
realizado en la aplicación.

Un posible enfoque es hacer que su caja principal maneje la selección de
rutas/páginas, y luego hacer una caja diferente para cada página. diferente para
cada página, donde cada página podría ser un componente diferente, o simplemente
una gran función que produce `Html`. El código que se comparte entre los crates
que contienen diferentes partes de de la aplicación puede ser almacenado en una
caja separada de la que se depende en todo el proyecto. En el mejor de los
casos, se pasa de reconstruir todo el código en cada compilación a reconstruir
sólo el crate principal, y uno de sus crates de página. En el peor de los casos,
cuando se edita algo en el "común", volverá al punto de partida: la compilación
de todo el código que depende de ese que depende de ese crate común, que es
probablemente todo lo demás.

Si su caja principal es demasiado pesada, o quiere iterar rápidamente en una
página profundamente anidada (por ejemplo (por ejemplo, una página que se
renderiza encima de otra página), puede utilizar una caja de ejemplo para crear
una implementación simplificada de la página principal y renderizarla.
simplificada de la página principal y renderizar el componente en el que está
trabajando.

## Reducir el tamaño de los binarios

- optimizar el código Rust
  - `wee_alloc` \( usando el asignador minúsculo \)
  - `cargo.toml` ( definiendo el perfil de liberación)
- optimizar el código de Rust usando `wasm-opt`.

**Nota: se puede encontrar más información sobre la reducción del tamaño de los
binarios en el
[Rust Wasm Book](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size).**

### wee_alloc

[wee_alloc](https://github.com/rustwasm/wee_alloc) es un pequeño asignador que
es mucho más pequeño que el asignador que se utiliza normalmente en los binarios
de Rust. Reemplazar el asignador por defecto por este dará como resultado
archivos Wasm más pequeños más pequeños, a expensas de la velocidad y la
sobrecarga de memoria.

La velocidad más lenta y la sobrecarga de memoria son menores en comparación con
las ganancias de tamaño que se obtiene al no incluir el asignador por defecto.
Este menor tamaño de archivo significa que que su página se cargará más rápido,
por lo que generalmente se recomienda utilizar este sobre el predeterminado, a
menos que su aplicación esté haciendo algún trabajo pesado de asignación.

`` rust ,ignore // Usar `wee_alloc` como asignador global. #[global_allocator] static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; ``

### Cargo.toml

Es posible configurar las compilaciones de la versión para que sean más pequeñas
utilizando los ajustes disponibles en la sección `[profile.release]` de su
archivo `Cargo.toml`. en la sección `[profile.release]` de su `Cargo.toml`.

```texto
[profile.release]
# menos código para incluir en el binario
panic = 'abort'
# optimización sobre todo el código base ( mejor optimización, construcción más lenta )
codegen-units = 1
# optimización para el tamaño ( más agresivo )
opt-level = 'z'
# optimización para el tamaño
# opt-level = 's'
# optimización del tiempo de enlace utilizando el análisis de todo el programa
lto = true
```

### wasm-opt

Además es posible optimizar el tamaño del código `wasm`.

El libro de Rust Wasm tiene una sección sobre cómo reducir el tamaño de los
binarios Wasm:
[Reducir el tamaño del .wasm](https://rustwasm.github.io/book/game-of-life/code-size.html)

- usando `wasm-pack` que por defecto optimiza el código `wasm` en las
  compilaciones de lanzamiento
- usando `wasm-opt` directamente en los archivos `wasm`.

```texto
wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm
```

#### Construir el tamaño del ejemplo 'mínimo' en yew/examples/

Nota: `wasm-pack` combina la optimización del código Rust y Wasm. El código
`wasm-bindgen` se utiliza en este ejemplo sin ninguna optimización del tamaño de
Rust.

| used tool                   | size  |
| :-------------------------- | :---- |
| wasm-bindgen                | 158KB |
| wasm-bindgen + wasm-opt -Os | 116KB |
| wasm-pack                   | 99 KB |

## Lectura adicional

- [El capítulo de The Rust Book sobre punteros inteligentes](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Información del Libro de Rust sobre la reducción del tamaño de los binarios](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)
- [Documentación sobre los perfiles de Rust](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Proyecto binaryen](https://github.com/WebAssembly/binaryen)
