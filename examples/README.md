# Yew Examples

<!-- TODO: add a table explaining the purpose of each example -->

In order to run the examples, we provide the `run_example.sh` script.
This script takes care of compiling the example and runs a web server for you.
All that's left for you to do is to play around with the examples :).

> **A note for Windows users:**
>
> Depending on how you installed `git` you will already have a bash emulator at your disposal. This allows you to run the `run_example.sh` script (and any other bash script) normally. <br>
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

There's an optional dependency for [Python](https://www.python.org/) (version 3.6 and up).
Python is required to run the web server.
If you don't have it installed you will need to manually start a server for the generated files.

> **Note:**
>
> Some examples don't have an `index.html` file in their static directory.
> The python web server handles this by serving a default index file.
> If you aren't using it, you will need to create the index file manually.

As an alternative to the built-in web server there is an extension for [Visual Studio Code](https://code.visualstudio.com/) called [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer).
This extension is used to serve static files.
To do so, open the `index.html` file in the static directory of the example you wish to run and press "Open with Live Server" in the context menu.

## Quickstart

```bash
# download the source code
git clone https://github.com/yewstack/yew.git

# move into the examples folder
cd yew/examples

# run the "todomvc" example
./run_example.sh todomvc
```

## Script options

The `run_example.sh` script understands a few options.

| Option                   | Description                                                                                                         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `--debug`<br>`--release` | Specifies which profile to use for `cargo build`. Defaults to `--debug`.                                              |
| `--build-only`           | Disables the built-in server.<br>Use this if you don't have Python installed or if you want to use your own server. |

## Next steps

Have a look at Yew's [starter templates](https://yew.rs/docs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.
