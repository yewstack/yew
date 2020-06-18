## Yew TodoMVC Demo

This is an implementation of [TodoMVC](http://todomvc.com/) app.

Unlike other implementations, this stores the full state of the model,
including: all entries, entered text and chosen filter.

## Running

cargo install wasm-pack  
wasm-pack build --target web --out-name wasm --out-dir ./static  
cargo +nightly install miniserve  
miniserve ./static --index index.html  
