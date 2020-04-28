# Yew Examples

Use `build.sh` script to build examples. 

Examples are prepared for `wasm-bindgen` expect `minimal_wp` which is prepared for `wasm-pack`.

## How to run

```
git clone https://github.com/yewstack/yew.git
cd yew
examples/build.sh minimal # example subfolder
python3 -m http.server 8000 --directory examples/static # open localhost:8000 in browser
```

Note: VSCode has extension "Live Server" which can be used to run example in browser with automatic reload

## Build size otimalization 

- It is possible to optimize code size using `wasm-opt` when building using `wasm-bindgen`
    - possible to enable opt when building `example/build.sh minimal --opt
- `wasm-pack` does that by default in release build

**Release size of example 'minimal'**

| release commands            | size ( uncompressed )
|---                          |---
| wasm-bindgen                | 158KB  
| wasm-binggen + wasm-opt -Os | 116KB 
| wasm-pack                   | 99 KB

## Known issues

- `multi_thread` not working correctly. 
