# Yew Examples

## Dependencies

The examples are built with [trunk](https://github.com/thedodd/trunk).
You can install it with the following command:

```bash
# at some point in the future, trunk should automatically download wasm-bindgen for you
cargo install trunk wasm-bindgen-cli
```

## Run an example

```bash
# download the source code.
git clone https://github.com/yewstack/yew.git

# move into the examples folder
cd yew/examples

# run the "todomvc" example
cd todomvc
trunk serve
```

Some examples will may require additional steps.
In those cases, instructions are placed in the example's README.md file.

## List of examples

| Example                                | Description                                                                                                                        |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [counter](counter)                     | A single component which displays a stateful number. The number can be incremented and decremented using buttons                   |
| [crm](crm)                             | See the `README` file for details                                                                                                  |
| [custom_components](custom_components) | Demonstrates the use of components                                                                                                 |
| [dashboard](dashboard)                 | Uses the `fetch` and `websocket` services to load external data                                                                    |
| [file_upload](file_upload)             | Uses the `reader` service to read the content of user uploaded files                                                               |
| [fragments](fragments)                 | Similar to the counter example but demonstrating the use of [fragments](https://yew.rs/docs/concepts/html/lists#fragments)         |
| [futures_wp](futures_wp)               | Demonstrates how you can use futures and async code with Yew. Features a Markdown renderer.                                        |
| [game_of_life](game_of_life)           | Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)                                   |
| [inner_html](inner_html)               | Embeds an external document as raw HTML by manually managing the element                                                           |
| [js_callback](js_callback)             | Interacts with JavaScript code                                                                                                     |
| [keyed_list](keyed_list)               | Demonstrates how to use keys to improve the performance of lists                                                                   |
| [minimal](minimal)                     | A simple button that listens to click events                                                                                       |
| [minimal_wp](minimal)                  | Same as the minimal example but using `wasm-pack`                                                                                  |
| [mount_point](mount_point)             | Shows how to mount the root component to a custom element                                                                          |
| [multi_thread](multi_thread)           | Demonstrates the use of Web Workers to offload computation to the background                                                       |
| [nested_list](nested_list)             | Renders a styled list which tracks hover events                                                                                    |
| [node_refs](node_refs)                 | Uses a [`NodeRef`](https://yew.rs/docs/concepts/components/refs) to focus the input element under the cursor                       |
| [npm_and_rest](npm_and_rest)           | A more elaborate demonstration of the `fetch` service                                                                              |
| [pub_sub](pub_sub)                     | Cross-component communication using [Agents](https://yew.rs/docs/concepts/agents)                                                  |
| [store](store)                         | Showcases the `yewtil::store` API                                                                                                  |
| [textarea](textarea)                   | Shows how to use the value of a textarea or input tag                                                                              |
| [timer](timer)                         | Demonstrates the use of the interval and timeout services                                                                          |
| [todomvc](todomvc)                     | Implementation of the [TodoMVC](http://todomvc.com/) app                                                                           |
| [two_apps](two_apps)                   | Runs two separate Yew apps at the same time                                                                                        |
| [webgl](webgl)                         | Controls a [WebGL canvas](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL) from Yew |

## Next steps

Have a look at Yew's [starter templates](https://yew.rs/docs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.

## Help out

Most examples have an "improvements" section in their README.md which lists ways to improve the example.

The biggest point of improvement is the presentation of the examples (ex. styling).
