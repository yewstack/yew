#!/usr/bin/env bash
set -x

# inspired by https://github.com/rust-analyzer/rust-analyzer/blob/master/.travis.yml
find ./target/debug -maxdepth 1 -type f -delete
find ./target/tests/target/debug -maxdepth 1 -type f -delete
find ./target/asmjs-unknown-emscripten/debug -maxdepth 1 -type f -delete
find ./target/wasm32-unknown-emscripten/debug -maxdepth 1 -type f -delete
find ./target/wasm32-unknown-unknown/debug -maxdepth 1 -type f -delete
rm -fr ./target/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./target/tests/target/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./target/asmjs-unknown-emscripten/debug/{deps,.fingerprint}/{*yew*,*\.was,*\.js*,*test*}
rm -fr ./target/wasm32-unknown-emscripten/debug/{deps,.fingerprint}/{*yew*,*\.was*,*\.js*,*test*}
rm -fr ./target/wasm32-unknown-unknown/debug/{deps,.fingerprint}/{*yew*,*\.was*,*\.js*,*test*}
rm -fr ./target/debug/incremental
rm -fr ./target/tests/target/debug/incremental
rm -fr ./target/asmjs-unknown-emscripten/debug/incremental
rm -fr ./target/wasm32-unknown-emscripten/debug/incremental
rm -fr ./target/wasm32-unknown-unknown/debug/incremental
rm -f  ./target/.rustc_info.json
rm -f  ./target/tests/target/.rustc_info.json
rm -fr ./target/wasm32-unknown-unknown/wbg-tmp
