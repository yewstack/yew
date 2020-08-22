# Yew Examples

In order to run the examples, we provide the `run_example.sh` script.
This script takes care of compiling the example and runs a web server for you.
All that's left is for you to play around with the examples :).

> **A note for Windows users:**<br>
> Depending on how you installed `git` you will already have a bash emulator at your disposal. This allows you to run the `run_example.sh` script (and any other bash script) normally.<br>
> See <https://gitforwindows.org/#bash> for more information.
>
> We're always trying to improve the developer experience for developers across all platforms.
> There's an ongoing effort to replace the bash scripts with a Rust command line tool ([#1418](https://github.com/yewstack/yew/issues/1418)).
> If at any point you encounter an issue, don't hesitate to ask a question or open an issue.

## Dependencies

Before we can run the examples we need to get a few things ready.

Some examples currently use `wasm-bindgen` and others use `wasm-pack`.
You can install them both using the following command:

```bash
cargo install wasm-pack wasm-bindgen-cli
```

### Optional dependencies

We've written a small web server which you can use to serve the built examples. In order to
use it, you'll need to have installed Python (3.6 or greater).
You can also use a different web server, provided that it can serve static files from a directory.

> **Note:**<br>
> Some examples don't have an `index.html` file in their static directory.
> The python web server handles this by serving a default index file.
> If you aren't using it, you will need to create the index file manually.

One alternative to the built-in web server there is an extension for [Visual Studio Code](https://code.visualstudio.com/) called [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer).
This extension is used to serve static files.
To do so, open the `index.html` file in the static directory of the example you wish to run and press "Open with Live Server" in the context menu.

## Run an example

```bash
# download the source code.
git clone https://github.com/yewstack/yew.git

# move into the examples folder
cd yew/examples

# run the "todomvc" example
./run_example.sh todomvc
```

## Script options

The general structure of the command looks like this:<br>
`./run_example.sh <example> [OPTIONS]`

`<example>` is the name of the example (i.e. the name of the directory).

The following table describes all possible options:

| Option                   | Description                                                                                                         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `--debug`<br>`--release` | Specifies which profile to use for `cargo build`. Defaults to `--debug`.                                            |
| `--build-only`           | Disables the built-in server.<br>Use this if you don't have Python installed or if you want to use your own server. |

## List of examples

| Example                                | Description                                                                                                                        | Has README |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | :--------: |
| [counter](counter)                     | A single component which displays a stateful number. The number can be incremented and decremented using buttons                   |     ✖      |
| [crm](crm)                             | See the `README` file for details                                                                                                  |     ✔      |
| [custom_components](custom_components) | Demonstrates the use of components                                                                                                 |     ✔      |
| [dashboard](dashboard)                 | Uses the `fetch` and `websocket` services to load external data                                                                    |     ✔      |
| [file_upload](file_upload)             | Uses the `reader` service to read the content of user uploaded files                                                               |     ✖      |
| [fragments](fragments)                 | Similar to the counter example but demonstrating the use of [fragments](https://yew.rs/docs/concepts/html/lists#fragments)         |     ✖      |
| [futures_wp](futures_wp)               | Demonstrates how you can use futures and async code with Yew                                                                       |     ✔      |
| [game_of_life](game_of_life)           | Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)                                   |     ✖      |
| [inner_html](inner_html)               | Embeds an external document as raw HTML by manually managing the element                                                           |     ✖      |
| [js_callback](js_callback)             | Interacts with JavaScript code                                                                                                     |     ✔      |
| [keyed_list](keyed_list)               | Demonstrates how to use keys to improve the performance of lists                                                                   |     ✖      |
| [large_table](large_table)             | Renders a big table which highlights the selected cell                                                                             |     ✖      |
| [minimal](minimal)                     | A simple button that listens to click events                                                                                       |     ✖      |
| [minimal_wp](minimal)                  | Same as the minimal example but using `wasm-pack`                                                                                  |     ✖      |
| [mount_point](mount_point)             | Shows how to mount the root component to a custom element                                                                          |     ✖      |
| [multi_thread](multi_thread)           | Demonstrates the use of Web Workers to offload computation to the background                                                       |     ✔      |
| [nested_list](nested_list)             | Renders a styled list which tracks hover events                                                                                    |     ✖      |
| [node_refs](node_refs)                 | Uses a [`NodeRef`](https://yew.rs/docs/concepts/components/refs) to focus the input element under the cursor                       |     ✖      |
| [npm_and_rest](npm_and_rest)           | A more elaborate demonstration of the `fetch` service                                                                              |     ✖      |
| [pub_sub](pub_sub)                     | Cross-component communication using [Agents](https://yew.rs/docs/concepts/agents)                                                  |     ✔      |
| [store](store)                         | Interacts with the [MediaDevices](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices) API                               |     ✖      |
| [textarea](textarea)                   | Shows how to use the value of a textarea or input tag                                                                              |     ✖      |
| [timer](timer)                         | Demonstrates the use of the interval and timeout services                                                                          |     ✖      |
| [todomvc](todomvc)                     | Implementation of the [TodoMVC](http://todomvc.com/) app                                                                           |     ✔      |
| [two_apps](two_apps)                   | Runs two separate Yew apps at the same time                                                                                        |     ✖      |
| [webgl](webgl)                         | Controls a [WebGL canvas](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL) from Yew |     ✔      |

## Next steps

Have a look at Yew's [starter templates](https://yew.rs/docs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.
