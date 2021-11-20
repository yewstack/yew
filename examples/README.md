# Yew Examples

## How to run

The examples are built with [trunk](https://github.com/thedodd/trunk).
You can install it with the following command:

```bash
# at some point in the future, trunk will automatically download wasm-bindgen
cargo install trunk wasm-bindgen-cli
```

Running an example is as easy as running a single command:

```bash
# move into the directory of the example you want to run
# In this case it's the todomvc example
cd examples/todomvc

# build and serve the example
trunk serve --release
```

We're also publicly hosting the examples at `https://examples.yew.rs/<EXAMPLE>`.
As an example, check out the TodoMVC example here: <https://examples.yew.rs/todomvc>

## List of examples

| Example                                             | Description                                                                                                                        |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [boids](boids)                                      | Yew port of [Boids](https://en.wikipedia.org/wiki/Boids)                                                                           |
| [counter](counter)                                  | Simple counter which can be incremented and decremented                                                                            |
| [crm](crm)                                          | Shallow customer relationship management tool                                                                                      |
| [dyn_create_destroy_apps](dyn_create_destroy_apps)  | Uses the function `start_app_in_element` and the `AppHandle` struct to dynamically create and delete Yew apps                      |
| [file_upload](file_upload)                          | Uses the `gloo::file` to read the content of user uploaded files                                                               |
| [function_todomvc](function_todomvc)                | Implementation of [TodoMVC](http://todomvc.com/) using function components and hooks.                                              |
| [futures](futures)                                  | Demonstrates how you can use futures and async code with Yew. Features a Markdown renderer.                                        |
| [game_of_life](game_of_life)                        | Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)                                   |
| [inner_html](inner_html)                            | Embeds an external document as raw HTML by manually managing the element                                                           |
| [js_callback](js_callback)                          | Interacts with JavaScript code                                                                                                     |
| [keyed_list](keyed_list)                            | Demonstrates how to use keys to improve the performance of lists                                                                   |
| [mount_point](mount_point)                          | Shows how to mount the root component to a custom element                                                                          |
| [multi_thread](multi_thread)                        | Demonstrates the use of Web Workers to offload computation to the background                                                       |
| [nested_list](nested_list)                          | Renders a styled list which tracks hover events                                                                                    |
| [node_refs](node_refs)                              | Uses a [`NodeRef`](https://yew.rs/concepts/components/refs) to focus the input element under the cursor                       |
| [pub_sub](pub_sub)                                  | Cross-component communication using [Agents](https://yew.rs/concepts/agents)                                                  |
| [router](router)                                    | The best yew blog built with `yew-router`                                                                                          |
| [store](store)                                      | Showcases the `yewtil::store` API                                                                                                  |
| [timer](timer)                                      | Demonstrates the use of the interval and timeout services                                                                          |
| [todomvc](todomvc)                                  | Implementation of [TodoMVC](http://todomvc.com/)                                                                                   |
| [two_apps](two_apps)                                | Runs two separate Yew apps which can communicate with each other                                                                   |
| [webgl](webgl)                                      | Controls a [WebGL canvas](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL) from Yew |

## Next steps

Have a look at Yew's [starter templates](https://yew.rs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.

## Help out

If one of the examples catches your interest, look for the "improvements" section in its `README` file.
Most examples list a few ideas for how to improve them.
Consider starting with those but don't hesitate to improve an example in other ways either.

One problem that currently plagues most examples is the lack of styling.
Please help us make the examples look as flashy as possible!
