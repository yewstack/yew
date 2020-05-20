# Example Servers

These servers allow you to serve your application from any non-api route.
This should prevent you from seeing 404 errors when refreshing your app after changing the route.


## Instructions

If you don't already have wasm-pack installed, run:
```shell script
cargo install wasm-pack
```

Change directories to your chosen example and run the following to build your wasm app.
```shell script
wasm-pack build --target web --out-dir ../static/ --out-name wasm
```

Change directories again to one of the servers in this directory and run `cargo run`.
They will serve files from the `static/` folder that lives in `examples/` at `localhost:8000`.


## As a template

You can use these as templates for your server, or incorporate them into an existing server.
You will likely have to change the assets dir constant to point elsewhere though.
```rust
    const ASSETS_DIR: &str = "your/assets/dir/here";
```

