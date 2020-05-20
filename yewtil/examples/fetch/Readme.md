Shows off ergonomic JSON deserialization fetch abstraction.

Run with:

```shell script
wasm-pack build --target web && rollup ./main.js --format iife --file ./pkg/bundle.js && python -m SimpleHTTPServer 8080
```

It is expected that you have a setup with wasm-pack, rollup, and python installed.
