# Example Servers

These servers allow you to serve your application from any non-api route.
This should prevent you from seeing 404 errors when refreshing your app after changing the route.

The servers rely on having the project already deployed by cargo-web, with some modifications to the output to ensure correctness.

## Instructions

Run `cargo web deploy` from the `/examples/router_component` folder to build the project that will be served by a server here.
Then, navigate to the `/target/deploy/` directory and run:
```sh
sed -i 's/router_component.wasm/\/router_component.wasm/g' router_component.js
```
and
```sh
sed -i 's/router_component.js/\/router_component.js/g' index.html
```
If these commands aren't ran, then the server will serve the wrong files when it tries to get a compound path.
For example, if you request "/some/path" because it will serve `index.html`'s content, but then the request for the "router_component.js" it makes will attempt to get it from "/some/router_component.js", which doesn't exist.
These replacements make the browser get the file from the absolute path, so it will always get the correct item.

Then go to your chosen server and run `cargo run` to start it.

## As a template

You can use these as templates for your server, or incorporate them into an existing server.
You will likely have to change the assets dir constant to point elsewhere though.
```rust
    const ASSETS_DIR: &str = "your/assets/dir/here";
```

### Non-cargo-web
The instructions for using `sed` and `cargo-web` above are irrelevant if you use these server templates for apps built with parcel or webpack.
